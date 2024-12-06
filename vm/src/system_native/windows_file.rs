use crate::error::Error;
use crate::heap::heap::{with_heap_read_lock, with_heap_write_lock};
use crate::helper::{i32toi64, i64_to_vec};
use std::fs::File;
use std::mem::ManuallyDrop;
use std::os::windows::io::{FromRawHandle, IntoRawHandle, RawHandle};
use winapi::shared::minwindef::DWORD;
use winapi::um::processenv::GetStdHandle;
use winapi::um::winbase::{STD_ERROR_HANDLE, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE};

pub struct PlatformFile {}

impl PlatformFile {
    pub fn close(file_descriptor_ref: i32) -> crate::error::Result<()> {
        let raw_fd = with_heap_read_lock(|heap| {
            let raw = heap.get_object_field_value(
                file_descriptor_ref,
                "java/io/FileDescriptor",
                "handle",
            )?;
            Ok::<i64, Error>(i32toi64(raw[0], raw[1]))
        })?;

        eprintln!("close0: raw_fd={raw_fd}");

        let file = unsafe { File::from_raw_handle(raw_fd as RawHandle) };
        drop(file);
        Ok(())
    }

    pub fn get_handle(fd: i32) -> crate::error::Result<i64> {
        match fd {
            0 => Self::get_std_handle(STD_INPUT_HANDLE),
            1 => Self::get_std_handle(STD_OUTPUT_HANDLE),
            2 => Self::get_std_handle(STD_ERROR_HANDLE),
            _ => Err(Error::new_execution(&format!("fd {fd} is not supported"))),
        }
    }

    fn get_std_handle(handle: DWORD) -> crate::error::Result<i64> {
        unsafe {
            let handle = GetStdHandle(handle);
            if handle.is_null() {
                return Err(Error::new_execution(&format!(
                    "Failed to get handle by {handle:?}"
                )));
            }

            Ok(handle as isize as i64)
        }
    }

    pub fn set_raw_id(output_stream_ref: i32, file: File) -> std::io::Result<()> {
        let raw_handle = file.into_raw_handle(); // move ownership out of file
        let handle = raw_handle as isize as i64;
        let raw = i64_to_vec(handle);

        with_heap_write_lock(|heap| {
            let fd_ref = heap
                .get_object_field_value(output_stream_ref, "java/io/FileOutputStream", "fd")
                .expect("fd field not found")[0];
            heap.set_object_field_value(fd_ref, "java/io/FileDescriptor", "handle", raw)
                .expect("handle field not found");
        });

        Ok(())
    }

    pub fn get_by_raw_id(obj_ref: i32) -> crate::error::Result<ManuallyDrop<File>> {
        let handle = with_heap_read_lock(|heap| {
            let fd_ref =
                heap.get_object_field_value(obj_ref, "java/io/FileOutputStream", "fd")?[0];
            let raw = heap.get_object_field_value(fd_ref, "java/io/FileDescriptor", "handle")?;

            Ok::<i64, Error>(i32toi64(raw[0], raw[1]))
        })?;

        let file = ManuallyDrop::new(unsafe { File::from_raw_handle(handle as RawHandle) }); // ManuallyDrop prevents `file` from being dropped
        Ok(file)
    }
}
