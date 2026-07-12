use crate::vm::error::{Error, Result};
use crate::vm::exception::pending_helpers::set_pending_io_exception;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::get_handle;
use crate::vm::system_native::platform_file::Mode;
use nix::fcntl;
use nix::fcntl::{FcntlArg, OFlag};
use std::fs::File;
use std::mem::ManuallyDrop;
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};

pub struct PlatformFile {}

impl PlatformFile {
    pub fn close(file_descriptor_ref: i32) -> Result<()> {
        let raw_fd = get_handle(file_descriptor_ref)?;

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

    pub(crate) fn get_append(raw_fd: i32) -> Result<bool> {
        let file = ManuallyDrop::new(unsafe { File::from_raw_fd(raw_fd) }); // ManuallyDrop prevents `file` from being dropped
        let flags = fcntl::fcntl(file.as_fd(), FcntlArg::F_GETFL)
            .map_err(|e| Error::new_native(&format!("Failed to get flags: {e}")))?;

        Ok(OFlag::from_bits_truncate(flags).contains(OFlag::O_APPEND))
    }

    pub fn set_raw_id<T: IntoRawFd>(output_stream_ref: i32, file: T, mode: Mode) -> Result<()> {
        let posix_fd = file.into_raw_fd(); // move ownership out of file
        let fd = posix_fd;

        let fd_ref = HEAP.get_object_field_value(output_stream_ref, mode.as_ref(), "fd")?[0];
        Self::set_fd(fd_ref, fd)
    }

    fn set_fd(fd_ref: i32, fd: i32) -> Result<()> {
        HEAP.set_object_field_value(fd_ref, "java/io/FileDescriptor", "fd", vec![fd])
    }

    pub fn get_by_fd(fd_ref: i32) -> Result<Option<ManuallyDrop<File>>> {
        let fd = get_handle(fd_ref)?;

        if fd == -1 {
            set_pending_io_exception("Stream Closed")?;
            return Ok(None);
        }

        let file = ManuallyDrop::new(unsafe { File::from_raw_fd(fd) }); // ManuallyDrop prevents `file` from being dropped
        Ok(Some(file))
    }
}
