use crate::from_mutf8_ptr;
use crate::vm::helper::{clazz_ref, klass};
use crate::vm::jni::utils::{
    set_pending_class_format_error, set_pending_internal_error,
    set_pending_no_class_def_found_error,
};
use crate::vm::method_area::instance_checker::InstanceChecker;
use crate::vm::method_area::method_area::with_method_area;
use crate::vm::method_area::primitives_helper::PRIMITIVE_TYPE_BY_CODE;
use jclassmodel::parse;
use jdescriptor::TypeDescriptor;
use jni_sys::{jboolean, jbyte, jclass, jobject, jsize, JNIEnv};
use std::ffi::{c_char, CStr};
use std::ptr::null_mut;

pub(super) extern "system" fn find_class(_env: *mut JNIEnv, name_mutf8: *const c_char) -> jclass {
    if name_mutf8.is_null() {
        set_pending_no_class_def_found_error("Failed to find class: null pointer for class name");
        return null_mut();
    }

    let name = match from_mutf8_ptr!(name_mutf8) {
        Ok(name) => name,
        Err(e) => {
            let bytes = unsafe { CStr::from_ptr(name_mutf8) }.to_bytes_with_nul();
            set_pending_class_format_error(&format!(
                "Failed to construct classname from bytes {bytes:?}: {e}"
            ));
            return null_mut();
        }
    };

    if !is_valid_find_class_name(&name) {
        set_pending_no_class_def_found_error(&name);
        return null_mut();
    }

    match clazz_ref(&name) {
        Ok(clazz) => clazz as jclass,
        Err(_) => {
            set_pending_no_class_def_found_error(&name);
            null_mut()
        }
    }
}

pub(super) extern "system" fn define_class(
    _env: *mut JNIEnv,
    name_mutf8: *const c_char,
    loader: jobject,
    buffer: *const jbyte,
    length: jsize,
) -> jclass {
    if buffer.is_null() || length < 0 {
        set_pending_class_format_error("Invalid class definition input");
        return null_mut();
    }
    let bytecode = unsafe { std::slice::from_raw_parts(buffer.cast::<u8>(), length as usize) };
    let name = if name_mutf8.is_null() {
        match parse(bytecode) {
            Ok(parsed) => match parsed.this_class_name() {
                Some(name) => name,
                None => {
                    set_pending_class_format_error("Class name is missing");
                    return null_mut();
                }
            },
            Err(error) => {
                set_pending_class_format_error(&error.to_string());
                return null_mut();
            }
        }
    } else {
        match from_mutf8_ptr!(name_mutf8) {
            Ok(name) => name.to_string(),
            Err(error) => {
                set_pending_class_format_error(&error.to_string());
                return null_mut();
            }
        }
    };
    let result = with_method_area(|method_area| {
        method_area.create_metaclass(&name, bytecode, loader as i32)
    })
    .and_then(|(class_name, _)| clazz_ref(&class_name));
    match result {
        Ok(class_ref) => class_ref as jclass,
        Err(error) => {
            set_pending_class_format_error(&error.to_string());
            null_mut()
        }
    }
}

fn is_valid_find_class_name(name: &str) -> bool {
    if name.starts_with('[') {
        let Ok(descriptor) = name.parse::<TypeDescriptor>() else {
            return false;
        };
        if descriptor.to_string() != name {
            return false;
        }
        return match descriptor {
            TypeDescriptor::Array(component, _) => match component.as_ref() {
                TypeDescriptor::Void => false,
                TypeDescriptor::Object(class_name) => is_valid_internal_class_name(class_name),
                _ => true,
            },
            _ => false,
        };
    }

    !PRIMITIVE_TYPE_BY_CODE.contains_key(name) && is_valid_internal_class_name(name)
}

fn is_valid_internal_class_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .split('/')
            .all(|part| !part.is_empty() && !part.contains(['.', ';', '[']))
}

pub(super) extern "system" fn get_superclass(_env: *mut JNIEnv, sub: jclass) -> jclass {
    let klass = klass(sub as i32).expect("Failed to get class from reference");
    let parent = if !klass.is_interface() {
        klass.parent().clone()
    } else {
        None
    };

    parent
        .map(|parent_name| clazz_ref(&parent_name).expect("Failed to get class from reference"))
        .unwrap_or(0) as jclass
}

pub(super) extern "system" fn is_assignable_from(
    _env: *mut JNIEnv,
    sub: jclass,
    sup: jclass,
) -> jboolean {
    let sub_klass = klass(sub as i32).expect("Failed to get class from reference");
    let sup_klass = klass(sup as i32).expect("Failed to get class from reference");

    match InstanceChecker::checkcast(sub_klass.this_class_name(), sup_klass.this_class_name()) {
        Ok(result) => result as jboolean,
        Err(e) => {
            set_pending_internal_error(&format!(
                "Failed to check assignability from {} to {}: {e}",
                sub_klass.this_class_name(),
                sup_klass.this_class_name()
            ));
            false
        }
    }
}
