use jni::sys::{jboolean, jclass, jfieldID, jint, jlong, jobject, jstring, JNIEnv};
use std::ptr::null_mut;

unsafe fn get_field_id(
    env: *mut JNIEnv,
    obj: jobject,
    name: jstring,
    signature: jstring,
) -> jfieldID {
    let name_chars = ((*(*env)).v24.GetStringUTFChars)(env, name, null_mut());
    let signature_chars = ((*(*env)).v24.GetStringUTFChars)(env, signature, null_mut());
    let class = ((*(*env)).v24.GetObjectClass)(env, obj);
    let field_id = ((*(*env)).v24.GetFieldID)(env, class, name_chars, signature_chars);
    ((*(*env)).v24.ReleaseStringUTFChars)(env, signature, signature_chars);
    ((*(*env)).v24.ReleaseStringUTFChars)(env, name, name_chars);
    field_id
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_inheritedfields_JniInheritedFieldsDemo_readInt(
    env: *mut JNIEnv,
    _class: jclass,
    obj: jobject,
    name: jstring,
    signature: jstring,
) -> jint {
    unsafe {
        let field_id = get_field_id(env, obj, name, signature);
        ((*(*env)).v24.GetIntField)(env, obj, field_id)
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_inheritedfields_JniInheritedFieldsDemo_readLong(
    env: *mut JNIEnv,
    _class: jclass,
    obj: jobject,
    name: jstring,
    signature: jstring,
) -> jlong {
    unsafe {
        let field_id = get_field_id(env, obj, name, signature);
        ((*(*env)).v24.GetLongField)(env, obj, field_id)
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_inheritedfields_JniInheritedFieldsDemo_hasField(
    env: *mut JNIEnv,
    _class: jclass,
    obj: jobject,
    name: jstring,
    signature: jstring,
) -> jboolean {
    unsafe { (!get_field_id(env, obj, name, signature).is_null()) as jboolean }
}
