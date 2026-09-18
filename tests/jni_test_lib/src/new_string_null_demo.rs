use jni::sys::{jclass, jstring, JNIEnv};

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_stringops_JniNewStringNullDemo_newString(
    env: *mut JNIEnv,
    _class: jclass,
) -> jstring {
    unsafe { ((*(*env)).v24.NewString)(env, std::ptr::null(), 1) }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_stringops_JniNewStringNullDemo_newStringUtf(
    env: *mut JNIEnv,
    _class: jclass,
) -> jstring {
    unsafe { ((*(*env)).v24.NewStringUTF)(env, std::ptr::null()) }
}
