use crate::vm::error::{Error, ErrorKind, Result};
use crate::vm::heap::heap::with_heap_write_lock;
use crate::vm::helper::{i32toi64, i64_to_vec};
use crate::vm::stack::stack_frame::StackFrames;
use crate::vm::system_native::platform_native_dispatcher::unix_helpers::{
    throw_unix_exception, throw_unix_exception_with_errno,
};
use nix::fcntl::{open, OFlag};
use nix::libc::{c_char, mode_t, rmdir};
use nix::sys::stat::{lstat, stat, FileStat, Mode};
use nix::unistd::{access, getcwd, mkdir, unlink, AccessFlags};
use std::ffi::{CStr, CString};
use std::os::fd::IntoRawFd;

pub fn get_cwd_wrp(_args: &[i32], stack_frames: &mut StackFrames) -> Result<Vec<i32>> {
    match get_cwd(stack_frames) {
        Ok(byte_array_ref) => Ok(vec![byte_array_ref]),
        Err(e) if matches!(e.kind(), ErrorKind::ExceptionThrown) => Ok(vec![]),
        Err(e) => Err(e),
    }
}
fn get_cwd(stack_frames: &mut StackFrames) -> Result<i32> {
    let path = match getcwd() {
        Ok(path) => path,
        Err(errno) => {
            throw_unix_exception_with_errno(errno as i32, stack_frames)?;
            return Err(Error::new_exception());
        }
    };

    let cwd = path
        .to_string_lossy()
        .chars()
        .map(|c| c as i32)
        .collect::<Vec<_>>();

    // Allocate the byte array in the heap and return its reference
    let array_ref = with_heap_write_lock(|heap| heap.create_array_with_values("[b", &cwd));

    Ok(array_ref)
}

pub fn unix_native_dispatcher_open0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let path_address = i32toi64(args[1], args[0]);
    let flags = args[2];
    let mode = args[3];

    match open0(path_address, flags, mode, stack_frames) {
        Ok(fd) => Ok(vec![fd]),
        Err(e) if matches!(e.kind(), ErrorKind::ExceptionThrown) => Ok(vec![]),
        Err(e) => Err(e),
    }
}
fn open0(path_address: i64, flags: i32, mode: i32, stack_frames: &mut StackFrames) -> Result<i32> {
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
            throw_unix_exception_with_errno(errno as i32, stack_frames)?;
            return Err(Error::new_exception());
        }
    };

    Ok(fd.into_raw_fd() as i32)
}

pub fn mkdir0_wrp(args: &[i32], stack_frames: &mut StackFrames) -> Result<Vec<i32>> {
    let path_ptr = i32toi64(args[1], args[0]);
    let mode = args[2];

    mkdir0(path_ptr, mode, stack_frames)?;

    Ok(vec![])
}

fn mkdir0(path_ptr: i64, mode: i32, stack_frames: &mut StackFrames) -> Result<()> {
    let path = cstring_from_i64(path_ptr)?;

    let mode = Mode::from_bits_truncate(mode as mode_t);
    match mkdir(path.as_c_str(), mode) {
        Ok(()) => (),
        Err(errno) => {
            throw_unix_exception_with_errno(errno as i32, stack_frames)?;
        }
    };
    Ok(())
}

pub fn unlink0_wrp(args: &[i32], stack_frames: &mut StackFrames) -> Result<Vec<i32>> {
    let path_ptr = i32toi64(args[1], args[0]);

    unlink0(path_ptr, stack_frames)?;

    Ok(vec![])
}
fn unlink0(path_ptr: i64, stack_frames: &mut StackFrames) -> Result<()> {
    let path = cstring_from_i64(path_ptr)?;

    match unlink(path.as_c_str()) {
        Ok(()) => (),
        Err(errno) => {
            throw_unix_exception_with_errno(errno as i32, stack_frames)?;
        }
    }
    Ok(())
}

pub fn rmdir0_wrp(args: &[i32], stack_frames: &mut StackFrames) -> Result<Vec<i32>> {
    let path_ptr = i32toi64(args[1], args[0]);

    rmdir0(path_ptr, stack_frames)?;

    Ok(vec![])
}

