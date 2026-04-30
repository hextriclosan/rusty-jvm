use jni::sys::{jclass, jstring, JNIEnv};

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_JniClassOperationsDemo_findClass(
    env: *mut JNIEnv,
    _class: jclass,
    name: jstring,
) -> jclass {
    unsafe {
        let chars_mutf8 = ((*(*env)).v24.GetStringUTFChars)(env, name, std::ptr::null_mut());
        let clazz = ((*(*env)).v24.FindClass)(env, chars_mutf8);
        ((*(*env)).v24.ReleaseStringUTFChars)(env, name, chars_mutf8);
        clazz
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_JniClassOperationsDemo_getSuperclass(
    env: *mut JNIEnv,
    _this: jclass,
    class: jclass,
) -> jclass {
    unsafe { ((*(*env)).v24.GetSuperclass)(env, class) }
}
