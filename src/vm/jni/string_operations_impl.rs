use crate::vm::execution_engine::executor::Executor;
use crate::vm::heap::heap::HEAP;
use crate::vm::jni::array_operations_impl::{
    new_byte_array, new_char_array, set_byte_array_region, set_char_array_region,
};
use crate::vm::jni::jni_invoke::jni_invoke_scalar;
use crate::vm::method_area::loaded_classes::CLASSES;
use crate::vm::system_native::string::get_raw_string_info;
use crate::{from_mutf8_ptr, to_mutf8_data};
use cesu8::to_java_cesu8;
use jni_sys::{jboolean, jbyte, jchar, jint, jlong, jsize, jstring, JNIEnv, JNI_TRUE};
use std::ffi::c_char;
use std::ptr;

/// Encode a slice of UTF-16 code units directly to Java's modified UTF-8 (CESU-8/MUTF-8).
/// This handles surrogate pairs and isolated surrogates correctly per JNI spec.
fn encode_utf16_to_modified_utf8(utf16: &[u16]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut i = 0;

    while i < utf16.len() {
        let c = utf16[i];

        // Check for surrogate
        if (0xD800..=0xDBFF).contains(&c) {
            // High surrogate - try to pair with next code unit
            if i + 1 < utf16.len() {
                let next = utf16[i + 1];
                if (0xDC00..=0xDFFF).contains(&next) {
                    // Valid surrogate pair - encode combined supplementary char
                    let high = (c as u32 - 0xD800) << 10;
                    let low = (next as u32) - 0xDC00;
                    let codepoint = high + low + 0x10000;
                    encode_codepoint_to_mutf8(codepoint, &mut result);
                    i += 2;
                    continue;
                }
            }
            // Isolated high surrogate - encode as individual code point
            encode_codepoint_to_mutf8(c as u32, &mut result);
        } else if (0xDC00..=0xDFFF).contains(&c) {
            // Isolated low surrogate - encode as individual code point
            encode_codepoint_to_mutf8(c as u32, &mut result);
        } else {
            // Regular BMP character
            encode_codepoint_to_mutf8(c as u32, &mut result);
        }
        i += 1;
    }

    result
}

/// Encode a single Unicode code point to modified UTF-8 bytes.
fn encode_codepoint_to_mutf8(codepoint: u32, buf: &mut Vec<u8>) {
    if codepoint == 0x0000 {
        // Modified UTF-8 encodes null as two bytes
        buf.push(0xC0);
        buf.push(0x80);
    } else if codepoint <= 0x007F {
        // ASCII
        buf.push(codepoint as u8);
    } else if codepoint <= 0x07FF {
        // 2-byte sequence
        buf.push(0xC0 | ((codepoint >> 6) as u8 & 0x1F));
        buf.push(0x80 | (codepoint as u8 & 0x3F));
    } else if codepoint <= 0xFFFF {
        // 3-byte sequence (BMP, including isolated surrogates)
        buf.push(0xE0 | ((codepoint >> 12) as u8 & 0x0F));
        buf.push(0x80 | ((codepoint >> 6) as u8 & 0x3F));
        buf.push(0x80 | (codepoint as u8 & 0x3F));
    } else {
        // Supplementary character - 6-byte CESU-8 sequence (not standard UTF-8!)
        // This encodes the supplementary char as if it were two surrogates.
        // The 0xA0/0xB0 bases encode the fixed upper bits of surrogate ranges.
        let high = 0xD800u32 | ((codepoint - 0x10000) >> 10);
        let low = 0xDC00u32 | ((codepoint - 0x10000) & 0x3FF);

        // high ∈ [0xD800,0xDBFF]: byte2 must be 0xA0-0xAF
        buf.push(0xED);
        buf.push(0xA0 | ((high >> 6) as u8 & 0x0F)); // bits [9:6], base 0xA0
        buf.push(0x80 | (high as u8 & 0x3F));

        // low ∈ [0xDC00,0xDFFF]: byte2 must be 0xB0-0xBF
        buf.push(0xED);
        buf.push(0xB0 | ((low >> 6) as u8 & 0x0F)); // bits [9:6], base 0xB0
        buf.push(0x80 | (low as u8 & 0x3F));
    }
}

