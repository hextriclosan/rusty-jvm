use crate::vm::error::Result;
use crate::vm::exception::helpers::throw_file_not_found_exception;
use crate::vm::heap::heap::with_heap_read_lock;
use crate::vm::stack::stack_frame::StackFrames;
use crate::vm::system_native::platform_file::PlatformFile;
use crate::vm::system_native::string::get_utf8_string_by_ref;
use std::fs::OpenOptions;
use std::io::Write;

pub(crate) fn file_output_stream_open0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let obj_ref = args[0];
    let string_ref = args[1];
    let append = args[2];
    open0(obj_ref, string_ref, append != 0, stack_frames)?;
    Ok(vec![])
}

fn open0(
    obj_ref: i32,
    file_name_ref: i32,
    append: bool,
    stack_frames: &mut StackFrames,
) -> Result<()> {
    let file_name = get_utf8_string_by_ref(file_name_ref)?;

    match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(!append)
        .append(append)
        .open(file_name)
    {
        Ok(file) => PlatformFile::set_raw_id(obj_ref, file),
        Err(e) => {
            let message = e.to_string();
            throw_file_not_found_exception(file_name_ref, message, stack_frames)?;
            Ok(())
        }
    }
}

pub(crate) fn file_output_stream_write_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let obj_ref = args[0];
    let byte = args[1];
    let append = args[2];
    write(obj_ref, byte, append != 0)?;
    Ok(vec![])
}
fn write(obj_ref: i32, byte: i32, _append: bool) -> Result<()> {
    let mut file = PlatformFile::get_by_raw_id(obj_ref)?;

    write!(file, "{}", byte as u8 as char)?;

    Ok(())
}

pub(crate) fn file_output_stream_write_bytes_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let obj_ref = args[0];
    let bytes_ref = args[1];
    let off = args[2];
    let len = args[3];
    let append = args[4];
    write_bytes(obj_ref, bytes_ref, off, len, append != 0)?;
    Ok(vec![])
}
fn write_bytes(obj_ref: i32, bytes_ref: i32, off: i32, len: i32, _append: bool) -> Result<()> {
    let mut file = PlatformFile::get_by_raw_id(obj_ref)?;
    let array = with_heap_read_lock(|heap| heap.get_entire_array(bytes_ref))?;
    let raw = array.get_entire_value();
    let mut vec = Vec::with_capacity(raw.len());
    for i in off..off + len {
        vec.push(raw[i as usize][0] as i8 as u8);
    }

    file.write_all(&vec)?;

    Ok(())
}
