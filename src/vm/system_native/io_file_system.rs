use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::i64_to_vec;
use crate::vm::system_native::string::get_utf8_string_by_ref;
use bitflags::bitflags;
use path_absolutize::Absolutize;
use std::path::Path;

pub(crate) fn canonicalize0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let _filesystem_impl_ref = args[0];
    let path_ref = args[1];
    let canonical_path_ref = canonicalize0(path_ref)?;

    Ok(vec![canonical_path_ref])
}
fn canonicalize0(path_ref: i32) -> Result<i32> {
    let path = get_utf8_string_by_ref(path_ref)?;
    let path = Path::new(&path);

    let canonical_path = path.absolutize()?;
    let canonical_name = canonical_path.to_str().ok_or_else(|| {
        Error::new_execution(&format!("Failed to convert path {canonical_path:?}"))
    })?;
    let canonical_name_ref = StringPoolHelper::get_string(canonical_name)?;

    Ok(canonical_name_ref)
}

pub(crate) fn create_file_exclusively0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let _filesystem_impl_ref = args[0];
    let path_ref = args[1];
    let created = create_file_exclusively0(path_ref)?;

    Ok(vec![if created { 1 } else { 0 }])
}
fn create_file_exclusively0(path_ref: i32) -> Result<bool> {
    let path = get_utf8_string_by_ref(path_ref)?;
    let path = Path::new(&path);

    if path.exists() {
        return Ok(false);
    }

    std::fs::File::create_new(path)
        .map(|_| true)
        .map_err(|e| Error::new_execution(&format!("Failed to create file {path:?}: {e}")))
}

#[cfg(unix)]
pub(crate) fn delete0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let _filesystem_impl_ref = args[0];
    let file_ref = args[1];
    let deleted = delete0(file_ref)?;

    Ok(vec![if deleted { 1 } else { 0 }])
}
pub(crate) fn delete0(file_ref: i32) -> Result<bool> {
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

pub(crate) fn get_boolean_attributes0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let _filesystem_impl_ref = args[0];
    let file_ref = args[1];
    let attributes = get_boolean_attributes0(file_ref)?;

    Ok(vec![attributes])
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
fn get_boolean_attributes0(file_ref: i32) -> Result<i32> {
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
#[cfg(target_os = "windows")]
fn is_hidden(path: &Path) -> bool {
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
#[cfg(unix)]
fn is_hidden(path: &Path) -> bool {
    if let Some(name) = path.file_name() {
        if let Some(name) = name.to_str() {
            return name.starts_with('.');
        }
    }

    false
}

pub(crate) fn check_access0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let _filesystem_impl_ref = args[0];
    let file_ref = args[1];
    let access = args[2];
    let result = check_access0(file_ref, access)?;

    Ok(vec![if result { 1 } else { 0 }])
}
bitflags! {
#[derive(Debug, PartialEq)]
    struct Access: i32 {
        const ACCESS_EXECUTE = 0x01;
        const ACCESS_WRITE = 0x02;
        const ACCESS_READ = 0x04;
    }
}
fn check_access0(file_ref: i32, access: i32) -> Result<bool> {
    let path_ref = extract_path(file_ref)?;

    let path = get_utf8_string_by_ref(path_ref)?;
    let path = Path::new(&path);

    if !path.exists() {
        return Ok(false);
    }

    let access_flags = Access::from_bits(access).expect("Invalid access flags");

    Ok(check_access_impl(path, access_flags))
}

pub(crate) fn get_length0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let _filesystem_impl_ref = args[0];
    let file_ref = args[1];
    let length = get_length0(file_ref)?;

    Ok(i64_to_vec(length))
}
/// Get the length of the file in bytes.
///
/// Return 0 if metadata cannot be retrieved (e.g., file not found, permission denied, I/O error).
/// This matches Java's File.length() behavior, which returns 0 for non-existent files or on error.
fn get_length0(file_ref: i32) -> Result<i64> {
    let path_ref = extract_path(file_ref)?;

    let path = get_utf8_string_by_ref(path_ref)?;
    let path = Path::new(&path);

    let len = std::fs::metadata(path)
        .map(|metadata| metadata.len())
        .unwrap_or(0);

    Ok(len as i64)
}

#[cfg(unix)]
fn check_access_impl(path: &Path, mode: Access) -> bool {
    use nix::unistd::{access, AccessFlags};

    let mut flags = AccessFlags::empty();
    if mode.contains(Access::ACCESS_READ) {
        flags |= AccessFlags::R_OK;
    }
    if mode.contains(Access::ACCESS_WRITE) {
        flags |= AccessFlags::W_OK;
    }
    if mode.contains(Access::ACCESS_EXECUTE) {
        flags |= AccessFlags::X_OK;
    }

    access(path, flags).is_ok()
}
#[cfg(target_os = "windows")]
fn check_access_impl(path: &Path, mode: Access) -> bool {
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

fn extract_path(file_ref: i32) -> Result<i32> {
    let raw = HEAP.get_object_field_value(file_ref, "java/io/File", "path")?;
    Ok(raw[0])
}
