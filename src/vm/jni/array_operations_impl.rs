use crate::vm::heap::heap::HEAP;
use crate::vm::helper::klass;
use jni_sys::{
    jarray, jbooleanArray, jbyteArray, jcharArray, jclass, jdoubleArray, jfloatArray, jint,
    jintArray, jlongArray, jobject, jobjectArray, jshortArray, jsize, JNIEnv,
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
    new_array_impl(len, "[Z") as jbooleanArray
}

pub(super) extern "system" fn new_byte_array(_env: *mut JNIEnv, len: jsize) -> jbyteArray {
    new_array_impl(len, "[B") as jbyteArray
}

pub(super) extern "system" fn new_char_array(_env: *mut JNIEnv, len: jsize) -> jcharArray {
    new_array_impl(len, "[C") as jcharArray
}

pub(super) extern "system" fn new_short_array(_env: *mut JNIEnv, len: jsize) -> jshortArray {
    new_array_impl(len, "[S") as jshortArray
}

pub(super) extern "system" fn new_int_array(_env: *mut JNIEnv, len: jsize) -> jintArray {
    new_array_impl(len, "[I") as jintArray
}

pub(super) extern "system" fn new_long_array(_env: *mut JNIEnv, len: jsize) -> jlongArray {
    new_array_impl(len, "[J") as jlongArray
}

pub(super) extern "system" fn new_float_array(_env: *mut JNIEnv, len: jsize) -> jfloatArray {
    new_array_impl(len, "[F") as jfloatArray
}

pub(super) extern "system" fn new_double_array(_env: *mut JNIEnv, len: jsize) -> jdoubleArray {
    new_array_impl(len, "[D") as jdoubleArray
}

fn new_array_impl(len: jsize, type_name: &str) -> jobjectArray {
    if len < 0 {
        panic!("Negative array length: {len}"); // todo throw NegativeArraySizeException here
    }

    HEAP.create_array(&type_name, len) as jarray
}
