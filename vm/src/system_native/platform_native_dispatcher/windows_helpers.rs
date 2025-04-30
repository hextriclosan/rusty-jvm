use std::ptr::null_mut;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winbase::{FormatMessageW, FORMAT_MESSAGE_FROM_SYSTEM};

pub fn get_last_error() -> String {
    let error_code = unsafe { GetLastError() };
    let mut message = [0u16; 256];

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
    format!("{} ({})", String::from_utf16_lossy(&message), error_code)
}
