use crate::error::Error;
use crate::helper::{get_fd, i32toi64};
use nix::unistd::write;
use std::os::fd::BorrowedFd;
use std::slice::from_raw_parts;

pub fn unix_file_dispatcher_impl_write0_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let fd_ref = args[0];
    let address = i32toi64(args[2], args[1]);
    let len = args[3];

    let result = write0(fd_ref, address, len)?;

    Ok(vec![result])
}
fn write0(fd_ref: i32, address: i64, len: i32) -> crate::error::Result<i32> {
    let address = address as usize as *const u8;
    let buf = unsafe { from_raw_parts(address, len as usize) };

    let fd = get_fd(fd_ref)?;
    let fd = unsafe { BorrowedFd::borrow_raw(fd) };
    let written = write(fd, buf)
        .map_err(|e| Error::new_native(&format!("Failed to write to file: {}", e)))?;

    Ok(written as i32)
}
