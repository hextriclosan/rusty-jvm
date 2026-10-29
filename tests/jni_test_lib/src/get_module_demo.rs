use jni::sys::{jclass, jobject, JNIEnv};

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_classops_JniGetModuleDemo_getModule(
    env: *mut JNIEnv,
    _class: jclass,
    target: jclass,
) -> jobject {
    unsafe { ((*(*env)).v24.GetModule)(env, target) }
}
