use jni::sys::{jclass, jint, jintArray, JNIEnv};

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_arrayops_JniPrimitiveArrayRegionBoundsDemo_get(
    env: *mut JNIEnv,
    _class: jclass,
    array: jintArray,
    start: jint,
    len: jint,
) {
    let mut buffer = [0; 2];
    unsafe { ((*(*env)).v24.GetIntArrayRegion)(env, array, start, len, buffer.as_mut_ptr()) }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_arrayops_JniPrimitiveArrayRegionBoundsDemo_set(
    env: *mut JNIEnv,
    _class: jclass,
    array: jintArray,
    start: jint,
    len: jint,
) {
    let buffer = [1, 2];
    unsafe { ((*(*env)).v24.SetIntArrayRegion)(env, array, start, len, buffer.as_ptr()) }
}
