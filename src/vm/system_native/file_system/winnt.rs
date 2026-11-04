use crate::vm::error::{Error, Result};
use crate::vm::exception::pending_helpers::{
    set_pending_io_exception, set_pending_null_pointer_exception_with_message,
};
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::system_native::file_system::{delete0 as delete_impl, file_path, Access};
use crate::vm::system_native::platform_native_dispatcher::windows_helpers::{
    get_last_error, strip_string,
};
use crate::vm::system_native::string::get_utf8_string_by_ref;
use crate::vm::system_native::win_util::wide_cstring::WideCString;
use std::fs::{FileTimes, Metadata, OpenOptions};
use std::os::windows::fs::{MetadataExt, OpenOptionsExt};
use std::path::Path;
use std::ptr::null_mut;
use std::time::{Duration, UNIX_EPOCH};
use winapi::shared::minwindef::{DWORD, MAX_PATH};
use winapi::um::errhandlingapi::{GetLastError, SetLastError};
use winapi::um::fileapi::{
    CreateFileW, GetDiskFreeSpaceExW, GetFileAttributesW, GetFinalPathNameByHandleW,
    GetVolumeInformationW, GetVolumePathNameW, SetFileAttributesW, INVALID_FILE_ATTRIBUTES,
    OPEN_EXISTING,
};
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::winbase::{MoveFileExW, FILE_FLAG_BACKUP_SEMANTICS, MOVEFILE_COPY_ALLOWED};
use winapi::um::winnt::{
    FILE_ATTRIBUTE_DIRECTORY, FILE_ATTRIBUTE_READONLY, FILE_ATTRIBUTE_REPARSE_POINT,
    FILE_READ_ATTRIBUTES, FILE_SHARE_DELETE, FILE_SHARE_READ, FILE_SHARE_WRITE,
    FILE_WRITE_ATTRIBUTES, HANDLE, ULARGE_INTEGER, WCHAR,
};

pub(super) fn check_access_winnt_impl(path: &Path, mode: Access) -> bool {
    use std::os::windows::ffi::OsStrExt;
    use winapi::um::fileapi::{GetFileAttributesW, INVALID_FILE_ATTRIBUTES};
    use winapi::um::winnt::{FILE_ATTRIBUTE_DIRECTORY, FILE_ATTRIBUTE_READONLY};

    let wide_path: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();
    let attr = unsafe { GetFileAttributesW(wide_path.as_ptr()) }; // todo: add support of reparse point cases
    if attr == INVALID_FILE_ATTRIBUTES {
        return false;
    }

    match mode {
        Access::ACCESS_READ | Access::ACCESS_EXECUTE => true,
        Access::ACCESS_WRITE
            if (attr & FILE_ATTRIBUTE_DIRECTORY) > 0 || (attr & FILE_ATTRIBUTE_READONLY) == 0 =>
        {
            true
        }
        _ => false,
    }
}

/// `java.io.WinNTFileSystem.getFinalPath0(Ljava/lang/String;)Ljava/lang/String;`
pub(crate) fn get_final_path0(_this: i32, path_ref: i32) -> Result<i32> {
    if path_ref == 0 {
        set_pending_null_pointer_exception_with_message("Path is null")?;
        return Ok(0);
    }

    let path = get_utf8_string_by_ref(path_ref)?;
    let wide_path = WideCString::new(&path);
    let final_path = match get_final_path0_impl(&wide_path) {
        Ok(final_path) => final_path,
        Err(e) => {
            let error_msg = format!("Bad pathname: {path} - ({e}) ({})", get_last_error()?);
            set_pending_io_exception(&error_msg)?;
            return Ok(0);
        }
    };
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

/// `java.io.WinNTFileSystem.delete0(Ljava/io/File;Z)Z`
pub(crate) fn delete0(this: i32, file_ref: i32, allow_delete_readonly: bool) -> Result<bool> {
    if allow_delete_readonly {
        return Err(Error::new_native(
            "-Djdk.io.File.allowDeleteReadOnlyFiles is not supported (JDK-8356195)",
        ));
    }

    let deleted = delete_impl(this, file_ref)?;
    Ok(deleted)
}

/// `java.io.WinNTFileSystem.getNameMax0(Ljava/lang/String;)I`
pub(crate) fn get_name_max0(_this: i32, path_ref: i32) -> Result<i32> {
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
        set_pending_io_exception(&error_msg)?;
    }

    Ok(maximum_component_length as i32)
}

pub(super) fn is_hidden_winnt(path: &Path) -> bool {
    use std::os::windows::ffi::OsStrExt;
    use winapi::um::fileapi::{GetFileAttributesW, INVALID_FILE_ATTRIBUTES};
    use winapi::um::winnt::FILE_ATTRIBUTE_HIDDEN;

    let wide_path = path
        .as_os_str()
        .encode_wide()
        .chain(Some(0))
        .collect::<Vec<_>>();
    let attr = unsafe { GetFileAttributesW(wide_path.as_ptr()) };
    if attr == INVALID_FILE_ATTRIBUTES {
        false
    } else {
        attr & FILE_ATTRIBUTE_HIDDEN != 0
    }
}

/// Whether `delete0` removes the entry as a directory: like HotSpot, by its directory attribute,
/// which a symbolic link or junction to a directory also has, and `RemoveDirectoryW` removes those
/// as links.
pub(super) fn is_directory_winnt(metadata: &Metadata) -> bool {
    metadata.file_attributes() & FILE_ATTRIBUTE_DIRECTORY != 0
}

