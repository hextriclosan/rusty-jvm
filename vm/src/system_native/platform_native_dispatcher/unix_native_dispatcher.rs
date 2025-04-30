use crate::error::Error;
use crate::heap::heap::with_heap_write_lock;
use crate::helper::i32toi64;
use nix::fcntl::{open, OFlag};
use nix::libc::{c_char, mode_t};
use nix::sys::stat::Mode;
use nix::unistd::getcwd;
use std::ffi::CStr;
use std::os::fd::IntoRawFd;

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

    let fd = open(
        path,
        OFlag::from_bits_truncate(flags),
        Mode::from_bits_truncate(mode as mode_t),
    )
    .map_err(|e| Error::new_native(&format!("Failed to open file: {} ({})", path, e)))?;

    Ok(fd.into_raw_fd() as i32)
}
