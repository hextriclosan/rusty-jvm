use jni::sys::{jclass, JNIEnv};

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_classops_JniGetSuperclassNullDemo_getSuperclass(
    env: *mut JNIEnv,
    _class: jclass,
    target: jclass,
) -> jclass {
    unsafe { ((*(*env)).v24.GetSuperclass)(env, target) }
}
