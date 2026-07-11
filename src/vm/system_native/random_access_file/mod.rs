use crate::vm::error::Result;
use bitflags::bitflags;
use std::io::{Read, Seek, Write};

#[cfg(windows)]
mod windows_impl;
#[cfg(windows)]
use windows_impl::open0 as open0_impl;

#[cfg(unix)]
mod unix_impl;
use crate::vm::exception::helpers::check_bounds;
use crate::vm::exception::pending_helpers::{
    set_pending_illegal_argument_exception, set_pending_io_exception,
};
use crate::vm::heap::heap::HEAP;
use crate::vm::system_native::platform_file::Mode::RandomAccessFile;
use crate::vm::system_native::platform_file::{get_by_raw_id, length};
#[cfg(unix)]
use unix_impl::open0 as open0_impl;

bitflags! {
    pub(crate) struct RandomAccessFileMode: i32 {
        const O_RDONLY = 1;
        const O_RDWR = 2;
        const O_SYNC = 4;
        const O_DSYNC = 8;
        const O_TEMPORARY = 16;
    }
}

/// `java.io.RandomAccessFile.initIDs()V`
pub(crate) fn init_ids() -> Result<()> {
    // todo: implement me
    Ok(())
}

/// `java.io.RandomAccessFile.open0(Ljava/lang/String;I)V`
pub(crate) fn open0(obj_ref: i32, file_name_ref: i32, mode_raw: i32) -> Result<()> {
    let Some(mode) = RandomAccessFileMode::from_bits(mode_raw) else {
        set_pending_illegal_argument_exception(&format!("Invalid mode: {}", mode_raw))?;
        return Ok(());
    };

    open0_impl(obj_ref, file_name_ref, mode)
}

/// `java.io.RandomAccessFile.seek0(J)V`
pub(crate) fn seek0(obj_ref: i32, offset: i64) -> Result<()> {
    let Some(mut file) = get_by_raw_id(obj_ref, RandomAccessFile)? else {
        return Ok(());
    };

    if offset < 0 {
        set_pending_io_exception(&format!("Negative seek offset {}", offset))?;
        return Ok(());
    }

    file.seek(std::io::SeekFrom::Start(offset as u64))?;
    Ok(())
}

/// `java.io.RandomAccessFile.writeBytes0([BII)V`
pub(crate) fn write_bytes0(obj_ref: i32, bytes_ref: i32, offset: i32, len: i32) -> Result<()> {
    if !check_bounds(bytes_ref, offset, len)? {
        return Ok(());
    }

    let Some(mut file) = get_by_raw_id(obj_ref, RandomAccessFile)? else {
        return Ok(());
    };

    let input_array = HEAP.get_entire_raw_data(bytes_ref)?;
    let input = &input_array[offset as usize..(offset + len) as usize];
    file.write_all(input)?;

    Ok(())
}

/// `java.io.RandomAccessFile.readBytes0([BII)I`
pub(crate) fn read_bytes0(obj_ref: i32, bytes_ref: i32, offset: i32, len: i32) -> Result<i32> {
    if !check_bounds(bytes_ref, offset, len)? {
        return Ok(-1);
    }

    if len == 0 {
        return Ok(0);
    }

    let Some(mut file) = get_by_raw_id(obj_ref, RandomAccessFile)? else {
        return Ok(-1);
    };

    let mut input_array = HEAP.get_entire_raw_data_mut(bytes_ref)?;
    let input = &mut input_array[offset as usize..(offset + len) as usize];
    let read = file.read(input);

    match read {
        Ok(n) if n == 0 => Ok(-1),
        Ok(n) => Ok(n as i32),
        Err(e) => {
            set_pending_io_exception(&e.to_string())?;
            Ok(-1)
        }
    }
}

/// `java.io.RandomAccessFile.length0()J`
pub(crate) fn length0(obj_ref: i32) -> Result<i64> {
    let len = length(obj_ref, RandomAccessFile)?.unwrap_or(-1);
    Ok(len)
}
