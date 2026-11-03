use crate::vm::error::Result;
use crate::vm::execution_engine::executor::Executor;
use crate::vm::execution_engine::static_init::StaticInit;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::{clazz_ref, klass};
use crate::vm::jni::handles;
use crate::vm::jni::jni_invoke::jni_invoke_scalar;
use crate::vm::jni::utils::{
    decode_method_id, set_pending_illegal_access_exception, set_pending_instantiation_exception,
    set_pending_internal_error, set_pending_null_pointer_exception, set_pending_vm_error,
    transform_args_to_vec,
};
use crate::vm::method_area::instance_checker::InstanceChecker;
use crate::vm::method_area::method_area::with_method_area;
use crate::vm::stack::rooted_slots::RootedSlots;
use crate::vm::stack::slot::Slot;
use crate::vm::system_native::class::class_is_instance;
use jclassmodel::modifiers::ClassModifier;
use jni_sys::{jboolean, jclass, jmethodID, jobject, jobjectRefType, jvalue, JNIEnv};
use std::ptr::null_mut;

pub(super) extern "system" fn get_object_class(_env: *mut JNIEnv, obj: jobject) -> jclass {
    let _managed = crate::vm::safepoint::ManagedAccess::enter();
    let instance_name = HEAP
        .get_instance_name(handles::resolve_or_return!(obj))
        .expect("Failed to get instance name from reference");
    match clazz_ref(&instance_name) {
        Ok(reference) => handles::new_local(reference),
        Err(error) => {
            set_pending_vm_error(&error);
            null_mut()
        }
    }
}

pub(super) extern "system" fn is_same_object(
    _env: *mut JNIEnv,
    first: jobject,
    second: jobject,
) -> jboolean {
    let _managed = crate::vm::safepoint::ManagedAccess::enter();
    let first = handles::resolve_or_return!(first);
    let second = handles::resolve_or_return!(second);
    first == second
}

pub(super) extern "system" fn get_object_ref_type(
    _env: *mut JNIEnv,
    object: jobject,
) -> jobjectRefType {
    let _managed = crate::vm::safepoint::ManagedAccess::enter();
    handles::reference_type(object)
}

pub(super) extern "system" fn is_instance_of(
    _env: *mut JNIEnv,
    obj: jobject,
    clazz: jclass,
) -> jboolean {
    let _managed = crate::vm::safepoint::ManagedAccess::enter();
    if obj.is_null() {
        return true as jboolean;
    }

    match class_is_instance(
        handles::resolve_or_return!(clazz),
        handles::resolve_or_return!(obj),
    ) {
        Ok(matches) => matches as jboolean,
        Err(error) => {
            if error.throwable_ref().is_some() {
                set_pending_vm_error(&error);
                return false as jboolean;
            }
            set_pending_internal_error(&format!("Failed to check object instance: {error}"));
            false as jboolean
        }
    }
}

pub(super) extern "system" fn is_virtual_thread(_env: *mut JNIEnv, thread: jobject) -> jboolean {
    let _managed = crate::vm::safepoint::ManagedAccess::enter();
    if thread.is_null() {
        return false as jboolean;
    }

    match HEAP
        .get_instance_name(handles::resolve_or_return!(thread))
        .and_then(|class_name| {
            InstanceChecker::checkcast(&class_name, "java/lang/BaseVirtualThread")
        }) {
        Ok(is_virtual) => is_virtual as jboolean,
        Err(error) => {
            if error.throwable_ref().is_some() {
                set_pending_vm_error(&error);
                return false as jboolean;
            }
            set_pending_internal_error(&format!("Failed to inspect thread type: {error}"));
            false as jboolean
        }
    }
}

pub(super) extern "system" fn alloc_object(_env: *mut JNIEnv, clazz: jclass) -> jobject {
    let _managed = crate::vm::safepoint::ManagedAccess::enter();
    if clazz.is_null() {
        set_pending_null_pointer_exception("Class is null");
        return null_mut();
    }

    let mut destination = RootedSlots::new();
    match alloc_object_impl(handles::resolve_or_return!(clazz), &mut destination) {
        Ok(obj_ref) => handles::new_local(obj_ref),
        Err(error) => {
            if let Some(throwable) = error.throwable_ref() {
                crate::vm::jni::java_thread::JavaThread::set_pending_if_empty(throwable);
            } else {
                set_pending_internal_error(&format!("Failed to allocate object: {error}"));
            }
            null_mut()
        }
    }
}

/// Returns `0` with an exception pending when the class cannot be instantiated, like HotSpot's
/// `check_valid_for_instantiation`.
fn alloc_object_impl(clazz_ref: i32, destination: &mut RootedSlots) -> Result<i32> {
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
    HEAP.create_instance(instance, destination)
}

/// Allocates like [`alloc_object`] and then runs the constructor `method_id` names on the new
/// object, like HotSpot's `NewObjectA`.
pub(super) extern "system" fn new_object_a(
    env: *mut JNIEnv,
    clazz: jclass,
    method_id: jmethodID,
    args: *const jvalue,
) -> jobject {
    let _managed = crate::vm::safepoint::ManagedAccess::enter();
    let obj = alloc_object(env, clazz);
    if obj.is_null() {
        return null_mut();
    }
    let obj_ref = handles::resolve_or_return!(obj);
    let destination = RootedSlots::from_slots(vec![Slot::Ref(obj_ref)]);

    let (declaring_class_ref, method_index) = decode_method_id(method_id as usize);
    let constructor = match klass(declaring_class_ref)
        .and_then(|class| class.get_method_by_index(method_index))
    {
        Ok(constructor) => constructor,
        Err(error) => {
            set_pending_internal_error(&format!("Failed to resolve constructor: {error}"));
            return null_mut();
        }
    };
    let Some(arguments) = transform_args_to_vec(&constructor, args) else {
        return null_mut();
    };
    let context = format!(
        "{}.{}",
        constructor.class_name(),
        constructor.name_signature()
    );
    let result = Executor::invoke_non_static_method(
        constructor.class_name(),
        constructor.name_signature(),
        obj_ref,
        &arguments,
    )
    .map(|_| destination);
    jni_invoke_scalar(result, &context)
}
