use crate::vm::error::Result;
use crate::vm::exception::pending_helpers::set_pending_io_exception;
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::heap::heap::HEAP;
use crate::vm::system_native::string::get_utf8_string_by_ref;
use bitflags::bitflags;
use path_absolutize::Absolutize;
use std::path::Path;
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

/// `java.io.WinNTFileSystem.delete0(Ljava/io/File;)Z`
/// `java.io.UnixFileSystem.delete0(Ljava/io/File;)Z`
pub(crate) fn delete0(_this: i32, file_ref: i32) -> Result<bool> {
    let path_ref = extract_path(file_ref)?;

    let path = get_utf8_string_by_ref(path_ref)?;
    let path = Path::new(&path);

    if !path.exists() {
        return Ok(false);
    }

    match std::fs::remove_file(path) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
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
