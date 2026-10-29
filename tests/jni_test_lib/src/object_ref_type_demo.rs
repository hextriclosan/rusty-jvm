use jni::sys::{jclass, jint, jobject, JNIEnv};

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_objectops_JniObjectRefTypeDemo_refType(
    env: *mut JNIEnv,
    _class: jclass,
    object: jobject,
) -> jint {
    unsafe { ((*(*env)).v24.GetObjectRefType)(env, object) as jint }
}

/// Passes a handle that does not refer to any heap object.
#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_objectops_JniObjectRefTypeDemo_danglingRefType(
    env: *mut JNIEnv,
    _class: jclass,
) -> jint {
    unsafe { ((*(*env)).v24.GetObjectRefType)(env, i32::MAX as usize as jobject) as jint }
}
