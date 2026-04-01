use crate::vm::heap::heap::HEAP;
use jni_sys::{jarray, jint, JNIEnv};

pub(super) extern "system" fn get_array_length(_env: *mut JNIEnv, input: jarray) -> jint {
    let array_ref = input as i32;
    if array_ref == 0 {
        panic!("Invalid array reference"); // OpenJDK crashes here, why we shouldn't
    }

    HEAP.get_array_len(array_ref).unwrap_or(0) as jint // OpenJDK returns 0 for non-arrays
}
