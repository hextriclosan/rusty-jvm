#[cfg(unix)]
pub(crate) mod unix_file;
#[cfg(windows)]
pub(crate) mod windows_file;

use crate::vm::error::Result;
use crate::vm::exception::pending_helpers::set_pending_io_exception;
use crate::vm::heap::heap::HEAP;
use std::fs::File;
use std::mem::ManuallyDrop;
use strum::{AsRefStr, EnumString};
#[cfg(unix)]
pub use unix_file::PlatformFile;
#[cfg(windows)]
pub use windows_file::PlatformFile;

#[derive(EnumString, AsRefStr, Debug)]
pub enum Mode {
    #[strum(serialize = "java/io/FileInputStream")]
    FileInputStream,
    #[strum(serialize = "java/io/FileOutputStream")]
    FileOutputStream,
    #[strum(serialize = "java/io/RandomAccessFile")]
    RandomAccessFile,
}

pub(crate) fn length(obj_ref: i32, mode: Mode) -> Result<Option<i64>> {
    let Some(file) = get_by_raw_id(obj_ref, mode)? else {
        return Ok(None);
    };
    let metadata = match file.metadata() {
        Ok(m) => m,
        Err(e) => {
            set_pending_io_exception(&e.to_string())?;
            return Ok(None);
        }
    };
    Ok(Some(metadata.len() as i64))
}

pub(crate) fn get_by_raw_id(obj_ref: i32, mode: Mode) -> Result<Option<ManuallyDrop<File>>> {
    let fd_ref = HEAP.get_object_field_value(obj_ref, mode.as_ref(), "fd")?[0];
    PlatformFile::get_by_fd(fd_ref)
}
