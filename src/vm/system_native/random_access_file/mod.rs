use crate::vm::error::{Error, Result};
use crate::vm::exception::throwing_result::ThrowingResult;
use crate::vm::stack::stack_frame::StackFrames;
use crate::{throw_and_return, unwrap_or_return_err, unwrap_result_or_return_default};
use bitflags::bitflags;
use std::io::{Read, Seek, Write};

#[cfg(windows)]
mod windows_impl;
#[cfg(windows)]
use windows_impl::open0;

#[cfg(unix)]
mod unix_impl;
use crate::vm::exception::helpers::throw_ioexception;
use crate::vm::heap::heap::with_heap_read_lock;
use crate::vm::helper::i32toi64;
use crate::vm::system_native::platform_file::{Mode, PlatformFile};
#[cfg(unix)]
use unix_impl::open0;

bitflags! {
    struct RandomAccessFileMode: i32 {
        const O_RDONLY = 1;
        const O_RDWR = 2;
        const O_SYNC = 4;
        const O_DSYNC = 8;
        const O_TEMPORARY = 16;
    }
}

pub(crate) fn random_access_file_open0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let obj_ref = args[0];
    let file_name_ref = args[1];
    let mode = RandomAccessFileMode::from_bits(args[2])
        .ok_or_else(|| Error::new_native(&format!("Invalid RandomAccessFileMode {}", args[2])))?;
    unwrap_result_or_return_default!(open0(obj_ref, file_name_ref, mode, stack_frames), vec![]);

    Ok(vec![])
}

pub(crate) fn random_access_file_seek0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let obj_ref = args[0];
    let offset = i32toi64(args[2], args[1]);
    unwrap_result_or_return_default!(seek0(obj_ref, offset, stack_frames), vec![]);

    Ok(vec![])
}
fn seek0(obj_ref: i32, offset: i64, stack_frames: &mut StackFrames) -> ThrowingResult<()> {
    let mut file = match PlatformFile::get_by_raw_id(obj_ref, Mode::ReadWrite, stack_frames) {
        ThrowingResult::Result(result) => unwrap_or_return_err!(result),
        ThrowingResult::ExceptionThrown => return ThrowingResult::ExceptionThrown,
    };

    unwrap_or_return_err!(file.seek(std::io::SeekFrom::Start(offset as u64)));
    ThrowingResult::ok(())
}

pub(crate) fn random_access_file_write_bytes0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let obj_ref = args[0];
    let bytes_ref = args[1];
    let offset = args[2];
    let len = args[3];
    unwrap_result_or_return_default!(
        write_bytes0(obj_ref, bytes_ref, offset, len, stack_frames),
        vec![]
    );

    Ok(vec![])
}
fn write_bytes0(
    obj_ref: i32,
    bytes_ref: i32,
    offset: i32,
    len: i32,
    stack_frames: &mut StackFrames,
) -> ThrowingResult<()> {
    let mut file = match PlatformFile::get_by_raw_id(obj_ref, Mode::ReadWrite, stack_frames) {
        ThrowingResult::Result(result) => unwrap_or_return_err!(result),
        ThrowingResult::ExceptionThrown => return ThrowingResult::ExceptionThrown,
    };

    let ret = with_heap_read_lock(|h| {
        let input_array = h.get_entire_raw_data(bytes_ref)?;
        let input = &input_array[offset as usize..(offset + len) as usize];
        file.write_all(input)?;
        Ok::<_, Error>(())
    });
    unwrap_or_return_err!(ret);
    ThrowingResult::ok(())
}

pub(crate) fn random_access_file_read_bytes0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let obj_ref = args[0];
    let bytes_ref = args[1];
    let offset = args[2];
    let len = args[3];
    let read = unwrap_result_or_return_default!(
        read_bytes0(obj_ref, bytes_ref, offset, len, stack_frames),
        vec![]
    );

    Ok(vec![read])
}
pub(super) fn read_bytes0(
    obj_ref: i32,
    bytes_ref: i32,
    offset: i32,
    len: i32,
    stack_frames: &mut StackFrames,
) -> ThrowingResult<i32> {
    let mut file = match PlatformFile::get_by_raw_id(obj_ref, Mode::ReadWrite, stack_frames) {
        ThrowingResult::Result(result) => unwrap_or_return_err!(result),
        ThrowingResult::ExceptionThrown => return ThrowingResult::ExceptionThrown,
    };

    let ret = with_heap_read_lock(|h| {
        let mut input_array = h.get_entire_raw_data_mut(bytes_ref)?;
        let mut input = &mut input_array[offset as usize..(offset + len) as usize];
        let result = file.read_exact(&mut input);
        let ret = match result {
            Ok(_) => Ok(len),
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => Ok(-1),
            Err(e) => Err(e),
        }?;
        Ok::<_, Error>(ret)
    });

    match ret {
        Ok(n) => ThrowingResult::ok(n),
        Err(e) => throw_and_return!(throw_ioexception(&e.to_string(), stack_frames)),
    }
}