/// `java.io.WinNTFileSystem.setPermission0(Ljava/io/File;IZZ)Z`
///
/// Like HotSpot, only write access of a file can change, through its read-only attribute. Read and
/// execute access can't be taken away, so changing them only succeeds when enabling them.
pub(crate) fn set_permission0(
    _this: i32,
    file_ref: i32,
    access: i32,
    enable: bool,
    _owner_only: bool,
) -> Result<bool> {
    if access == Access::ACCESS_READ.bits() || access == Access::ACCESS_EXECUTE.bits() {
        return Ok(enable);
    }
    let Some((path, attributes)) = file_target_attributes(&file_path(file_ref)?) else {
        return Ok(false);
    };
    let attributes = if enable {
        attributes & !FILE_ATTRIBUTE_READONLY
    } else {
        attributes | FILE_ATTRIBUTE_READONLY
    };
    Ok(unsafe { SetFileAttributesW(path.as_ptr(), attributes) } != 0)
}

/// `java.io.WinNTFileSystem.setReadOnly0(Ljava/io/File;)Z`
pub(crate) fn set_read_only0(_this: i32, file_ref: i32) -> Result<bool> {
    let Some((path, attributes)) = file_target_attributes(&file_path(file_ref)?) else {
        return Ok(false);
    };
    Ok(unsafe { SetFileAttributesW(path.as_ptr(), attributes | FILE_ATTRIBUTE_READONLY) } != 0)
}

/// The attributes of the file at `path`, or of the final target if it is a reparse point such as a
/// symbolic link, with the path they belong to. `None` if there is no such file, or if it is a
/// directory: like HotSpot, the read-only attribute of a directory isn't changed.
fn file_target_attributes(path: &str) -> Option<(WideCString, DWORD)> {
    let mut path = WideCString::new(path);
    let mut attributes = unsafe { GetFileAttributesW(path.as_ptr()) };
    if attributes != INVALID_FILE_ATTRIBUTES && attributes & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
        attributes = match get_final_path0_impl(&path) {
            Ok(final_path) => {
                path = WideCString::new(&final_path);
                unsafe { GetFileAttributesW(path.as_ptr()) }
            }
            Err(_) => INVALID_FILE_ATTRIBUTES,
        };
    }

    let is_file =
        attributes != INVALID_FILE_ATTRIBUTES && attributes & FILE_ATTRIBUTE_DIRECTORY == 0;
    is_file.then_some((path, attributes))
}

/// `java.io.WinNTFileSystem.rename0(Ljava/io/File;Ljava/io/File;)Z`
///
/// Like `_wrename`, which HotSpot calls, this doesn't replace an existing destination, and may move
/// a file to another volume.
pub(crate) fn rename0(_this: i32, from_ref: i32, to_ref: i32) -> Result<bool> {
    let from = WideCString::new(&file_path(from_ref)?);
    let to = WideCString::new(&file_path(to_ref)?);
    Ok(unsafe { MoveFileExW(from.as_ptr(), to.as_ptr(), MOVEFILE_COPY_ALLOWED) } != 0)
}

/// `java.io.WinNTFileSystem.setLastModifiedTime0(Ljava/io/File;J)Z`
///
/// Opens the file or directory like HotSpot, only to write its attributes, so that this works on
/// a read-only file. `File.setLastModified` rejects negative times.
pub(crate) fn set_last_modified_time0(_this: i32, file_ref: i32, time: i64) -> Result<bool> {
    let modified = UNIX_EPOCH + Duration::from_millis(time as u64);
    let result = OpenOptions::new()
        .access_mode(FILE_WRITE_ATTRIBUTES)
        .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE)
        .custom_flags(FILE_FLAG_BACKUP_SEMANTICS)
        .open(file_path(file_ref)?)
        .and_then(|file| file.set_times(FileTimes::new().set_modified(modified)));
    Ok(result.is_ok())
}

/// `java.io.WinNTFileSystem.getSpace0(Ljava/io/File;I)J`
///
/// The total, free or usable space (`FileSystem.SPACE_TOTAL`, `SPACE_FREE` and `SPACE_USABLE`) of
/// the volume holding the file, or 0 on failure.
pub(crate) fn get_space0(_this: i32, file_ref: i32, space_type: i32) -> Result<i64> {
    // like HotSpot's fileToNTPath, which makes relative paths absolute
    let Ok(path) = std::path::absolute(file_path(file_ref)?) else {
        return Ok(0);
    };
    let path = path.to_string_lossy();
    // the volume path is at most the path with a trailing backslash
    let capacity = path.encode_utf16().count() + 2;
    let mut volume = vec![0 as WCHAR; capacity];
    let path = WideCString::new(&path);
    if unsafe { GetVolumePathNameW(path.as_ptr(), volume.as_mut_ptr(), capacity as DWORD) } == 0 {
        return Ok(0);
    }

    let (mut usable, mut total, mut free): (ULARGE_INTEGER, ULARGE_INTEGER, ULARGE_INTEGER) =
        unsafe { std::mem::zeroed() };
    if unsafe { GetDiskFreeSpaceExW(volume.as_ptr(), &mut usable, &mut total, &mut free) } == 0 {
        return Ok(0);
    }
    let space = match space_type {
        0 => total,
        1 => free,
        2 => usable,
        _ => return Ok(0),
    };
    Ok(unsafe { *space.QuadPart() } as i64)
}
