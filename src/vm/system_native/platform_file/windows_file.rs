use crate::vm::error::{Error, Result};
use crate::vm::heap::heap::{with_heap_read_lock, with_heap_write_lock};
use crate::vm::helper;
use crate::vm::helper::{i64_to_vec, vec_to_i64};
use std::fs::File;
use std::mem::ManuallyDrop;
use std::os::windows::io::{FromRawHandle, IntoRawHandle, RawHandle};
use winapi::shared::minwindef::DWORD;
use winapi::um::processenv::GetStdHandle;
use winapi::um::winbase::{STD_ERROR_HANDLE, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE};

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

    pub fn set_raw_id(output_stream_ref: i32, file: File) -> Result<()> {
        let raw_handle = file.into_raw_handle(); // move ownership out of file
        let handle = raw_handle as isize as i64;
        let raw = i64_to_vec(handle);

        with_heap_write_lock(|heap| {
            let fd_ref =
                heap.get_object_field_value(output_stream_ref, "java/io/FileOutputStream", "fd")?
                    [0];
            heap.set_object_field_value(fd_ref, "java/io/FileDescriptor", "handle", raw)?;
            Ok::<(), Error>(())
        })
    }

    pub fn get_by_raw_id(obj_ref: i32) -> Result<ManuallyDrop<File>> {
        let handle = with_heap_read_lock(|heap| {
            let fd_ref =
                heap.get_object_field_value(obj_ref, "java/io/FileOutputStream", "fd")?[0];

            helper::get_handle(fd_ref)
        })?;

        let file = ManuallyDrop::new(unsafe { File::from_raw_handle(handle as RawHandle) }); // ManuallyDrop prevents `file` from being dropped
        Ok(file)
    }
}
