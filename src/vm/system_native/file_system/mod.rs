use crate::vm::error::{Error, Result};
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

pub(crate) fn init_ids() -> Result<()> {
    // todo: implement me
    Ok(()) // this method is for caching `path` field from java/io/File for faster access in other native methods
}

pub(crate) fn canonicalize0(_this: i32, path_ref: i32) -> Result<i32> {
    let path = get_utf8_string_by_ref(path_ref)?;
    let path = Path::new(&path);

    let canonical_path = path.absolutize()?;
    let canonical_name = canonical_path.to_str().ok_or_else(|| {
        Error::new_execution(&format!("Failed to convert path {canonical_path:?}"))
    })?;
    let canonical_name_ref = StringPoolHelper::get_string(canonical_name)?;

    Ok(canonical_name_ref)
}

pub(crate) fn create_file_exclusively0(_this: i32, path_ref: i32) -> Result<bool> {
    let path = get_utf8_string_by_ref(path_ref)?;
    let path = Path::new(&path);

    if path.exists() {
        return Ok(false);
    }

    std::fs::File::create_new(path)
        .map(|_| true)
        .map_err(|e| Error::new_execution(&format!("Failed to create file {path:?}: {e}")))
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
pub(crate) fn check_access0(_this: i32, file_ref: i32, access: i32) -> Result<bool> {
    let path_ref = extract_path(file_ref)?;

    let path = get_utf8_string_by_ref(path_ref)?;
    let path = Path::new(&path);

    if !path.exists() {
        return Ok(false);
    }

    let access_flags = Access::from_bits(access).expect("Invalid access flags");

    Ok(check_access(path, access_flags))
}

pub(crate) fn delete0(_this: i32, file_ref: i32) -> Result<bool> {
    let path_ref = extract_path(file_ref)?;

    let path = get_utf8_string_by_ref(path_ref)?;
    let path = Path::new(&path);

    if !path.exists() {
        return Ok(false);
    }

    std::fs::remove_file(path)
        .map(|_| true)
        .map_err(|e| Error::new_execution(&format!("Failed to delete file {path:?}: {e}")))
}

/// Get the length of the file in bytes.
///
/// Return 0 if metadata cannot be retrieved (e.g., file not found, permission denied, I/O error).
/// This matches Java's File.length() behavior, which returns 0 for non-existent files or on error.
pub(crate) fn get_length0(_this: i32, file_ref: i32) -> Result<i64> {
    let path_ref = extract_path(file_ref)?;

    let path = get_utf8_string_by_ref(path_ref)?;
    let path = Path::new(&path);

    let len = std::fs::metadata(path)
        .map(|metadata| metadata.len())
        .unwrap_or(0);

    Ok(len as i64)
}
