use jni::sys::{jboolean, jclass, jobject, JNIEnv};

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_objectops_JniIsInstanceOfDemo_isInstance(
    env: *mut JNIEnv,
    _class: jclass,
    object: jobject,
    target: jclass,
) -> jboolean {
    unsafe { ((*(*env)).v24.IsInstanceOf)(env, object, target) }
}
