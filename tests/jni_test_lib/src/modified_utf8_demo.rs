use jni::sys::{jbyteArray, jclass, jint, jlong, jstring, JNIEnv};
use std::ffi::{c_char, CStr};
use std::ptr;

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_stringops_JniModifiedUtf8Demo_newStringUtf(
    env: *mut JNIEnv,
    _class: jclass,
    bytes: jbyteArray,
) -> jstring {
    unsafe {
        let len = ((*(*env)).v24.GetArrayLength)(env, bytes);
        // One extra zero byte as the null terminator
        let mut buffer = vec![0 as c_char; len as usize + 1];
        ((*(*env)).v24.GetByteArrayRegion)(env, bytes, 0, len, buffer.as_mut_ptr().cast());
        ((*(*env)).v24.NewStringUTF)(env, buffer.as_ptr())
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_stringops_JniModifiedUtf8Demo_getStringUtfChars(
    env: *mut JNIEnv,
    _class: jclass,
    string: jstring,
) -> jbyteArray {
    unsafe {
        let chars = ((*(*env)).v24.GetStringUTFChars)(env, string, ptr::null_mut());
        let bytes = CStr::from_ptr(chars).to_bytes();
        let len = bytes.len() as jint;
        let result = ((*(*env)).v24.NewByteArray)(env, len);
        ((*(*env)).v24.SetByteArrayRegion)(env, result, 0, len, bytes.as_ptr().cast());
        ((*(*env)).v24.ReleaseStringUTFChars)(env, string, chars);
        result
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_stringops_JniModifiedUtf8Demo_getStringUtfLength(
    env: *mut JNIEnv,
    _class: jclass,
    string: jstring,
) -> jint {
    unsafe { ((*(*env)).v24.GetStringUTFLength)(env, string) }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_stringops_JniModifiedUtf8Demo_getStringUtfLengthAsLong(
    env: *mut JNIEnv,
    _class: jclass,
    string: jstring,
) -> jlong {
    unsafe { ((*(*env)).v24.GetStringUTFLengthAsLong)(env, string) }
}
