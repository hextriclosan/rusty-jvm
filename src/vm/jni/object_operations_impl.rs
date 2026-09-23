use crate::vm::heap::heap::HEAP;
use crate::vm::helper::clazz_ref;
use jni_sys::{jboolean, jclass, jobject, jobjectRefType, JNIEnv};

pub(super) extern "system" fn get_object_class(_env: *mut JNIEnv, obj: jobject) -> jclass {
    let instance_name = HEAP
        .get_instance_name(obj as i32)
        .expect("Failed to get instance name from reference");
    clazz_ref(&instance_name).expect("Failed to get class reference from instance name") as jclass
}

pub(super) extern "system" fn is_same_object(
    _env: *mut JNIEnv,
    first: jobject,
    second: jobject,
) -> jboolean {
    (first == second) as jboolean
}

pub(super) extern "system" fn get_object_ref_type(
    _env: *mut JNIEnv,
    object: jobject,
) -> jobjectRefType {
    if object.is_null() || HEAP.get_instance_name(object as i32).is_err() {
        jobjectRefType::JNIInvalidRefType
    } else {
        jobjectRefType::JNILocalRefType
    }
}
