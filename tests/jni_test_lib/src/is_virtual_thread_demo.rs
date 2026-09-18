use jni::sys::{jboolean, jclass, jobject, JNIEnv};

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_threadops_JniIsVirtualThreadDemo_isVirtual(
    env: *mut JNIEnv,
    _class: jclass,
    thread: jobject,
) -> jboolean {
    unsafe { ((*(*env)).v24.IsVirtualThread)(env, thread) }
}
