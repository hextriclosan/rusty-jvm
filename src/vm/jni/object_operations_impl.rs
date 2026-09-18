use crate::vm::heap::heap::HEAP;
use crate::vm::helper::{clazz_ref, klass};
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

pub(super) extern "system" fn is_instance_of(
    _env: *mut JNIEnv,
    obj: jobject,
    clazz: jclass,
) -> jboolean {
    if obj.is_null() {
        return true as jboolean;
    }

    let result = HEAP
        .get_instance_name(obj as i32)
        .and_then(|instance_name| {
            let target = klass(clazz as i32)?;
            InstanceChecker::checkcast(&instance_name, target.this_class_name())
        });
    match result {
        Ok(matches) => matches as jboolean,
        Err(error) => {
            set_pending_internal_error(&format!("Failed to check object instance: {error}"));
            false as jboolean
        }
    }
}
