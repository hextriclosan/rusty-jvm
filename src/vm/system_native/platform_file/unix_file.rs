use crate::vm::error::Result;
use crate::vm::exception::helpers::throw_ioexception;
use crate::vm::exception::throwing_result::ThrowingResult;
use crate::vm::heap::heap::{with_heap_read_lock, with_heap_write_lock};
use crate::vm::helper::get_fd;
use crate::vm::stack::stack_frame::StackFrames;
use crate::vm::system_native::platform_file::Mode;
use crate::{throw_and_return, unwrap_or_return_err};
use std::fs::File;
use std::mem::ManuallyDrop;
use std::os::fd::{FromRawFd, IntoRawFd};

pub struct PlatformFile {}

impl PlatformFile {
    pub fn close(file_descriptor_ref: i32) -> Result<()> {
        let raw_fd = get_fd(file_descriptor_ref)?;

        if raw_fd == -1 {
            return Ok(()); // already closed
        }

        Self::set_fd(file_descriptor_ref, -1)?;
        let file = unsafe { File::from_raw_fd(raw_fd) };
        drop(file);

        Ok(())
    }

    pub fn get_handle(_fd: i32) -> Result<i64> {
        Ok(0)
    }

    pub fn set_raw_id(output_stream_ref: i32, file: File, mode: Mode) -> Result<()> {
        let posix_fd = file.into_raw_fd(); // move ownership out of file
        let fd = posix_fd as i32;

        let fd_ref = with_heap_write_lock(|heap| {
            heap.get_object_field_value(output_stream_ref, mode.as_ref(), "fd")
        })?[0];
        Self::set_fd(fd_ref, fd)
    }

    fn set_fd(fd_ref: i32, fd: i32) -> crate::vm::error::Result<()> {
        with_heap_read_lock(|heap| {
            heap.set_object_field_value(fd_ref, "java/io/FileDescriptor", "fd", vec![fd])
        })
    }

    pub fn get_by_raw_id(
        obj_ref: i32,
        mode: Mode,
        stack_frames: &mut StackFrames,
    ) -> ThrowingResult<ManuallyDrop<File>> {
        let fd_ref =
            with_heap_read_lock(|heap| heap.get_object_field_value(obj_ref, mode.as_ref(), "fd"));
        let fd_ref = unwrap_or_return_err!(fd_ref)[0];
        Self::get_by_fd(fd_ref, stack_frames)
    }

    pub fn get_by_fd(
        fd_ref: i32,
        stack_frames: &mut StackFrames,
    ) -> ThrowingResult<ManuallyDrop<File>> {
        let fd = unwrap_or_return_err!(get_fd(fd_ref));

        if fd == -1 {
            throw_and_return!(throw_ioexception("Stream Closed", stack_frames))
        }

        let file = ManuallyDrop::new(unsafe { File::from_raw_fd(fd) }); // ManuallyDrop prevents `file` from being dropped
        ThrowingResult::ok(file)
    }
}
