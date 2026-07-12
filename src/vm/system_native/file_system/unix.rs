use crate::vm::error::Result;
use crate::vm::system_native::file_system::Access;
use crate::vm::system_native::string::get_utf8_string_by_ref;
use nix::libc::{pathconf, _PC_NAME_MAX};
use std::ffi::CString;
use std::path::Path;

pub(super) fn check_access_unix_impl(path: &Path, mode: Access) -> bool {
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

/// `java.io.UnixFileSystem.getNameMax0(Ljava/lang/String;)J`
pub(crate) fn get_name_max0(_this: i32, string_ref: i32) -> Result<i64> {
    const NAME_MAX_FALLBACK: i64 = 255;

    let string_content = get_utf8_string_by_ref(string_ref)?;
    let c_string = CString::new(&*string_content)?;
    let name_max = unsafe { pathconf(c_string.as_ptr(), _PC_NAME_MAX) };

    Ok(if name_max != -1 {
        name_max
    } else {
        NAME_MAX_FALLBACK
    })
}

pub(super) fn is_hidden_unix(path: &Path) -> bool {
    if let Some(name) = path.file_name() {
        if let Some(name) = name.to_str() {
            return name.starts_with('.');
        }
    }

    false
}
