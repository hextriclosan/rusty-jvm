use crate::vm::heap::heap::HEAP;
use crate::vm::helper::klass;
use jni_sys::{
    jarray, jboolean, jbooleanArray, jbyte, jbyteArray, jchar, jcharArray, jclass, jdouble,
    jdoubleArray, jfloat, jfloatArray, jint, jintArray, jlong, jlongArray, jobject, jobjectArray,
    jshort, jshortArray, jsize, JNIEnv, JNI_ABORT, JNI_COMMIT, JNI_TRUE,
};

pub(super) extern "system" fn get_array_length(_env: *mut JNIEnv, input: jarray) -> jint {
    let array_ref = input as i32;
    if array_ref == 0 {
        panic!("Invalid array reference"); // OpenJDK crashes here, why we shouldn't
    }

    HEAP.get_array_len(array_ref).unwrap_or(0) as jint // OpenJDK returns 0 for non-arrays
}

pub(super) extern "system" fn new_object_array(
    _env: *mut JNIEnv,
    len: jsize,
    clazz: jclass,
    init: jobject,
) -> jobjectArray {
    let clazz_ref = clazz as i32;
    let init_obj_ref = init as i32;

    if clazz_ref == 0 {
        panic!("Class reference is null"); // OpenJDK crashes here, why we shouldn't
    }

    if len < 0 {
        panic!("Negative array length: {len}"); // todo throw NegativeArraySizeException here
    }

    if HEAP
        .get_instance_name(clazz_ref)
        .expect("Invalid class reference")
        != "java/lang/Class"
    {
        panic!("Class reference is not a Class"); // OpenJDK crashes here, why we shouldn't
    }

    let klass = klass(clazz_ref).expect("Failed to get class from reference");
    let class_name = klass.this_class_name();
    let type_name = if class_name.starts_with("[") {
        format!("[{class_name}")
    } else {
        format!("[L{};", class_name)
    };

    let arr_ref = if init_obj_ref == 0 {
        HEAP.create_array(&type_name, len)
    } else {
        HEAP.create_array_with_values(&type_name, vec![init_obj_ref; len as usize].as_ref())
    };

    arr_ref as jobjectArray
}

pub(super) extern "system" fn get_object_array_element(
    env: *mut JNIEnv,
    array: jobjectArray,
    index: jsize,
) -> jobject {
    let array_ref = array as i32;
    if array_ref == 0 {
        panic!("Invalid array reference"); // OpenJDK crashes here, why we shouldn't
    }

    let len = get_array_length(env, array);
    if index >= len || index < 0 {
        panic!("Out of bounds array index: index={index}, length={len}"); // todo: throw java.lang.ArrayIndexOutOfBoundsException here
    }

    let raw = HEAP
        .get_array_value(array_ref, index as i32)
        .expect("Failed to get array element");
    raw[0] as jobject
}

pub(super) extern "system" fn set_object_array_element(
    env: *mut JNIEnv,
    array: jobjectArray,
    index: jsize,
    value: jobject,
) {
    let array_ref = array as i32;
    if array_ref == 0 {
        panic!("Invalid array reference"); // OpenJDK crashes here, why we shouldn't
    }

    let len = get_array_length(env, array);
    if index >= len || index < 0 {
        panic!("Out of bounds array index: index={index}, length={len}"); // todo: throw java.lang.ArrayIndexOutOfBoundsException here
    }

    HEAP.set_array_value(array_ref, index as i32, vec![value as i32])
        .expect("Failed to set array element");
}

pub(super) extern "system" fn new_boolean_array(_env: *mut JNIEnv, len: jsize) -> jbooleanArray {
    new_primitive_type_array_impl(len, "[Z") as jbooleanArray
}

pub(super) extern "system" fn new_byte_array(_env: *mut JNIEnv, len: jsize) -> jbyteArray {
    new_primitive_type_array_impl(len, "[B") as jbyteArray
}

pub(super) extern "system" fn new_char_array(_env: *mut JNIEnv, len: jsize) -> jcharArray {
    new_primitive_type_array_impl(len, "[C") as jcharArray
}

pub(super) extern "system" fn new_short_array(_env: *mut JNIEnv, len: jsize) -> jshortArray {
    new_primitive_type_array_impl(len, "[S") as jshortArray
}

pub(super) extern "system" fn new_int_array(_env: *mut JNIEnv, len: jsize) -> jintArray {
    new_primitive_type_array_impl(len, "[I") as jintArray
}

pub(super) extern "system" fn new_long_array(_env: *mut JNIEnv, len: jsize) -> jlongArray {
    new_primitive_type_array_impl(len, "[J") as jlongArray
}

pub(super) extern "system" fn new_float_array(_env: *mut JNIEnv, len: jsize) -> jfloatArray {
    new_primitive_type_array_impl(len, "[F") as jfloatArray
}

