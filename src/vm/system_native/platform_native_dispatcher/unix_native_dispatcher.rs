use crate::vm::error::{Error, Result};
use crate::vm::exception::pending_helpers_unix::{
    set_pending_unix_exception, set_pending_unix_exception_with_errno,
};
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::i64_to_vec;
use nix::fcntl::{open, OFlag};
use nix::libc::{c_char, mode_t, rmdir};
use nix::sys::stat::{lstat, stat, FileStat, Mode};
use nix::unistd::{access, getcwd, mkdir, unlink, AccessFlags};
use realpath_ext::{realpath, RealpathFlags};
use std::ffi::{CStr, CString, OsString};
use std::os::fd::IntoRawFd;
use std::os::unix::prelude::OsStringExt;
use std::path::Path;

pub(crate) fn get_cwd() -> Result<i32> {
    let path = match getcwd() {
        Ok(path) => path,
        Err(errno) => {
            set_pending_unix_exception_with_errno(errno as i32)?;
            return Ok(0);
        }
    };

    let cwd = path
        .to_string_lossy()
        .chars()
        .map(|c| c as i32)
        .collect::<Vec<_>>();

    // Allocate the byte array in the heap and return its reference
    let array_ref = HEAP.create_array_with_values("[B", &cwd);

    Ok(array_ref)
}

pub(crate) fn init() -> Result<i32> {
    Ok(0) // todo: return real capability flags
}

pub(crate) fn open0(path_address: i64, flags: i32, mode: i32) -> Result<i32> {
    let path = unsafe { CStr::from_ptr(path_address as *const c_char) };
    let path = match path.to_str() {
        Ok(path) => path,
        Err(e) => {
            return Err(Error::new_native(&format!(
                "Failed to convert path to string: {}",
                e
            )))
        }
    };

    let fd = match open(
        path,
        OFlag::from_bits_truncate(flags),
        Mode::from_bits_truncate(mode as mode_t),
    ) {
        Ok(fd) => fd,
        Err(errno) => {
            set_pending_unix_exception_with_errno(errno as i32)?;
            return Ok(0);
        }
    };

    Ok(fd.into_raw_fd() as i32)
}

pub(crate) fn access0(path_ptr: i64, mode: i32) -> Result<i32> {
    let path = cstring_from_i64(path_ptr)?;

    let flags =
        AccessFlags::from_bits(mode).ok_or_else(|| Error::new_native("Invalid access mode"))?;

    match access(path.as_c_str(), flags) {
        Ok(_) => Ok(0),
        Err(e) => Ok(e as i32),
    }
}

pub(crate) fn stat0(path_ptr: i64, attr_ref: i32) -> Result<i32> {
    let path = cstring_from_i64(path_ptr)?;

    match stat(path.as_c_str()) {
        Ok(stat) => {
            copy_stat_attributes(attr_ref, stat)?;
            Ok(0)
        }
        Err(e) => Ok(e as i32),
    }
}

pub(crate) fn lstat0(path_ptr: i64, attr_ref: i32) -> Result<()> {
    let path = cstring_from_i64(path_ptr)?;

    match lstat(path.as_c_str()) {
        Ok(stat) => {
            copy_stat_attributes(attr_ref, stat)?;
        }
        Err(errno) => {
            set_pending_unix_exception_with_errno(errno as i32)?;
            return Ok(());
        }
    }
    Ok(())
}

pub(crate) fn mkdir0(path_ptr: i64, mode: i32) -> Result<()> {
    let path = cstring_from_i64(path_ptr)?;

    let mode = Mode::from_bits_truncate(mode as mode_t);
    match mkdir(path.as_c_str(), mode) {
        Ok(()) => (),
        Err(errno) => {
            set_pending_unix_exception_with_errno(errno as i32)?;
            return Ok(());
        }
    };
    Ok(())
}

pub(crate) fn unlink0(path_ptr: i64) -> Result<()> {
    let path = cstring_from_i64(path_ptr)?;

    match unlink(path.as_c_str()) {
        Ok(()) => (),
        Err(errno) => {
            set_pending_unix_exception_with_errno(errno as i32)?;
            return Ok(());
        }
    }
    Ok(())
}

pub(crate) fn rmdir0(path_ptr: i64) -> Result<()> {
    let path = cstring_from_i64(path_ptr)?;
    let result = unsafe { rmdir(path.as_ptr()) };
    if result == -1 {
        set_pending_unix_exception()?;
    }

    Ok(())
}

fn cstring_from_i64(ptr_value: i64) -> Result<CString> {
    let c_str_ptr = ptr_value as *const c_char;
    if c_str_ptr.is_null() {
        return Err(Error::new_native("Null pointer"));
    }
    unsafe { Ok(CStr::from_ptr(c_str_ptr).to_owned()) }
}

fn copy_stat_attributes(attr_ref: i32, stat: FileStat) -> Result<()> {
    let name = "sun/nio/fs/UnixFileAttributes";
    HEAP.set_object_field_value(attr_ref, name, "st_mode", vec![stat.st_mode as i32])?;
    HEAP.set_object_field_value(attr_ref, name, "st_ino", i64_to_vec(stat.st_ino as i64))?;
    HEAP.set_object_field_value(attr_ref, name, "st_nlink", vec![stat.st_nlink as i32])?;
    HEAP.set_object_field_value(attr_ref, name, "st_uid", vec![stat.st_uid as i32])?;
    HEAP.set_object_field_value(attr_ref, name, "st_gid", vec![stat.st_gid as i32])?;
    HEAP.set_object_field_value(attr_ref, name, "st_size", i64_to_vec(stat.st_size))?;
    HEAP.set_object_field_value(attr_ref, name, "st_atime_sec", i64_to_vec(stat.st_atime))?;
    HEAP.set_object_field_value(attr_ref, name, "st_mtime_sec", i64_to_vec(stat.st_mtime))?;
    HEAP.set_object_field_value(attr_ref, name, "st_ctime_sec", i64_to_vec(stat.st_ctime))?;
    HEAP.set_object_field_value(
        attr_ref,
        name,
        "st_atime_nsec",
        i64_to_vec(stat.st_atime_nsec),
    )?;
    HEAP.set_object_field_value(
        attr_ref,
        name,
        "st_mtime_nsec",
        i64_to_vec(stat.st_mtime_nsec),
    )?;
    HEAP.set_object_field_value(
        attr_ref,
        name,
        "st_ctime_nsec",
        i64_to_vec(stat.st_ctime_nsec),
    )
}

pub(crate) fn realpath0(path_ptr: i64) -> Result<i32> {
    let path = cstring_from_i64(path_ptr)?;
    let path = OsString::from_vec(path.into_bytes());
    let result_path = realpath(Path::new(&path), RealpathFlags::empty());
    let result_path = match result_path {
        Ok(p) => p,
        Err(errno) => {
            let errno = errno
                .raw_os_error()
                .ok_or_else(|| Error::new_native("Failed to get errno from realpath error"))?;
            set_pending_unix_exception_with_errno(errno)?;
            return Ok(0);
        }
    };

    let path_bytes = result_path
        .to_string_lossy()
        .chars()
        .map(|c| c as i32)
        .collect::<Vec<_>>();

    // Allocate the byte array in the heap and return its reference
    let array_ref = HEAP.create_array_with_values("[B", &path_bytes);

    Ok(array_ref)
}
