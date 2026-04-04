use crate::vm::execution_engine::executor::Executor;
use crate::vm::heap::heap::HEAP;
use crate::vm::jni::array_operations_impl::get_array_length;
use crate::vm::system_native::string::get_raw_string_info;
use jni_sys::{jboolean, jchar, jint, jsize, jstring, JNIEnv, JNI_TRUE};
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
    _env: *mut JNIEnv,
    unicode: *const jchar,
    len: jsize,
) -> jstring {
    if len < 0 {
        panic!("negative array size"); // todo throw NegativeArraySizeException here
    }
    if unicode.is_null() && len > 0 {
        panic!("unicode array is null but length is {len}");
    }
    let arr_ref = HEAP.create_array("[C", len as i32);
    if len > 0 {
        let mut guard = HEAP
            .get_entire_raw_data_mut(arr_ref)
            .expect("Failed to get array data");
        guard.copy_from_slice(unsafe {
            std::slice::from_raw_parts(unicode as *const u8, len as usize * size_of::<jchar>())
        });
    }
    let string_instance_ref = Executor::invoke_args_constructor(
        "java/lang/String",
        "<init>:([C)V",
        &[arr_ref.into()],
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

    let boxed_slice = raw_data.into_boxed_slice();
    let raw_ptr = Box::into_raw(boxed_slice) as *const u8 as *const jchar;

    if !is_copy.is_null() {
        unsafe {
            *is_copy = JNI_TRUE; // we always return a copy
        }
    }

    raw_ptr
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
