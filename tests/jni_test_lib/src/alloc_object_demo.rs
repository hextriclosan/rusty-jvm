use jni::sys::{jclass, jobject, JNIEnv};

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_objectops_JniAllocObjectDemo_allocate(
    env: *mut JNIEnv,
    _class: jclass,
    target: jclass,
) -> jobject {
    unsafe { ((*(*env)).v24.AllocObject)(env, target) }
}
