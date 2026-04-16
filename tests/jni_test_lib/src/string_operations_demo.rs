use cesu8::to_java_cesu8;
use jni::sys::{
    jchar, jcharArray, jclass, jint, jlong, jobject, jsize, jstring, JNIEnv, JNI_ABORT,
};
use std::ffi::CString;
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

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_StringOperationsDemo_NewStringUTF(
    env: *mut JNIEnv,
    _class: jclass,
) -> jstring {
    let text = "Hello from Rust! 💅☕️";
    let cesu8 = to_java_cesu8(text);
    let cstr = CString::new(cesu8).unwrap();
    unsafe { ((*(*env)).v24.NewStringUTF)(env, cstr.as_ptr()) }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_StringOperationsDemo_GetStringUTFLength(
    env: *mut JNIEnv,
    _class: jclass,
    input: jobject,
) -> jint {
    unsafe { ((*(*env)).v24.GetStringUTFLength)(env, input) }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_StringOperationsDemo_GetStringUTFLengthAsLong(
    env: *mut JNIEnv,
    _class: jclass,
    input: jobject,
) -> jlong {
    unsafe { ((*(*env)).v24.GetStringUTFLengthAsLong)(env, input) }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_StringOperationsDemo_GetStringUTFChars(
    env: *mut JNIEnv,
    _class: jclass,
    input: jstring,
) -> jstring {
    let chars = unsafe { ((*(*env)).v24.GetStringUTFChars)(env, input, null_mut()) };
    let string_ref = unsafe { ((*(*env)).v24.NewStringUTF)(env, chars) };
    unsafe {
        ((*(*env)).v24.ReleaseStringUTFChars)(env, input, chars);
    }

    string_ref
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_StringOperationsDemo_GetStringRegion(
    env: *mut JNIEnv,
    _class: jclass,
    input: jstring,
    start: jint,
    len: jint,
) -> jcharArray {
    let length = len as jsize;
    let char_array = unsafe { ((*(*env)).v24.NewCharArray)(env, length) };
    let mut buf: Vec<jchar> = vec![0; len as usize];
    unsafe {
        ((*(*env)).v24.GetStringRegion)(env, input, start as jsize, length, buf.as_mut_ptr());
        ((*(*env)).v24.SetCharArrayRegion)(env, char_array, 0, length, buf.as_ptr());
    }

    char_array
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_StringOperationsDemo_GetStringUTFRegion(
    env: *mut JNIEnv,
    _class: jclass,
    input: jstring,
    start: jint,
    len: jint,
) -> jstring {
    let mut buf: Vec<u8> = vec![0; (len * 3 + 1) as usize]; // max 3 bytes per char + null terminator
    unsafe {
        ((*(*env)).v24.GetStringUTFRegion)(
            env,
            input,
            start as jsize,
            len as jsize,
            buf.as_mut_ptr() as *mut i8,
        );
        ((*(*env)).v24.NewStringUTF)(env, buf.as_ptr() as *const i8)
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_StringOperationsDemo_GetStringCritical(
    env: *mut JNIEnv,
    _class: jclass,
    input: jstring,
) -> jcharArray {
    let length = unsafe { ((*(*env)).v24.GetStringLength)(env, input) } as jsize;
    let chars = unsafe { ((*(*env)).v24.GetStringCritical)(env, input, null_mut()) };
    let char_array = unsafe { ((*(*env)).v24.NewCharArray)(env, length) };
    unsafe {
        ((*(*env)).v24.GetCharArrayRegion)(env, char_array, 0, length, chars);
        ((*(*env)).v24.ReleaseStringCritical)(env, input, chars);
    }

    char_array
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_StringOperationsDemo_GetStringCriticalAndRelease(
    env: *mut JNIEnv,
    _class: jclass,
    input: jstring,
) -> jstring {
    let chars = unsafe { ((*(*env)).v24.GetStringCritical)(env, input, null_mut()) };
    let length = unsafe { ((*(*env)).v24.GetStringLength)(env, input) } as jsize;

    // Create a new string from the critical chars
    let new_str = unsafe { ((*(*env)).v24.NewString)(env, chars, length) };

    unsafe {
        ((*(*env)).v24.ReleaseStringCritical)(env, input, chars);
    }

    new_str
}
