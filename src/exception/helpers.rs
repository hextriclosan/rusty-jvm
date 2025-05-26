use crate::exception::common::construct_exception_and_throw;
use crate::execution_engine::string_pool_helper::StringPoolHelper;
use crate::stack::stack_frame::StackFrames;
use crate::stack::stack_value::StackValueKind;

pub fn throw_ioexception(
    message: String,
    stack_frames: &mut StackFrames,
) -> crate::error::Result<()> {
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
    reason: String,
    stack_frames: &mut StackFrames,
) -> crate::error::Result<()> {
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
