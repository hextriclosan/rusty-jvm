use jni::sys::{jarray, jclass, jint, jobject, jobjectArray, JNIEnv};

/// Takes the pending exception, if any, so Java can inspect it instead of having it thrown.
/// Returns the exception when the allocation correctly returned null, otherwise the array itself.
unsafe fn exception_or_array(env: *mut JNIEnv, array: jarray) -> jobject {
    let exception = ((*(*env)).v24.ExceptionOccurred)(env);
    if !exception.is_null() {
        ((*(*env)).v24.ExceptionClear)(env);
        if array.is_null() {
            return exception;
        }
    }
    array
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_arrayops_JniArrayAllocationDemo_create(
    env: *mut JNIEnv,
    _class: jclass,
    component: jclass,
) -> jobjectArray {
    unsafe { ((*(*env)).v24.NewObjectArray)(env, 1, component, std::ptr::null_mut()) }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_arrayops_JniArrayAllocationDemo_newObjectArray(
    env: *mut JNIEnv,
    _class: jclass,
    component: jclass,
    len: jint,
) -> jobject {
    unsafe {
        let array = ((*(*env)).v24.NewObjectArray)(env, len, component, std::ptr::null_mut());
        exception_or_array(env, array)
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_arrayops_JniArrayAllocationDemo_newIntArray(
    env: *mut JNIEnv,
    _class: jclass,
    len: jint,
) -> jobject {
    unsafe {
        let array = ((*(*env)).v24.NewIntArray)(env, len);
        exception_or_array(env, array)
    }
}
