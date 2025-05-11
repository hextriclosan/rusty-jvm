use crate::error::Error;
use crate::execution_engine::string_pool_helper::StringPoolHelper;
use crate::heap::heap::with_heap_write_lock;
use crate::helper::{i32toi64, i64_to_vec};
use crate::stack::stack_frame::StackFrames;
use crate::system_native::platform_native_dispatcher::windows_helpers::{
    strip_string, throw_windows_exception,
};
use std::mem::zeroed;
use std::ptr::null_mut;
use winapi::shared::minwindef::{DWORD, FALSE, LPVOID, MAX_PATH, TRUE};
use winapi::shared::winerror::{ERROR_INSUFFICIENT_BUFFER, ERROR_NO_TOKEN};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::fileapi::{
    CreateDirectoryW, DeleteFileW, GetDriveTypeW, GetFileAttributesExW, GetVolumeInformationW,
    GetVolumePathNameW, RemoveDirectoryW, SetEndOfFile,
};
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::minwinbase::{GetFileExInfoStandard, LPSECURITY_ATTRIBUTES, SECURITY_ATTRIBUTES};
use winapi::um::processthreadsapi::{
    GetCurrentProcess, GetCurrentThread, OpenProcessToken, OpenThreadToken,
};
use winapi::um::securitybaseapi::{
    AccessCheck, DuplicateTokenEx, GetFileSecurityW, MapGenericMask,
};
use winapi::um::winbase::{FormatMessageW, FORMAT_MESSAGE_FROM_SYSTEM};
use winapi::um::winnt::{
    SecurityImpersonation, TokenImpersonation, GENERIC_MAPPING, HANDLE, LPCWSTR, PRIVILEGE_SET,
    PSECURITY_DESCRIPTOR, SECURITY_INFORMATION, WCHAR,
};

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
        return Err(Error::new_exception());
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

pub fn get_file_security0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> crate::error::Result<Vec<i32>> {
    let lp_file_name_addr = i32toi64(args[1], args[0]);
    let requested_information = args[2];
    let p_security_descriptor_addr = i32toi64(args[4], args[3]);
    let n_length = args[5];

    match get_file_security0(
        lp_file_name_addr,
        requested_information,
        p_security_descriptor_addr,
        n_length,
        stack_frames,
    ) {
        Ok(result) => Ok(vec![result]),
        Err(e) if matches!(e.kind(), crate::error::ErrorKind::ExceptionThrown) => Ok(vec![]),
        Err(e) => Err(e),
    }
}
fn get_file_security0(
    lp_file_name_addr: i64,
    requested_information: i32,
    p_security_descriptor_addr: i64,
    n_length: i32,
    stack_frames: &mut StackFrames,
) -> crate::error::Result<i32> {
    let address = lp_file_name_addr as usize as LPCWSTR;
    let mut length_needed = 0 as DWORD;
    let res = unsafe {
        GetFileSecurityW(
            address,
            requested_information as SECURITY_INFORMATION,
            p_security_descriptor_addr as PSECURITY_DESCRIPTOR,
            n_length as DWORD,
            &mut length_needed,
        )
    };
    if res == 0 {
        let last_error = unsafe { GetLastError() };
        if last_error == ERROR_INSUFFICIENT_BUFFER {
            Ok(length_needed as i32)
        } else {
            throw_windows_exception(stack_frames)?;
            Err(Error::new_exception())
        }
    } else {
        Ok(n_length)
    }
}

pub fn get_current_process_wrp(_args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let process_id = get_current_process()?;
    Ok(i64_to_vec(process_id))
}
fn get_current_process() -> crate::error::Result<i64> {
    let handle = unsafe { GetCurrentProcess() as usize as i64 };
    Ok(handle)
}

pub fn open_process_token_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> crate::error::Result<Vec<i32>> {
    let process = i32toi64(args[1], args[0]);
    let desired_access = args[2];

    match open_process_token(process, desired_access, stack_frames) {
        Ok(token) => Ok(i64_to_vec(token)),
        Err(e) if matches!(e.kind(), crate::error::ErrorKind::ExceptionThrown) => Ok(vec![]),
        Err(e) => Err(e),
    }
}
fn open_process_token(
    process: i64,
    desired_access: i32,
    stack_frames: &mut StackFrames,
) -> crate::error::Result<i64> {
    let mut h_token = 0i64 as usize as HANDLE;
    let result = unsafe {
        OpenProcessToken(
            process as usize as HANDLE,
            desired_access as DWORD,
            &mut h_token,
        )
    };
    if result == 0 {
        throw_windows_exception(stack_frames)?;
        return Err(Error::new_exception());
    }

    Ok(h_token as usize as i64)
}

pub fn get_current_thread_wrp(_args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let process_id = get_current_thread()?;
    Ok(i64_to_vec(process_id))
}
fn get_current_thread() -> crate::error::Result<i64> {
    let handle = unsafe { GetCurrentThread() as usize as i64 };
    Ok(handle)
}

