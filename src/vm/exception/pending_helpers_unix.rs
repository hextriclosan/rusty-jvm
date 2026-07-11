use crate::vm::error::Result;
use crate::vm::exception::pending_helpers::set_pending_exception;
use crate::vm::stack::stack_value::StackValueKind;
use nix::errno::Errno;

pub(crate) fn set_pending_unix_exception_with_errno(errno: i32) -> Result<()> {
    let args = vec![StackValueKind::from(errno)];
    set_pending_exception("sun/nio/fs/UnixException", "<init>:(I)V", &args)?;

    Ok(())
}

pub(crate) fn set_pending_unix_exception() -> Result<()> {
    let errno = Errno::last_raw();
    set_pending_unix_exception_with_errno(errno)
}
