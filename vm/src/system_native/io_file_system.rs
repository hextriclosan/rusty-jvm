use crate::error::Error;
use crate::execution_engine::string_pool_helper::StringPoolHelper;
use crate::heap::heap::with_heap_read_lock;
use crate::system_native::string::get_utf8_string_by_ref;
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
    let path_ref =
        with_heap_read_lock(|heap| heap.get_object_field_value(file_ref, "java/io/File", "path"))?
            [0];

    let path = get_utf8_string_by_ref(path_ref)?;
    let path = Path::new(&path);

    if !path.exists() {
        return Ok(false);
    }

    std::fs::remove_file(path)
        .map(|_| true)
        .map_err(|e| Error::new_execution(&format!("Failed to delete file {path:?}: {e}")))
}
