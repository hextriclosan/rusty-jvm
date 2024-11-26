use crate::heap::heap::with_heap_read_lock;
use std::fs::File;
use std::os::fd::FromRawFd;

pub(crate) fn file_descriptor_close0_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let fd_ref = args[0];

    close0(fd_ref)?;
    Ok(vec![])
}

fn close0(fd_ref: i32) -> crate::error::Result<()> {
    let raw_fd = with_heap_read_lock(|heap| {
        heap.get_object_field_value(fd_ref, "java/io/FileDescriptor", "fd")
    })?[0];

    let file = unsafe { File::from_raw_fd(raw_fd) };
    drop(file);
    Ok(())
}
