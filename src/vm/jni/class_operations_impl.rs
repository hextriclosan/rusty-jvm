use crate::from_mutf8_ptr;
use crate::vm::helper::{clazz_ref, klass};
use crate::vm::jni::utils::{
    set_pending_class_format_error, set_pending_internal_error,
    set_pending_no_class_def_found_error,
};
use crate::vm::method_area::instance_checker::InstanceChecker;
use jni_sys::{jboolean, jclass, JNIEnv};
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

    match clazz_ref(&name) {
        Ok(clazz) => clazz as jclass,
        Err(_) => {
            set_pending_no_class_def_found_error(&name);
            null_mut()
        }
    }
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