pub(super) extern "system" fn new_double_array(_env: *mut JNIEnv, len: jsize) -> jdoubleArray {
    new_primitive_type_array_impl(len, "[D") as jdoubleArray
}

fn new_primitive_type_array_impl(len: jsize, type_name: &str) -> jarray {
    if len < 0 {
        panic!("Negative array length: {len}"); // todo throw NegativeArraySizeException here
    }

    HEAP.create_array(&type_name, len) as jarray
}

macro_rules! impl_get_array {
    ($name:ident, $jni_ty:ty, $array_ty:ty) => {
        pub(super) extern "system" fn $name(
            _env: *mut JNIEnv,
            array: $array_ty,
            is_copy: *mut jboolean,
        ) -> *mut $jni_ty {
            get_primitive_type_array_elements::<$jni_ty>(array as i32, is_copy)
        }
    };
}
impl_get_array!(get_boolean_array_elements, jboolean, jbooleanArray);
impl_get_array!(get_byte_array_elements, jbyte, jbyteArray);
impl_get_array!(get_char_array_elements, jchar, jcharArray);
impl_get_array!(get_short_array_elements, jshort, jshortArray);
impl_get_array!(get_int_array_elements, jint, jintArray);
impl_get_array!(get_long_array_elements, jlong, jlongArray);
impl_get_array!(get_float_array_elements, jfloat, jfloatArray);
impl_get_array!(get_double_array_elements, jdouble, jdoubleArray);
fn get_primitive_type_array_elements<T: Copy>(array_ref: i32, is_copy: *mut jboolean) -> *mut T {
    if array_ref == 0 {
        panic!("Invalid array reference"); // OpenJDK crashes here, why we shouldn't
    }

    let raw_data = HEAP
        .get_entire_raw_data(array_ref)
        .expect("Failed to get array elements")
        .value()
        .clone();
    let boxed_slice = raw_data.into_boxed_slice();
    let raw_ptr = Box::into_raw(boxed_slice) as *mut u8 as *mut T;

    if !is_copy.is_null() {
        unsafe {
            *is_copy = JNI_TRUE; // we always return a copy
        }
    }

    raw_ptr
}

macro_rules! impl_release_array {
    ($name:ident, $jni_ty:ty, $array_ty:ty) => {
        pub(super) extern "system" fn $name(
            env: *mut JNIEnv,
            array: $array_ty,
            elems: *mut $jni_ty,
            mode: jint,
        ) {
            release_primitive_type_array_elements::<$jni_ty>(env, array as jarray, elems, mode)
        }
    };
}
impl_release_array!(release_boolean_array_elements, jboolean, jbooleanArray);
impl_release_array!(release_byte_array_elements, jbyte, jbyteArray);
impl_release_array!(release_char_array_elements, jchar, jcharArray);
impl_release_array!(release_short_array_elements, jshort, jshortArray);
impl_release_array!(release_int_array_elements, jint, jintArray);
impl_release_array!(release_long_array_elements, jlong, jlongArray);
impl_release_array!(release_float_array_elements, jfloat, jfloatArray);
impl_release_array!(release_double_array_elements, jdouble, jdoubleArray);
fn release_primitive_type_array_elements<T>(
    env: *mut JNIEnv,
    array: jarray,
    elems: *mut T,
    mode: jint,
) {
    let array_ref = array as i32;
    if array_ref == 0 {
        panic!("Invalid array reference"); // OpenJDK crashes here, why we shouldn't
    }

    let elems = elems as *mut u8;
    let len_in_bytes = get_array_length(env, array) as usize * size_of::<T>();

    match mode {
        0 => {
            write_to_array(array_ref, elems, 0, len_in_bytes);
            free_buffer(elems, len_in_bytes);
        }
        JNI_COMMIT => {
            write_to_array(array_ref, elems, 0, len_in_bytes);
        }
        JNI_ABORT => {
            free_buffer(elems, len_in_bytes);
        }
        _ => panic!("Invalid mode: {mode}"),
    };
}

