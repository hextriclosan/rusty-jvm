use crate::vm::error::Result;
use crate::vm::exception::helpers::throw_ioexception;
use crate::vm::exception::throwing_result::ThrowingResult;
use crate::vm::helper::{get_handle, i32toi64, i64_to_vec};
use crate::vm::stack::stack_frame::StackFrames;
use crate::vm::system_native::platform_native_dispatcher::windows_helpers::get_last_error;
use crate::{throw_and_return, unwrap_or_return_err, unwrap_result_or_return_default};
use std::mem::zeroed;
use std::ptr::null_mut;
use winapi::shared::minwindef::{DWORD, LPCVOID, LPVOID};
use winapi::shared::winerror::{ERROR_BROKEN_PIPE, ERROR_NO_DATA};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::fileapi::{GetFileSizeEx, ReadFile, SetFilePointerEx, WriteFile};
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::minwinbase::OVERLAPPED;
use winapi::um::winbase::{FILE_BEGIN, FILE_CURRENT};
use winapi::um::winnt::{HANDLE, LARGE_INTEGER};

const IOS_EOF: i32 = -1; // End of file
const IOS_UNAVAILABLE: i32 = -2; // Nothing available (non-blocking)

pub fn windows_file_dispatcher_write0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let fd_ref = args[0];
    let address = i32toi64(args[2], args[1]);
    let len = args[3];
    let append = args[4] != 0;

    let ret = unwrap_result_or_return_default!(
        write0(fd_ref, address, len, append, stack_frames),
        vec![]
    );
    Ok(vec![ret])
}
fn write0(
    fd_ref: i32,
    address: i64,
    len: i32,
    append: bool,
    stack_frames: &mut StackFrames,
) -> ThrowingResult<i32> {
    let handle = unwrap_or_return_err!(get_handle(fd_ref));
    let handle = handle as usize as HANDLE;
    if handle == INVALID_HANDLE_VALUE {
        throw_and_return!(throw_ioexception("Invalid handle", stack_frames))
    }

    let mut ov: OVERLAPPED = unsafe { zeroed() };
    let mut written = 0 as DWORD;
    let lp_ov = if append {
        unsafe { ov.u.s_mut().Offset = 0xFFFFFFFF };
        unsafe { ov.u.s_mut().OffsetHigh = 0xFFFFFFFF };
        &mut ov
    } else {
        null_mut()
    };
    let result = unsafe {
        WriteFile(
            handle,             /* File handle to write */
            address as LPCVOID, /* pointer to the buffer */
            len as DWORD,       /* number of bytes to write */
            &mut written,       /* receives number of bytes written */
            lp_ov,              /* overlapped struct */
        )
    };

    if result == 0 {
        let error_msg = unwrap_or_return_err!(get_last_error());
        throw_and_return!(throw_ioexception(&error_msg, stack_frames))
    }

    ThrowingResult::ok(written as i32)
}

pub fn windows_file_dispatcher_read0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let fd_ref = args[0];
    let address = i32toi64(args[2], args[1]);
    let len = args[3];

    let ret = unwrap_result_or_return_default!(read0(fd_ref, address, len, stack_frames), vec![]);
    Ok(vec![ret])
}
fn read0(
    fd_ref: i32,
    address: i64,
    len: i32,
    stack_frames: &mut StackFrames,
) -> ThrowingResult<i32> {
    let handle = unwrap_or_return_err!(get_handle(fd_ref)) as usize as HANDLE;
    if handle == INVALID_HANDLE_VALUE {
        throw_and_return!(throw_ioexception("Invalid handle", stack_frames))
    }

    let mut read: DWORD = 0;
    let result = unsafe {
        ReadFile(
            handle,
            address as LPVOID,
            len as DWORD,
            &mut read,
            0 as *mut OVERLAPPED,
        )
    };
    if result == 0 {
        let error_code = unsafe { GetLastError() };
        if error_code == ERROR_BROKEN_PIPE {
            return ThrowingResult::ok(IOS_EOF);
        }
        if error_code == ERROR_NO_DATA {
            return ThrowingResult::ok(IOS_UNAVAILABLE);
        }
        throw_and_return!(throw_ioexception(
            &format!("Read failed: {}", unwrap_or_return_err!(get_last_error())),
            stack_frames
        ))
    }

    ThrowingResult::ok(if read == 0 { IOS_EOF } else { read as i32 })
}

