use crate::vm::execution_engine::executor::Executor;
use jni_sys::{jint, jstring, JNIEnv};

pub(super) extern "system" fn get_string_length(_env: *mut JNIEnv, input: jstring) -> jint {
    let string_ref = input as i32;

    if string_ref == 0 {
        panic!("Invalid string reference"); // throw NPE here
    }

    let raw =
        Executor::invoke_non_static_method("java/lang/String", "length:()I", string_ref, &[])
            .expect("Failed to invoke String.length()"); // todo handle exception here

    raw[0] as jint
}