fn rmdir0(path_ptr: i64, stack_frames: &mut StackFrames) -> Result<()> {
    let path = cstring_from_i64(path_ptr)?;
    let result = unsafe { rmdir(path.as_ptr()) };
    if result == -1 {
        throw_unix_exception(stack_frames)?
    }
    Ok(())
}

pub fn get_access0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let path_ptr = i32toi64(args[1], args[0]);
    let mode = args[2];

    let result = get_access0(path_ptr, mode)?;

    Ok(vec![result])
}
fn get_access0(path_ptr: i64, mode: i32) -> Result<i32> {
    let path = cstring_from_i64(path_ptr)?;

    let flags = AccessFlags::from_bits(mode)
        .ok_or_else(|| Error::new_native(&"Invalid access mode".to_string()))?;

    match access(path.as_c_str(), flags) {
        Ok(_) => Ok(0),
        Err(e) => Ok(e as i32),
    }
}

pub fn stat0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let path_ptr = i32toi64(args[1], args[0]);
    let attr_ref = args[2];

    let result = stat0(path_ptr, attr_ref)?;

    Ok(vec![result])
}
fn stat0(path_ptr: i64, attr_ref: i32) -> Result<i32> {
    let path = cstring_from_i64(path_ptr)?;

    match stat(path.as_c_str()) {
        Ok(stat) => {
            copy_stat_attributes(attr_ref, stat)?;
            Ok(0)
        }
        Err(e) => Ok(e as i32),
    }
}

pub fn lstat0_wrp(args: &[i32], stack_frames: &mut StackFrames) -> Result<Vec<i32>> {
    let path_ptr = i32toi64(args[1], args[0]);
    let attr_ref = args[2];

    lstat0(path_ptr, attr_ref, stack_frames)?;

    Ok(vec![])
}
fn lstat0(path_ptr: i64, attr_ref: i32, stack_frames: &mut StackFrames) -> Result<()> {
    let path = cstring_from_i64(path_ptr)?;

    match lstat(path.as_c_str()) {
        Ok(stat) => {
            copy_stat_attributes(attr_ref, stat)?;
        }
        Err(errno) => {
            throw_unix_exception_with_errno(errno as i32, stack_frames)?;
        }
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
    with_heap_write_lock(|heap| {
        heap.set_object_field_value(attr_ref, name, "st_mode", vec![stat.st_mode as i32])?;
        heap.set_object_field_value(attr_ref, name, "st_ino", i64_to_vec(stat.st_ino as i64))?;
        heap.set_object_field_value(attr_ref, name, "st_nlink", vec![stat.st_nlink as i32])?;
        heap.set_object_field_value(attr_ref, name, "st_uid", vec![stat.st_uid as i32])?;
        heap.set_object_field_value(attr_ref, name, "st_gid", vec![stat.st_gid as i32])?;
        heap.set_object_field_value(attr_ref, name, "st_size", i64_to_vec(stat.st_size as i64))?;
        heap.set_object_field_value(
            attr_ref,
            name,
            "st_atime_sec",
            i64_to_vec(stat.st_atime as i64),
        )?;
        heap.set_object_field_value(
            attr_ref,
            name,
            "st_mtime_sec",
            i64_to_vec(stat.st_mtime as i64),
        )?;
        heap.set_object_field_value(
            attr_ref,
            name,
            "st_ctime_sec",
            i64_to_vec(stat.st_ctime as i64),
        )?;
        heap.set_object_field_value(
            attr_ref,
            name,
            "st_atime_nsec",
            i64_to_vec(stat.st_atime_nsec),
        )?;
        heap.set_object_field_value(
            attr_ref,
            name,
            "st_mtime_nsec",
            i64_to_vec(stat.st_mtime_nsec),
        )?;
        heap.set_object_field_value(
            attr_ref,
            name,
            "st_ctime_nsec",
            i64_to_vec(stat.st_ctime_nsec),
        )?;

        Ok::<(), Error>(())
    })
}
