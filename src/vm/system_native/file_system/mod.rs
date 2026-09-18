use crate::vm::error::Result;
use crate::vm::exception::pending_helpers::set_pending_io_exception;
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::heap::heap::HEAP;
use crate::vm::system_native::string::get_utf8_string_by_ref;
use bitflags::bitflags;
use path_absolutize::Absolutize;
use std::fs::{FileTimes, OpenOptions};
use std::path::Path;
use std::time::UNIX_EPOCH;
use std::time::{Duration, UNIX_EPOCH};
#[cfg(unix)]
use unix::check_access_unix_impl as check_access;
#[cfg(unix)]
use unix::is_hidden_unix as is_hidden;
#[cfg(windows)]
use winnt::check_access_winnt_impl as check_access;
#[cfg(windows)]
use winnt::is_hidden_winnt as is_hidden;

#[cfg(unix)]
pub(crate) mod unix;
#[cfg(windows)]
pub(crate) mod winnt;

/// `java.io.WinNTFileSystem.initIDs()V`
/// `java.io.UnixFileSystem.initIDs()V`
pub(crate) fn init_ids() -> Result<()> {
    // todo: implement me
    Ok(()) // this method is for caching `path` field from java/io/File for faster access in other native methods
}

/// `java.io.WinNTFileSystem.canonicalize0(Ljava/lang/String;)Ljava/lang/String;`
/// `java.io.UnixFileSystem.canonicalize0(Ljava/lang/String;)Ljava/lang/String;`
pub(crate) fn canonicalize0(_this: i32, path_ref: i32) -> Result<i32> {
    let path = get_utf8_string_by_ref(path_ref)?;
    let path = Path::new(&path);

    let canonical_path = match path.absolutize() {
        Ok(path) => path,
        Err(e) => {
            set_pending_io_exception(&format!("Failed to canonicalize path {path:?}: {e}"))?;
            return Ok(0);
        }
    };
    let canonical_name = match canonical_path.to_str() {
        Some(name) => name,
        None => {
            set_pending_io_exception(&format!("Failed to convert path {canonical_path:?}"))?;
            return Ok(0);
        }
    };
    let canonical_name_ref = StringPoolHelper::get_string(canonical_name)?;

    Ok(canonical_name_ref)
}

/// `java.io.WinNTFileSystem.createFileExclusively0(Ljava/lang/String;)Z`
/// `java.io.UnixFileSystem.createFileExclusively0(Ljava/lang/String;)Z`
pub(crate) fn create_file_exclusively0(_this: i32, path_ref: i32) -> Result<bool> {
    let path = get_utf8_string_by_ref(path_ref)?;
    let path = Path::new(&path);

    match std::fs::File::create_new(path) {
        Ok(_) => Ok(true),
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => Ok(false),
        Err(e) => {
            set_pending_io_exception(&format!("Failed to create file {path:?}: {e}"))?;
            Ok(false)
        }
    }
}

bitflags! {
#[derive(Debug, PartialEq)]
    struct AttributeFlags: i32 {
        const BA_EXISTS = 0x01;
        const BA_REGULAR = 0x02;
        const BA_DIRECTORY = 0x04;
        const BA_HIDDEN = 0x08;
    }
}
/// `java.io.WinNTFileSystem.getBooleanAttributes0(Ljava/io/File;)I`
/// `java.io.UnixFileSystem.getBooleanAttributes0(Ljava/io/File;)I`
pub(crate) fn get_boolean_attributes0(_this: i32, file_ref: i32) -> Result<i32> {
    let path_ref = extract_path(file_ref)?;

    let path = get_utf8_string_by_ref(path_ref)?;
    let path = Path::new(&path);

    if !path.exists() {
        return Ok(0);
    }

    let mut attributes = AttributeFlags::BA_EXISTS;
    if path.is_file() {
        attributes |= AttributeFlags::BA_REGULAR;
    }

    if path.is_dir() {
        attributes |= AttributeFlags::BA_DIRECTORY;
    }

    if is_hidden(path) {
        attributes |= AttributeFlags::BA_HIDDEN;
    }

    Ok(attributes.bits())
}

fn extract_path(file_ref: i32) -> Result<i32> {
    let raw = HEAP.get_object_field_value(file_ref, "java/io/File", "path")?;
    Ok(raw[0])
}

bitflags! {
#[derive(Debug, PartialEq)]
    struct Access: i32 {
        const ACCESS_EXECUTE = 0x01;
        const ACCESS_WRITE = 0x02;
        const ACCESS_READ = 0x04;
    }
}
/// `java.io.WinNTFileSystem.checkAccess0(Ljava/io/File;I)Z`
/// `java.io.UnixFileSystem.checkAccess0(Ljava/io/File;I)Z`
pub(crate) fn check_access0(_this: i32, file_ref: i32, access: i32) -> Result<bool> {
    let path_ref = extract_path(file_ref)?;

    let path = get_utf8_string_by_ref(path_ref)?;
    let path = Path::new(&path);

    if !path.exists() {
        return Ok(false);
    }

    let access_flags = Access::from_bits_truncate(access);
    Ok(check_access(path, access_flags))
}

pub(crate) fn set_permission0(
    _this: i32,
    file_ref: i32,
    access: i32,
    enable: bool,
    _owner_only: bool,
) -> Result<bool> {
    let path_ref = extract_path(file_ref)?;
    let path = get_utf8_string_by_ref(path_ref)?;
    let mut permissions = match std::fs::metadata(&path) {
        Ok(metadata) => metadata.permissions(),
        Err(_) => return Ok(false),
    };

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let mask = match Access::from_bits(access) {
            Some(Access::ACCESS_READ) => 0o400 | if _owner_only { 0 } else { 0o044 },
            Some(Access::ACCESS_WRITE) => 0o200 | if _owner_only { 0 } else { 0o022 },
            Some(Access::ACCESS_EXECUTE) => 0o100 | if _owner_only { 0 } else { 0o011 },
            _ => return Ok(false),
        };
        let mode = if enable {
            permissions.mode() | mask
        } else {
            permissions.mode() & !mask
        };
        permissions.set_mode(mode);
    }

    #[cfg(windows)]
    {
        if access != Access::ACCESS_WRITE.bits() {
            return Ok(enable);
        }
        permissions.set_readonly(!enable);
    }

    Ok(std::fs::set_permissions(path, permissions).is_ok())
}

