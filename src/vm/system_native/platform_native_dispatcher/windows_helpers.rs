use crate::vm::error::Result;
use crate::vm::exception::common::construct_exception_and_throw;
use crate::vm::stack::stack_frame::StackFrames;
use crate::vm::stack::stack_value::StackValueKind;
use std::ptr::null_mut;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winbase::{FormatMessageW, FORMAT_MESSAGE_FROM_SYSTEM};
use winapi::um::winnt::WCHAR;

pub fn get_last_error() -> Result<String> {
    let error_code = unsafe { GetLastError() };
    let mut message = [0 as WCHAR; 256];

    let msg_len = unsafe {
        FormatMessageW(
            FORMAT_MESSAGE_FROM_SYSTEM,
            null_mut(),
            error_code,
            0,
            message.as_mut_ptr(),
            255,
            null_mut(),
        )
    };

    let message = &message[..msg_len as usize];
    Ok(format!(
        "{} ({})",
        String::from_utf16(&message)?,
        error_code
    ))
}

pub fn throw_windows_exception(stack_frames: &mut StackFrames) -> Result<()> {
    let last_error = unsafe { GetLastError() };
    let args = vec![StackValueKind::from(last_error as i32)];
    construct_exception_and_throw(
        "sun/nio/fs/WindowsException",
        "<init>:(I)V",
        &args,
        stack_frames,
    )?;

    Ok(())
}

pub fn strip_string(volume_name: &[WCHAR]) -> Result<String> {
    let mut len = 0;
    while volume_name[len] != 0 {
        len += 1;
    }
    let slice = &volume_name[0..len];
    let stripped = String::from_utf16(&slice)?;
    Ok(stripped)
}
