use crate::vm::error::Result;
use crate::vm::exception::common::construct_exception_and_throw;
use crate::vm::exception::throwing_result::ThrowingResult;
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::heap::heap::HEAP;
use crate::vm::stack::stack_frame::StackFrames;
use crate::vm::stack::stack_value::StackValueKind;
use crate::{throw_and_return, unwrap_or_return_err};

pub fn throw_ioexception(message: &str, stack_frames: &mut StackFrames) -> Result<()> {
    throw_exception_with_message("java/io/IOException", message, stack_frames)
}

pub fn throw_file_not_found_exception(
    path_ref: i32,
    reason: &str,
    stack_frames: &mut StackFrames,
) -> Result<()> {
    let reason_ref = StringPoolHelper::get_string(reason)?;
    let args = vec![path_ref.into(), StackValueKind::from(reason_ref)];
    construct_exception_and_throw(
        "java/io/FileNotFoundException",
        "<init>:(Ljava/lang/String;Ljava/lang/String;)V",
        &args,
        stack_frames,
    )?;

    Ok(())
}

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

pub fn throw_class_not_found_exception(
    message: &str,
    stack_frames: &mut StackFrames,
) -> Result<()> {
    throw_exception_with_message("java/lang/ClassNotFoundException", message, stack_frames)
}

#[cfg(windows)]
pub fn throw_internal_error(message: &str, stack_frames: &mut StackFrames) -> Result<()> {
    throw_exception_with_message("java/lang/InternalError", message, stack_frames)
}

pub fn check_bounds(
    arr_ref: i32,
    offset: i32,
    len: i32,
    stack_frames: &mut StackFrames,
) -> ThrowingResult<()> {
    if arr_ref == 0 {
        throw_and_return!(throw_null_pointer_exception_with_message(
            "array is null",
            stack_frames
        ));
    }

    let arr_len = match HEAP.get_array_len_throwing(arr_ref, stack_frames) {
        ThrowingResult::Result(result) => unwrap_or_return_err!(result),
        ThrowingResult::ExceptionThrown => return ThrowingResult::ExceptionThrown,
    };

    if offset < 0 || len < 0 || arr_len < offset + len {
        throw_and_return!(throw_index_out_of_bounds_exception(
            offset + len,
            arr_len,
            stack_frames
        ))
    }

    ThrowingResult::ok(())
}

fn throw_index_out_of_bounds_exception(
    index: i32,
    size: i32,
    stack_frames: &mut StackFrames,
) -> Result<()> {
    let message = format!("Index: {index}, Size: {size}");
    throw_exception_with_message(
        "java/lang/IndexOutOfBoundsException",
        &message,
        stack_frames,
    )
}

pub fn throw_array_index_out_of_bounds_exception_with_message(
    message: &str,
    stack_frames: &mut StackFrames,
) -> Result<()> {
    throw_exception_with_message(
        "java/lang/ArrayIndexOutOfBoundsException",
        message,
        stack_frames,
    )
}

pub fn throw_array_store_exception(message: &str, stack_frames: &mut StackFrames) -> Result<()> {
    throw_exception_with_message("java/lang/ArrayStoreException", message, stack_frames)
}

pub fn throw_unsatisfied_link_error(message: &str, stack_frames: &mut StackFrames) -> Result<()> {
    throw_exception_with_message("java/lang/UnsatisfiedLinkError", message, stack_frames)
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
