#[cfg(unix)]
pub(crate) mod unix_file;
#[cfg(windows)]
pub(crate) mod windows_file;

use crate::bail_thrown;
use crate::vm::error::Result;
use crate::vm::exception::helpers::throw_ioexception;
use crate::vm::exception::pending::{thrown, Throws};
use crate::vm::exception::pending_helpers::set_pending_io_exception;
use crate::vm::stack::stack_frame::StackFrames;
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

pub(crate) fn length(obj_ref: i32, mode: Mode, stack_frames: &mut StackFrames) -> Throws<i64> {
    let Some(file) = PlatformFile::get_by_raw_id(obj_ref, mode, stack_frames)? else {
        return thrown();
    };
    let metadata = match file.metadata() {
        Ok(m) => m,
        Err(e) => bail_thrown!(throw_ioexception(&e.to_string(), stack_frames)),
    };
    Ok(Some(metadata.len() as i64))
}

pub(crate) fn length_pending(obj_ref: i32, mode: Mode) -> Result<Option<i64>> {
    let Some(file) = PlatformFile::get_by_raw_id_pending(obj_ref, mode)? else {
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
