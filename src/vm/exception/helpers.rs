use crate::vm::error::{Error, Result};
use crate::vm::exception::common::construct_exception_and_throw;
use crate::vm::exception::pending_helpers::{
    set_pending_array_index_out_of_bounds_exception,
    set_pending_null_pointer_exception_with_message,
};
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::heap::heap::HEAP;
use crate::vm::jni::set_pending_internal_error;
use crate::vm::method_area::instance_checker::InstanceChecker;
use crate::vm::stack::stack_frame::StackFrames;
use crate::vm::stack::stack_value::StackValueKind;

/// Throws an implicit (VM-raised) `NullPointerException` with **no** detail message. The helpful
/// message describing the null reference (JEP 358) is computed lazily by
/// `NullPointerException.getExtendedNPEMessage()`, which the JDK's `getMessage()` invokes only when
/// the detail message is null — so VM-thrown NPEs must be constructed message-less.
pub fn throw_null_pointer_exception(stack_frames: &mut StackFrames) -> Result<()> {
    construct_exception_and_throw(
        "java/lang/NullPointerException",
        "<init>:()V",
        &[],
        stack_frames,
    )
}

pub fn check_array_access(
    array_ref: i32,
    index: i32,
    stack_frames: &mut StackFrames,
) -> Result<bool> {
    if array_ref == 0 {
        throw_null_pointer_exception(stack_frames)?;
        return Ok(false);
    }

    let length = HEAP.get_array_len(array_ref)?;
    if index < 0 || index >= length {
        // Same message as HotSpot, which the (int) constructor would not give
        let message = format!("Index {index} out of bounds for length {length}");
        let message_ref = StringPoolHelper::create_string(&message)?;
        construct_exception_and_throw(
            "java/lang/ArrayIndexOutOfBoundsException",
            "<init>:(Ljava/lang/String;)V",
            &[StackValueKind::from(message_ref)],
            stack_frames,
        )?;
        return Ok(false);
    }

    Ok(true)
}

/// Checks that `value_ref` can be stored in the reference array `array_ref`, as `aastore` requires,
/// and throws an `ArrayStoreException` named after the value's class if it can't.
pub fn check_array_store(
    array_ref: i32,
    value_ref: i32,
    stack_frames: &mut StackFrames,
) -> Result<bool> {
    if value_ref == 0 {
        return Ok(true);
    }

    let array_class = HEAP.get_instance_name(array_ref)?;
    let element_type = array_class
        .strip_prefix('[')
        .ok_or_else(|| Error::new_execution(&format!("{array_class} is not an array")))?;
    if InstanceChecker::can_store_in_array(value_ref, element_type)? {
        return Ok(true);
    }

    let value_class = HEAP.get_instance_name(value_ref)?;
    let message_ref = StringPoolHelper::create_string(&value_class.replace('/', "."))?;
    construct_exception_and_throw(
        "java/lang/ArrayStoreException",
        "<init>:(Ljava/lang/String;)V",
        &[StackValueKind::from(message_ref)],
        stack_frames,
    )?;
    Ok(false)
}

pub fn check_bounds(arr_ref: i32, offset: i32, len: i32) -> Result<bool> {
    if arr_ref == 0 {
        set_pending_null_pointer_exception_with_message("array is null")?;
        return Ok(false);
    }

    let arr_len = match HEAP.get_array_len(arr_ref) {
        Ok(len) => len,
        Err(e) => {
            set_pending_internal_error(&format!("Failed to get array length: {}", e));
            return Ok(false);
        }
    };

    if offset < 0 || len < 0 || arr_len < offset + len {
        set_pending_array_index_out_of_bounds_exception(&format!(
            "Index: {}, Size: {}",
            offset + len,
            arr_len
        ))?;
        return Ok(false);
    }

    Ok(true)
}
