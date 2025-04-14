use crate::error::Error;
use crate::execution_engine::string_pool_helper::StringPoolHelper;
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
