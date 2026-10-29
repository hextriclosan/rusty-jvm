use crate::vm::error::Result;
use crate::vm::execution_engine::static_init::StaticInit;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::{clazz_ref, klass};
use crate::vm::jni::utils::{
    set_pending_illegal_access_exception, set_pending_instantiation_exception,
    set_pending_internal_error, set_pending_null_pointer_exception,
};
use crate::vm::method_area::method_area::with_method_area;
use crate::vm::system_native::class::class_is_instance;
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

pub(super) extern "system" fn is_instance_of(
    _env: *mut JNIEnv,
    obj: jobject,
    clazz: jclass,
) -> jboolean {
    if obj.is_null() {
        return true as jboolean;
    }

    match class_is_instance(clazz as i32, obj as i32) {
        Ok(matches) => matches as jboolean,
        Err(error) => {
            set_pending_internal_error(&format!("Failed to check object instance: {error}"));
            false as jboolean
        }
    }
}

pub(super) extern "system" fn alloc_object(_env: *mut JNIEnv, clazz: jclass) -> jobject {
    if clazz.is_null() {
        set_pending_null_pointer_exception("Class is null");
        return null_mut();
    }

    match alloc_object_impl(clazz as i32) {
        Ok(obj_ref) => obj_ref as jobject,
        Err(error) => {
            set_pending_internal_error(&format!("Failed to allocate object: {error}"));
            null_mut()
        }
    }
}

/// Returns `0` with an exception pending when the class cannot be instantiated, like HotSpot's
/// `check_valid_for_instantiation`.
fn alloc_object_impl(clazz_ref: i32) -> Result<i32> {
    let target = klass(clazz_ref)?;
    // Primitive and array classes are synthetic and carry the `Abstract` modifier too.
    if target
        .class_modifiers()
        .intersects(ClassModifier::Interface | ClassModifier::Abstract)
    {
        set_pending_instantiation_exception(target.external_name());
        return Ok(0);
    }
    if target.this_class_name() == "java/lang/Class" {
        set_pending_illegal_access_exception(target.external_name());
        return Ok(0);
    }

    StaticInit::initialize_java_class(&target)?;
    let instance = with_method_area(|method_area| {
        method_area.create_instance_with_default_fields(target.this_class_name())
    })?;
    Ok(HEAP.create_instance(instance))
}
