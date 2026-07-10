use crate::bail_thrown;
use crate::vm::error::{Error, Result};
use crate::vm::exception::helpers::{
    throw_ioexception, throw_null_pointer_exception_with_message,
};
use crate::vm::exception::pending::Throws;
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::stack::stack_frame::StackFrames;
use crate::vm::system_native::io_file_system::delete0;
use crate::vm::system_native::platform_native_dispatcher::windows_helpers::{
    get_last_error, strip_string,
};
use crate::vm::system_native::platform_specific_files::wide_cstring::WideCString;
use crate::vm::system_native::string::get_utf8_string_by_ref;
use std::ptr::null_mut;
use winapi::shared::minwindef::{DWORD, MAX_PATH};
use winapi::um::errhandlingapi::{GetLastError, SetLastError};
use winapi::um::fileapi::{
    CreateFileW, GetFinalPathNameByHandleW, GetVolumeInformationW, OPEN_EXISTING,
};
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::winbase::FILE_FLAG_BACKUP_SEMANTICS;
use winapi::um::winnt::{
    FILE_READ_ATTRIBUTES, FILE_SHARE_DELETE, FILE_SHARE_READ, FILE_SHARE_WRITE, HANDLE, WCHAR,
};

pub(crate) fn get_final_path0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let _filesystem_impl_ref = args[0];
    let path_ref = args[1];

    let Some(ret) = get_final_path0(path_ref, stack_frames)? else {
        return Ok(vec![]);
    };
    Ok(vec![ret])
}
fn get_final_path0(path_ref: i32, stack_frames: &mut StackFrames) -> Throws<i32> {
    if path_ref == 0 {
        bail_thrown!(throw_null_pointer_exception_with_message(
            "Path is null",
            stack_frames
        ))
    }

    let path = get_utf8_string_by_ref(path_ref)?;
    let wide_path = WideCString::new(&path);
    let final_path = match get_final_path0_impl(&wide_path) {
        Ok(final_path) => final_path,
        Err(e) => {
            let error_msg = format!("Bad pathname: {path} - ({e}) ({})", get_last_error()?);
            bail_thrown!(throw_ioexception(&error_msg, stack_frames))
        }
    };
    let final_path_ref = StringPoolHelper::get_string(&final_path)?;
    Ok(Some(final_path_ref))
}
fn get_final_path0_impl(path: &WideCString) -> Result<String> {
    let handle = unsafe {
        CreateFileW(
            path.as_ptr(),
            FILE_READ_ATTRIBUTES,
            FILE_SHARE_DELETE | FILE_SHARE_READ | FILE_SHARE_WRITE,
            null_mut(),
            OPEN_EXISTING,
            FILE_FLAG_BACKUP_SEMANTICS,
            null_mut(),
        )
    };
    if handle == INVALID_HANDLE_VALUE {
        return Err(Error::new_native("CreateFileW"));
    }

    let mut result = vec![0 as WCHAR; MAX_PATH];
    let mut len =
        unsafe { GetFinalPathNameByHandleW(handle, result.as_mut_ptr(), MAX_PATH as DWORD, 0) };

    if len >= MAX_PATH as DWORD {
        result.resize((len + 1) as usize, 0);
        len = unsafe { GetFinalPathNameByHandleW(handle, result.as_mut_ptr(), len, 0) };
    }

    unsafe {
        let error = GetLastError();
        if CloseHandle(handle as HANDLE) == 0 {
            // If CloseHandle fails, we need to restore the error code from GetFinalPathNameByHandleW
            SetLastError(error);
        }
    }

    if len == 0 {
        return Err(Error::new_native("GetFinalPathNameByHandleW returned 0"));
    }

    let result = strip_string(&result)?;
    let result = if let Some(rest) = result.strip_prefix(r"\\?\UNC") {
        // \\?\UNC\server\share → \\server\share
        format!(r"\{}", rest)
    } else if let Some(rest) = result.strip_prefix(r"\\?\") {
        // \\?\C:\foo → C:\foo
        rest.to_string()
    } else {
        result
    };

    Ok(result)
}

pub(crate) fn winnt_file_system_delete0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let _filesystem_impl_ref = args[0];
    let file_ref = args[1];
    let allow_delete_readonly = args[2] != 0;

    if allow_delete_readonly {
        return Err(Error::new_native(
            "-Djdk.io.File.allowDeleteReadOnlyFiles is not supported (JDK-8356195)",
        ));
    }

    let deleted = delete0(file_ref)?;
    Ok(vec![if deleted { 1 } else { 0 }])
}

pub(crate) fn get_name_max0_wrp(args: &[i32], stack_frames: &mut StackFrames) -> Result<Vec<i32>> {
    let _filesystem_impl_ref = args[0];
    let path_ref = args[1];

    let Some(ret) = get_name_max0(path_ref, stack_frames)? else {
        return Ok(vec![]);
    };
    Ok(vec![ret])
}
fn get_name_max0(path_ref: i32, stack_frames: &mut StackFrames) -> Throws<i32> {
    let path_storage;
    let path = if path_ref == 0 {
        std::ptr::null()
    } else {
        let path_str = get_utf8_string_by_ref(path_ref)?;
        path_storage = WideCString::new(&path_str);
        path_storage.as_ptr()
    };

    let mut maximum_component_length: DWORD = 0;
    let res = unsafe {
        GetVolumeInformationW(
            path,
            null_mut(),
            0,
            null_mut(),
            &mut maximum_component_length,
            null_mut(),
            null_mut(),
            0,
        )
    };

    if res == 0 {
        let error_code = unsafe { GetLastError() };
        let error_msg = format!(
            "GetVolumeInformationW failed for path {path_ref} with error code {error_code}"
        );
        bail_thrown!(throw_ioexception(&error_msg, stack_frames))
    }

    Ok(Some(maximum_component_length as i32))
}
