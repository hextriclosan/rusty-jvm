use jni::sys::{jclass, jobject, JNIEnv};

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_objectops_JniGetObjectClassNullDemo_getClass(
    env: *mut JNIEnv,
    _class: jclass,
    object: jobject,
) -> jclass {
    unsafe { ((*(*env)).v24.GetObjectClass)(env, object) }
}
