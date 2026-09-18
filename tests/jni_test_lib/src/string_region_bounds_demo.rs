use jni::sys::{jbyteArray, jchar, jclass, jint, jstring, JNIEnv};
use std::ffi::c_char;
use std::ptr;

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_stringops_JniStringRegionBoundsDemo_get(
    env: *mut JNIEnv,
    _class: jclass,
    string: jstring,
    start: jint,
    len: jint,
) -> jstring {
    let mut buffer = [0 as jchar; 8];
    unsafe {
        ((*(*env)).v24.GetStringRegion)(env, string, start, len, buffer.as_mut_ptr());
        if ((*(*env)).v24.ExceptionCheck)(env) {
            return ptr::null_mut();
        }
        ((*(*env)).v24.NewString)(env, buffer.as_ptr(), len)
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_stringops_JniStringRegionBoundsDemo_getUtf(
    env: *mut JNIEnv,
    _class: jclass,
    string: jstring,
    start: jint,
    len: jint,
) -> jbyteArray {
    // Pre-filled with non-zero bytes to check that the result is null-terminated
    let mut buffer = [b'#' as c_char; 32];
    unsafe {
        ((*(*env)).v24.GetStringUTFRegion)(env, string, start, len, buffer.as_mut_ptr());
        if ((*(*env)).v24.ExceptionCheck)(env) {
            return ptr::null_mut();
        }
        let utf_len = buffer
            .iter()
            .position(|&byte| byte == 0)
            .unwrap_or(buffer.len()) as jint;
        let result = ((*(*env)).v24.NewByteArray)(env, utf_len);
        ((*(*env)).v24.SetByteArrayRegion)(env, result, 0, utf_len, buffer.as_ptr().cast());
        result
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_stringops_JniStringRegionBoundsDemo_getNullBuffer(
    env: *mut JNIEnv,
    _class: jclass,
    string: jstring,
    start: jint,
    len: jint,
) {
    unsafe { ((*(*env)).v24.GetStringRegion)(env, string, start, len, ptr::null_mut()) }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_stringops_JniStringRegionBoundsDemo_getUtfNullBuffer(
    env: *mut JNIEnv,
    _class: jclass,
    string: jstring,
    start: jint,
    len: jint,
) {
    unsafe { ((*(*env)).v24.GetStringUTFRegion)(env, string, start, len, ptr::null_mut()) }
}
