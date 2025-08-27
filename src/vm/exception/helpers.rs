use crate::vm::error::Result;
use crate::vm::exception::common::construct_exception_and_throw;
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::stack::stack_frame::StackFrames;
use crate::vm::stack::stack_value::StackValueKind;

pub fn throw_ioexception(message: &str, stack_frames: &mut StackFrames) -> Result<()> {
    let message_ref = StringPoolHelper::get_string(message)?;
    let args = vec![StackValueKind::from(message_ref)];
    construct_exception_and_throw(
        "java/io/IOException",
        "<init>:(Ljava/lang/String;)V",
        &args,
        stack_frames,
    )?;

    Ok(())
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

pub fn throw_null_pointer_exception(message: &str, stack_frames: &mut StackFrames) -> Result<()> {
    let message_ref = StringPoolHelper::get_string(message)?;
    let args = vec![StackValueKind::from(message_ref)];
    construct_exception_and_throw(
        "java/lang/NullPointerException",
        "<init>:(Ljava/lang/String;)V",
        &args,
        stack_frames,
    )?;

    Ok(())
}
