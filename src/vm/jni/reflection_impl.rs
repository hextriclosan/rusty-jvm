use crate::vm::error::{Error, Result};
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::klass;
use crate::vm::jni::utils::{decode_method_id, encode_method_id, set_pending_internal_error};
use crate::vm::jni::{object_fields_impl, static_fields_impl};
use crate::vm::system_native::string::get_utf8_string_by_ref;
use jclassmodel::modifiers::FieldModifier;
use jni_sys::{jboolean, jclass, jfieldID, jmethodID, jobject, JNIEnv};
use std::ptr::null_mut;

pub(super) extern "system" fn from_reflected_field(
    _env: *mut JNIEnv,
    reflected: jobject,
) -> jfieldID {
    match from_reflected_field_impl(reflected as i32) {
        Ok(field) => field,
        Err(error) => {
            set_pending_internal_error(&format!("Failed to convert reflected field: {error}"));
            null_mut()
        }
    }
}

fn from_reflected_field_impl(reflected: i32) -> Result<jfieldID> {
    let class_ref = HEAP.get_object_field_value(reflected, "java/lang/reflect/Field", "clazz")?[0];
    let name_ref = HEAP.get_object_field_value(reflected, "java/lang/reflect/Field", "name")?[0];
    let name = get_utf8_string_by_ref(name_ref)?;
    let class = klass(class_ref)?;
    let field = class.field_info(&name).ok_or_else(|| {
        Error::new_execution("Reflected field is missing from its declaring class")
    })?;
    if field.flags() & FieldModifier::Static.bits() != 0 {
        Ok(static_fields_impl::encode_field_id(
            class_ref,
            class.get_static_field_offset(&name)?,
        ))
    } else {
        Ok(object_fields_impl::encode_field_id(
            class.get_field_offset(&format!("{}.{name}", class.this_class_name()))?,
        ))
    }
}

pub(super) extern "system" fn to_reflected_field(
    _env: *mut JNIEnv,
    target: jclass,
    field_id: jfieldID,
    is_static: jboolean,
) -> jobject {
    let result = if is_static {
        let (class_ref, offset) = static_fields_impl::decode_field_id(field_id);
        klass(class_ref).and_then(|class| {
            class
                .get_static_field_info_by_offset(offset)?
                .reflection_ref()
        })
    } else {
        klass(target as i32).and_then(|class| {
            let (owner, name) =
                class.get_field_name_by_offset(object_fields_impl::decode_field_id(field_id))?;
            let owner = crate::vm::method_area::loaded_classes::CLASSES.get(&owner)?;
            owner
                .field_info(&name)
                .ok_or_else(|| {
                    Error::new_execution("JNI field is missing from its declaring class")
                })?
                .reflection_ref()
        })
    };
    match result {
        Ok(reference) => reference as jobject,
        Err(error) => {
            set_pending_internal_error(&format!("Failed to create reflected field: {error}"));
            null_mut()
        }
    }
}

pub(super) extern "system" fn from_reflected_method(
    _env: *mut JNIEnv,
    reflected: jobject,
) -> jmethodID {
    let result = HEAP
        .get_instance_name(reflected as i32)
        .and_then(|class_name| {
            let declaring_class =
                HEAP.get_object_field_value(reflected as i32, &class_name, "clazz")?;
            let slot = HEAP.get_object_field_value(reflected as i32, &class_name, "slot")?;
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
    let (class_ref, method_index) = decode_method_id(method_id as usize);
    let result = klass(class_ref)
        .and_then(|class| class.get_method_by_index(method_index))
        .and_then(|method| method.reflection_ref());
    match result {
        Ok(reference) => reference as jobject,
        Err(error) => {
            set_pending_internal_error(&format!("Failed to create reflected method: {error}"));
            null_mut()
        }
    }
}
