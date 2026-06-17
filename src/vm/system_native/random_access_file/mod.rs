use crate::vm::error::{Error, Result};
use crate::vm::stack::stack_frame::StackFrames;
use bitflags::bitflags;
use std::io::{Read, Seek, Write};

#[cfg(windows)]
mod windows_impl;
#[cfg(windows)]
use windows_impl::open0;

#[cfg(unix)]
mod unix_impl;
use crate::bail_thrown;
use crate::vm::exception::helpers::{check_bounds, throw_ioexception};
use crate::vm::exception::pending::{thrown, Throws};
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::{i32toi64, i64_to_vec};
use crate::vm::system_native::platform_file::Mode::RandomAccessFile;
use crate::vm::system_native::platform_file::{length, Mode, PlatformFile};
#[cfg(unix)]
use unix_impl::open0;

bitflags! {
    pub(crate) struct RandomAccessFileMode: i32 {
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
    let Some(()) = open0(obj_ref, file_name_ref, mode, stack_frames)? else {
        return Ok(vec![]);
    };

    Ok(vec![])
}

pub(crate) fn random_access_file_seek0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let obj_ref = args[0];
    let offset = i32toi64(args[2], args[1]);
    let Some(()) = seek0(obj_ref, offset, stack_frames)? else {
        return Ok(vec![]);
    };

    Ok(vec![])
}
fn seek0(obj_ref: i32, offset: i64, stack_frames: &mut StackFrames) -> Throws<()> {
    let Some(mut file) =
        PlatformFile::get_by_raw_id(obj_ref, Mode::RandomAccessFile, stack_frames)?
    else {
        return thrown();
    };

    if offset < 0 {
        bail_thrown!(throw_ioexception(
            &format!("Negative seek offset {}", offset),
            stack_frames
        ))
    }

    file.seek(std::io::SeekFrom::Start(offset as u64))?;
    Ok(Some(()))
}

pub(crate) fn random_access_file_write_bytes0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let obj_ref = args[0];
    let bytes_ref = args[1];
    let offset = args[2];
    let len = args[3];
    let Some(()) = write_bytes0(obj_ref, bytes_ref, offset, len, stack_frames)? else {
        return Ok(vec![]);
    };

    Ok(vec![])
}
fn write_bytes0(
    obj_ref: i32,
    bytes_ref: i32,
    offset: i32,
    len: i32,
    stack_frames: &mut StackFrames,
) -> Throws<()> {
    let Some(()) = check_bounds(bytes_ref, offset, len, stack_frames)? else {
        return thrown();
    };

    let Some(mut file) =
        PlatformFile::get_by_raw_id(obj_ref, Mode::RandomAccessFile, stack_frames)?
    else {
        return thrown();
    };

    let input_array = HEAP.get_entire_raw_data(bytes_ref)?;
    let input = &input_array[offset as usize..(offset + len) as usize];
    file.write_all(input)?;

    Ok(Some(()))
}

pub(crate) fn random_access_file_read_bytes0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let obj_ref = args[0];
    let bytes_ref = args[1];
    let offset = args[2];
    let len = args[3];
    let Some(read) = read_bytes0(obj_ref, bytes_ref, offset, len, stack_frames)? else {
        return Ok(vec![]);
    };

    Ok(vec![read])
}
pub(super) fn read_bytes0(
    obj_ref: i32,
    bytes_ref: i32,
    offset: i32,
    len: i32,
    stack_frames: &mut StackFrames,
) -> Throws<i32> {
    let Some(()) = check_bounds(bytes_ref, offset, len, stack_frames)? else {
        return thrown();
    };

    if len == 0 {
        return Ok(Some(0));
    }

    let Some(mut file) =
        PlatformFile::get_by_raw_id(obj_ref, Mode::RandomAccessFile, stack_frames)?
    else {
        return thrown();
    };

    let mut input_array = HEAP.get_entire_raw_data_mut(bytes_ref)?;
    let input = &mut input_array[offset as usize..(offset + len) as usize];
    let read = file.read(input);

    match read {
        Ok(n) if n == 0 => Ok(Some(-1)),
        Ok(n) => Ok(Some(n as i32)),
        Err(e) => bail_thrown!(throw_ioexception(&e.to_string(), stack_frames)),
    }
}

pub(crate) fn random_access_file_length0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let obj_ref = args[0];
    let Some(length) = length0(obj_ref, stack_frames)? else {
        return Ok(vec![]);
    };

    Ok(i64_to_vec(length))
}
fn length0(obj_ref: i32, stack_frames: &mut StackFrames) -> Throws<i64> {
    length(obj_ref, RandomAccessFile, stack_frames)
}
