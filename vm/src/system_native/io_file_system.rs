use crate::error::Error;
use crate::execution_engine::string_pool_helper::StringPoolHelper;
use crate::heap::heap::with_heap_read_lock;
use crate::system_native::string::get_utf8_string_by_ref;
use bitflags::bitflags;
use path_absolutize::Absolutize;
use std::path::Path;

pub(crate) fn canonicalize0_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let _filesystem_impl_ref = args[0];
    let path_ref = args[1];
    let canonical_path_ref = canonicalize0(path_ref)?;

    Ok(vec![canonical_path_ref])
}
fn canonicalize0(path_ref: i32) -> crate::error::Result<i32> {
    let path = get_utf8_string_by_ref(path_ref)?;
    let path = Path::new(&path);

    let canonical_path = path.absolutize()?;
    let canonical_name = canonical_path.to_str().ok_or_else(|| {
        Error::new_execution(&format!("Failed to convert path {canonical_path:?}"))
    })?;
    let canonical_name_ref = StringPoolHelper::get_string(canonical_name.to_string())?;

    Ok(canonical_name_ref)
}

pub(crate) fn create_file_exclusively0_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let _filesystem_impl_ref = args[0];
    let path_ref = args[1];
    let created = create_file_exclusively0(path_ref)?;

    Ok(vec![if created { 1 } else { 0 }])
}
fn create_file_exclusively0(path_ref: i32) -> crate::error::Result<bool> {
    let path = get_utf8_string_by_ref(path_ref)?;
    let path = Path::new(&path);

    if path.exists() {
        return Ok(false);
    }

    std::fs::File::create_new(path)
        .map(|_| true)
        .map_err(|e| Error::new_execution(&format!("Failed to create file {path:?}: {e}")))
}

pub(crate) fn delete0_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let _filesystem_impl_ref = args[0];
    let file_ref = args[1];
    let deleted = delete0(file_ref)?;

    Ok(vec![if deleted { 1 } else { 0 }])
}
fn delete0(file_ref: i32) -> crate::error::Result<bool> {
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

pub(crate) fn get_boolean_attributes0_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
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
fn get_boolean_attributes0(file_ref: i32) -> crate::error::Result<i32> {
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

fn extract_path(file_ref: i32) -> crate::error::Result<i32> {
    Ok(
        with_heap_read_lock(|heap| heap.get_object_field_value(file_ref, "java/io/File", "path"))?
            [0],
    )
}
