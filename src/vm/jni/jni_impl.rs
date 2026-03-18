use crate::vm::jni::jni_env::get_jni_vm;
use jni_sys::{jboolean, jint, JNIEnv, JavaVM, JNI_FALSE};

const JNI_VERSION_24: i32 = 0x00180000;

pub(super) extern "system" fn get_version(_env: *mut JNIEnv) -> jint {
    JNI_VERSION_24
}

pub(super) extern "system" fn exception_check(_env: *mut JNIEnv) -> jboolean {
    JNI_FALSE // todo: implement me
}

pub(super) extern "system" fn get_java_vm(_env: *mut JNIEnv, vm: *mut *mut JavaVM) -> jint {
    unsafe {
        let jni_vm = get_jni_vm();
        *vm = jni_vm;
    }
    0 // illustrates success
}
