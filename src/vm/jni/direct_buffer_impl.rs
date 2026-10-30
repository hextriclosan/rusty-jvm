use crate::vm::error::Result;
use crate::vm::execution_engine::executor::Executor;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::vec_to_i64;
use crate::vm::jni::jni_invoke::jni_invoke_scalar;
use crate::vm::method_area::instance_checker::InstanceChecker;
use jni_sys::{jlong, jobject, JNIEnv};
use std::ffi::c_void;
use std::ptr::null_mut;

pub(super) extern "system" fn new_direct_byte_buffer(
    _env: *mut JNIEnv,
    address: *mut c_void,
    capacity: jlong,
) -> jobject {
    // The constructor rejects a negative or oversized capacity with an IllegalArgumentException,
    // which is left pending for the caller.
    let result = Executor::invoke_args_constructor(
        "java/nio/DirectByteBuffer",
        "<init>:(JJ)V",
        &[(address as usize as i64).into(), capacity.into()],
        Some("JNI direct byte buffer creation"),
    );
    jni_invoke_scalar(result, "NewDirectByteBuffer")
}

/// Returns null for anything but a direct buffer: a heap buffer's `address` field holds the array
/// base offset, not a native address.
pub(super) extern "system" fn get_direct_buffer_address(
    _env: *mut JNIEnv,
    buffer: jobject,
) -> *mut c_void {
    direct_buffer_field(buffer, "address")
        .map(|value| vec_to_i64(&value) as usize as *mut c_void)
        .unwrap_or(null_mut())
}

/// Returns -1 for anything but a direct buffer, like HotSpot.
pub(super) extern "system" fn get_direct_buffer_capacity(
    _env: *mut JNIEnv,
    buffer: jobject,
) -> jlong {
    direct_buffer_field(buffer, "capacity")
        .and_then(|value| value.first().copied())
        .map(jlong::from)
        .unwrap_or(-1)
}

fn direct_buffer_field(buffer: jobject, field_name: &str) -> Option<Vec<i32>> {
    let buffer_ref = buffer as i32;
    is_direct_buffer(buffer_ref)
        .ok()?
        .then(|| HEAP.get_object_field_value(buffer_ref, "java/nio/Buffer", field_name))?
        .ok()
}

fn is_direct_buffer(buffer_ref: i32) -> Result<bool> {
    if buffer_ref == 0 {
        return Ok(false);
    }
    let instance_name = HEAP.get_instance_name(buffer_ref)?;
    InstanceChecker::checkcast(&instance_name, "sun/nio/ch/DirectBuffer")
}
