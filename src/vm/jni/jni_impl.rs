use jni_sys::{jboolean, jint, JNIEnv, JNI_FALSE};

const JNI_VERSION_24: i32 = 0x00180000;

pub(super) extern "system" fn get_version(_env: *mut JNIEnv) -> jint {
    JNI_VERSION_24
}

pub(super) extern "system" fn exception_check(_env: *mut JNIEnv) -> jboolean {
    JNI_FALSE // todo: implement me
}
