use crate::vm::heap::heap::HEAP;
use crate::vm::helper::clazz_ref;
use crate::vm::jni::utils::set_pending_internal_error;
use crate::vm::method_area::instance_checker::InstanceChecker;
use jni_sys::{jboolean, jclass, jobject, JNIEnv};

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

pub(super) extern "system" fn is_virtual_thread(_env: *mut JNIEnv, thread: jobject) -> jboolean {
    if thread.is_null() {
        return false as jboolean;
    }

    match HEAP
        .get_instance_name(thread as i32)
        .and_then(|class_name| {
            InstanceChecker::checkcast(&class_name, "java/lang/BaseVirtualThread")
        }) {
        Ok(is_virtual) => is_virtual as jboolean,
        Err(error) => {
            set_pending_internal_error(&format!("Failed to inspect thread type: {error}"));
            false as jboolean
        }
    }
}
