use crate::bail_thrown;
use crate::vm::error::Result;
use crate::vm::exception::helpers::throw_ioexception;
use crate::vm::exception::pending::Throws;
use crate::vm::helper::{get_handle, i32toi64, i64_to_vec};
use crate::vm::stack::stack_frame::StackFrames;
use crate::vm::system_native::platform_native_dispatcher::windows_helpers::get_last_error;
use std::mem::zeroed;
use std::ptr::null_mut;
use winapi::shared::minwindef::{DWORD, FALSE, LPCVOID, LPVOID};
use winapi::shared::winerror::{ERROR_BROKEN_PIPE, ERROR_NO_DATA};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::fileapi::{
    GetFileSizeEx, ReadFile, SetFileInformationByHandle, SetFilePointerEx, WriteFile,
    FILE_END_OF_FILE_INFO,
};
use winapi::um::handleapi::{DuplicateHandle, INVALID_HANDLE_VALUE};
use winapi::um::minwinbase::{FileEndOfFileInfo, OVERLAPPED};
use winapi::um::processthreadsapi::GetCurrentProcess;
use winapi::um::winbase::{FILE_BEGIN, FILE_CURRENT};
use winapi::um::winnt::{DUPLICATE_SAME_ACCESS, HANDLE, LARGE_INTEGER};

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

    let Some(result) = write0(fd_ref, address, len, append, stack_frames)? else {
        return Ok(vec![]);
    };
    Ok(vec![result])
}
fn write0(
    fd_ref: i32,
    address: i64,
    len: i32,
    append: bool,
    stack_frames: &mut StackFrames,
) -> Throws<i32> {
    let handle = get_handle(fd_ref)?;
    let handle = handle as usize as HANDLE;
    if handle == INVALID_HANDLE_VALUE {
        bail_thrown!(throw_ioexception("Invalid handle", stack_frames))
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
        let error_msg = get_last_error()?;
        bail_thrown!(throw_ioexception(&error_msg, stack_frames))
    }

    Ok(Some(written as i32))
}

pub fn windows_file_dispatcher_read0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let fd_ref = args[0];
    let address = i32toi64(args[2], args[1]);
    let len = args[3];

    let Some(ret) = read0(fd_ref, address, len, stack_frames)? else {
        return Ok(vec![]);
    };
    Ok(vec![ret])
}
fn read0(fd_ref: i32, address: i64, len: i32, stack_frames: &mut StackFrames) -> Throws<i32> {
    let handle = (get_handle(fd_ref))? as usize as HANDLE;
    if handle == INVALID_HANDLE_VALUE {
        bail_thrown!(throw_ioexception("Invalid handle", stack_frames))
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
            return Ok(Some(IOS_EOF));
        }
        if error_code == ERROR_NO_DATA {
            return Ok(Some(IOS_UNAVAILABLE));
        }
        bail_thrown!(throw_ioexception(
            &format!("Read failed: {}", (get_last_error())?),
            stack_frames
        ))
    }

    Ok(Some(if read == 0 { IOS_EOF } else { read as i32 }))
}

pub fn windows_file_dispatcher_pread0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let fd_ref = args[0];
    let address = i32toi64(args[2], args[1]);
    let len = args[3];
    let position = i32toi64(args[5], args[4]);

    let Some(ret) = pread0(fd_ref, address, len, position, stack_frames)? else {
        return Ok(vec![]);
    };
    Ok(vec![ret])
}
fn pread0(
    fd_ref: i32,
    address: i64,
    len: i32,
    offset: i64,
    stack_frames: &mut StackFrames,
) -> Throws<i32> {
    let handle = (get_handle(fd_ref))? as usize as HANDLE;
    if handle == INVALID_HANDLE_VALUE {
        bail_thrown!(throw_ioexception("Invalid handle", stack_frames))
    }

    let mut curr_pos: LARGE_INTEGER = unsafe { zeroed() };
    let mut ov: OVERLAPPED = unsafe { zeroed() };

    let ret = unsafe { SetFilePointerEx(handle, curr_pos, &mut curr_pos, FILE_CURRENT) };
    if ret == 0 {
        bail_thrown!(throw_ioexception(
            &format!("Seek failed: {}", (get_last_error())?),
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
            return Ok(Some(IOS_EOF));
        }
        if error_code == ERROR_NO_DATA {
            return Ok(Some(IOS_UNAVAILABLE));
        }
        bail_thrown!(throw_ioexception(
            &format!("Read failed: {}", (get_last_error())?),
            stack_frames
        ))
    }

    let ret = unsafe { SetFilePointerEx(handle, curr_pos, null_mut(), FILE_BEGIN) };
    if ret == 0 {
        bail_thrown!(throw_ioexception(
            &format!("Seek failed: {}", (get_last_error())?),
            stack_frames
        ))
    }

    Ok(Some(if read == 0 { IOS_EOF } else { read as i32 }))
}

pub fn windows_file_dispatcher_size0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let fd_ref = args[0];

    let Some(size) = size0(fd_ref, stack_frames)? else {
        return Ok(vec![]);
    };
    Ok(i64_to_vec(size))
}
fn size0(fd_ref: i32, stack_frames: &mut StackFrames) -> Throws<i64> {
    let handle = (get_handle(fd_ref))? as usize as HANDLE;
    let mut size: LARGE_INTEGER = unsafe { zeroed() };

    let result = unsafe { GetFileSizeEx(handle, &mut size) };
    if result == 0 {
        bail_thrown!(throw_ioexception(
            &format!("GetFileSizeEx failed: {}", (get_last_error())?),
            stack_frames
        ))
    }
    let size = unsafe { *size.QuadPart() };
    Ok(Some(size))
}

pub(super) fn truncate0(fd_ref: i32, size: i64, stack_frames: &mut StackFrames) -> Throws<i32> {
    let handle = (get_handle(fd_ref))? as usize as HANDLE;
    let mut eof_info: FILE_END_OF_FILE_INFO = unsafe { zeroed() };
    unsafe {
        *eof_info.EndOfFile.QuadPart_mut() = size;
    }

    let result = unsafe {
        SetFileInformationByHandle(
            handle,
            FileEndOfFileInfo,
            &mut eof_info as *mut _ as LPVOID,
            size_of::<FILE_END_OF_FILE_INFO>() as DWORD,
        )
    };
    if result == 0 {
        bail_thrown!(throw_ioexception(
            &format!("Truncation failed: {}", (get_last_error())?),
            stack_frames
        ))
    }

    Ok(Some(0))
}

pub fn windows_file_dispatcher_duplicate_handle_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let fd = i32toi64(args[1], args[0]);

    let Some(dup_fd) = duplicate_handle(fd, stack_frames)? else {
        return Ok(vec![]);
    };
    Ok(i64_to_vec(dup_fd))
}
fn duplicate_handle(fd: i64, stack_frames: &mut StackFrames) -> Throws<i64> {
    let handle = fd as usize as HANDLE;
    if handle == INVALID_HANDLE_VALUE {
        bail_thrown!(throw_ioexception("Invalid handle", stack_frames))
    }

    let h_process = unsafe { GetCurrentProcess() };
    let mut h_result: HANDLE = null_mut();
    let ret = unsafe {
        DuplicateHandle(
            h_process,
            handle,
            h_process,
            &mut h_result,
            0,
            FALSE,
            DUPLICATE_SAME_ACCESS,
        )
    };
    if ret == 0 {
        bail_thrown!(throw_ioexception(
            &format!("DuplicateHandle failed: {}", (get_last_error())?),
            stack_frames
        ))
    }

    Ok(Some(h_result as usize as i64))
}
