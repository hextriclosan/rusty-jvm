use crate::vm::heap::heap::HEAP;
use crate::vm::helper::klass;
use crate::vm::jni::handles;
use crate::vm::jni::utils::{decode_method_id, encode_method_id, set_pending_internal_error};
use jni_sys::{jboolean, jclass, jmethodID, jobject, JNIEnv};
use std::ptr::null_mut;

pub(super) extern "system" fn from_reflected_method(
    _env: *mut JNIEnv,
    reflected: jobject,
) -> jmethodID {
    let _managed = crate::vm::safepoint::ManagedAccess::enter();
    let reflected = handles::resolve_or_return!(reflected);
    let result = HEAP.get_instance_name(reflected).and_then(|class_name| {
        let declaring_class = HEAP.get_object_field_value(reflected, &class_name, "clazz")?;
        let slot = HEAP.get_object_field_value(reflected, &class_name, "slot")?;
        Ok(encode_method_id(declaring_class[0], slot[0] as usize) as jmethodID)
    });
    match result {
        Ok(method_id) => method_id,
        Err(error) => {
            set_pending_internal_error(&format!("Failed to convert reflected method: {error}"));
            null_mut()
        }
    }
}

pub(super) extern "system" fn to_reflected_method(
    _env: *mut JNIEnv,
    _clazz: jclass,
    method_id: jmethodID,
    _is_static: jboolean,
) -> jobject {
    let _managed = crate::vm::safepoint::ManagedAccess::enter();
    let (class_ref, method_index) = decode_method_id(method_id as usize);
    let result = klass(class_ref)
        .and_then(|class| class.get_method_by_index(method_index))
        .and_then(|method| method.reflection_ref());
    match result {
        Ok(reference) => handles::new_local(reference),
        Err(error) => {
            set_pending_internal_error(&format!("Failed to create reflected method: {error}"));
            null_mut()
        }
    }
}
