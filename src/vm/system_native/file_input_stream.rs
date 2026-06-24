use crate::bail_thrown;
use crate::vm::error::Result;
use crate::vm::exception::helpers::{throw_file_not_found_exception, throw_ioexception};
use crate::vm::exception::pending::{thrown, Throws};
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::i64_to_vec;
use crate::vm::stack::stack_frame::StackFrames;
use crate::vm::system_native::platform_file::Mode::FileInputStream;
use crate::vm::system_native::platform_file::{length, PlatformFile};
use crate::vm::system_native::string::get_utf8_string_by_ref;
use std::fs::OpenOptions;
use std::io;
use std::io::{Read as IoRead, Seek};

pub(crate) fn file_input_stream_open0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let obj_ref = args[0];
    let string_ref = args[1];
    let Some(()) = open0(obj_ref, string_ref, stack_frames)? else {
        return Ok(vec![]);
    };
    Ok(vec![])
}

fn open0(obj_ref: i32, file_name_ref: i32, stack_frames: &mut StackFrames) -> Throws<()> {
    let file_name = get_utf8_string_by_ref(file_name_ref)?;
    match OpenOptions::new().read(true).open(&file_name) {
        Ok(file) => {
            let metadata = file.metadata()?;
            if metadata.is_dir() {
                let error = io::Error::new(io::ErrorKind::IsADirectory, "Is a directory");
                bail_thrown!(throw_file_not_found_exception(
                    file_name_ref,
                    &error.to_string(),
                    stack_frames
                ))
            }

            PlatformFile::set_raw_id(obj_ref, file, FileInputStream)?;
            Ok(Some(()))
        }
        Err(e) => {
            let message = e.to_string();
            bail_thrown!(throw_file_not_found_exception(
                file_name_ref,
                &message,
                stack_frames
            ));
        }
    }
}

pub(crate) fn file_input_stream_length0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let obj_ref = args[0];
    let Some(len) = length0(obj_ref, stack_frames)? else {
        return Ok(vec![]);
    };
    Ok(i64_to_vec(len))
}
fn length0(obj_ref: i32, stack_frames: &mut StackFrames) -> Throws<i64> {
    length(obj_ref, FileInputStream, stack_frames)
}

pub(crate) fn file_input_stream_position0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let obj_ref = args[0];
    let Some(len) = position0(obj_ref, stack_frames)? else {
        return Ok(vec![]);
    };
    Ok(i64_to_vec(len))
}
fn position0(obj_ref: i32, stack_frames: &mut StackFrames) -> Throws<i64> {
    let Some(mut file) = PlatformFile::get_by_raw_id(obj_ref, FileInputStream, stack_frames)?
    else {
        return thrown();
    };
    let pos = match file.stream_position() {
        Ok(p) => p,
        Err(e) => bail_thrown!(throw_ioexception(&e.to_string(), stack_frames)),
    };
    Ok(Some(pos as i64))
}

pub(crate) fn file_input_stream_available0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let obj_ref = args[0];
    let Some(available) = available0(obj_ref, stack_frames)? else {
        return Ok(vec![]);
    };
    Ok(vec![available])
}
fn available0(obj_ref: i32, stack_frames: &mut StackFrames) -> Throws<i32> {
    let Some(mut file) = PlatformFile::get_by_raw_id(obj_ref, FileInputStream, stack_frames)?
    else {
        return thrown();
    };
    let current_pos = match file.stream_position() {
        Ok(p) => p,
        Err(e) => bail_thrown!(throw_ioexception(&e.to_string(), stack_frames)),
    };
    let metadata = match file.metadata() {
        Ok(m) => m,
        Err(e) => bail_thrown!(throw_ioexception(&e.to_string(), stack_frames)),
    };
    let file_size = metadata.len();
    if file_size < current_pos {
        return Ok(Some(0));
    }
    let available = file_size - current_pos;
    let available = i32::try_from(available)?;
    Ok(Some(available))
}

pub(crate) fn file_input_stream_read_bytes_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let obj_ref = args[0];
    let bytes_ref = args[1];
    let off = args[2];
    let len = args[3];
    let Some(read_bytes) = read_bytes(obj_ref, bytes_ref, off, len, stack_frames)? else {
        return Ok(vec![]);
    };
    Ok(vec![read_bytes])
}
fn read_bytes(
    obj_ref: i32,
    bytes_ref: i32,
    off: i32,
    len: i32,
    stack_frames: &mut StackFrames,
) -> Throws<i32> {
    let Some(mut file) = PlatformFile::get_by_raw_id(obj_ref, FileInputStream, stack_frames)?
    else {
        return thrown();
    };
    let mut buffer = vec![0u8; len as usize];
    let read_bytes = match file.read(&mut buffer) {
        Ok(n) => n,
        Err(e) => bail_thrown!(throw_ioexception(&e.to_string(), stack_frames)),
    };
    let mut array = HEAP.get_entire_array(bytes_ref)?;

    for i in 0..read_bytes {
        array.set_value(off + i as i32, vec![buffer[i] as i8 as i32])?;
    }

    HEAP.set_entire_array(bytes_ref, array)?;

    Ok(Some(read_bytes as i32))
}

pub(crate) fn file_input_stream_read0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let obj_ref = args[0];
    let Some(read) = read0(obj_ref, stack_frames)? else {
        return Ok(vec![]);
    };
    Ok(vec![read])
}
fn read0(obj_ref: i32, stack_frames: &mut StackFrames) -> Throws<i32> {
    let Some(mut file) = PlatformFile::get_by_raw_id(obj_ref, FileInputStream, stack_frames)?
    else {
        return thrown();
    };
    let mut buffer = [0u8; 1];
    let read_bytes = match file.read(&mut buffer) {
        Ok(n) => n,
        Err(e) => bail_thrown!(throw_ioexception(&e.to_string(), stack_frames)),
    };
    if read_bytes == 0 {
        return Ok(Some(-1));
    }
    Ok(Some(buffer[0] as i32))
}

pub(crate) fn file_input_stream_is_regular_file0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let _obj_ref = args[0];
    let fd_ref = args[1];
    let Some(regular) = is_regular_file0(fd_ref, stack_frames)? else {
        return Ok(vec![]);
    };
    Ok(vec![if regular { 1 } else { 0 }])
}
fn is_regular_file0(fd_ref: i32, stack_frames: &mut StackFrames) -> Throws<bool> {
    let Some(file) = PlatformFile::get_by_fd(fd_ref, stack_frames)? else {
        return thrown();
    };
    let metadata = match file.metadata() {
        Ok(m) => m,
        Err(e) => bail_thrown!(throw_ioexception(&e.to_string(), stack_frames)),
    };

    Ok(Some(metadata.is_file()))
}
