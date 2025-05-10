use crate::helper::{i32toi64, i64_to_vec};
use crate::stack::stack_frame::StackFrames;
use crate::system_native::platform_native_dispatcher::windows_helpers::throw_windows_exception;
use std::mem::zeroed;
use std::ptr::null_mut;
use winapi::shared::minwindef::{DWORD, FALSE, LPVOID};
use winapi::um::fileapi::{
    CreateDirectoryW, DeleteFileW, GetFileAttributesExW, RemoveDirectoryW, SetEndOfFile,
};
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::minwinbase::{GetFileExInfoStandard, LPSECURITY_ATTRIBUTES, SECURITY_ATTRIBUTES};
use winapi::um::winnt::{HANDLE, LPCWSTR, PSECURITY_DESCRIPTOR};

pub fn create_file0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> crate::error::Result<Vec<i32>> {
    let filename_ptr = i32toi64(args[1], args[0]);
    let desired_access = args[2];
    let share_mode = args[3];
    let sec_desc_ptr = i32toi64(args[5], args[4]);
    let creation_disposition = args[6];
    let flags_and_attributes = args[7];

    match create_file0(
        filename_ptr,
        desired_access,
        share_mode,
        sec_desc_ptr,
        creation_disposition,
        flags_and_attributes,
        stack_frames,
    ) {
        Ok(handle) => Ok(i64_to_vec(handle)),
        Err(e) if matches!(e.kind(), crate::error::ErrorKind::ExceptionThrown) => Ok(vec![]),
        Err(e) => Err(e),
    }
}
fn create_file0(
    filename_ptr: i64,
    desired_access: i32,
    share_mode: i32,
    sec_desc_ptr: i64,
    creation_disposition: i32,
    flags_and_attributes: i32,
    stack_frames: &mut StackFrames,
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
            null_mut(),
        )
    };

    if result == INVALID_HANDLE_VALUE {
        throw_windows_exception(stack_frames)?;
        return Err(crate::error::Error::new_exception());
    }

    Ok(result as i64)
}

pub(crate) fn set_end_of_file_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> crate::error::Result<Vec<i32>> {
    let handle_ptr = i32toi64(args[1], args[0]);
    set_end_of_file(handle_ptr, stack_frames)?;
    Ok(vec![])
}
fn set_end_of_file(handle_ptr: i64, stack_frames: &mut StackFrames) -> crate::error::Result<()> {
    let handle = handle_ptr as usize as HANDLE;
    let result = unsafe { SetEndOfFile(handle) };

    if result == 0 {
        throw_windows_exception(stack_frames)?;
    }

    Ok(())
}

pub fn create_directory0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> crate::error::Result<Vec<i32>> {
    let address_ptr = i32toi64(args[1], args[0]);
    let sd_address_ptr = i32toi64(args[3], args[2]);
    create_directory0(address_ptr, sd_address_ptr, stack_frames)?;

    Ok(vec![])
}
fn create_directory0(
    address_ptr: i64,
    sd_address_ptr: i64,
    stack_frames: &mut StackFrames,
) -> crate::error::Result<()> {
    let sd_address = sd_address_ptr as usize as PSECURITY_DESCRIPTOR;
    let mut sec_attr: SECURITY_ATTRIBUTES = unsafe { zeroed() };
    let lp_security_attributes: LPSECURITY_ATTRIBUTES = if !sd_address.is_null() {
        sec_attr.nLength = size_of::<SECURITY_ATTRIBUTES>() as DWORD;
        sec_attr.lpSecurityDescriptor = sd_address;
        sec_attr.bInheritHandle = FALSE;
        &mut sec_attr
    } else {
        null_mut()
    };

    let address = address_ptr as usize as LPCWSTR;
    let result = unsafe { CreateDirectoryW(address, lp_security_attributes) };

    if result == 0 {
        throw_windows_exception(stack_frames)?;
    }

    Ok(())
}

pub fn get_file_attributes_ex0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> crate::error::Result<Vec<i32>> {
    let lp_file_name_addr = i32toi64(args[1], args[0]);
    let output_addr = i32toi64(args[3], args[2]);
    get_file_attributes_ex0(lp_file_name_addr, output_addr, stack_frames)?;

    Ok(vec![])
}
fn get_file_attributes_ex0(
    lp_file_name_addr: i64,
    output_addr: i64,
    stack_frames: &mut StackFrames,
) -> crate::error::Result<()> {
    let address = lp_file_name_addr as usize as LPCWSTR;
    let res = unsafe {
        GetFileAttributesExW(
            address,
            GetFileExInfoStandard,
            output_addr as usize as LPVOID,
        )
    };
    if res == 0 {
        throw_windows_exception(stack_frames)?;
    }

    Ok(())
}

pub fn delete_file0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> crate::error::Result<Vec<i32>> {
    let lp_file_name_addr = i32toi64(args[1], args[0]);
    delete_file0(lp_file_name_addr, stack_frames)?;

    Ok(vec![])
}
fn delete_file0(
    lp_file_name_addr: i64,
    stack_frames: &mut StackFrames,
) -> crate::error::Result<()> {
    let address = lp_file_name_addr as usize as LPCWSTR;
    let res = unsafe { DeleteFileW(address) };
    if res == 0 {
        throw_windows_exception(stack_frames)?;
    }

    Ok(())
}

pub fn remove_directory0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> crate::error::Result<Vec<i32>> {
    let lp_file_name_addr = i32toi64(args[1], args[0]);
    remove_directory0(lp_file_name_addr, stack_frames)?;

    Ok(vec![])
}
fn remove_directory0(
    lp_file_name_addr: i64,
    stack_frames: &mut StackFrames,
) -> crate::error::Result<()> {
    let address = lp_file_name_addr as usize as LPCWSTR;
    let res = unsafe { RemoveDirectoryW(address) };
    if res == 0 {
        throw_windows_exception(stack_frames)?;
    }

    Ok(())
}
