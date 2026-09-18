use crate::vm::execution_engine::executor::Executor;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::vec_to_i64;
use crate::vm::jni::utils::set_pending_internal_error;
use jni_sys::{jlong, jobject, JNIEnv};
use std::ffi::c_void;
use std::ptr::null_mut;

pub(super) extern "system" fn new_direct_byte_buffer(
    _env: *mut JNIEnv,
    address: *mut c_void,
    capacity: jlong,
) -> jobject {
    match Executor::invoke_args_constructor(
        "java/nio/DirectByteBuffer",
        "<init>:(JJ)V",
        &[(address as usize as i64).into(), capacity.into()],
        Some("JNI direct byte buffer creation"),
    ) {
        Ok(reference) => reference as jobject,
        Err(error) => {
            set_pending_internal_error(&format!("Failed to create direct byte buffer: {error}"));
            null_mut()
        }
    }
}

pub(super) extern "system" fn get_direct_buffer_address(
    _env: *mut JNIEnv,
    buffer: jobject,
) -> *mut c_void {
    HEAP.get_object_field_value(buffer as i32, "java/nio/Buffer", "address")
        .map(|value| vec_to_i64(&value) as usize as *mut c_void)
        .unwrap_or(null_mut())
}

pub(super) extern "system" fn get_direct_buffer_capacity(
    _env: *mut JNIEnv,
    buffer: jobject,
) -> jlong {
    HEAP.get_object_field_value(buffer as i32, "java/nio/Buffer", "capacity")
        .ok()
        .and_then(|value| value.first().copied())
        .map(jlong::from)
        .unwrap_or(-1)
}
