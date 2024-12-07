use crate::error::Error;
use crate::heap::heap::{with_heap_read_lock, with_heap_write_lock};
use std::fs::File;
use std::mem::ManuallyDrop;
use std::os::fd::{FromRawFd, IntoRawFd};

pub struct PlatformFile {}

impl PlatformFile {
    pub fn close(file_descriptor_ref: i32) -> crate::error::Result<()> {
        let raw_fd = with_heap_read_lock(|heap| {
            heap.get_object_field_value(file_descriptor_ref, "java/io/FileDescriptor", "fd")
        })?[0];

        let file = unsafe { File::from_raw_fd(raw_fd) };
        drop(file);

        Ok(())
    }

    pub fn get_handle(_fd: i32) -> crate::error::Result<i64> {
        Ok(0)
    }

    pub fn set_raw_id(output_stream_ref: i32, file: File) -> std::io::Result<()> {
        let posix_fd = file.into_raw_fd(); // move ownership out of file
        let fd = posix_fd as i32;

        with_heap_write_lock(|heap| {
            let fd_ref = heap
                .get_object_field_value(output_stream_ref, "java/io/FileOutputStream", "fd")
                .expect("fd field not found")[0];
            heap.set_object_field_value(fd_ref, "java/io/FileDescriptor", "fd", vec![fd])
                .expect("fd field not found");
        });

        Ok(())
    }

    pub fn get_by_raw_id(obj_ref: i32) -> crate::error::Result<ManuallyDrop<File>> {
        let fd = with_heap_read_lock(|heap| {
            let fd_ref =
                heap.get_object_field_value(obj_ref, "java/io/FileOutputStream", "fd")?[0];
            let fd = heap.get_object_field_value(fd_ref, "java/io/FileDescriptor", "fd")?[0];

            Ok::<i32, Error>(fd)
        })?;

        let file = ManuallyDrop::new(unsafe { File::from_raw_fd(fd) }); // ManuallyDrop prevents `file` from being dropped
        Ok(file)
    }
}
