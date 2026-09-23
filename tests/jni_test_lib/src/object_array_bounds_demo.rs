use jni::sys::{jclass, jobject, jobjectArray, JNIEnv};

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_arrayops_JniObjectArrayBoundsDemo_get(
    env: *mut JNIEnv,
    _class: jclass,
    array: jobjectArray,
    index: i32,
) -> jobject {
    unsafe { ((*(*env)).v24.GetObjectArrayElement)(env, array, index) }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_arrayops_JniObjectArrayBoundsDemo_set(
    env: *mut JNIEnv,
    _class: jclass,
    array: jobjectArray,
    index: i32,
    value: jobject,
) {
    unsafe { ((*(*env)).v24.SetObjectArrayElement)(env, array, index, value) }
}