pub fn windows_file_dispatcher_pread0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let fd_ref = args[0];
    let address = i32toi64(args[2], args[1]);
    let len = args[3];
    let position = i32toi64(args[5], args[4]);

    let ret = unwrap_result_or_return_default!(
        pread0(fd_ref, address, len, position, stack_frames),
        vec![]
    );
    Ok(vec![ret])
}
fn pread0(
    fd_ref: i32,
    address: i64,
    len: i32,
    offset: i64,
    stack_frames: &mut StackFrames,
) -> ThrowingResult<i32> {
    let handle = unwrap_or_return_err!(get_handle(fd_ref)) as usize as HANDLE;
    if handle == INVALID_HANDLE_VALUE {
        throw_and_return!(throw_ioexception("Invalid handle", stack_frames))
    }

    let mut curr_pos: LARGE_INTEGER = unsafe { zeroed() };
    let mut ov: OVERLAPPED = unsafe { zeroed() };

    let ret = unsafe { SetFilePointerEx(handle, curr_pos, &mut curr_pos, FILE_CURRENT) };
    if ret == 0 {
        throw_and_return!(throw_ioexception(
            &format!("Seek failed: {}", unwrap_or_return_err!(get_last_error())),
            stack_frames
        ))
    }

    unsafe {
        ov.u.s_mut().Offset = offset as DWORD;
        ov.u.s_mut().OffsetHigh = (offset >> 32) as DWORD;
    }

    let mut read: DWORD = 0;
    let ret = unsafe { ReadFile(handle, address as LPVOID, len as DWORD, &mut read, &mut ov) };

    if ret == 0 {
        let error_code = unsafe { GetLastError() };
        if error_code == ERROR_BROKEN_PIPE {
            return ThrowingResult::ok(IOS_EOF);
        }
        if error_code == ERROR_NO_DATA {
            return ThrowingResult::ok(IOS_UNAVAILABLE);
        }
        throw_and_return!(throw_ioexception(
            &format!("Read failed: {}", unwrap_or_return_err!(get_last_error())),
            stack_frames
        ))
    }

    let ret = unsafe { SetFilePointerEx(handle, curr_pos, null_mut(), FILE_BEGIN) };
    if ret == 0 {
        throw_and_return!(throw_ioexception(
            &format!("Seek failed: {}", unwrap_or_return_err!(get_last_error())),
            stack_frames
        ))
    }

    ThrowingResult::ok(if read == 0 { IOS_EOF } else { read as i32 })
}

pub fn windows_file_dispatcher_size0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let fd_ref = args[0];

    let size = unwrap_result_or_return_default!(size0(fd_ref, stack_frames), vec![]);
    Ok(i64_to_vec(size))
}
fn size0(fd_ref: i32, stack_frames: &mut StackFrames) -> ThrowingResult<i64> {
    let handle = unwrap_or_return_err!(get_handle(fd_ref)) as usize as HANDLE;
    let mut size: LARGE_INTEGER = unsafe { zeroed() };

    let result = unsafe { GetFileSizeEx(handle, &mut size) };
    if result == 0 {
        throw_and_return!(throw_ioexception(
            &format!(
                "GetFileSizeEx failed: {}",
                unwrap_or_return_err!(get_last_error())
            ),
            stack_frames
        ))
    }
    let size = unsafe { *size.QuadPart() };
    ThrowingResult::ok(size)
}