/// `java.io.WinNTFileSystem.delete0(Ljava/io/File;)Z`
/// `java.io.UnixFileSystem.delete0(Ljava/io/File;)Z`
pub(crate) fn delete0(_this: i32, file_ref: i32) -> Result<bool> {
    let path_ref = extract_path(file_ref)?;

    let path = get_utf8_string_by_ref(path_ref)?;
    let path = Path::new(&path);

    if !path.exists() {
        return Ok(false);
    }

    let result = if path.is_dir() {
        std::fs::remove_dir(path)
    } else {
        std::fs::remove_file(path)
    };

    match result {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

/// `java.io.WinNTFileSystem.list0(Ljava/io/File;)[Ljava/lang/String;`
/// `java.io.UnixFileSystem.list0(Ljava/io/File;)[Ljava/lang/String;`
///
/// Return null (0) if the directory cannot be read. Names that aren't valid Unicode are decoded
/// with replacement characters instead of hiding the whole listing.
pub(crate) fn list0(_this: i32, file_ref: i32) -> Result<i32> {
    let path_ref = extract_path(file_ref)?;
    let path = get_utf8_string_by_ref(path_ref)?;
    let entries = match std::fs::read_dir(path) {
        Ok(entries) => entries,
        Err(_) => return Ok(0),
    };
    let mut names = Vec::new();
    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => return Ok(0),
        };
        let file_name = entry.file_name();
        names.push(StringPoolHelper::get_string(&file_name.to_string_lossy())?);
    }

    Ok(HEAP.create_array_with_values("[Ljava/lang/String;", &names))
pub(crate) fn create_directory0(_this: i32, file_ref: i32) -> Result<bool> {
    let path_ref = extract_path(file_ref)?;
    let path = get_utf8_string_by_ref(path_ref)?;
    Ok(std::fs::create_dir(path).is_ok())
pub(crate) fn rename0(_this: i32, source_ref: i32, destination_ref: i32) -> Result<bool> {
    let source_path_ref = extract_path(source_ref)?;
    let destination_path_ref = extract_path(destination_ref)?;
    let source = get_utf8_string_by_ref(source_path_ref)?;
    let destination = get_utf8_string_by_ref(destination_path_ref)?;
    Ok(std::fs::rename(source, destination).is_ok())
}

/// Get the length of the file in bytes.
///
/// Return 0 if metadata cannot be retrieved (e.g., file not found, permission denied, I/O error).
/// This matches Java's File.length() behavior, which returns 0 for non-existent files or on error.
/// `java.io.WinNTFileSystem.getLength0(Ljava/io/File;)J`
/// `java.io.UnixFileSystem.getLength0(Ljava/io/File;)J`
pub(crate) fn get_length0(_this: i32, file_ref: i32) -> Result<i64> {
    let path_ref = extract_path(file_ref)?;

    let path = get_utf8_string_by_ref(path_ref)?;
    let path = Path::new(&path);

    let len = std::fs::metadata(path)
        .map(|metadata| metadata.len())
        .unwrap_or(0);

    Ok(len as i64)
}

pub(crate) fn get_last_modified_time0(_this: i32, file_ref: i32) -> Result<i64> {
    let path_ref = extract_path(file_ref)?;
    let path = get_utf8_string_by_ref(path_ref)?;
    let modified = std::fs::metadata(path)
        .and_then(|metadata| metadata.modified())
        .ok()
        .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
        .map(|duration| duration.as_millis().min(i64::MAX as u128) as i64)
        .unwrap_or(0);

    Ok(modified)
pub(crate) fn set_last_modified_time0(
    _this: i32,
    file_ref: i32,
    time_millis: i64,
) -> Result<bool> {
    if time_millis < 0 {
        return Ok(false);
    }
    let path_ref = extract_path(file_ref)?;
    let path = get_utf8_string_by_ref(path_ref)?;
    let Some(modified) = UNIX_EPOCH.checked_add(Duration::from_millis(time_millis as u64)) else {
        return Ok(false);
    };
    let result = OpenOptions::new()
        .write(true)
        .open(path)
        .and_then(|file| file.set_times(FileTimes::new().set_modified(modified)));

    Ok(result.is_ok())
pub(crate) fn set_read_only0(_this: i32, file_ref: i32) -> Result<bool> {
    let path_ref = extract_path(file_ref)?;
    let path = get_utf8_string_by_ref(path_ref)?;
    let mut permissions = match std::fs::metadata(&path) {
        Ok(metadata) => metadata.permissions(),
        Err(_) => return Ok(false),
    };
    permissions.set_readonly(true);
    Ok(std::fs::set_permissions(path, permissions).is_ok())
pub(crate) fn get_space0(_this: i32, file_ref: i32, space_type: i32) -> Result<i64> {
    let path_ref = extract_path(file_ref)?;
    let path = get_utf8_string_by_ref(path_ref)?;
    let space = match space_type {
        0 => fs2::total_space(path),
        1 => fs2::free_space(path),
        2 => fs2::available_space(path),
        _ => return Ok(0),
    }
    .unwrap_or(0)
    .min(i64::MAX as u64);

    Ok(space as i64)
}
