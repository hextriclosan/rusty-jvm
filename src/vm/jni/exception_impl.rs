use jni_sys::{jboolean, jthrowable, JNIEnv, JNI_FALSE};
use std::ptr::null_mut;

pub(super) extern "system" fn exception_check(_env: *mut JNIEnv) -> jboolean {
    JNI_FALSE // todo: implement me
}

pub(super) extern "system" fn exception_occurred(_env: *mut JNIEnv) -> jthrowable {
    null_mut() as jthrowable // todo: implement me
}
