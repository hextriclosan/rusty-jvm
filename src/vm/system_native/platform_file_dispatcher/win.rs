use crate::vm::error::Result;
use crate::vm::exception::pending_helpers::set_pending_io_exception;
use crate::vm::helper::get_handle;
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

pub(crate) fn max_direct_transfer_size0() -> Result<i32> {
    Ok(i32::MAX - 1) // Integer.MAX_VALUE - 1 is the maximum transfer size for TransmitFile()
}

pub(crate) fn write0(fd_ref: i32, address: i64, len: i32, append: bool) -> Result<i32> {
    let handle = get_handle(fd_ref)?;
    let handle = handle as usize as HANDLE;
    if handle == INVALID_HANDLE_VALUE {
        set_pending_io_exception("Invalid handle")?;
        return Ok(0);
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
        set_pending_io_exception(&format!("Write failed: {}", error_msg))?;
        return Ok(0);
    }

    Ok(written as i32)
}

pub(crate) fn read0(fd_ref: i32, address: i64, len: i32) -> Result<i32> {
    let handle = get_handle(fd_ref)? as usize as HANDLE;
    if handle == INVALID_HANDLE_VALUE {
        set_pending_io_exception("Invalid handle")?;
        return Ok(0);
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
            return Ok(IOS_EOF);
        }
        if error_code == ERROR_NO_DATA {
            return Ok(IOS_UNAVAILABLE);
        }
        let error_msg = get_last_error()?;
        set_pending_io_exception(&format!("Read failed: {}", error_msg))?;
        return Ok(0);
    }

    Ok(if read == 0 { IOS_EOF } else { read as i32 })
}

pub(crate) fn pread0(fd_ref: i32, address: i64, len: i32, offset: i64) -> Result<i32> {
    let handle = get_handle(fd_ref)? as usize as HANDLE;
    if handle == INVALID_HANDLE_VALUE {
        set_pending_io_exception("Invalid handle")?;
        return Ok(0);
    }

    let mut curr_pos: LARGE_INTEGER = unsafe { zeroed() };
    let mut ov: OVERLAPPED = unsafe { zeroed() };

    let ret = unsafe { SetFilePointerEx(handle, curr_pos, &mut curr_pos, FILE_CURRENT) };
    if ret == 0 {
        let error_msg = get_last_error()?;
        set_pending_io_exception(&format!("Seek failed: {}", error_msg))?;
        return Ok(0);
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
            return Ok(IOS_EOF);
        }
        if error_code == ERROR_NO_DATA {
            return Ok(IOS_UNAVAILABLE);
        }
        let error_msg = get_last_error()?;
        set_pending_io_exception(&format!("Read failed: {}", error_msg))?;
        return Ok(0);
    }

    let ret = unsafe { SetFilePointerEx(handle, curr_pos, null_mut(), FILE_BEGIN) };
    if ret == 0 {
        let error_msg = get_last_error()?;
        set_pending_io_exception(&format!("Seek failed: {}", error_msg))?;
        return Ok(0);
    }

    Ok(if read == 0 { IOS_EOF } else { read as i32 })
}

pub(crate) fn size0(fd_ref: i32) -> Result<i64> {
    let handle = get_handle(fd_ref)? as usize as HANDLE;
    let mut size: LARGE_INTEGER = unsafe { zeroed() };

    let result = unsafe { GetFileSizeEx(handle, &mut size) };
    if result == 0 {
        let error_msg = get_last_error()?;
        set_pending_io_exception(&format!("GetFileSizeEx failed: {}", error_msg))?;
        return Ok(0);
    }
    let size = unsafe { *size.QuadPart() };
    Ok(size)
}

pub(crate) fn truncate0(fd_ref: i32, size: i64) -> Result<i32> {
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
        let error_msg = get_last_error()?;
        set_pending_io_exception(&format!("Truncation failed: {}", error_msg))?;
        return Ok(0);
    }

    Ok(0)
}

pub(crate) fn duplicate_handle(fd: i64) -> Result<i64> {
    let handle = fd as usize as HANDLE;
    if handle == INVALID_HANDLE_VALUE {
        set_pending_io_exception("Invalid handle")?;
        return Ok(0);
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
        let error_msg = get_last_error()?;
        set_pending_io_exception(&format!("DuplicateHandle failed: {}", error_msg))?;
        return Ok(0);
    }

    Ok(h_result as usize as i64)
}
