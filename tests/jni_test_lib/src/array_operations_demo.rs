use jni::sys::{
    jarray, jboolean, jbooleanArray, jbyte, jbyteArray, jchar, jcharArray, jclass, jdouble,
    jdoubleArray, jfloat, jfloatArray, jint, jintArray, jlong, jlongArray, jobject, jshort,
    jshortArray, JNIEnv, JNI_FALSE, JNI_TRUE,
};
use std::slice;

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

macro_rules! impl_array_demo {
    ($name:ident, $jni_ty:ty, $array_ty:ty, $get:ident, $release:ident) => {
        #[no_mangle]
        pub extern "system" fn $name(env: *mut JNIEnv, _class: jclass, array: $array_ty) {
            unsafe {
                get_and_release_array_demo::<$jni_ty>(
                    env,
                    array as jarray,
                    (*(*env)).v24.$get,
                    (*(*env)).v24.$release,
                    stringify!($name),
                );
            }
        }
    };
}
impl_array_demo!(
    Java_samples_javacore_loadlibrary_example_ArrayOperationsDemo_GetAndReleaseBooleanArrayDemo,
    jboolean,
    jbooleanArray,
    GetBooleanArrayElements,
    ReleaseBooleanArrayElements
);
impl_array_demo!(
    Java_samples_javacore_loadlibrary_example_ArrayOperationsDemo_GetAndReleaseByteArrayDemo,
    jbyte,
    jbyteArray,
    GetByteArrayElements,
    ReleaseByteArrayElements
);
impl_array_demo!(
    Java_samples_javacore_loadlibrary_example_ArrayOperationsDemo_GetAndReleaseCharArrayDemo,
    jchar,
    jcharArray,
    GetCharArrayElements,
    ReleaseCharArrayElements
);
impl_array_demo!(
    Java_samples_javacore_loadlibrary_example_ArrayOperationsDemo_GetAndReleaseShortArrayDemo,
    jshort,
    jshortArray,
    GetShortArrayElements,
    ReleaseShortArrayElements
);
impl_array_demo!(
    Java_samples_javacore_loadlibrary_example_ArrayOperationsDemo_GetAndReleaseIntArrayDemo,
    jint,
    jintArray,
    GetIntArrayElements,
    ReleaseIntArrayElements
);
impl_array_demo!(
    Java_samples_javacore_loadlibrary_example_ArrayOperationsDemo_GetAndReleaseLongArrayDemo,
    jlong,
    jlongArray,
    GetLongArrayElements,
    ReleaseLongArrayElements
);
impl_array_demo!(
    Java_samples_javacore_loadlibrary_example_ArrayOperationsDemo_GetAndReleaseFloatArrayDemo,
    jfloat,
    jfloatArray,
    GetFloatArrayElements,
    ReleaseFloatArrayElements
);
impl_array_demo!(
    Java_samples_javacore_loadlibrary_example_ArrayOperationsDemo_GetAndReleaseDoubleArrayDemo,
    jdouble,
    jdoubleArray,
    GetDoubleArrayElements,
    ReleaseDoubleArrayElements
);
unsafe fn get_and_release_array_demo<T: Copy + std::fmt::Debug>(
    env: *mut JNIEnv,
    array: jarray,
    get_fn: unsafe extern "system" fn(*mut JNIEnv, jarray, *mut jboolean) -> *mut T,
    release_fn: unsafe extern "system" fn(*mut JNIEnv, jarray, *mut T, jint),
    label: &str,
) {
    let array_len = ((*(*env)).v24.GetArrayLength)(env, array);

    let mut is_copy = JNI_FALSE;
    let raw_ptr = get_fn(env, array, &mut is_copy);

    let slice = slice::from_raw_parts_mut(raw_ptr, array_len as usize);

    println!("JNI: {label}: value={slice:?}, is_copy={is_copy}");

    // demo mutation (swap 0 and 2)
    if slice.len() >= 3 {
        let one = slice[0];
        let three = slice[2];
        slice[0] = three;
        slice[2] = one;
    }

    release_fn(env, array, raw_ptr, 0);
}