pub(super) extern "system" fn get_string_length(_env: *mut JNIEnv, input: jstring) -> jint {
    let string_ref = input as i32;

    if string_ref == 0 {
        panic!("Invalid string reference"); // OpenJDK crashes here, why we shouldn't
    }

    let raw =
        Executor::invoke_non_static_method("java/lang/String", "length:()I", string_ref, &[])
            .unwrap_or(vec![0]); // OpenJDK returns 0 for non-strings

    raw[0] as jint
}

pub(super) extern "system" fn new_string(
    env: *mut JNIEnv,
    unicode: *const jchar,
    len: jsize,
) -> jstring {
    if len < 0 {
        panic!("negative array size"); // todo throw NegativeArraySizeException here
    }
    if unicode.is_null() && len > 0 {
        panic!("unicode array is null but length is {len}");
    }
    let arr_ref = new_char_array(env, len);
    set_char_array_region(env, arr_ref, 0, len, unicode);
    let result = Executor::invoke_args_constructor(
        "java/lang/String",
        "<init>:([C)V",
        &[(arr_ref as i32).into()],
        Some(""),
    );
    jni_invoke_scalar(result, "NewString")
}

pub(super) extern "system" fn get_string_chars(
    _env: *mut JNIEnv,
    from: jstring,
    is_copy: *mut jboolean,
) -> *const jchar {
    let string_ref = from as i32;
    if string_ref == 0 {
        panic!("Invalid string reference: null");
    }

    let raw_data = get_string_raw_data(string_ref);
    let boxed_slice = raw_data.into_boxed_slice();
    let raw_ptr = Box::into_raw(boxed_slice) as *const u8 as *const jchar;

    if !is_copy.is_null() {
        unsafe {
            *is_copy = JNI_TRUE; // we always return a copy
        }
    }

    raw_ptr
}

fn get_string_raw_data(string_ref: i32) -> Vec<jchar> {
    let (is_latin, array_ref) = match get_raw_string_info(string_ref) {
        Ok(info) => info,
        Err(e) => panic!("Failed to get raw string info: {e}"),
    };
    let raw_data = if is_latin {
        let guard = match HEAP.get_entire_raw_data(array_ref) {
            Ok(g) => g,
            Err(e) => panic!("Failed to get string data: {e}"),
        };
        guard.iter().map(|x| *x as jchar).collect::<Vec<_>>()
    } else {
        let guard = match HEAP.get_entire_raw_data(array_ref) {
            Ok(g) => g,
            Err(e) => panic!("Failed to get string data: {e}"),
        };
        if guard.len() % 2 != 0 {
            panic!("Invalid UTF16 String data");
        }
        guard
            .chunks_exact(2)
            .map(|chunk| u16::from_ne_bytes([chunk[0], chunk[1]]))
            .collect::<Vec<_>>()
    };
    raw_data
}

pub(super) extern "system" fn release_string_chars(
    _env: *mut JNIEnv,
    str: jstring,
    chars: *const jchar,
) {
    let string_ref = str as i32;
    if string_ref == 0 {
        panic!("Invalid string reference: null");
    }

    if chars.is_null() {
        return;
    }

    let (is_latin, array_ref) = get_raw_string_info(string_ref)
        .unwrap_or_else(|_| panic!("Failed to get raw string info for string_ref={string_ref}"));
    let byte_len = HEAP
        .get_entire_raw_data(array_ref)
        .unwrap_or_else(|_| panic!("Failed to get string data for array_ref={array_ref}"))
        .len();
    let len = if is_latin { byte_len } else { byte_len / 2 };
    unsafe {
        let _boxed: Box<_> =
            Box::from_raw(ptr::slice_from_raw_parts_mut(chars as *mut jchar, len));
    }
}

