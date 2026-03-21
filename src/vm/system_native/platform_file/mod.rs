#[cfg(unix)]
pub(crate) mod unix_file;
#[cfg(windows)]
pub(crate) mod windows_file;

use crate::vm::exception::helpers::throw_ioexception;
use crate::vm::exception::throwing_result::ThrowingResult;
use crate::vm::stack::stack_frame::StackFrames;
use crate::{throw_and_return, unwrap_or_return_err};
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

pub(crate) fn length(
    obj_ref: i32,
    mode: Mode,
    stack_frames: &mut StackFrames,
) -> ThrowingResult<i64> {
    let file = match PlatformFile::get_by_raw_id(obj_ref, mode, stack_frames) {
        ThrowingResult::Result(result) => unwrap_or_return_err!(result),
        ThrowingResult::ExceptionThrown => return ThrowingResult::ExceptionThrown,
    };
    let metadata = match file.metadata() {
        Ok(m) => m,
        Err(e) => throw_and_return!(throw_ioexception(&e.to_string(), stack_frames)),
    };
    ThrowingResult::ok(metadata.len() as i64)
}
