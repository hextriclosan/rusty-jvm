use crate::vm::error::{Error, Result};
use crate::vm::heap::heap::{with_heap_read_lock, with_heap_write_lock};
use crate::vm::helper::get_fd;
use crate::vm::stack::stack_frame::StackFrames;
use std::fs::File;
use std::mem::ManuallyDrop;
use std::os::fd::{FromRawFd, IntoRawFd};

pub struct PlatformFile {}

impl PlatformFile {
    pub fn close(file_descriptor_ref: i32, stack_frames: &mut StackFrames) -> Result<()> {
        let raw_fd = get_fd(file_descriptor_ref, stack_frames)?;

        let file = unsafe { File::from_raw_fd(raw_fd) };
        drop(file);

        Ok(())
    }

    pub fn get_handle(_fd: i32) -> Result<i64> {
        Ok(0)
    }

    pub fn set_raw_id(
        output_stream_ref: i32,
        file: File,
        stack_frames: &mut StackFrames,
    ) -> Result<()> {
        let posix_fd = file.into_raw_fd(); // move ownership out of file
        let fd = posix_fd as i32;

        with_heap_write_lock(|heap| {
            let fd_ref = heap.get_object_field_value(
                output_stream_ref,
                "java/io/FileOutputStream",
                "fd",
                stack_frames,
            )?[0];
            heap.set_object_field_value(fd_ref, "java/io/FileDescriptor", "fd", vec![fd])?;
            Ok::<(), Error>(())
        })
    }

    pub fn get_by_raw_id(
        obj_ref: i32,
        stack_frames: &mut StackFrames,
    ) -> Result<ManuallyDrop<File>> {
        let fd = with_heap_read_lock(|heap| {
            let fd_ref = heap.get_object_field_value(
                obj_ref,
                "java/io/FileOutputStream",
                "fd",
                stack_frames,
            )?[0];
            let fd = get_fd(fd_ref, stack_frames)?;

            Ok::<i32, Error>(fd)
        })?;

        let file = ManuallyDrop::new(unsafe { File::from_raw_fd(fd) }); // ManuallyDrop prevents `file` from being dropped
        Ok(file)
    }
}
