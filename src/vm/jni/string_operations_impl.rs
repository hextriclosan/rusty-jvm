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
    let arr_ref = HEAP.create_array("[C", len as i32);
    for i in 0..len {
        let char_value = unsafe { *unicode.add(i as usize) } as jchar;
        HEAP.set_array_value(arr_ref, i as i32, vec![char_value as i32])
            .expect("Failed to set array value"); // todo handle exception here
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
