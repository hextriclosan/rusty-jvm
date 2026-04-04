use jni::sys::{
    jarray, jboolean, jbooleanArray, jbyte, jbyteArray, jchar, jcharArray, jclass, jdouble,
    jdoubleArray, jfloat, jfloatArray, jint, jintArray, jlong, jlongArray, jobject, jshort,
    jshortArray, jsize, JNIEnv, JNI_ABORT, JNI_FALSE,
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

macro_rules! impl_set_array_region_demo {
    ($name:ident, $jni_ty:ty, $array_ty:ty, $get:ident, $set:ident, $release:ident) => {
        #[no_mangle]
        pub extern "system" fn $name(
            env: *mut JNIEnv,
            _class: jclass,
            dest: $array_ty,
            start: jint,
            length: jint,
            source: $array_ty,
        ) {
            unsafe {
                set_array_region_demo::<$jni_ty>(
                    env,
                    _class,
                    dest as jarray,
                    start,
                    length,
                    source as jarray,
                    (*(*env)).v24.$get,
                    (*(*env)).v24.$set,
                    (*(*env)).v24.$release,
                )
            }
        }
    };
}
impl_set_array_region_demo!(
    Java_samples_javacore_loadlibrary_example_ArrayOperationsDemo_SetBooleanArrayRegionDemo,
    jboolean,
    jbooleanArray,
    GetBooleanArrayElements,
    SetBooleanArrayRegion,
    ReleaseBooleanArrayElements
);
impl_set_array_region_demo!(
    Java_samples_javacore_loadlibrary_example_ArrayOperationsDemo_SetByteArrayRegionDemo,
    jbyte,
    jbyteArray,
    GetByteArrayElements,
    SetByteArrayRegion,
    ReleaseByteArrayElements
);
impl_set_array_region_demo!(
    Java_samples_javacore_loadlibrary_example_ArrayOperationsDemo_SetCharArrayRegionDemo,
    jchar,
    jcharArray,
    GetCharArrayElements,
    SetCharArrayRegion,
    ReleaseCharArrayElements
);
impl_set_array_region_demo!(
    Java_samples_javacore_loadlibrary_example_ArrayOperationsDemo_SetShortArrayRegionDemo,
    jshort,
    jshortArray,
    GetShortArrayElements,
    SetShortArrayRegion,
    ReleaseShortArrayElements
);
impl_set_array_region_demo!(
    Java_samples_javacore_loadlibrary_example_ArrayOperationsDemo_SetIntArrayRegionDemo,
    jint,
    jintArray,
    GetIntArrayElements,
    SetIntArrayRegion,
    ReleaseIntArrayElements
);
impl_set_array_region_demo!(
    Java_samples_javacore_loadlibrary_example_ArrayOperationsDemo_SetLongArrayRegionDemo,
    jlong,
    jlongArray,
    GetLongArrayElements,
    SetLongArrayRegion,
    ReleaseLongArrayElements
);
impl_set_array_region_demo!(
    Java_samples_javacore_loadlibrary_example_ArrayOperationsDemo_SetFloatArrayRegionDemo,
    jfloat,
    jfloatArray,
    GetFloatArrayElements,
    SetFloatArrayRegion,
    ReleaseFloatArrayElements
);
impl_set_array_region_demo!(
    Java_samples_javacore_loadlibrary_example_ArrayOperationsDemo_SetDoubleArrayRegionDemo,
    jdouble,
    jdoubleArray,
    GetDoubleArrayElements,
    SetDoubleArrayRegion,
    ReleaseDoubleArrayElements
);
unsafe fn set_array_region_demo<T: Copy + std::fmt::Debug>(
    env: *mut JNIEnv,
    _class: jclass,
    dest: jarray,
    start: jint,
    length: jint,
    source: jarray,
    get_fn: unsafe extern "system" fn(*mut JNIEnv, jarray, *mut jboolean) -> *mut T,
    set_fn: unsafe extern "system" fn(*mut JNIEnv, jarray, jsize, jsize, *const T),
    release_fn: unsafe extern "system" fn(*mut JNIEnv, jarray, *mut T, jint),
) {
    let source_ptr = get_fn(env, source, std::ptr::null_mut());
    set_fn(env, dest, start, length, source_ptr as *const T);
    release_fn(env, source, source_ptr, JNI_ABORT);
}

macro_rules! impl_get_array_region_demo {
    ($name:ident, $jni_ty:ty, $array_ty:ty, $new:ident, $get:ident, $get_region:ident, $release:ident) => {
        #[no_mangle]
        pub extern "system" fn $name(
            env: *mut JNIEnv,
            _class: jclass,
            from: $array_ty,
            start: jint,
            length: jint,
        ) -> $array_ty {
            unsafe {
                get_array_region_demo::<$jni_ty>(
                    env,
                    _class,
                    from as jarray,
                    start,
                    length,
                    (*(*env)).v24.$new,
                    (*(*env)).v24.$get,
                    (*(*env)).v24.$get_region,
                    (*(*env)).v24.$release,
                )
            }
        }
    };
}
impl_get_array_region_demo!(
    Java_samples_javacore_loadlibrary_example_ArrayOperationsDemo_GetBooleanArrayRegionDemo,
    jboolean,
    jbooleanArray,
    NewBooleanArray,
    GetBooleanArrayElements,
    GetBooleanArrayRegion,
    ReleaseBooleanArrayElements
);
impl_get_array_region_demo!(
    Java_samples_javacore_loadlibrary_example_ArrayOperationsDemo_GetByteArrayRegionDemo,
    jbyte,
    jbyteArray,
    NewByteArray,
    GetByteArrayElements,
    GetByteArrayRegion,
    ReleaseByteArrayElements
);
impl_get_array_region_demo!(
    Java_samples_javacore_loadlibrary_example_ArrayOperationsDemo_GetCharArrayRegionDemo,
    jchar,
    jcharArray,
    NewCharArray,
    GetCharArrayElements,
    GetCharArrayRegion,
    ReleaseCharArrayElements
);
impl_get_array_region_demo!(
    Java_samples_javacore_loadlibrary_example_ArrayOperationsDemo_GetShortArrayRegionDemo,
    jshort,
    jshortArray,
    NewShortArray,
    GetShortArrayElements,
    GetShortArrayRegion,
    ReleaseShortArrayElements
);
impl_get_array_region_demo!(
    Java_samples_javacore_loadlibrary_example_ArrayOperationsDemo_GetIntArrayRegionDemo,
    jint,
    jintArray,
    NewIntArray,
    GetIntArrayElements,
    GetIntArrayRegion,
    ReleaseIntArrayElements
);
impl_get_array_region_demo!(
    Java_samples_javacore_loadlibrary_example_ArrayOperationsDemo_GetLongArrayRegionDemo,
    jlong,
    jlongArray,
    NewLongArray,
    GetLongArrayElements,
    GetLongArrayRegion,
    ReleaseLongArrayElements
);
impl_get_array_region_demo!(
    Java_samples_javacore_loadlibrary_example_ArrayOperationsDemo_GetFloatArrayRegionDemo,
    jfloat,
    jfloatArray,
    NewFloatArray,
    GetFloatArrayElements,
    GetFloatArrayRegion,
    ReleaseFloatArrayElements
);
impl_get_array_region_demo!(
    Java_samples_javacore_loadlibrary_example_ArrayOperationsDemo_GetDoubleArrayRegionDemo,
    jdouble,
    jdoubleArray,
    NewDoubleArray,
    GetDoubleArrayElements,
    GetDoubleArrayRegion,
    ReleaseDoubleArrayElements
);
unsafe fn get_array_region_demo<T: Copy + std::fmt::Debug>(
    env: *mut JNIEnv,
    _class: jclass,
    from: jarray,
    start: jint,
    length: jint,
    new_fn: unsafe extern "system" fn(*mut JNIEnv, jsize) -> jarray,
    get_fn: unsafe extern "system" fn(*mut JNIEnv, jarray, *mut jboolean) -> *mut T,
    get_region_fn: unsafe extern "system" fn(*mut JNIEnv, jarray, jsize, jsize, *mut T),
    release_fn: unsafe extern "system" fn(*mut JNIEnv, jarray, *mut T, jint),
) -> jarray {
    let new_array = new_fn(env, length);
    let buffer = get_fn(env, new_array, std::ptr::null_mut());
    get_region_fn(env, from, start, length, buffer);
    release_fn(env, new_array, buffer, 0);

    new_array
}
