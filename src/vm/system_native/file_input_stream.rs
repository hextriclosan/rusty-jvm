use crate::vm::error::Result;
use crate::vm::exception::helpers::{throw_file_not_found_exception, throw_ioexception};
use crate::vm::exception::throwing_result::ThrowingResult;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::i64_to_vec;
use crate::vm::stack::stack_frame::StackFrames;
use crate::vm::system_native::platform_file::Mode::FileInputStream;
use crate::vm::system_native::platform_file::PlatformFile;
use crate::vm::system_native::string::get_utf8_string_by_ref;
use crate::{throw_and_return, unwrap_or_return_err, unwrap_result_or_return_default};
use std::fs::OpenOptions;
use std::io;
use std::io::{Read as IoRead, Seek};

pub(crate) fn file_input_stream_open0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let obj_ref = args[0];
    let string_ref = args[1];
    unwrap_result_or_return_default!(open0(obj_ref, string_ref, stack_frames), vec![]);
    Ok(vec![])
}

fn open0(obj_ref: i32, file_name_ref: i32, stack_frames: &mut StackFrames) -> ThrowingResult<()> {
    let file_name = unwrap_or_return_err!(get_utf8_string_by_ref(file_name_ref));
    match OpenOptions::new().read(true).open(&file_name) {
        Ok(file) => {
            let metadata = unwrap_or_return_err!(file.metadata());
            if metadata.is_dir() {
                let error = io::Error::new(io::ErrorKind::IsADirectory, "Is a directory");
                throw_and_return!(throw_file_not_found_exception(
                    file_name_ref,
                    &error.to_string(),
                    stack_frames
                ))
            }

            unwrap_or_return_err!(PlatformFile::set_raw_id(obj_ref, file, FileInputStream));
            ThrowingResult::ok(())
        }
        Err(e) => {
            let message = e.to_string();
            throw_and_return!(throw_file_not_found_exception(
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
    let len = unwrap_result_or_return_default!(length0(obj_ref, stack_frames), vec![]);
    Ok(i64_to_vec(len))
}
fn length0(obj_ref: i32, stack_frames: &mut StackFrames) -> ThrowingResult<i64> {
    let file = match PlatformFile::get_by_raw_id(obj_ref, FileInputStream, stack_frames) {
        ThrowingResult::Result(result) => unwrap_or_return_err!(result),
        ThrowingResult::ExceptionThrown => return ThrowingResult::ExceptionThrown,
    };
    let metadata = match file.metadata() {
        Ok(m) => m,
        Err(e) => throw_and_return!(throw_ioexception(&e.to_string(), stack_frames)),
    };
    ThrowingResult::ok(metadata.len() as i64)
}

pub(crate) fn file_input_stream_position0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let obj_ref = args[0];
    let len = unwrap_result_or_return_default!(position0(obj_ref, stack_frames), vec![]);
    Ok(i64_to_vec(len))
}
fn position0(obj_ref: i32, stack_frames: &mut StackFrames) -> ThrowingResult<i64> {
    let mut file = match PlatformFile::get_by_raw_id(obj_ref, FileInputStream, stack_frames) {
        ThrowingResult::Result(result) => unwrap_or_return_err!(result),
        ThrowingResult::ExceptionThrown => return ThrowingResult::ExceptionThrown,
    };
    let pos = match file.stream_position() {
        Ok(p) => p,
        Err(e) => throw_and_return!(throw_ioexception(&e.to_string(), stack_frames)),
    };
    ThrowingResult::ok(pos as i64)
}

pub(crate) fn file_input_stream_available0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let obj_ref = args[0];
    let available = unwrap_result_or_return_default!(available0(obj_ref, stack_frames), vec![]);
    Ok(vec![available])
}
fn available0(obj_ref: i32, stack_frames: &mut StackFrames) -> ThrowingResult<i32> {
    let mut file = match PlatformFile::get_by_raw_id(obj_ref, FileInputStream, stack_frames) {
        ThrowingResult::Result(result) => unwrap_or_return_err!(result),
        ThrowingResult::ExceptionThrown => return ThrowingResult::ExceptionThrown,
    };
    let current_pos = match file.stream_position() {
        Ok(p) => p,
        Err(e) => throw_and_return!(throw_ioexception(&e.to_string(), stack_frames)),
    };
    let metadata = match file.metadata() {
        Ok(m) => m,
        Err(e) => throw_and_return!(throw_ioexception(&e.to_string(), stack_frames)),
    };
    let file_size = metadata.len();
    if file_size < current_pos {
        return ThrowingResult::ok(0);
    }
    let available = file_size - current_pos;
    let available = unwrap_or_return_err!(i32::try_from(available));
    ThrowingResult::ok(available)
}

pub(crate) fn file_input_stream_read_bytes_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let obj_ref = args[0];
    let bytes_ref = args[1];
    let off = args[2];
    let len = args[3];
    let read_bytes = unwrap_result_or_return_default!(
        read_bytes(obj_ref, bytes_ref, off, len, stack_frames),
        vec![]
    );
    Ok(vec![read_bytes])
}
fn read_bytes(
    obj_ref: i32,
    bytes_ref: i32,
    off: i32,
    len: i32,
    stack_frames: &mut StackFrames,
) -> ThrowingResult<i32> {
    let mut file = match PlatformFile::get_by_raw_id(obj_ref, FileInputStream, stack_frames) {
        ThrowingResult::Result(result) => unwrap_or_return_err!(result),
        ThrowingResult::ExceptionThrown => return ThrowingResult::ExceptionThrown,
    };
    let mut buffer = vec![0u8; len as usize];
    let read_bytes = match file.read(&mut buffer) {
        Ok(n) => n,
        Err(e) => throw_and_return!(throw_ioexception(&e.to_string(), stack_frames)),
    };
    let array = HEAP.get_entire_array(bytes_ref);
    let mut array = unwrap_or_return_err!(array);

    for i in 0..read_bytes {
        unwrap_or_return_err!(array.set_value(off + i as i32, vec![buffer[i] as i8 as i32]));
    }

    unwrap_or_return_err!(HEAP.set_entire_array(bytes_ref, array));

    ThrowingResult::ok(read_bytes as i32)
}

pub(crate) fn file_input_stream_read0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let obj_ref = args[0];
    let read = unwrap_result_or_return_default!(read0(obj_ref, stack_frames), vec![]);
    Ok(vec![read])
}
fn read0(obj_ref: i32, stack_frames: &mut StackFrames) -> ThrowingResult<i32> {
    let mut file = match PlatformFile::get_by_raw_id(obj_ref, FileInputStream, stack_frames) {
        ThrowingResult::Result(result) => unwrap_or_return_err!(result),
        ThrowingResult::ExceptionThrown => return ThrowingResult::ExceptionThrown,
    };
    let mut buffer = [0u8; 1];
    let read_bytes = match file.read(&mut buffer) {
        Ok(n) => n,
        Err(e) => throw_and_return!(throw_ioexception(&e.to_string(), stack_frames)),
    };
    if read_bytes == 0 {
        return ThrowingResult::ok(-1);
    }
    ThrowingResult::ok(buffer[0] as i32)
}

pub(crate) fn file_input_stream_is_regular_file0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let _obj_ref = args[0];
    let fd_ref = args[1];
    let regular = unwrap_result_or_return_default!(is_regular_file0(fd_ref, stack_frames), vec![]);
    Ok(vec![if regular { 1 } else { 0 }])
}
fn is_regular_file0(fd_ref: i32, stack_frames: &mut StackFrames) -> ThrowingResult<bool> {
    let file = match PlatformFile::get_by_fd(fd_ref, stack_frames) {
        ThrowingResult::Result(result) => unwrap_or_return_err!(result),
        ThrowingResult::ExceptionThrown => return ThrowingResult::ExceptionThrown,
    };
    let metadata = match file.metadata() {
        Ok(m) => m,
        Err(e) => throw_and_return!(throw_ioexception(&e.to_string(), stack_frames)),
    };

    ThrowingResult::ok(metadata.is_file())
}
