use crate::vm::error::{Error, Result};
use crate::vm::exception::pending_helpers_unix::{
    set_pending_unix_exception, set_pending_unix_exception_with_errno,
};
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::i64_to_vec;
use nix::errno::Errno;
use nix::fcntl::{open, OFlag};
use nix::libc::{c_char, fstatat, mode_t, rmdir, strerror_r};
use nix::sys::stat::{lstat, stat, FileStat, Mode};
use nix::unistd::{access, getcwd, mkdir, unlink, AccessFlags};
use realpath_ext::{realpath, RealpathFlags};
use std::ffi::{CStr, CString, OsString};
use std::mem::MaybeUninit;
use std::os::fd::IntoRawFd;
use std::os::unix::prelude::OsStringExt;
use std::path::Path;

/// `sun.nio.fs.UnixNativeDispatcher.getcwd()[B`
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

/// `sun.nio.fs.UnixNativeDispatcher.init()I`
pub(crate) fn init() -> Result<i32> {
    Ok(0) // todo: return real capability flags
}

/// `sun.nio.fs.UnixNativeDispatcher.open0(JII)I`
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

/// `sun.nio.fs.UnixNativeDispatcher.access0(JI)I`
pub(crate) fn access0(path_ptr: i64, mode: i32) -> Result<i32> {
    let path = cstring_from_i64(path_ptr)?;

    let flags =
        AccessFlags::from_bits(mode).ok_or_else(|| Error::new_native("Invalid access mode"))?;

    match access(path.as_c_str(), flags) {
        Ok(_) => Ok(0),
        Err(e) => Ok(e as i32),
    }
}

/// `sun.nio.fs.UnixNativeDispatcher.stat0(JLsun/nio/fs/UnixFileAttributes;)I`
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

/// `sun.nio.fs.UnixNativeDispatcher.lstat0(JLsun/nio/fs/UnixFileAttributes;)V`
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

/// `sun.nio.fs.UnixNativeDispatcher.fstatat0(IJILsun/nio/fs/UnixFileAttributes;)I` (JDK 26+)
pub(crate) fn fstatat0(dfd: i32, path_ptr: i64, flag: i32, attr_ref: i32) -> Result<i32> {
    let path = cstring_from_i64(path_ptr)?;

    let mut stat = MaybeUninit::<FileStat>::uninit();
    // Safety: `path` is a valid NUL-terminated string and `stat` is a writable `struct stat`.
    if unsafe { fstatat(dfd, path.as_ptr(), stat.as_mut_ptr(), flag) } != 0 {
        return Ok(Errno::last_raw());
    }
    // Safety: `fstatat` succeeded, so it has fully initialized `stat`.
    copy_stat_attributes(attr_ref, unsafe { stat.assume_init() })?;
    Ok(0)
}

/// `sun.nio.fs.UnixNativeDispatcher.fstatat0(IJILsun/nio/fs/UnixFileAttributes;)V` (JDK 25)
pub(crate) fn fstatat0_throwing(dfd: i32, path_ptr: i64, flag: i32, attr_ref: i32) -> Result<()> {
    match fstatat0(dfd, path_ptr, flag, attr_ref)? {
        0 => Ok(()),
        errno => set_pending_unix_exception_with_errno(errno),
    }
}

/// `sun.nio.fs.UnixNativeDispatcher.mkdir0(JI)V`
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

/// `sun.nio.fs.UnixNativeDispatcher.unlink0(J)V`
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

/// `sun.nio.fs.UnixNativeDispatcher.rmdir0(J)V`
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

/// `sun.nio.fs.UnixNativeDispatcher.realpath0(J)[B`
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

/// `sun.nio.fs.UnixNativeDispatcher.strerror(I)[B`
///
/// The message `UnixException` reports for `errnum`, e.g. `Not a directory`.
pub(crate) fn strerror(errnum: i32) -> Result<i32> {
    let mut buf = [0 as c_char; 1024];
    // Safety: `buf` is writable for its whole length, which is passed along.
    if unsafe { strerror_r(errnum, buf.as_mut_ptr(), buf.len()) } != 0 {
        return Err(Error::new_native(&format!(
            "strerror_r failed for errno {errnum}"
        )));
    }
    // Safety: on success `strerror_r` has written a NUL-terminated string into `buf`.
    let message = unsafe { CStr::from_ptr(buf.as_ptr()) };
    let bytes = message
        .to_bytes()
        .iter()
        .map(|&b| b as i8 as i32)
        .collect::<Vec<_>>();

    Ok(HEAP.create_array_with_values("[B", &bytes))
}
