use crate::vm::exception::pending_helpers::set_pending_null_pointer_exception_with_message;
use crate::vm::monitor;
use jni_sys::{jint, jobject, JNIEnv, JNI_ERR, JNI_OK};

pub(super) extern "system" fn monitor_enter(_env: *mut JNIEnv, object: jobject) -> jint {
    if object.is_null() {
        set_pending_null_pointer_exception_with_message("monitor object is null")
            .expect("Failed to create NullPointerException");
        return JNI_ERR;
    }
    monitor::enter(object as i32);
    JNI_OK
}

pub(super) extern "system" fn monitor_exit(_env: *mut JNIEnv, object: jobject) -> jint {
    if object.is_null() {
        set_pending_null_pointer_exception_with_message("monitor object is null")
            .expect("Failed to create NullPointerException");
        return JNI_ERR;
    }
    let owned = monitor::holds_lock(object as i32);
    monitor::exit(object as i32).expect("Failed to exit monitor");
    if owned {
        JNI_OK
    } else {
        JNI_ERR
    }
}
