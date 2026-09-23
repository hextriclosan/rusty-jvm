use jni::sys::{jchar, jclass, jint, jstring, JNIEnv};
use std::ffi::c_char;

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_stringops_JniStringRegionBoundsDemo_get(
    env: *mut JNIEnv,
    _class: jclass,
    string: jstring,
    start: jint,
    len: jint,
) {
    let mut buffer = [0 as jchar; 8];
    unsafe { ((*(*env)).v24.GetStringRegion)(env, string, start, len, buffer.as_mut_ptr()) }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_stringops_JniStringRegionBoundsDemo_getUtf(
    env: *mut JNIEnv,
    _class: jclass,
    string: jstring,
    start: jint,
    len: jint,
) {
    let mut buffer = [0 as c_char; 32];
    unsafe { ((*(*env)).v24.GetStringUTFRegion)(env, string, start, len, buffer.as_mut_ptr()) }
}
