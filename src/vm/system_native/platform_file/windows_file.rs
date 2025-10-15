use crate::vm::error::{Error, Result};
use crate::vm::exception::helpers::throw_ioexception;
use crate::vm::exception::throwing_result::ThrowingResult;
use crate::vm::heap::heap::{with_heap_read_lock, with_heap_write_lock};
use crate::vm::helper;
use crate::vm::helper::{i64_to_vec, vec_to_i64};
use crate::vm::stack::stack_frame::StackFrames;
use crate::vm::system_native::platform_file::Mode;
use crate::{throw_and_return, unwrap_or_return_err};
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
        let raw_fd = with_heap_read_lock(|heap| {
            let raw = heap.get_object_field_value(
                file_descriptor_ref,
                "java/io/FileDescriptor",
                "handle",
            )?;
            Ok::<i64, Error>(vec_to_i64(&raw))
        })?;

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

        let fd_ref = with_heap_write_lock(|heap| {
            heap.get_object_field_value(output_stream_ref, mode.as_ref(), "fd")
        })?[0];
        Self::set_handle(fd_ref, handle)
    }

    fn set_handle(fd_ref: i32, handle: i64) -> Result<()> {
        with_heap_read_lock(|heap| {
            heap.set_object_field_value(
                fd_ref,
                "java/io/FileDescriptor",
                "handle",
                i64_to_vec(handle),
            )
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
        let handle = unwrap_or_return_err!(helper::get_handle(fd_ref));

        if handle == -1 {
            throw_and_return!(throw_ioexception("Stream Closed", stack_frames))
        }

        let file = ManuallyDrop::new(unsafe { File::from_raw_handle(handle as RawHandle) }); // ManuallyDrop prevents `file` from being dropped
        ThrowingResult::ok(file)
    }
}
