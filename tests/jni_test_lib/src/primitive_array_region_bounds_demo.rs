use jni::sys::{jclass, jint, jintArray, JNIEnv};
use std::ptr;

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_arrayops_JniPrimitiveArrayRegionBoundsDemo_get(
    env: *mut JNIEnv,
    _class: jclass,
    array: jintArray,
    start: jint,
    len: jint,
) -> jintArray {
    let mut buffer = [0; 2];
    unsafe {
        ((*(*env)).v24.GetIntArrayRegion)(env, array, start, len, buffer.as_mut_ptr());
        if ((*(*env)).v24.ExceptionCheck)(env) {
            return ptr::null_mut();
        }
        let result = ((*(*env)).v24.NewIntArray)(env, 2);
        ((*(*env)).v24.SetIntArrayRegion)(env, result, 0, 2, buffer.as_ptr());
        result
    }
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

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_arrayops_JniPrimitiveArrayRegionBoundsDemo_getNullBuffer(
    env: *mut JNIEnv,
    _class: jclass,
    array: jintArray,
    start: jint,
    len: jint,
) {
    unsafe { ((*(*env)).v24.GetIntArrayRegion)(env, array, start, len, ptr::null_mut()) }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_arrayops_JniPrimitiveArrayRegionBoundsDemo_setNullBuffer(
    env: *mut JNIEnv,
    _class: jclass,
    array: jintArray,
    start: jint,
    len: jint,
) {
    unsafe { ((*(*env)).v24.SetIntArrayRegion)(env, array, start, len, ptr::null()) }
}
