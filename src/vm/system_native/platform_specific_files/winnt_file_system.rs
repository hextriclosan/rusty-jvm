use crate::vm::error::{Error, ErrorKind, Result};
use crate::vm::exception::helpers::{throw_ioexception, throw_null_pointer_exception};
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::stack::stack_frame::StackFrames;
use crate::vm::system_native::platform_native_dispatcher::windows_helpers::{
    get_last_error, strip_string,
};
use crate::vm::system_native::platform_specific_files::wide_cstring::WideCString;
use crate::vm::system_native::string::get_utf8_string_by_ref;
use std::ptr::null_mut;
use winapi::shared::minwindef::{DWORD, MAX_PATH};
use winapi::um::errhandlingapi::{GetLastError, SetLastError};
use winapi::um::fileapi::{CreateFileW, GetFinalPathNameByHandleW, OPEN_EXISTING};
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

    match get_final_path0(path_ref, stack_frames) {
        Ok(final_path_ref) => Ok(vec![final_path_ref]),
        Err(e) if matches!(e.kind(), ErrorKind::ExceptionThrown) => Ok(vec![]),
        Err(e) => Err(e),
    }
}
fn get_final_path0(path_ref: i32, stack_frames: &mut StackFrames) -> Result<i32> {
    if path_ref == 0 {
        throw_null_pointer_exception("Path is null", stack_frames)?;
        return Err(Error::new_exception());
    }

    let path = get_utf8_string_by_ref(path_ref)?;
    let wide_path = WideCString::new(&path);
    let final_path = match get_final_path0_impl(&wide_path) {
        Ok(final_path) => Ok::<String, Error>(final_path),
        Err(e) => {
            let error_msg = format!("Bad pathname: {path}, ({e}) ({})", get_last_error()?);
            throw_ioexception(&error_msg, stack_frames)?;
            Err(Error::new_exception())
        }
    }?;
    let final_path_ref = StringPoolHelper::get_string(&final_path)?;
    Ok(final_path_ref)
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
