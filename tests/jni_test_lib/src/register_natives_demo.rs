use jni::sys::{jclass, jint, jintArray, jobjectArray, jstring, JNIEnv, JNINativeMethod};
use std::ffi::c_void;
use std::ptr::null_mut;

unsafe extern "system" fn add(_env: *mut JNIEnv, _class: jclass, left: jint, right: jint) -> jint {
    left + right
}

unsafe extern "system" fn subtract(
    _env: *mut JNIEnv,
    _class: jclass,
    left: jint,
    right: jint,
) -> jint {
    left - right
}

unsafe extern "system" fn hundred(_env: *mut JNIEnv, _class: jclass) -> jint {
    100
}

/// The functions that Java chooses by number: none, add, subtract, hundred.
fn function(number: jint) -> *mut c_void {
    match number {
        1 => add as *const () as *mut c_void,
        2 => subtract as *const () as *mut c_void,
        3 => hundred as *const () as *mut c_void,
        _ => null_mut(),
    }
}

/// `RegisterNatives` of `target` with a method for each name, signature and function number.
#[no_mangle]
pub extern "system" fn Java_samples_jni_registernatives_RegisterNativesDemo_registerAll(
    env: *mut JNIEnv,
    _class: jclass,
    target: jclass,
    names: jobjectArray,
    signatures: jobjectArray,
    functions: jintArray,
) -> jint {
    unsafe {
        let count = ((*(*env)).v24.GetArrayLength)(env, names);
        let mut numbers = vec![0; count as usize];
        ((*(*env)).v24.GetIntArrayRegion)(env, functions, 0, count, numbers.as_mut_ptr());
        let mut strings = Vec::new();
        let mut methods = Vec::new();
        for index in 0..count {
            let name = ((*(*env)).v24.GetObjectArrayElement)(env, names, index) as jstring;
            let signature =
                ((*(*env)).v24.GetObjectArrayElement)(env, signatures, index) as jstring;
            let name_chars = ((*(*env)).v24.GetStringUTFChars)(env, name, null_mut());
            let signature_chars = ((*(*env)).v24.GetStringUTFChars)(env, signature, null_mut());
            strings.push((name, name_chars));
            strings.push((signature, signature_chars));
            methods.push(JNINativeMethod {
                name: name_chars as *mut _,
                signature: signature_chars as *mut _,
                fnPtr: function(numbers[index as usize]),
            });
        }
        let result = ((*(*env)).v24.RegisterNatives)(env, target, methods.as_ptr(), count);
        for (string, chars) in strings {
            ((*(*env)).v24.ReleaseStringUTFChars)(env, string, chars);
        }
        result
    }
}

/// `RegisterNatives` of `target` with no methods but the given count.
#[no_mangle]
pub extern "system" fn Java_samples_jni_registernatives_RegisterNativesDemo_registerCount(
    env: *mut JNIEnv,
    _class: jclass,
    target: jclass,
    count: jint,
) -> jint {
    unsafe { ((*(*env)).v24.RegisterNatives)(env, target, null_mut(), count) }
}

#[no_mangle]
pub extern "system" fn Java_samples_jni_registernatives_RegisterNativesDemo_unregister(
    env: *mut JNIEnv,
    _class: jclass,
    target: jclass,
) -> jint {
    unsafe { ((*(*env)).v24.UnregisterNatives)(env, target) }
}

/// The function linked by JNI symbol name, until `RegisterNatives` binds another.
#[no_mangle]
pub extern "system" fn Java_samples_jni_registernatives_RegisterNativesDemo_exported(
    _env: *mut JNIEnv,
    _class: jclass,
    left: jint,
    right: jint,
) -> jint {
    left * right
}

#[no_mangle]
pub extern "system" fn Java_samples_jni_registernatives_NativeParent_parentValue(
    _env: *mut JNIEnv,
    _class: jclass,
) -> jint {
    1
}
