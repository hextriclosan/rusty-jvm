use jni_sys::{jint, JNIEnv};

pub(super) extern "system" fn get_version(_env: *mut JNIEnv) -> jint {
    const JNI_VERSION_24: i32 = 0x00180000;
    JNI_VERSION_24
}
