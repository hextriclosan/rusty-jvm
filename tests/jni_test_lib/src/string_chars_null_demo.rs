use jni::sys::{jboolean, jchar, jclass, jstring, JNIEnv};
use std::ffi::c_char;

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_stringops_JniStringCharsNullDemo_chars(
    env: *mut JNIEnv,
    _class: jclass,
    string: jstring,
) -> jboolean {
    unsafe { !((*(*env)).v24.GetStringChars)(env, string, std::ptr::null_mut()).is_null() }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_stringops_JniStringCharsNullDemo_utfChars(
    env: *mut JNIEnv,
    _class: jclass,
    string: jstring,
) -> jboolean {
    let chars: *const c_char =
        unsafe { ((*(*env)).v24.GetStringUTFChars)(env, string, std::ptr::null_mut()) };
    !chars.is_null()
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_stringops_JniStringCharsNullDemo_critical(
    env: *mut JNIEnv,
    _class: jclass,
    string: jstring,
) -> jboolean {
    let chars: *const jchar =
        unsafe { ((*(*env)).v24.GetStringCritical)(env, string, std::ptr::null_mut()) };
    !chars.is_null()
}
