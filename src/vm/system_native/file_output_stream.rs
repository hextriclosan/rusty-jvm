use crate::vm::error::Result;
use crate::vm::exception::pending_helpers::{
    set_pending_file_not_found_exception, set_pending_io_exception,
};
use crate::vm::heap::heap::HEAP;
use crate::vm::system_native::platform_file::Mode::FileOutputStream;
use crate::vm::system_native::platform_file::{get_by_raw_id, PlatformFile};
use crate::vm::system_native::string::get_utf8_string_by_ref;
use std::fs::OpenOptions;
use std::io::Write as IoWrite;

/// `java.io.FileOutputStream.initIDs()V`
pub(crate) fn init_ids() -> Result<()> {
    // todo: implement me
    Ok(())
}

/// `java.io.FileOutputStream.open0(Ljava/lang/String;Z)V`
pub(crate) fn open0(obj_ref: i32, file_name_ref: i32, append: bool) -> Result<()> {
    let file_name = get_utf8_string_by_ref(file_name_ref)?;

    match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(!append)
        .append(append)
        .open(file_name)
    {
        Ok(file) => {
            PlatformFile::set_raw_id(obj_ref, file, FileOutputStream)?;
            Ok(())
        }
        Err(e) => {
            set_pending_file_not_found_exception(file_name_ref, &e.to_string())?;
            Ok(())
        }
    }
}

/// `java.io.FileOutputStream.write(IZ)V`
pub(crate) fn write(obj_ref: i32, byte: i32, _append: bool) -> Result<()> {
    let Some(mut file) = get_by_raw_id(obj_ref, FileOutputStream)? else {
        return Ok(());
    };

    if let Err(e) = file.write_all(&[byte as u8]) {
        set_pending_io_exception(&e.to_string())?;
    }

    Ok(())
}

/// `java.io.FileOutputStream.writeBytes([BIIZ)V`
pub(crate) fn write_bytes(
    obj_ref: i32,
    bytes_ref: i32,
    off: i32,
    len: i32,
    _append: bool,
) -> Result<()> {
    let Some(mut file) = get_by_raw_id(obj_ref, FileOutputStream)? else {
        return Ok(());
    };
    let array = HEAP.get_entire_array(bytes_ref)?;
    let raw = array.get_entire_value();
    let mut vec = Vec::with_capacity(raw.len());
    for i in off..off + len {
        vec.push(raw[i as usize][0] as i8 as u8);
    }

    if let Err(e) = file.write_all(&vec) {
        set_pending_io_exception(&e.to_string())?;
    };

    Ok(())
}
