use jni::sys::{
    jbooleanArray, jbyteArray, jcharArray, jclass, jdoubleArray, jfloatArray, jint, jintArray,
    jlongArray, jobject, jshortArray, JNIEnv,
};

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

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_ArrayOperationsDemo_SetObjectArrayElement(
    env: *mut JNIEnv,
    _class: jclass,
    array: jobject,
    index: jint,
    value: jobject,
) {
    unsafe { ((*(*env)).v24.SetObjectArrayElement)(env, array, index, value) }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_ArrayOperationsDemo_NewBooleanArray(
    env: *mut JNIEnv,
    _class: jclass,
    length: jint,
) -> jbooleanArray {
    unsafe { ((*(*env)).v24.NewBooleanArray)(env, length) }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_ArrayOperationsDemo_NewByteArray(
    env: *mut JNIEnv,
    _class: jclass,
    length: jint,
) -> jbyteArray {
    unsafe { ((*(*env)).v24.NewByteArray)(env, length) }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_ArrayOperationsDemo_NewCharArray(
    env: *mut JNIEnv,
    _class: jclass,
    length: jint,
) -> jcharArray {
    unsafe { ((*(*env)).v24.NewCharArray)(env, length) }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_ArrayOperationsDemo_NewShortArray(
    env: *mut JNIEnv,
    _class: jclass,
    length: jint,
) -> jshortArray {
    unsafe { ((*(*env)).v24.NewShortArray)(env, length) }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_ArrayOperationsDemo_NewIntArray(
    env: *mut JNIEnv,
    _class: jclass,
    length: jint,
) -> jintArray {
    unsafe { ((*(*env)).v24.NewIntArray)(env, length) }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_ArrayOperationsDemo_NewLongArray(
    env: *mut JNIEnv,
    _class: jclass,
    length: jint,
) -> jlongArray {
    unsafe { ((*(*env)).v24.NewLongArray)(env, length) }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_ArrayOperationsDemo_NewFloatArray(
    env: *mut JNIEnv,
    _class: jclass,
    length: jint,
) -> jfloatArray {
    unsafe { ((*(*env)).v24.NewFloatArray)(env, length) }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_ArrayOperationsDemo_NewDoubleArray(
    env: *mut JNIEnv,
    _class: jclass,
    length: jint,
) -> jdoubleArray {
    unsafe { ((*(*env)).v24.NewDoubleArray)(env, length) }
}
