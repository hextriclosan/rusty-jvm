use winapi::shared::minwindef::UINT;
use winapi::um::errhandlingapi::SetErrorMode;
use crate::vm::error::Result;

/// `sun.io.Win32ErrorMode.setErrorMode(J)J`
pub(crate) fn set_error_mode(mode: i64) -> Result<i64> {
    let ret = unsafe { SetErrorMode(mode as UINT) as i64 };
    Ok(ret)
}
