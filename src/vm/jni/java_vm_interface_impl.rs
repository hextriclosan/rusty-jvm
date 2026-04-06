use crate::vm::jni::jni_env::get_jni_vm;
use jni_sys::{jint, JNIEnv, JavaVM, JNI_ERR, JNI_OK};

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
