use jni::sys::{jclass, jint, JNIEnv, JNINativeMethod};
use std::ffi::{c_char, c_void};

unsafe extern "system" fn registered_add(
    _env: *mut JNIEnv,
    _class: jclass,
    left: jint,
    right: jint,
) -> jint {
    left + right
}

#[no_mangle]
pub extern "system" fn Java_samples_jni_registernatives_RegisterNativesDemo_register(
    env: *mut JNIEnv,
    class: jclass,
) -> jint {
    let method = JNINativeMethod {
        name: b"registeredAdd\0".as_ptr() as *mut c_char,
        signature: b"(II)I\0".as_ptr() as *mut c_char,
        fnPtr: registered_add as *const () as *mut c_void,
    };
    unsafe { ((*(*env)).v24.RegisterNatives)(env, class, &method, 1) }
}

#[no_mangle]
pub extern "system" fn Java_samples_jni_registernatives_RegisterNativesDemo_unregister(
    env: *mut JNIEnv,
    class: jclass,
) -> jint {
    unsafe { ((*(*env)).v24.UnregisterNatives)(env, class) }
}

#[no_mangle]
pub extern "system" fn Java_samples_jni_registernatives_RegisterNativesDemo_registerMissing(
    env: *mut JNIEnv,
    class: jclass,
) -> jint {
    let method = JNINativeMethod {
        name: b"missing\0".as_ptr() as *mut c_char,
        signature: b"()V\0".as_ptr() as *mut c_char,
        fnPtr: registered_add as *const () as *mut c_void,
    };
    unsafe { ((*(*env)).v24.RegisterNatives)(env, class, &method, 1) }
}
