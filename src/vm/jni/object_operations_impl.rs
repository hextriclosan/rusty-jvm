use crate::vm::heap::heap::HEAP;
use crate::vm::helper::{clazz_ref, klass};
use crate::vm::jni::utils::set_pending_instantiation_exception;
use crate::vm::method_area::method_area::with_method_area;
use jclassmodel::modifiers::ClassModifier;
use jni_sys::{jboolean, jclass, jobject, JNIEnv};
use std::ptr::null_mut;

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

pub(super) extern "system" fn alloc_object(_env: *mut JNIEnv, clazz: jclass) -> jobject {
    let target = klass(clazz as i32).expect("Failed to get class from reference");
    let modifiers = target.class_modifiers();
    if modifiers.intersects(ClassModifier::Interface | ClassModifier::Abstract) {
        set_pending_instantiation_exception(target.this_class_name());
        return null_mut();
    }

    let instance = with_method_area(|method_area| {
        method_area.create_instance_with_default_fields(target.this_class_name())
    })
    .expect("Failed to allocate object");
    HEAP.create_instance(instance) as jobject
}
