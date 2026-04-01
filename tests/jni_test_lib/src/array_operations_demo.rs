use jni::sys::{jclass, jint, jobject, JNIEnv};

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_ArrayOperationsDemo_GetArrayLength(
    env: *mut JNIEnv,
    _class: jclass,
    input: jobject,
) -> jint {
    unsafe { ((*(*env)).v24.GetArrayLength)(env, input) }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_ArrayOperationsDemo_NewObjectArray(
    env: *mut JNIEnv,
    _class: jclass,
    length: jint,
    element_class: jobject,
    initial_element: jobject,
) -> jobject {
    unsafe { ((*(*env)).v24.NewObjectArray)(env, length, element_class, initial_element) }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_ArrayOperationsDemo_GetObjectArrayElement(
    env: *mut JNIEnv,
    _class: jclass,
    array: jobject,
    index: jint,
) -> jobject {
    unsafe { ((*(*env)).v24.GetObjectArrayElement)(env, array, index) }
}