pub fn open_thread_token_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> crate::error::Result<Vec<i32>> {
    let thread = i32toi64(args[1], args[0]);
    let desired_access = args[2];
    let open_as_self = args[3] != 0;

    match open_thread_token(thread, desired_access, open_as_self, stack_frames) {
        Ok(token) => Ok(i64_to_vec(token)),
        Err(e) if matches!(e.kind(), crate::error::ErrorKind::ExceptionThrown) => Ok(vec![]),
        Err(e) => Err(e),
    }
}
fn open_thread_token(
    thread: i64,
    desired_access: i32,
    open_as_self: bool,
    stack_frames: &mut StackFrames,
) -> crate::error::Result<i64> {
    let h_thread = thread as usize as HANDLE;
    let mut h_token = 0i64 as usize as HANDLE;
    let open_as_self = if open_as_self { TRUE } else { FALSE };

    let result = unsafe {
        OpenThreadToken(
            h_thread,
            desired_access as DWORD,
            open_as_self,
            &mut h_token,
        )
    };
    if result == 0 {
        match unsafe { GetLastError() } {
            ERROR_NO_TOKEN => Ok(0),
            _ => {
                throw_windows_exception(stack_frames)?;
                Err(Error::new_exception())
            }
        }
    } else {
        Ok(h_token as usize as i64)
    }
}

pub fn duplicate_token_ex_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> crate::error::Result<Vec<i32>> {
    let token = i32toi64(args[1], args[0]);
    let desired_access = args[2];

    match duplicate_token_ex(token, desired_access, stack_frames) {
        Ok(token) => Ok(i64_to_vec(token)),
        Err(e) if matches!(e.kind(), crate::error::ErrorKind::ExceptionThrown) => Ok(vec![]),
        Err(e) => Err(e),
    }
}
fn duplicate_token_ex(
    token: i64,
    desired_access: i32,
    stack_frames: &mut StackFrames,
) -> crate::error::Result<i64> {
    let mut result_token = 0i64 as usize as HANDLE;
    let result = unsafe {
        DuplicateTokenEx(
            token as usize as HANDLE,
            desired_access as DWORD,
            null_mut(),
            SecurityImpersonation,
            TokenImpersonation,
            &mut result_token,
        )
    };
    if result == 0 {
        throw_windows_exception(stack_frames)?;
        return Err(Error::new_exception());
    }

    Ok(result_token as usize as i64)
}

pub fn access_check_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> crate::error::Result<Vec<i32>> {
    let token = i32toi64(args[1], args[0]);
    let security_info = i32toi64(args[3], args[2]);
    let access_mask = args[4];
    let generic_read = args[5];
    let generic_write = args[6];
    let generic_execute = args[7];
    let generic_all = args[8];

    match access_check(
        token,
        security_info,
        access_mask,
        generic_read,
        generic_write,
        generic_execute,
        generic_all,
        stack_frames,
    ) {
        Ok(result) => Ok(vec![if result { 1 } else { 0 }]),
        Err(e) if matches!(e.kind(), crate::error::ErrorKind::ExceptionThrown) => Ok(vec![]),
        Err(e) => Err(e),
    }
}
fn access_check(
    token: i64,
    security_info: i64,
    access_mask: i32,
    generic_read: i32,
    generic_write: i32,
    generic_execute: i32,
    generic_all: i32,
    stack_frames: &mut StackFrames,
) -> crate::error::Result<bool> {
    let mut mapping = GENERIC_MAPPING {
        GenericRead: generic_read as DWORD,
        GenericWrite: generic_write as DWORD,
        GenericExecute: generic_execute as DWORD,
        GenericAll: generic_all as DWORD,
    };
    let mut privileges: PRIVILEGE_SET = unsafe { zeroed() };
    let mut privileges_length = size_of::<PRIVILEGE_SET>() as DWORD;
    let mut granted_access = 0 as DWORD;
    let mut result = FALSE;

    let mut access_mask_mut = access_mask as DWORD;
    unsafe {
        MapGenericMask(&mut access_mask_mut, &mut mapping);
    }

    let ret = unsafe {
        AccessCheck(
            security_info as PSECURITY_DESCRIPTOR,
            token as HANDLE,
            access_mask_mut,
            &mut mapping,
            &mut privileges,
            &mut privileges_length,
            &mut granted_access,
            &mut result,
        )
    };
    if ret == 0 {
        throw_windows_exception(stack_frames)?;
        Err(Error::new_exception())
    } else {
        Ok(if result == TRUE { true } else { false })
    }
}

pub fn close_handle_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let handle = i32toi64(args[1], args[0]);
    close_handle(handle);

    Ok(vec![])
}
fn close_handle(handle: i64) {
    let _result = unsafe { CloseHandle(handle as HANDLE) };
}

