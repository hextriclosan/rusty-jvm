use crate::heap::heap::{with_heap_read_lock, with_heap_write_lock};
use crate::system_native::string::get_utf8_string_by_ref;
use std::fs::OpenOptions;
use std::io::Write;
use std::mem::ManuallyDrop;
use std::os::fd::{FromRawFd, IntoRawFd, RawFd};

pub(crate) fn file_output_stream_open0_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let obj_ref = args[0];
    let string_ref = args[1];
    let append = args[2];
    open0(obj_ref, string_ref, if append != 0 { true } else { false })?;
    Ok(vec![])
}

fn open0(obj_ref: i32, file_name_ref: i32, append: bool) -> crate::error::Result<()> {
    let file_name = get_utf8_string_by_ref(file_name_ref)?;

    let posix_fd = open_file_raw(&file_name, append)?; // throw FileNotFoundException if file not found
    let fd = posix_fd as i32;

    with_heap_write_lock(|heap| {
        let fd_ref = heap
            .get_object_field_value(obj_ref, "java/io/FileOutputStream", "fd")
            .expect("fd field not found")[0];
        heap.set_object_field_value(fd_ref, "java/io/FileDescriptor", "fd", vec![fd])
            .expect("fd field not found");
    });

    Ok(())
}

pub(crate) fn file_output_stream_write_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let obj_ref = args[0];
    let byte = args[1];
    let append = args[2];
    write(obj_ref, byte, if append != 0 { true } else { false })?;
    Ok(vec![])
}
fn write(obj_ref: i32, byte: i32, _append: bool) -> crate::error::Result<()> {
    let fd = extract_fd(obj_ref)?;

    let mut file = ManuallyDrop::new(unsafe { std::fs::File::from_raw_fd(fd) }); // ManuallyDrop prevents `file` from being dropped

    write!(file, "{}", byte as u8 as char)?;

    Ok(())
}

pub(crate) fn file_output_stream_write_bytes_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let obj_ref = args[0];
    let bytes_ref = args[1];
    let off = args[2];
    let len = args[3];
    let append = args[4];
    write_bytes(
        obj_ref,
        bytes_ref,
        off,
        len,
        if append != 0 { true } else { false },
    )?;
    Ok(vec![])
}
fn write_bytes(
    obj_ref: i32,
    bytes_ref: i32,
    off: i32,
    len: i32,
    _append: bool,
) -> crate::error::Result<()> {
    let fd = extract_fd(obj_ref)?;

    let mut file = ManuallyDrop::new(unsafe { std::fs::File::from_raw_fd(fd) }); // ManuallyDrop prevents `file` from being dropped

    let array = with_heap_read_lock(|heap| {
        heap.get_entire_array(bytes_ref)
            .expect("error getting array value")
    });
    let raw = array.get_entire_value();
    let mut vec = Vec::with_capacity(raw.len());
    for i in off..off + len {
        vec.push(raw[i as usize][0] as i8 as u8);
    }

    file.write_all(&vec)?;

    Ok(())
}

fn extract_fd(obj_ref: i32) -> crate::error::Result<i32> {
    with_heap_read_lock(|heap| {
        let fd_ref = heap.get_object_field_value(obj_ref, "java/io/FileOutputStream", "fd")?[0];
        let fd = heap.get_object_field_value(fd_ref, "java/io/FileDescriptor", "fd")?[0];

        Ok(fd)
    })
}

fn open_file_raw(file_name: &str, append: bool) -> std::io::Result<RawFd> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(!append)
        .append(append)
        .open(file_name)?;
    Ok(file.into_raw_fd()) // move ownership out of file
}