pub(super) extern "system" fn new_string_utf8(
    env: *mut JNIEnv,
    mutf8_bytes: *const c_char,
) -> jstring {
    if mutf8_bytes.is_null() {
        panic!("modified utf-8 array is null");
    }

    let decoded = match from_mutf8_ptr!(mutf8_bytes) {
        Ok(d) => d,
        Err(e) => panic!("Failed to decode modified UTF-8 bytes: {e}"),
    };
    let decoded_bytes = decoded.as_bytes();

    let len = decoded_bytes.len() as jsize;
    let byte_array = new_byte_array(env, len);
    set_byte_array_region(
        env,
        byte_array,
        0,
        len,
        decoded_bytes.as_ptr() as *const jbyte,
    );

    let utf8_charset_ref = {
        let class = match CLASSES.get("java/nio/charset/StandardCharsets") {
            Ok(c) => c,
            Err(e) => panic!("Failed to get StandardCharsets class: {e}"),
        };
        let field = match class.static_field("UTF_8") {
            Some(f) => f,
            None => panic!("Failed to get UTF_8 field"),
        };
        match field.raw_value() {
            Ok(v) => v[0],
            Err(e) => panic!("Failed to get UTF_8 value: {e}"),
        }
    };

    let result = Executor::invoke_args_constructor(
        "java/lang/String",
        "<init>:([BLjava/nio/charset/Charset;)V",
        &[(byte_array as i32).into(), utf8_charset_ref.into()],
        Some(""),
    );
    jni_invoke_scalar(result, "NewStringUTF")
}

pub(super) extern "system" fn get_string_utf_length_as_long(
    _env: *mut JNIEnv,
    input: jstring,
) -> jlong {
    let string_ref = input as i32;

    if string_ref == 0 {
        panic!("Invalid string reference"); // OpenJDK crashes here, why we shouldn't
    }

    let raw_data = get_string_raw_data(string_ref);
    let data = match String::from_utf16(&raw_data) {
        Ok(s) => s,
        Err(e) => panic!("Failed to build string from UTF-16 data: {e}"),
    };
    to_java_cesu8(&data).len() as jlong
}

pub(super) extern "system" fn get_string_utf_length(_env: *mut JNIEnv, input: jstring) -> jint {
    get_string_utf_length_as_long(_env, input) as jint
}

pub(super) extern "system" fn get_string_utf_chars(
    _env: *mut JNIEnv,
    from: jstring,
    is_copy: *mut jboolean,
) -> *const c_char {
    let string_ref = from as i32;
    if string_ref == 0 {
        panic!("Invalid string reference: null");
    }

    let mutf8_data = to_mutf8_data!(string_ref);
    let boxed_slice = mutf8_data.into_boxed_slice();
    let raw_ptr = Box::into_raw(boxed_slice) as *const u8 as *const c_char;

    if !is_copy.is_null() {
        unsafe {
            *is_copy = JNI_TRUE; // we always return a copy
        }
    }

    raw_ptr
}

pub(super) extern "system" fn release_string_utf_chars(
    env: *mut JNIEnv,
    str: jstring,
    chars: *const c_char,
) {
    let string_ref = str as i32;
    if string_ref == 0 {
        panic!("Invalid string reference: null");
    }

    if chars.is_null() {
        return;
    }

    let len = get_string_utf_length_as_long(env, str) as usize + 1/*null terminator*/;
    unsafe {
        let _boxed: Box<_> = Box::from_raw(ptr::slice_from_raw_parts_mut(chars as *mut u8, len));
    }
}