pub fn get_volume_path_name0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> crate::error::Result<Vec<i32>> {
    let address = i32toi64(args[1], args[0]);
    match get_volume_path_name0(address, stack_frames) {
        Ok(string_ref) => Ok(vec![string_ref]),
        Err(e) if matches!(e.kind(), crate::error::ErrorKind::ExceptionThrown) => Ok(vec![]),
        Err(e) => Err(e),
    }
}
fn get_volume_path_name0(
    address: i64,
    stack_frames: &mut StackFrames,
) -> crate::error::Result<i32> {
    let mut volume_name = [0 as WCHAR; MAX_PATH + 1];
    let result = unsafe {
        GetVolumePathNameW(
            address as LPCWSTR,
            volume_name.as_mut_ptr(),
            MAX_PATH as DWORD + 1,
        )
    };

    if result == 0 {
        throw_windows_exception(stack_frames)?;
        return Err(Error::new_exception());
    }

    let string = strip_string(&volume_name)?;
    let string_ref = StringPoolHelper::get_string(string)?;

    Ok(string_ref)
}

pub fn get_volume_information0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> crate::error::Result<Vec<i32>> {
    let address = i32toi64(args[1], args[0]);
    let volume_information_ref = args[2];
    match get_volume_information0(address, volume_information_ref, stack_frames) {
        Ok(_) => Ok(vec![]),
        Err(e) if matches!(e.kind(), crate::error::ErrorKind::ExceptionThrown) => Ok(vec![]),
        Err(e) => Err(e),
    }
}
fn get_volume_information0(
    address: i64,
    volume_information_ref: i32,
    stack_frames: &mut StackFrames,
) -> crate::error::Result<()> {
    let mut volume_name = [0 as WCHAR; MAX_PATH + 1];
    let mut volume_serial_number = 0 as DWORD;
    let mut max_component_length = 0 as DWORD;
    let mut flags = 0 as DWORD;
    let mut filesystem_name = [0 as WCHAR; MAX_PATH + 1];

    let result = unsafe {
        GetVolumeInformationW(
            address as usize as LPCWSTR,
            volume_name.as_mut_ptr(),
            MAX_PATH as DWORD + 1,
            &mut volume_serial_number,
            &mut max_component_length,
            &mut flags,
            filesystem_name.as_mut_ptr(),
            MAX_PATH as DWORD + 1,
        )
    };

    if result == 0 {
        throw_windows_exception(stack_frames)?;
        return Err(Error::new_exception());
    }

    let volume_name = strip_string(&volume_name)?;
    let filesystem_name = strip_string(&filesystem_name)?;

    let volume_name_ref = StringPoolHelper::get_string(volume_name)?;
    let filesystem_name_ref = StringPoolHelper::get_string(filesystem_name)?;

    with_heap_write_lock(|heap| {
        heap.set_object_field_value(
            volume_information_ref,
            "sun/nio/fs/WindowsNativeDispatcher$VolumeInformation",
            "volumeName",
            vec![volume_name_ref],
        )?;
        heap.set_object_field_value(
            volume_information_ref,
            "sun/nio/fs/WindowsNativeDispatcher$VolumeInformation",
            "fileSystemName",
            vec![filesystem_name_ref],
        )?;

        heap.set_object_field_value(
            volume_information_ref,
            "sun/nio/fs/WindowsNativeDispatcher$VolumeInformation",
            "volumeSerialNumber",
            vec![volume_serial_number as i32],
        )?;
        heap.set_object_field_value(
            volume_information_ref,
            "sun/nio/fs/WindowsNativeDispatcher$VolumeInformation",
            "flags",
            vec![flags as i32],
        )?;
        Ok::<(), Error>(())
    })?;

    Ok(())
}

pub fn get_drive_type0_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let address = i32toi64(args[1], args[0]);
    let drive_type = get_drive_type0(address);

    Ok(vec![drive_type])
}
fn get_drive_type0(address: i64) -> i32 {
    unsafe { GetDriveTypeW(address as LPCWSTR) as i32 }
}

pub fn format_message_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let error_code = args[0];
    let string_ref = format_message(error_code)?;

    Ok(vec![string_ref])
}
fn format_message(error_code: i32) -> crate::error::Result<i32> {
    let mut message = [0 as WCHAR; 256];
    let msg_len = unsafe {
        FormatMessageW(
            FORMAT_MESSAGE_FROM_SYSTEM,
            null_mut(),
            error_code as DWORD,
            0,
            message.as_mut_ptr(),
            255,
            null_mut(),
        )
    };

    if msg_len == 0 {
        return Ok(0);
    }

    let message = &message[..msg_len as usize];
    let message = String::from_utf16(&message)?;
    let message = message.strip_suffix("\n\r.").unwrap_or(&message);
    let string_ref = StringPoolHelper::get_string(message.to_string())?;

    Ok(string_ref)
}