macro_rules! impl_set_array_region {
    ($name:ident, $jni_ty:ty, $array_ty:ty) => {
        pub(super) extern "system" fn $name(
            env: *mut JNIEnv,
            array: $array_ty,
            start: jsize,
            len: jsize,
            buf: *const $jni_ty,
        ) {
            set_primitive_type_array_region::<$jni_ty>(env, array as jarray, start, len, buf)
        }
    };
}
impl_set_array_region!(set_boolean_array_region, jboolean, jbooleanArray);
impl_set_array_region!(set_byte_array_region, jbyte, jbyteArray);
impl_set_array_region!(set_char_array_region, jchar, jcharArray);
impl_set_array_region!(set_short_array_region, jshort, jshortArray);
impl_set_array_region!(set_int_array_region, jint, jintArray);
impl_set_array_region!(set_long_array_region, jlong, jlongArray);
impl_set_array_region!(set_float_array_region, jfloat, jfloatArray);
impl_set_array_region!(set_double_array_region, jdouble, jdoubleArray);
fn set_primitive_type_array_region<T>(
    _env: *mut JNIEnv,
    array: jarray,
    start: jsize,
    len: jsize,
    buf: *const T,
) {
    let array_ref = array as i32;
    if array_ref == 0 {
        panic!("Invalid array reference"); // OpenJDK crashes here, why we shouldn't
    }

    let array_len = get_array_length(_env, array) as usize;
    if start < 0 {
        panic!("Negative start index: {start}"); // todo throw ArrayIndexOutOfBoundsException here
    }
    if len < 0 {
        panic!("Negative length: {len}"); // todo throw ArrayIndexOutOfBoundsException here
    }

    if start as usize > array_len {
        panic!("Start index out of bounds: start={start}, array length={array_len}");
        // todo throw ArrayIndexOutOfBoundsException here
    }
    if (start + len) as usize > array_len {
        panic!("End index out of bounds: start={start}, len={len}, array length={array_len}");
        // todo throw ArrayIndexOutOfBoundsException here
    }

    if len == 0 {
        return;
    }
    let byte_buf = buf as *const u8;
    if byte_buf.is_null() {
        panic!("Invalid buffer: null pointer with non-zero length");
    }
    let start_in_bytes = start as usize * size_of::<T>();
    let len_in_bytes = len as usize * size_of::<T>();
    write_to_array(array_ref, byte_buf, start_in_bytes, len_in_bytes);
}

macro_rules! impl_get_array_region {
    ($name:ident, $jni_ty:ty, $array_ty:ty) => {
        pub(super) extern "system" fn $name(
            env: *mut JNIEnv,
            array: $array_ty,
            start: jsize,
            len: jsize,
            buf: *mut $jni_ty,
        ) {
            get_primitive_type_array_region::<$jni_ty>(env, array as jarray, start, len, buf)
        }
    };
}
impl_get_array_region!(get_boolean_array_region, jboolean, jbooleanArray);
impl_get_array_region!(get_byte_array_region, jbyte, jbyteArray);
impl_get_array_region!(get_char_array_region, jchar, jcharArray);
impl_get_array_region!(get_short_array_region, jshort, jshortArray);
impl_get_array_region!(get_int_array_region, jint, jintArray);
impl_get_array_region!(get_long_array_region, jlong, jlongArray);
impl_get_array_region!(get_float_array_region, jfloat, jfloatArray);
impl_get_array_region!(get_double_array_region, jdouble, jdoubleArray);
pub(super) extern "system" fn get_primitive_type_array_region<T>(
    _env: *mut JNIEnv,
    array: jarray,
    start: jsize,
    len: jsize,
    buf: *mut T,
) {
    let array_ref = array as i32;
    if array_ref == 0 {
        panic!("Invalid array reference"); // OpenJDK crashes here, why we shouldn't
    }

    let array_len = get_array_length(_env, array) as usize;
    if start < 0 {
        panic!("Negative start index: {start}"); // todo throw ArrayIndexOutOfBoundsException here
    }
    if len < 0 {
        panic!("Negative length: {len}"); // todo throw ArrayIndexOutOfBoundsException here
    }

    if start as usize > array_len {
        panic!("Start index out of bounds: start={start}, array length={array_len}");
        // todo throw ArrayIndexOutOfBoundsException here
    }
    if (start + len) as usize > array_len {
        panic!("End index out of bounds: start={start}, len={len}, array length={array_len}");
        // todo throw ArrayIndexOutOfBoundsException here
    }

    if len == 0 {
        return;
    }
    let byte_buf = buf as *mut u8;
    if byte_buf.is_null() {
        panic!("Invalid buffer: null pointer with non-zero length");
    }
    let start_in_bytes = start as usize * size_of::<T>();
    let len_in_bytes = len as usize * size_of::<T>();
    read_from_array(array_ref, byte_buf, start_in_bytes, len_in_bytes);
}

fn free_buffer(elems: *mut u8, len: usize) {
    unsafe {
        let _boxed: Box<_> = Box::from_raw(std::slice::from_raw_parts_mut(elems, len));
    }
}

fn write_to_array(array_ref: i32, elems: *const u8, start: usize, len: usize) {
    let slice = unsafe { std::slice::from_raw_parts(elems, len) };
    let mut guard = HEAP
        .get_entire_raw_data_mut(array_ref)
        .expect("Failed to commit array elements");
    let _ = &guard[start..(start + len)].copy_from_slice(slice);
}

fn read_from_array(array_ref: i32, elems: *mut u8, start: usize, len: usize) {
    let guard = HEAP
        .get_entire_raw_data(array_ref)
        .expect("Failed to read array elements");
    let slice = &guard[start..(start + len)];
    unsafe {
        std::ptr::copy_nonoverlapping(slice.as_ptr(), elems, len);
    }
}
