use std::os::fd::{BorrowedFd};
use nix::unistd::write;
use crate::error::Error;
use crate::heap::heap::with_heap_read_lock;
use crate::helper::{i32toi64};

pub fn unix_file_dispatcher_impl_write0_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let fd_ref = args[0];
    let address = i32toi64(args[2], args[1]);
    let len = args[3];

    let result = write0(fd_ref, address, len)?;

    Ok(vec![result])
}

fn write0(fd_ref: i32, address: i64, len: i32) -> crate::error::Result<i32> {
    let fd = with_heap_read_lock(|heap| {
        let raw = heap.get_object_field_value(fd_ref, "java/io/FileDescriptor", "fd")?;

        Ok::<i32, Error>(raw[0])
    })?;

    let address = address as *const u8;
    let buf = unsafe { std::slice::from_raw_parts(address, len as usize) };

    let fd = unsafe { BorrowedFd::borrow_raw(fd) };
    let written = write(fd, buf)
        .map_err(|e| Error::new_native(&format!("Failed to write to file: {}", e)))?;

    Ok(written as i32)
}
