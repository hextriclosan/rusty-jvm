use crate::vm::error::Result;
use crate::vm::exception::pending_helpers::set_pending_exception;
use crate::vm::stack::stack_value::StackValueKind;
use winapi::um::errhandlingapi::GetLastError;

pub fn set_pending_windows_exception() -> Result<()> {
    let last_error = unsafe { GetLastError() };
    let args = vec![StackValueKind::from(last_error as i32)];
    set_pending_exception("sun/nio/fs/WindowsException", "<init>:(I)V", &args)?;

    Ok(())
}
