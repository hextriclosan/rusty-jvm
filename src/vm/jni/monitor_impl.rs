use crate::vm::exception::pending_helpers::set_pending_null_pointer_exception_with_message;
use crate::vm::jni::handles;
use crate::vm::jni::utils::set_pending_vm_error;
use crate::vm::monitor;
use jni_sys::{jint, jobject, JNIEnv, JNI_ERR, JNI_OK};

pub(super) extern "system" fn monitor_enter(_env: *mut JNIEnv, object: jobject) -> jint {
    let _managed = crate::vm::safepoint::ManagedAccess::enter();
    if object.is_null() {
        if let Err(error) =
            set_pending_null_pointer_exception_with_message("monitor object is null")
        {
            set_pending_vm_error(&error);
        }
        return JNI_ERR;
    }
    monitor::enter(handles::resolve_or_return!(object, JNI_ERR));
    JNI_OK
}

pub(super) extern "system" fn monitor_exit(_env: *mut JNIEnv, object: jobject) -> jint {
    let _managed = crate::vm::safepoint::ManagedAccess::enter();
    if object.is_null() {
        if let Err(error) =
            set_pending_null_pointer_exception_with_message("monitor object is null")
        {
            set_pending_vm_error(&error);
        }
        return JNI_ERR;
    }
    let object = handles::resolve_or_return!(object, JNI_ERR);
    let owned = monitor::holds_lock(object);
    if let Err(error) = monitor::exit(object) {
        set_pending_vm_error(&error);
        return JNI_ERR;
    }
    if owned {
        JNI_OK
    } else {
        JNI_ERR
    }
}
