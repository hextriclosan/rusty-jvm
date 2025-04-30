use crate::error::Error;
use crate::helper::{i32toi64, i64_to_vec};
use crate::system_native::platform_native_dispatcher::windows_helpers::get_last_error;
use std::mem::zeroed;
use std::ptr::null_mut;
use winapi::shared::minwindef::{DWORD, FALSE};
use winapi::um::fileapi::SetEndOfFile;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::minwinbase::{LPSECURITY_ATTRIBUTES, SECURITY_ATTRIBUTES};
use winapi::um::winnt::{HANDLE, LPCWSTR};

pub fn create_file0_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let filename_ptr = i32toi64(args[1], args[0]);
    let desired_access = args[2];
    let share_mode = args[3];
    let sec_desc_ptr = i32toi64(args[5], args[4]);
    let creation_disposition = args[6];
    let flags_and_attributes = args[7];

    let result = create_file0(
        filename_ptr,
        desired_access,
        share_mode,
        sec_desc_ptr,
        creation_disposition,
        flags_and_attributes,
    )?;

    Ok(i64_to_vec(result))
}
fn create_file0(
    filename_ptr: i64,
    desired_access: i32,
    share_mode: i32,
    sec_desc_ptr: i64,
    creation_disposition: i32,
    flags_and_attributes: i32,
) -> crate::error::Result<i64> {
    let filename = filename_ptr as usize as LPCWSTR;

    let security_descriptor = sec_desc_ptr as usize as LPSECURITY_ATTRIBUTES;
    let mut sec_attr: SECURITY_ATTRIBUTES = unsafe { zeroed() };
    let sec_attr_ptr = if security_descriptor.is_null() {
        null_mut()
    } else {
        sec_attr.nLength = size_of::<SECURITY_ATTRIBUTES>() as DWORD;
        sec_attr.lpSecurityDescriptor = security_descriptor as *mut _;
        sec_attr.bInheritHandle = FALSE;
        &mut sec_attr
    };

    let result = unsafe {
        winapi::um::fileapi::CreateFileW(
            filename,
            desired_access as DWORD,
            share_mode as DWORD,
            sec_attr_ptr,
            creation_disposition as DWORD,
            flags_and_attributes as DWORD,
            0 as HANDLE,
        )
    };

    if result == INVALID_HANDLE_VALUE {
        let error_message = get_last_error();
        return Err(Error::new_execution(&format!(
            "create_file0 sun/nio/fs/WindowsException: {error_message}"
        )));
    }

    Ok(result as i64)
}

pub(crate) fn set_end_of_file_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let handle_ptr = i32toi64(args[1], args[0]);
    set_end_of_file(handle_ptr)?;
    Ok(vec![])
}
fn set_end_of_file(handle_ptr: i64) -> crate::error::Result<()> {
    let handle = handle_ptr as usize as HANDLE;
    let result = unsafe { SetEndOfFile(handle) };

    if result == 0 {
        let error_message = get_last_error();
        return Err(Error::new_execution(&format!(
            "set_end_of_file failed: {error_message}"
        )));
    }

    Ok(())
}
