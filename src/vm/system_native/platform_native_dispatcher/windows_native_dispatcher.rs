use crate::bail_thrown;
use crate::vm::error::Result;
use crate::vm::exception::helpers::throw_internal_error;
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::i64_to_vec;
use crate::vm::system_native::platform_native_dispatcher::windows_helpers::{
    throw_windows_exception, wchar_to_string_ref,
};
use std::mem::zeroed;
use std::ptr::null_mut;
use winapi::shared::minwindef::{DWORD, FALSE, LPVOID, MAX_PATH, TRUE};
use winapi::shared::winerror::{ERROR_INSUFFICIENT_BUFFER, ERROR_NO_MORE_FILES, ERROR_NO_TOKEN};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::fileapi::{
    CreateDirectoryW, DeleteFileW, FindClose, FindFirstFileW, GetDriveTypeW, GetFileAttributesExW,
    GetFullPathNameW, GetVolumeInformationW, GetVolumePathNameW, RemoveDirectoryW, SetEndOfFile,
};
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::minwinbase::{
    GetFileExInfoStandard, LPSECURITY_ATTRIBUTES, SECURITY_ATTRIBUTES, WIN32_FIND_DATAW,
};
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

pub(crate) fn init_ids() -> Result<()> {
    // todo: implement me
    Ok(())
}

pub(crate) fn create_directory0(address_ptr: i64, sd_address_ptr: i64) -> Result<()> {
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

pub(crate) fn get_file_attributes_ex0(lp_file_name_addr: i64, output_addr: i64) -> Result<()> {
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

pub(crate) fn delete_file0(lp_file_name_addr: i64) -> Result<()> {
    let address = lp_file_name_addr as usize as LPCWSTR;
    let res = unsafe { DeleteFileW(address) };
    if res == 0 {
        throw_windows_exception(stack_frames)?;
    }

    Ok(())
}

pub(crate) fn remove_directory0(lp_file_name_addr: i64) -> Result<()> {
    let address = lp_file_name_addr as usize as LPCWSTR;
    let res = unsafe { RemoveDirectoryW(address) };
    if res == 0 {
        throw_windows_exception(stack_frames)?;
    }

    Ok(())
}

pub(crate) fn create_file0(
    filename_ptr: i64,
    desired_access: i32,
    share_mode: i32,
    sec_desc_ptr: i64,
    creation_disposition: i32,
    flags_and_attributes: i32,
) -> Result<i64> {
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
        bail_thrown!(throw_windows_exception(stack_frames))
    }

    Ok(Some(result as i64))
}

pub(crate) fn set_end_of_file(handle_ptr: i64) -> Result<()> {
    let handle = handle_ptr as usize as HANDLE;
    let result = unsafe { SetEndOfFile(handle) };

    if result == 0 {
        throw_windows_exception(stack_frames)?;
    }

    Ok(())
}

pub(crate) fn get_file_security0(
    lp_file_name_addr: i64,
    requested_information: i32,
    p_security_descriptor_addr: i64,
    n_length: i32,
) -> Result<i32> {
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
            Ok(Some(length_needed as i32))
        } else {
            bail_thrown!(throw_windows_exception(stack_frames))
        }
    } else {
        Ok(Some(n_length))
    }
}

pub(crate) fn get_current_process() -> Result<i64> {
    let handle = unsafe { GetCurrentProcess() as usize as i64 };
    Ok(handle)
}

pub(crate) fn open_process_token(process: i64, desired_access: i32) -> Result<i64> {
    let mut h_token = 0i64 as usize as HANDLE;
    let result = unsafe {
        OpenProcessToken(
            process as usize as HANDLE,
            desired_access as DWORD,
            &mut h_token,
        )
    };
    if result == 0 {
        bail_thrown!(throw_windows_exception(stack_frames))
    }

    Ok(Some(h_token as usize as i64))
}

pub(crate) fn get_current_thread() -> Result<i64> {
    let handle = unsafe { GetCurrentThread() as usize as i64 };
    Ok(handle)
}

pub(crate) fn open_thread_token(
    thread: i64,
    desired_access: i32,
    open_as_self: bool,
) -> Result<i64> {
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
            ERROR_NO_TOKEN => Ok(Some(0)),
            _ => {
                bail_thrown!(throw_windows_exception(stack_frames))
            }
        }
    } else {
        Ok(Some(h_token as usize as i64))
    }
}

pub(crate) fn duplicate_token_ex(token: i64, desired_access: i32) -> Result<i64> {
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
        bail_thrown!(throw_windows_exception(stack_frames))
    }

    Ok(Some(result_token as usize as i64))
}

pub(crate) fn access_check(
    token: i64,
    security_info: i64,
    access_mask: i32,
    generic_read: i32,
    generic_write: i32,
    generic_execute: i32,
    generic_all: i32,
) -> Result<bool> {
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
        bail_thrown!(throw_windows_exception(stack_frames))
    } else {
        Ok(Some(result == TRUE))
    }
}

pub(crate) fn close_handle(handle: i64) {
    let _result = unsafe { CloseHandle(handle as HANDLE) };
}

