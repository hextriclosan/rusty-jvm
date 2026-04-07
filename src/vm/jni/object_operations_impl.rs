use crate::vm::heap::heap::HEAP;
use crate::vm::helper::clazz_ref;
use jni_sys::{jclass, jobject, JNIEnv};

pub(super) extern "system" fn get_object_class(_env: *mut JNIEnv, obj: jobject) -> jclass {
    let instance_name = HEAP
        .get_instance_name(obj as i32)
        .expect("Failed to get instance name from reference");
    clazz_ref(&instance_name).expect("Failed to get class reference from instance name") as jclass
}
