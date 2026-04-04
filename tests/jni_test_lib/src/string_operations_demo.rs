use jni::sys::{jchar, jcharArray, jclass, jint, jobject, jsize, jstring, JNIEnv, JNI_ABORT};
use std::ptr::null_mut;

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_StringOperationsDemo_GetStringLength(
    env: *mut JNIEnv,
    _class: jclass,
    input: jobject,
) -> jint {
    unsafe { ((*(*env)).v24.GetStringLength)(env, input) }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_StringOperationsDemo_NewString(
    env: *mut JNIEnv,
    _class: jclass,
    unicode: jcharArray,
) -> jstring {
    let len = unsafe { ((*(*env)).v24.GetArrayLength)(env, unicode) } as jsize;
    let chars =
        unsafe { ((*(*env)).v24.GetCharArrayElements)(env, unicode, std::ptr::null_mut()) };
    let string_ref = unsafe { ((*(*env)).v24.NewString)(env, chars as *const jchar, len) };
    unsafe { ((*(*env)).v24.ReleaseCharArrayElements)(env, unicode, chars, JNI_ABORT) };

    string_ref
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_StringOperationsDemo_GetStringChars(
    env: *mut JNIEnv,
    _class: jclass,
    input: jobject,
) -> jcharArray {
    let length = unsafe { ((*(*env)).v24.GetStringLength)(env, input) } as jsize;
    let chars = unsafe { ((*(*env)).v24.GetStringChars)(env, input, null_mut()) };
    let char_array = unsafe { ((*(*env)).v24.NewCharArray)(env, length) };
    unsafe {
        ((*(*env)).v24.SetCharArrayRegion)(env, char_array, 0, length, chars);
        ((*(*env)).v24.ReleaseStringChars)(env, input, chars);
    }

    char_array
}
