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
        "java.io.IOException",
        "<init>:(Ljava/lang/String;)V",
        &args,
        stack_frames,
    )?;

    Ok(())
}
