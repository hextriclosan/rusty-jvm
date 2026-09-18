use jni::sys::{jclass, jintArray, jobjectArray, JNIEnv};

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_arrayops_JniNegativeArraySizeDemo_newIntArray(
    env: *mut JNIEnv,
    _class: jclass,
) -> jintArray {
    unsafe { ((*(*env)).v24.NewIntArray)(env, -1) }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_arrayops_JniNegativeArraySizeDemo_newObjectArray(
    env: *mut JNIEnv,
    _class: jclass,
    component: jclass,
) -> jobjectArray {
    unsafe { ((*(*env)).v24.NewObjectArray)(env, -2, component, std::ptr::null_mut()) }
}