pub(crate) fn get_volume_path_name0(address: i64) -> Result<i32> {
    let mut volume_name = [0 as WCHAR; MAX_PATH + 1];
    let result = unsafe {
        GetVolumePathNameW(
            address as LPCWSTR,
            volume_name.as_mut_ptr(),
            MAX_PATH as DWORD + 1,
        )
    };

    if result == 0 {
        bail_thrown!(throw_windows_exception(stack_frames))
    }

    let string_ref = wchar_to_string_ref(&volume_name)?;

    Ok(Some(string_ref))
}

pub(crate) fn get_volume_information0(address: i64, volume_information_ref: i32) -> Result<()> {
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
        bail_thrown!(throw_windows_exception(stack_frames))
    }

    let volume_name_ref = (wchar_to_string_ref(&volume_name))?;
    let filesystem_name_ref = (wchar_to_string_ref(&filesystem_name))?;

    HEAP.set_object_field_value(
        volume_information_ref,
        "sun/nio/fs/WindowsNativeDispatcher$VolumeInformation",
        "volumeName",
        vec![volume_name_ref],
    )?;
    HEAP.set_object_field_value(
        volume_information_ref,
        "sun/nio/fs/WindowsNativeDispatcher$VolumeInformation",
        "fileSystemName",
        vec![filesystem_name_ref],
    )?;
    HEAP.set_object_field_value(
        volume_information_ref,
        "sun/nio/fs/WindowsNativeDispatcher$VolumeInformation",
        "volumeSerialNumber",
        vec![volume_serial_number as i32],
    )?;
    HEAP.set_object_field_value(
        volume_information_ref,
        "sun/nio/fs/WindowsNativeDispatcher$VolumeInformation",
        "flags",
        vec![flags as i32],
    )?;

    Ok(Some(()))
}

pub(crate) fn get_drive_type0(address: i64) -> i32 {
    unsafe { GetDriveTypeW(address as LPCWSTR) as i32 }
}

pub(crate) fn format_message(error_code: i32) -> Result<i32> {
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
    let string_ref = StringPoolHelper::get_string(message)?;

    Ok(string_ref)
}

pub(crate) fn get_full_path_name0(address: i64) -> Result<i32> {
    let lp_file_name = address as usize as LPCWSTR;
    let mut result = vec![0 as WCHAR; MAX_PATH + 1];

    let len = unsafe {
        GetFullPathNameW(
            lp_file_name,
            MAX_PATH as DWORD,
            result.as_mut_ptr(),
            0 as *mut _,
        )
    };
    if len == 0 {
        bail_thrown!(throw_windows_exception(stack_frames))
    }
    if len < MAX_PATH as DWORD {
        let string_ref = wchar_to_string_ref(&result[..(len + 1) as usize])?;
        Ok(Some(string_ref))
    } else {
        result.resize((len + 1) as usize, 0);
        let len = unsafe { GetFullPathNameW(lp_file_name, len, result.as_mut_ptr(), 0 as *mut _) };
        if len == 0 {
            bail_thrown!(throw_internal_error(
                "GetFullPathNameW failed",
                stack_frames
            ))
        } else {
            let string_ref = wchar_to_string_ref(&result[..len as usize])?;
            Ok(Some(string_ref))
        }
    }
}

pub(crate) fn find_first_file0(lp_file_name: i64, first_file_obj_ref: i32) -> Result<()> {
    let mut data: WIN32_FIND_DATAW = unsafe { zeroed() };
    let lp_file_name = lp_file_name as usize as LPCWSTR;
    let handle = unsafe { FindFirstFileW(lp_file_name, &mut data) };
    if handle == INVALID_HANDLE_VALUE {
        bail_thrown!(throw_windows_exception(stack_frames))
    }

    let c_file_name = data.cFileName;
    let name_ref = (wchar_to_string_ref(&c_file_name))?;
    let attributes = data.dwFileAttributes as i32;

    HEAP.set_object_field_value(
        first_file_obj_ref,
        "sun/nio/fs/WindowsNativeDispatcher$FirstFile",
        "handle",
        i64_to_vec(handle as i64),
    )?;
    HEAP.set_object_field_value(
        first_file_obj_ref,
        "sun/nio/fs/WindowsNativeDispatcher$FirstFile",
        "name",
        vec![name_ref],
    )?;
    HEAP.set_object_field_value(
        first_file_obj_ref,
        "sun/nio/fs/WindowsNativeDispatcher$FirstFile",
        "attributes",
        vec![attributes],
    )?;

    Ok(Some(()))
}

pub(crate) fn find_next_file0(handle: i64, address: i64) -> Result<i32> {
    let handle = handle as usize as HANDLE;
    let data = address as usize as *mut WIN32_FIND_DATAW;
    let result = unsafe { winapi::um::fileapi::FindNextFileW(handle, data) };
    if result != 0 {
        let c_file_name = unsafe { (*data).cFileName };
        let name_ref = (wchar_to_string_ref(&c_file_name))?;
        Ok(Some(name_ref))
    } else {
        let last_error = unsafe { GetLastError() };
        if last_error == ERROR_NO_MORE_FILES {
            Ok(Some(0))
        } else {
            bail_thrown!(throw_windows_exception(stack_frames))
        }
    }
}

pub(crate) fn find_close(handle: i64) -> Result<()> {
    let handle = handle as usize as HANDLE;
    let result = unsafe { FindClose(handle) };
    if result == 0 {
        bail_thrown!(throw_windows_exception(stack_frames))
    }
    Ok(Some(()))
}
