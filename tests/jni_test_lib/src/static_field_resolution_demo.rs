use jni::sys::{jclass, jfieldID, jint, jlong, jobject, jstring, JNIEnv};
use std::ffi::c_char;
use std::ptr::null_mut;

/// `GetStaticFieldID` with a name and signature from Java strings. Null if the field isn't found,
/// with a `NoSuchFieldError` pending.
unsafe fn static_field_id(
    env: *mut JNIEnv,
    class: jclass,
    name: jstring,
    signature: *const c_char,
) -> jfieldID {
    let name_chars = ((*(*env)).v24.GetStringUTFChars)(env, name, null_mut());
    let field_id = ((*(*env)).v24.GetStaticFieldID)(env, class, name_chars, signature);
    ((*(*env)).v24.ReleaseStringUTFChars)(env, name, name_chars);
    field_id
}

#[no_mangle]
pub extern "system" fn Java_samples_jni_staticfields_JniStaticFieldResolution_getInt(
    env: *mut JNIEnv,
    _class: jclass,
    target: jclass,
    name: jstring,
) -> jint {
    unsafe {
        let field_id = static_field_id(env, target, name, c"I".as_ptr());
        if field_id.is_null() {
            return 0;
        }
        ((*(*env)).v24.GetStaticIntField)(env, target, field_id)
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_jni_staticfields_JniStaticFieldResolution_getLong(
    env: *mut JNIEnv,
    _class: jclass,
    target: jclass,
    name: jstring,
) -> jlong {
    unsafe {
        let field_id = static_field_id(env, target, name, c"J".as_ptr());
        if field_id.is_null() {
            return 0;
        }
        ((*(*env)).v24.GetStaticLongField)(env, target, field_id)
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_jni_staticfields_JniStaticFieldResolution_setInt(
    env: *mut JNIEnv,
    _class: jclass,
    target: jclass,
    name: jstring,
    value: jint,
) {
    unsafe {
        let field_id = static_field_id(env, target, name, c"I".as_ptr());
        if !field_id.is_null() {
            ((*(*env)).v24.SetStaticIntField)(env, target, field_id, value);
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_jni_staticfields_JniStaticFieldResolution_getObject(
    env: *mut JNIEnv,
    _class: jclass,
    target: jclass,
    name: jstring,
    signature: jstring,
) -> jobject {
    unsafe {
        let signature_chars = ((*(*env)).v24.GetStringUTFChars)(env, signature, null_mut());
        let field_id = static_field_id(env, target, name, signature_chars);
        ((*(*env)).v24.ReleaseStringUTFChars)(env, signature, signature_chars);
        if field_id.is_null() {
            return null_mut();
        }
        ((*(*env)).v24.GetStaticObjectField)(env, target, field_id)
    }
}
