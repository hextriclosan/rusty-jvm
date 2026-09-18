use jni::sys::{jclass, jint, jlong, jstring, JNIEnv};

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_stringops_JniStringLengthNullDemo_length(
    env: *mut JNIEnv,
    _class: jclass,
    string: jstring,
) -> jint {
    unsafe { ((*(*env)).v24.GetStringLength)(env, string) }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_stringops_JniStringLengthNullDemo_utfLength(
    env: *mut JNIEnv,
    _class: jclass,
    string: jstring,
) -> jlong {
    unsafe { ((*(*env)).v24.GetStringUTFLengthAsLong)(env, string) }
}
