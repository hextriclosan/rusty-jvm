use crate::vm::error::Result;
use crate::vm::exception::helpers::{throw_file_not_found_exception, throw_ioexception};
use crate::vm::exception::throwing_result::ThrowingResult;
use crate::vm::heap::heap::with_heap_read_lock;
use crate::vm::stack::stack_frame::StackFrames;
use crate::vm::system_native::platform_file::Mode::FileOutputStream;
use crate::vm::system_native::platform_file::PlatformFile;
use crate::vm::system_native::string::get_utf8_string_by_ref;
use crate::{throw_and_return, unwrap_or_return_err, unwrap_result_or_return_default};
use std::fs::OpenOptions;
use std::io::Write as IoWrite;

pub(crate) fn file_output_stream_open0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let obj_ref = args[0];
    let string_ref = args[1];
    let append = args[2] != 0;
    unwrap_result_or_return_default!(open0(obj_ref, string_ref, append, stack_frames), vec![]);
    Ok(vec![])
}

fn open0(
    obj_ref: i32,
    file_name_ref: i32,
    append: bool,
    stack_frames: &mut StackFrames,
) -> ThrowingResult<()> {
    let file_name = unwrap_or_return_err!(get_utf8_string_by_ref(file_name_ref));

    match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(!append)
        .append(append)
        .open(file_name)
    {
        Ok(file) => ThrowingResult::ok(unwrap_or_return_err!(PlatformFile::set_raw_id(
            obj_ref,
            file,
            FileOutputStream
        ))),
        Err(e) => throw_and_return!(throw_file_not_found_exception(
            file_name_ref,
            &e.to_string(),
            stack_frames
        )),
    }
}

pub(crate) fn file_output_stream_write_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let obj_ref = args[0];
    let byte = args[1];
    let append = args[2] != 0;
    unwrap_result_or_return_default!(write(obj_ref, byte, append, stack_frames), vec![]);
    Ok(vec![])
}
fn write(
    obj_ref: i32,
    byte: i32,
    _append: bool,
    stack_frames: &mut StackFrames,
) -> ThrowingResult<()> {
    let mut file = match PlatformFile::get_by_raw_id(obj_ref, FileOutputStream, stack_frames) {
        ThrowingResult::Result(result) => unwrap_or_return_err!(result),
        ThrowingResult::ExceptionThrown => return ThrowingResult::ExceptionThrown,
    };

    match write!(file, "{}", byte as u8 as char) {
        Ok(_) => ThrowingResult::ok(()),
        Err(e) => throw_and_return!(throw_ioexception(&e.to_string(), stack_frames)),
    }
}

pub(crate) fn file_output_stream_write_bytes_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let obj_ref = args[0];
    let bytes_ref = args[1];
    let off = args[2];
    let len = args[3];
    let append = args[4] != 0;
    unwrap_result_or_return_default!(
        write_bytes(obj_ref, bytes_ref, off, len, append, stack_frames),
        vec![]
    );
    Ok(vec![])
}
fn write_bytes(
    obj_ref: i32,
    bytes_ref: i32,
    off: i32,
    len: i32,
    _append: bool,
    stack_frames: &mut StackFrames,
) -> ThrowingResult<()> {
    let mut file = match PlatformFile::get_by_raw_id(obj_ref, FileOutputStream, stack_frames) {
        ThrowingResult::Result(result) => unwrap_or_return_err!(result),
        ThrowingResult::ExceptionThrown => return ThrowingResult::ExceptionThrown,
    };
    let array =
        unwrap_or_return_err!(with_heap_read_lock(|heap| heap.get_entire_array(bytes_ref)));
    let raw = array.get_entire_value();
    let mut vec = Vec::with_capacity(raw.len());
    for i in off..off + len {
        vec.push(raw[i as usize][0] as i8 as u8);
    }

    match file.write_all(&vec) {
        Ok(_) => ThrowingResult::ok(()),
        Err(e) => throw_and_return!(throw_ioexception(&e.to_string(), stack_frames)),
    }
}
