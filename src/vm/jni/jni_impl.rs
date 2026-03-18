use crate::vm::jni::jni_env::get_jni_vm;
use jni_sys::{jboolean, jint, JNIEnv, JavaVM, JNI_ERR, JNI_FALSE, JNI_OK};

const JNI_VERSION_24: i32 = 0x00180000;

pub(super) extern "system" fn get_version(_env: *mut JNIEnv) -> jint {
    JNI_VERSION_24
}

pub(super) extern "system" fn exception_check(_env: *mut JNIEnv) -> jboolean {
    JNI_FALSE // todo: implement me
}

pub(super) extern "system" fn get_java_vm(_env: *mut JNIEnv, vm: *mut *mut JavaVM) -> jint {
    if vm.is_null() {
        return JNI_ERR;
    }

    let jni_vm = get_jni_vm();
    unsafe {
        *vm = jni_vm;
    }

    JNI_OK
}
