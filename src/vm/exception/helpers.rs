use crate::vm::error::Result;
use crate::vm::exception::common::construct_exception_and_throw;
use crate::vm::exception::pending_helpers::{
    set_pending_array_index_out_of_bounds_exception,
    set_pending_null_pointer_exception_with_message,
};
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::heap::heap::HEAP;
use crate::vm::jni::set_pending_internal_error;
use crate::vm::stack::stack_frame::StackFrames;
use crate::vm::stack::stack_value::StackValueKind;

pub fn throw_null_pointer_exception(stack_frames: &mut StackFrames) -> Result<()> {
    construct_exception_and_throw(
        "java/lang/NullPointerException",
        "<init>:()V",
        &[],
        stack_frames,
    )?;

    Ok(())
}

pub fn throw_null_pointer_exception_with_message(
    message: &str,
    stack_frames: &mut StackFrames,
) -> Result<()> {
    throw_exception_with_message("java/lang/NullPointerException", message, stack_frames)
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

pub fn throw_unsatisfied_link_error(message: &str, stack_frames: &mut StackFrames) -> Result<()> {
    throw_exception_with_message("java/lang/UnsatisfiedLinkError", message, stack_frames)
}

pub fn throw_illegal_argument_exception(
    message: &str,
    stack_frames: &mut StackFrames,
) -> Result<()> {
    throw_exception_with_message("java/lang/IllegalArgumentException", message, stack_frames)
}

fn throw_exception_with_message(
    class_name: &str,
    message: &str,
    stack_frames: &mut StackFrames,
) -> Result<()> {
    let message_ref = StringPoolHelper::get_string(message)?;
    let args = vec![StackValueKind::from(message_ref)];
    construct_exception_and_throw(
        class_name,
        "<init>:(Ljava/lang/String;)V",
        &args,
        stack_frames,
    )?;

    Ok(())
}
