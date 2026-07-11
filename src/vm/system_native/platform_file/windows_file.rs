use crate::vm::error::{Error, Result};
use crate::vm::exception::pending_helpers::set_pending_io_exception;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::{get_handle, i64_to_vec, vec_to_i64};
use crate::vm::system_native::platform_file::Mode;
use std::fs::File;
use std::mem::ManuallyDrop;
use std::os::windows::io::{FromRawHandle, IntoRawHandle, RawHandle};
use winapi::shared::minwindef::DWORD;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::processenv::GetStdHandle;
use winapi::um::winbase::{STD_ERROR_HANDLE, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE};
use winapi::um::winnt::HANDLE;

pub struct PlatformFile {}

impl PlatformFile {
    pub fn close(file_descriptor_ref: i32) -> Result<()> {
        let raw =
            HEAP.get_object_field_value(file_descriptor_ref, "java/io/FileDescriptor", "handle")?;
        let raw_fd = vec_to_i64(&raw);

        if raw_fd as HANDLE == INVALID_HANDLE_VALUE {
            return Ok(()); // already closed
        }

        Self::set_handle(file_descriptor_ref, -1)?;
        let file = unsafe { File::from_raw_handle(raw_fd as RawHandle) };
        drop(file);
        Ok(())
    }

    pub fn get_handle(fd: i32) -> Result<i64> {
        match fd {
            0 => Self::get_std_handle(STD_INPUT_HANDLE),
            1 => Self::get_std_handle(STD_OUTPUT_HANDLE),
            2 => Self::get_std_handle(STD_ERROR_HANDLE),
            _ => Err(Error::new_execution(&format!("fd {fd} is not supported"))),
        }
    }

    pub(crate) fn get_append(_fd: i32) -> Result<bool> {
        Ok(false)
    }

    fn get_std_handle(handle: DWORD) -> Result<i64> {
        let std_handle = unsafe { GetStdHandle(handle) };
        if std_handle.is_null() {
            return Err(Error::new_execution(&format!(
                "Failed to get std_handle by {handle:?}"
            )));
        }

        Ok(std_handle as isize as i64)
    }

    pub fn set_raw_id<T: IntoRawHandle>(
        output_stream_ref: i32,
        file: T,
        mode: Mode,
    ) -> Result<()> {
        let raw_handle = file.into_raw_handle(); // move ownership out of file
        let handle = raw_handle as isize as i64;

        let fd_ref = HEAP.get_object_field_value(output_stream_ref, mode.as_ref(), "fd")?[0];
        Self::set_handle(fd_ref, handle)
    }

    fn set_handle(fd_ref: i32, handle: i64) -> Result<()> {
        HEAP.set_object_field_value(
            fd_ref,
            "java/io/FileDescriptor",
            "handle",
            i64_to_vec(handle),
        )
    }

    pub fn get_by_fd_pending(fd_ref: i32) -> Result<Option<ManuallyDrop<File>>> {
        let handle = get_handle(fd_ref)?;

        if handle == -1 {
            set_pending_io_exception("Stream Closed")?;
            return Ok(None);
        }

        let file = ManuallyDrop::new(unsafe { File::from_raw_handle(handle as RawHandle) }); // ManuallyDrop prevents `file` from being dropped
        Ok(Some(file))
    }
}
