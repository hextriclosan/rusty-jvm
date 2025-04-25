use std::ffi::{CStr, CString};
use nix::fcntl::{open, OFlag};
use nix::libc::{c_char, mode_t};
use nix::sys::stat::{stat, Mode};
use crate::error::Error;
use crate::heap::heap::with_heap_write_lock;
use nix::unistd::{access, getcwd, AccessFlags};
use crate::helper::{i32toi64, i64_to_vec};

pub fn get_cwd_wrp(_args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let byte_array_ref = get_cwd()?;

    Ok(vec![byte_array_ref])
}
fn get_cwd() -> crate::error::Result<i32> {
    let path = match getcwd() {
        Ok(path) => path,
        Err(e) => {
            return Err(Error::new_native(&format!(
                "Failed to get current directory: {}",
                e
            )))
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
pub fn unix_native_dispatcher_open0_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let path_address = i32toi64(args[1], args[0]);
    let flags = args[2];
    let mode = args[3];
    let fd = open0(path_address, flags, mode)?;

    Ok(vec![fd])
}
fn open0(path_address: i64, flags: i32, mode: i32) -> crate::error::Result<i32> {
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

    eprintln!("!!! open0 path: {path}, flags: {flags}, mode: {mode}");
    let fd = open(path, OFlag::from_bits_truncate(flags), Mode::from_bits_truncate(mode as mode_t))
        .map_err(|e| Error::new_native(&format!("Failed to open file: {} ({})", path, e)))?;

    Ok(fd as i32)
}

pub fn get_access0_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let path_ptr = i32toi64(args[1], args[0]);
    let mode = args[2];

    let result = get_access0(path_ptr, mode)?;

    Ok(vec![result])
}
fn get_access0(path_ptr: i64, mode: i32) -> crate::error::Result<i32> {
    let path = cstring_from_i64(path_ptr).ok_or_else(|| {
        Error::new_native(&"Failed to convert path pointer to CString".to_string())
    })?;

    let flags = AccessFlags::from_bits(mode)
        .ok_or_else(|| Error::new_native(&"Invalid access mode".to_string()))?;

    match access(path.as_c_str(), flags) {
        Ok(_) => Ok(0),
        Err(e) => Ok(e as i32),
    }
}

pub fn stat0_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let path_ptr = i32toi64(args[1], args[0]);
    let attr_ref = args[2];

    let result = stat0(path_ptr, attr_ref)?;

    Ok(vec![result])
}
fn stat0(path_ptr: i64, attr_ref: i32) -> crate::error::Result<i32> {
    let path = cstring_from_i64(path_ptr).ok_or_else(|| {
        Error::new_native(&"Failed to convert path pointer to CString".to_string())
    })?;

    match stat(path.as_c_str()) {
        Ok(stat) => {
            //stat.st_atime_nsec

            // (*env)->SetLongField(env, attrs, attrs_st_birthtime_sec, (jlong)buf->stx_btime.tv_sec);
            // (*env)->SetLongField(env, attrs, attrs_st_birthtime_nsec, (jlong)buf->stx_btime.tv_nsec);
            // (*env)->SetLongField(env, attrs, attrs_st_atime_nsec, (jlong)buf->stx_atime.tv_nsec);
            // (*env)->SetLongField(env, attrs, attrs_st_mtime_nsec, (jlong)buf->stx_mtime.tv_nsec);
            // (*env)->SetLongField(env, attrs, attrs_st_ctime_nsec, (jlong)buf->stx_ctime.tv_nsec);

            let name = "sun/nio/fs/UnixFileAttributes";
            with_heap_write_lock(|heap| {
                heap.set_object_field_value(attr_ref, name, "st_mode", vec![stat.st_mode as i32])?;
                heap.set_object_field_value(
                    attr_ref,
                    name,
                    "st_ino",
                    i64_to_vec(stat.st_ino as i64),
                )?;
                heap.set_object_field_value(
                    attr_ref,
                    name,
                    "st_nlink",
                    vec![stat.st_nlink as i32],
                )?;
                heap.set_object_field_value(attr_ref, name, "st_uid", vec![stat.st_uid as i32])?;
                heap.set_object_field_value(attr_ref, name, "st_gid", vec![stat.st_gid as i32])?;
                heap.set_object_field_value(
                    attr_ref,
                    name,
                    "st_size",
                    i64_to_vec(stat.st_size as i64),
                )?;
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
            })?;

            Ok(0)
        }
        Err(e) => Ok(e as i32),
    }
}

fn cstring_from_i64(ptr_value: i64) -> Option<CString> {
    if ptr_value == 0 {
        return None;
    }

    unsafe {
        let c_str_ptr = ptr_value as *const c_char;
        if c_str_ptr.is_null() {
            return None;
        }

        Some(CStr::from_ptr(c_str_ptr).to_owned())
    }
}
