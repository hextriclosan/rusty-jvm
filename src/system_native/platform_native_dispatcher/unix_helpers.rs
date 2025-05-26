use crate::exception::common::construct_exception_and_throw;
use crate::stack::stack_frame::StackFrames;
use crate::stack::stack_value::StackValueKind;
use nix::errno::Errno;

pub fn throw_unix_exception_with_errno(
    errno: i32,
    stack_frames: &mut StackFrames,
) -> crate::error::Result<()> {
    let args = vec![StackValueKind::from(errno)];
    construct_exception_and_throw(
        "sun/nio/fs/UnixException",
        "<init>:(I)V",
        &args,
        stack_frames,
    )?;

    Ok(())
}

pub fn throw_unix_exception(stack_frames: &mut StackFrames) -> crate::error::Result<()> {
    let errno = Errno::last_raw();
    throw_unix_exception_with_errno(errno, stack_frames)
}
