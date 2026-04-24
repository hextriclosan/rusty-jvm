use crate::vm::error::Result;
use crate::vm::exception::throwing_result::ThrowingResult;
use crate::vm::helper::{get_handle, i32toi64, i64_to_vec};
use crate::vm::stack::stack_frame::StackFrames;
use crate::{throw_and_return, unwrap_or_return_err, unwrap_result_or_return_default};
use std::io::Seek;

#[cfg(windows)]
pub mod windows_file_dispatcher;
#[cfg(windows)]
use windows_file_dispatcher::truncate0;

mod mmap_registry;
#[cfg(unix)]
pub mod unix_file_dispatcher;

use crate::vm::exception::helpers::throw_ioexception;
use crate::vm::system_native::platform_file::PlatformFile;
use crate::vm::system_native::platform_file_dispatcher::mmap_registry::MmapVariant;
#[cfg(unix)]
use unix_file_dispatcher::truncate0;

pub fn allocation_granularity0_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(i64_to_vec(allocation_granularity0()))
}
fn allocation_granularity0() -> i64 {
    page_size::get_granularity() as i64
}

pub fn file_dispatcher_impl_truncate0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let fd_ref = args[0];
    let size = i32toi64(args[2], args[1]);

    let result = unwrap_result_or_return_default!(truncate0(fd_ref, size, stack_frames), vec![]);
    Ok(vec![result])
}

pub fn file_dispatcher_map0_wrp(args: &[i32], stack_frames: &mut StackFrames) -> Result<Vec<i32>> {
    let fd_ref = args[0];
    let prot = args[1];
    let position = i32toi64(args[3], args[2]);
    let length = i32toi64(args[5], args[4]);
    let is_sync = args[6] != 0;

    let address = unwrap_result_or_return_default!(
        map0(fd_ref, prot, position, length, is_sync, stack_frames),
        vec![]
    );
    Ok(i64_to_vec(address))
}
pub(super) fn map0(
    fd_ref: i32,
    prot: i32,
    position: i64,
    length: i64,
    _is_sync: bool, // is not supported
    stack_frames: &mut StackFrames,
) -> ThrowingResult<i64> {
    #[cfg(windows)]
    let raw_handle = unwrap_or_return_err!(get_handle(fd_ref)) as std::os::windows::io::RawHandle;
    #[cfg(unix)]
    let raw_handle = unwrap_or_return_err!(get_handle(fd_ref));

    let addr = match MmapVariant::register(raw_handle, prot, position as u64, length as usize) {
        Ok(addr) => addr,
        Err(e) => {
            throw_and_return!(throw_ioexception(
                &format!("Memory mapping failed: {}", e),
                stack_frames
            ))
        }
    };

    ThrowingResult::ok(addr)
}

pub fn mapped_memory_utils_force0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let fd_ref = args[0];
    let address = i32toi64(args[2], args[1]);
    let length = i32toi64(args[4], args[3]);

    unwrap_result_or_return_default!(force0(fd_ref, address, length, stack_frames), vec![]);
    Ok(vec![])
}
fn force0(
    _fd_ref: i32,
    address: i64,
    length: i64,
    stack_frames: &mut StackFrames,
) -> ThrowingResult<()> {
    match MmapVariant::flush(address, length as usize) {
        Ok(_) => ThrowingResult::ok(()),
        Err(e) => {
            throw_and_return!(throw_ioexception(
                &format!("Forcing mapped memory to storage failed: {}", e),
                stack_frames
            ))
        }
    }
}

pub fn file_dispatcher_is_other0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let fd_ref = args[0];
    let res = unwrap_result_or_return_default!(is_other0(fd_ref, stack_frames), vec![]);
    Ok(vec![if res { 1 } else { 0 }])
}
fn is_other0(fd_ref: i32, stack_frames: &mut StackFrames) -> ThrowingResult<bool> {
    let file = match PlatformFile::get_by_fd(fd_ref, stack_frames) {
        ThrowingResult::Result(result) => unwrap_or_return_err!(result),
        ThrowingResult::ExceptionThrown => return ThrowingResult::ExceptionThrown,
    };

    let metadata = unwrap_or_return_err!(file.metadata());

    ThrowingResult::ok(
        !metadata.is_file() && !metadata.is_dir() && !metadata.file_type().is_symlink(),
    )
}

pub fn file_dispatcher_is_seek0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let fd_ref = args[0];
    let offset = i32toi64(args[2], args[1]);
    let current_offset =
        unwrap_result_or_return_default!(seek0(fd_ref, offset, stack_frames), vec![]);
    Ok(i64_to_vec(current_offset))
}
fn seek0(fd_ref: i32, offset: i64, stack_frames: &mut StackFrames) -> ThrowingResult<i64> {
    let mut file = match PlatformFile::get_by_fd(fd_ref, stack_frames) {
        ThrowingResult::Result(result) => unwrap_or_return_err!(result),
        ThrowingResult::ExceptionThrown => return ThrowingResult::ExceptionThrown,
    };

    let current_offset = if offset < 0 {
        unwrap_or_return_err!(file.seek(std::io::SeekFrom::Current(0i64)))
    } else {
        unwrap_or_return_err!(file.seek(std::io::SeekFrom::Start(offset as u64)))
    };

    ThrowingResult::ok(current_offset as i64)
}
