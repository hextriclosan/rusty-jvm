use crate::from_mutf8_ptr;
use crate::vm::execution_engine::executor::Executor;
use crate::vm::heap::heap::HEAP;
use crate::vm::jni::array_operations_impl::{
    new_byte_array, new_char_array, set_byte_array_region, set_char_array_region,
};
use crate::vm::method_area::loaded_classes::CLASSES;
use crate::vm::system_native::string::get_raw_string_info;
use cesu8::to_java_cesu8;
use jni_sys::{jboolean, jbyte, jchar, jint, jlong, jsize, jstring, JNIEnv, JNI_TRUE};
use std::ffi::c_char;
use std::ptr;

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
    let string_instance_ref = Executor::invoke_args_constructor(
        "java/lang/String",
        "<init>:([C)V",
        &[(arr_ref as i32).into()],
        Some(""),
    )
    .expect("Failed to invoke String constructor"); // todo handle exception here

    string_instance_ref as jstring
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
    let (is_latin, array_ref) =
        get_raw_string_info(string_ref).expect("Failed to get raw string info");
    let raw_data = if is_latin {
        let guard = HEAP
            .get_entire_raw_data(array_ref)
            .expect("Failed to get string data");
        guard.iter().map(|x| *x as jchar).collect::<Vec<_>>()
    } else {
        let guard = HEAP
            .get_entire_raw_data(array_ref)
            .expect("Failed to get string data");
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

    let (is_latin, array_ref) = get_raw_string_info(string_ref).expect(&format!(
        "Failed to get raw string info for string_ref={string_ref}"
    ));
    let byte_len = HEAP
        .get_entire_raw_data(array_ref)
        .expect(&format!(
            "Failed to get string data for array_ref={array_ref}"
        ))
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

    let decoded = from_mutf8_ptr!(mutf8_bytes).expect("Failed to decode modified UTF-8 bytes");
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

    let utf8_charset_ref = CLASSES
        .get("java/nio/charset/StandardCharsets")
        .expect("Failed to get StandardCharsets class")
        .static_field("UTF_8")
        .expect("Failed to get UTF_8 field")
        .raw_value()
        .expect("Failed to get UTF_8 value")[0];

    let string_instance_ref = Executor::invoke_args_constructor(
        "java/lang/String",
        "<init>:([BLjava/nio/charset/Charset;)V",
        &[(byte_array as i32).into(), utf8_charset_ref.into()],
        Some(""),
    )
    .expect("Failed to invoke String constructor"); // todo handle exception here

    string_instance_ref as jstring
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
    let data = String::from_utf16(&raw_data).expect("Failed to build string from UTF-16 data");
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

    let raw_data = get_string_raw_data(string_ref);
    let data = String::from_utf16(&raw_data).expect("Failed to build string from UTF-16 data");
    let mut mutf8_data = to_java_cesu8(&data).to_vec();
    mutf8_data.push(0); // null terminator
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
