use crate::vm::execution_engine::executor::Executor;
use crate::vm::heap::heap::HEAP;
use jni_sys::{jchar, jint, jsize, jstring, JNIEnv};

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
    let arr_ref = HEAP.create_array("[C", len as i32);
    {
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