/// GetStringRegion: Copies a region of the string into the provided buffer.
/// Throws StringIndexOutOfBoundsException if the range is invalid.
pub(super) extern "system" fn get_string_region(
    _env: *mut JNIEnv,
    str: jstring,
    start: jsize,
    len: jsize,
    buf: *mut jchar,
) {
    if str.is_null() || buf.is_null() {
        panic!("Null pointer in GetStringRegion"); // todo throw NullPointerException
    }

    let string_len = get_string_length(_env, str) as jsize;

    // Check bounds (using overflow-safe comparison)
    if start < 0 || len < 0 || start > string_len || len > string_len - start {
        panic!("Invalid string region: start={start}, len={len}, string_len={string_len}");
        // todo throw StringIndexOutOfBoundsException
    }

    // Get the raw UTF-16 data
    let string_ref = str as i32;
    let raw_data = get_string_raw_data(string_ref);

    // Copy the region to the buffer
    unsafe {
        let src_slice = &raw_data[start as usize..start as usize + len as usize];
        let dst_slice = std::slice::from_raw_parts_mut(buf, len as usize);
        dst_slice.copy_from_slice(src_slice);
    }
}

/// GetStringUTFRegion: Copies a region of the string as modified UTF-8 into the provided buffer.
/// Throws StringIndexOutOfBoundsException if the range is invalid.
pub(super) extern "system" fn get_string_utf_region(
    _env: *mut JNIEnv,
    str: jstring,
    start: jsize,
    len: jsize,
    buf: *mut c_char,
) {
    if str.is_null() || buf.is_null() {
        panic!("Null pointer in GetStringUTFRegion"); // todo throw NullPointerException
    }

    let string_len = get_string_length(_env, str) as jsize;

    // Check bounds (using overflow-safe comparison)
    if start < 0 || len < 0 || start > string_len || len > string_len - start {
        panic!("Invalid string region: start={start}, len={len}, string_len={string_len}");
        // todo throw StringIndexOutOfBoundsException
    }

    // Get the raw UTF-16 data and extract the region
    let string_ref = str as i32;
    let raw_data = get_string_raw_data(string_ref);
    let region = &raw_data[start as usize..start as usize + len as usize];

    // Encode directly to modified UTF-8 without round-tripping through Rust String.
    // This handles isolated surrogates and split surrogate pairs correctly per JNI spec.
    let mutf8_data = encode_utf16_to_modified_utf8(region);

    // Copy to the caller's buffer: mutf8_data.len() bytes plus a null terminator.
    // The caller should size buf via GetStringUTFLength on the same region, since
    // the modified UTF-8 encoding may produce more bytes than the number of UTF-16 code units.
    unsafe {
        let dst_slice = std::slice::from_raw_parts_mut(buf as *mut u8, mutf8_data.len() + 1);
        dst_slice[..mutf8_data.len()].copy_from_slice(&mutf8_data);
        dst_slice[mutf8_data.len()] = 0; // null terminator
    }
}

/// GetStringCritical: Returns a pointer to the string's characters.
/// This is similar to GetStringChars but signals that a GC-critical section has started.
/// Since there's no GC yet in rusty-jvm, this is just a wrapper around GetStringChars.
/// TODO(GC): When GC is implemented, this must:
///   1. Pin the string object to prevent it from being moved during collection
///   2. Disable GC for the duration of the critical section
///   3. Return a direct pointer to the string's internal char array (no copy) when possible
pub(super) extern "system" fn get_string_critical(
    env: *mut JNIEnv,
    str: jstring,
    is_copy: *mut jboolean,
) -> *const jchar {
    // No GC yet, so this is equivalent to GetStringChars
    // TODO(GC): Pin object and disable GC here
    get_string_chars(env, str, is_copy)
}

/// ReleaseStringCritical: Releases the pointer obtained from GetStringCritical.
/// Since there's no GC yet in rusty-jvm, this is just a wrapper around ReleaseStringChars.
/// TODO(GC): When GC is implemented, this must:
///   1. Unpin the string object
///   2. Re-enable GC
///   3. Must NOT be called between GetStringCritical and ReleaseStringCritical:
///      - NewString, NewCharArray, SetCharArrayRegion (any allocating JNI function)
pub(super) extern "system" fn release_string_critical(
    env: *mut JNIEnv,
    str: jstring,
    carray: *const jchar,
) {
    // No GC yet, so this is equivalent to ReleaseStringChars
    // TODO(GC): Unpin object and re-enable GC here
    release_string_chars(env, str, carray);
}
