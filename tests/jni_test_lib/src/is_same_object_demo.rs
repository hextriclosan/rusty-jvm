use jni::sys::{jboolean, jclass, jobject, JNIEnv};

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_objectops_JniIsSameObjectDemo_isSame(
    env: *mut JNIEnv,
    _class: jclass,
    first: jobject,
    second: jobject,
) -> jboolean {
    unsafe { ((*(*env)).v24.IsSameObject)(env, first, second) }
}
