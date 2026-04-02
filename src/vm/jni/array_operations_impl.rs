use crate::vm::heap::heap::HEAP;
use crate::vm::helper::klass;
use jni_sys::{jarray, jclass, jint, jobject, jobjectArray, jsize, JNIEnv};

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
    array: jobject,
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
