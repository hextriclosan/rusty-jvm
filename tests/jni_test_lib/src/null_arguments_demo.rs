use jni::sys::{jboolean, jclass, jint, jlong, jobject, jstring, JNIEnv};
use std::ptr;

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_nullargs_JniNullArgumentsDemo_getArrayLength(
    env: *mut JNIEnv,
    _class: jclass,
    array: jobject,
) -> jint {
    unsafe { ((*(*env)).v24.GetArrayLength)(env, array) }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_nullargs_JniNullArgumentsDemo_getStringLength(
    env: *mut JNIEnv,
    _class: jclass,
    string: jstring,
) -> jint {
    unsafe { ((*(*env)).v24.GetStringLength)(env, string) }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_nullargs_JniNullArgumentsDemo_getStringUtfLength(
    env: *mut JNIEnv,
    _class: jclass,
    string: jstring,
) -> jint {
    unsafe { ((*(*env)).v24.GetStringUTFLength)(env, string) }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_nullargs_JniNullArgumentsDemo_getStringUtfLengthAsLong(
    env: *mut JNIEnv,
    _class: jclass,
    string: jstring,
) -> jlong {
    unsafe { ((*(*env)).v24.GetStringUTFLengthAsLong)(env, string) }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_nullargs_JniNullArgumentsDemo_getStringChars(
    env: *mut JNIEnv,
    _class: jclass,
    string: jstring,
) -> jstring {
    unsafe {
        let chars = ((*(*env)).v24.GetStringChars)(env, string, ptr::null_mut());
        if chars.is_null() {
            return ptr::null_mut();
        }
        let len = ((*(*env)).v24.GetStringLength)(env, string);
        let copy = ((*(*env)).v24.NewString)(env, chars, len);
        ((*(*env)).v24.ReleaseStringChars)(env, string, chars);
        copy
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_nullargs_JniNullArgumentsDemo_getStringUtfChars(
    env: *mut JNIEnv,
    _class: jclass,
    string: jstring,
) -> jstring {
    unsafe {
        let chars = ((*(*env)).v24.GetStringUTFChars)(env, string, ptr::null_mut());
        if chars.is_null() {
            return ptr::null_mut();
        }
        let copy = ((*(*env)).v24.NewStringUTF)(env, chars);
        ((*(*env)).v24.ReleaseStringUTFChars)(env, string, chars);
        copy
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_nullargs_JniNullArgumentsDemo_getStringCritical(
    env: *mut JNIEnv,
    _class: jclass,
    string: jstring,
) -> jint {
    unsafe {
        let chars = ((*(*env)).v24.GetStringCritical)(env, string, ptr::null_mut());
        if chars.is_null() {
            return -1;
        }
        let first = *chars as jint;
        ((*(*env)).v24.ReleaseStringCritical)(env, string, chars);
        first
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_nullargs_JniNullArgumentsDemo_newStringFromNull(
    env: *mut JNIEnv,
    _class: jclass,
    len: jint,
) -> jstring {
    unsafe { ((*(*env)).v24.NewString)(env, ptr::null(), len) }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_nullargs_JniNullArgumentsDemo_newStringUtfFromNull(
    env: *mut JNIEnv,
    _class: jclass,
) -> jstring {
    unsafe { ((*(*env)).v24.NewStringUTF)(env, ptr::null()) }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_nullargs_JniNullArgumentsDemo_getObjectClass(
    env: *mut JNIEnv,
    _class: jclass,
    object: jobject,
) -> jclass {
    unsafe { ((*(*env)).v24.GetObjectClass)(env, object) }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_nullargs_JniNullArgumentsDemo_getSuperclass(
    env: *mut JNIEnv,
    _class: jclass,
    class: jclass,
) -> jclass {
    unsafe { ((*(*env)).v24.GetSuperclass)(env, class) }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_nullargs_JniNullArgumentsDemo_isAssignableFrom(
    env: *mut JNIEnv,
    _class: jclass,
    sub: jclass,
    sup: jclass,
) -> jboolean {
    unsafe { ((*(*env)).v24.IsAssignableFrom)(env, sub, sup) }
}
