use crate::vm::jni::jni_env::JNIEnv;
use jni_sys::{jboolean, jint, JNI_FALSE};

const JNI_VERSION_24: i32 = 0x00180000;

pub(super) extern "system" fn get_version(_env: JNIEnv) -> jint {
    JNI_VERSION_24
}

pub(super) extern "system" fn exception_check(_env: JNIEnv) -> jboolean {
    JNI_FALSE // todo: implement me
}
