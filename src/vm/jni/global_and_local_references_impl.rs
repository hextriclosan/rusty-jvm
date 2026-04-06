use jni_sys::{jint, jobject, JNIEnv};
use std::ptr::null_mut;

pub(super) extern "system" fn push_local_frame(_env: *mut JNIEnv, _capacity: jint) -> jint {
    0 // todo: implement me
}

pub(super) extern "system" fn pop_local_frame(_env: *mut JNIEnv, _result: jobject) -> jobject {
    null_mut() // todo: implement me
}
