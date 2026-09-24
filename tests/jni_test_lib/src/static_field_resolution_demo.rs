use jni::sys::{jclass, jint, jlong, JNIEnv};
use std::ffi::c_char;

#[no_mangle]
pub extern "system" fn Java_samples_jni_staticfields_JniStaticFieldResolution_readInt(
    env: *mut JNIEnv,
    _class: jclass,
    target: jclass,
) -> jint {
    unsafe {
        let field_id = ((*(*env)).v24.GetStaticFieldID)(
            env,
            target,
            b"value\0".as_ptr() as *const c_char,
            b"I\0".as_ptr() as *const c_char,
        );
        ((*(*env)).v24.GetStaticIntField)(env, target, field_id)
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_jni_staticfields_JniStaticFieldResolution_readLong(
    env: *mut JNIEnv,
    _class: jclass,
    target: jclass,
) -> jlong {
    unsafe {
        let field_id = ((*(*env)).v24.GetStaticFieldID)(
            env,
            target,
            b"value\0".as_ptr() as *const c_char,
            b"J\0".as_ptr() as *const c_char,
        );
        ((*(*env)).v24.GetStaticLongField)(env, target, field_id)
    }
}
