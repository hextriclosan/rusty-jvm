use jni::sys::{jboolean, jclass, JNIEnv};

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_classops_JniIsAssignableNullDemo_isAssignable(
    env: *mut JNIEnv,
    _class: jclass,
    sub: jclass,
    sup: jclass,
) -> jboolean {
    unsafe { ((*(*env)).v24.IsAssignableFrom)(env, sub, sup) }
}
