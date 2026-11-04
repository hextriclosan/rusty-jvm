use crate::vm::error::Result;
use crate::vm::system_native::file_system::{file_path, Access};
use crate::vm::system_native::string::get_utf8_string_by_ref;
use nix::libc::{pathconf, _PC_NAME_MAX};
use nix::sys::stat::{stat, utimes};
use nix::sys::time::TimeVal;
use std::ffi::CString;
use std::fs::Metadata;
use std::os::unix::fs::PermissionsExt;
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

/// Whether `delete0` removes the entry as a directory. A symbolic link to a directory is removed as
/// a link, since `metadata` is that of the link.
pub(super) fn is_directory_unix(metadata: &Metadata) -> bool {
    metadata.is_dir()
}

/// `java.io.UnixFileSystem.setPermission0(Ljava/io/File;IZZ)Z`
pub(crate) fn set_permission0(
    _this: i32,
    file_ref: i32,
    access: i32,
    enable: bool,
    owner_only: bool,
) -> Result<bool> {
    let path = file_path(file_ref)?;
    // HotSpot asserts that access is one of these
    let mode_bits = match Access::from_bits(access) {
        Some(Access::ACCESS_READ) => 0o444,
        Some(Access::ACCESS_WRITE) => 0o222,
        Some(Access::ACCESS_EXECUTE) => 0o111,
        _ => 0,
    };
    let mask = mode_bits & if owner_only { 0o700 } else { 0o777 };

    let Ok(metadata) = std::fs::metadata(&path) else {
        return Ok(false);
    };
    let mut permissions = metadata.permissions();
    let mode = permissions.mode();
    permissions.set_mode(if enable { mode | mask } else { mode & !mask });
    Ok(std::fs::set_permissions(&path, permissions).is_ok())
}

/// `java.io.UnixFileSystem.setReadOnly0(Ljava/io/File;)Z`
///
/// Clears all write permissions, as HotSpot does.
pub(crate) fn set_read_only0(_this: i32, file_ref: i32) -> Result<bool> {
    let path = file_path(file_ref)?;
    let Ok(metadata) = std::fs::metadata(&path) else {
        return Ok(false);
    };
    let mut permissions = metadata.permissions();
    permissions.set_readonly(true);
    Ok(std::fs::set_permissions(&path, permissions).is_ok())
}

/// `java.io.UnixFileSystem.rename0(Ljava/io/File;Ljava/io/File;)Z`
///
/// `rename`, which replaces an existing destination.
pub(crate) fn rename0(_this: i32, from_ref: i32, to_ref: i32) -> Result<bool> {
    Ok(std::fs::rename(file_path(from_ref)?, file_path(to_ref)?).is_ok())
}

/// `java.io.UnixFileSystem.setLastModifiedTime0(Ljava/io/File;J)Z`
///
/// Like HotSpot, `utimes` with the access time kept, cut to microseconds. The owner of a file may
/// change its times without write access to it. `File.setLastModified` rejects negative times.
pub(crate) fn set_last_modified_time0(_this: i32, file_ref: i32, time: i64) -> Result<bool> {
    let path = file_path(file_ref)?;
    let Ok(stat) = stat(path.as_str()) else {
        return Ok(false);
    };
    let accessed = TimeVal::new(stat.st_atime as _, (stat.st_atime_nsec / 1000) as _);
    let modified = TimeVal::new((time / 1000) as _, ((time % 1000) * 1000) as _);
    Ok(utimes(path.as_str(), &accessed, &modified).is_ok())
}

/// `java.io.UnixFileSystem.getSpace0(Ljava/io/File;I)J`
///
/// The total, free or usable space (`FileSystem.SPACE_TOTAL`, `SPACE_FREE` and `SPACE_USABLE`), or
/// 0 on failure. Like HotSpot, uses `statfs` on macOS, where `statvfs` has 32-bit block counts.
pub(crate) fn get_space0(_this: i32, file_ref: i32, space_type: i32) -> Result<i64> {
    let path = file_path(file_ref)?;

    #[cfg(target_os = "macos")]
    let (block_size, blocks) = {
        let Ok(stat) = nix::sys::statfs::statfs(path.as_str()) else {
            return Ok(0);
        };
        let blocks = [stat.blocks(), stat.blocks_free(), stat.blocks_available()];
        (stat.block_size() as i64, blocks)
    };
    #[cfg(not(target_os = "macos"))]
    let (block_size, blocks) = {
        let Ok(stat) = nix::sys::statvfs::statvfs(path.as_str()) else {
            return Ok(0);
        };
        let blocks = [stat.blocks(), stat.blocks_free(), stat.blocks_available()];
        (stat.fragment_size() as i64, blocks)
    };

    let count = usize::try_from(space_type)
        .ok()
        .and_then(|index| blocks.get(index));
    Ok(count.map_or(0, |&count| block_size.wrapping_mul(count as i64)))
}
