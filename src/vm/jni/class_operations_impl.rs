use crate::from_mutf8_ptr;
use crate::vm::helper::clazz_ref;
use crate::vm::jni::utils::{
    set_pending_class_format_error, set_pending_no_class_def_found_error,
};
use jni_sys::{jclass, JNIEnv};
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
