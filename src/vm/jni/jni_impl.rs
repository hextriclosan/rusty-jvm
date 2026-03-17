use crate::vm::jni::jni_env::JNIEnv;

const JNI_VERSION_24: i32 = 0x00180000;

pub(super) extern "system" fn get_version(_env: *mut JNIEnv) -> i32 {
    JNI_VERSION_24
}
