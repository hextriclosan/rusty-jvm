use crate::vm::error::Result;
use crate::vm::exception::pending_helpers::{
    set_pending_file_not_found_exception, set_pending_io_exception,
};
use crate::vm::heap::heap::HEAP;
use crate::vm::system_native::platform_file::Mode::FileInputStream;
use crate::vm::system_native::platform_file::{get_by_raw_id, length, PlatformFile};
use crate::vm::system_native::string::get_utf8_string_by_ref;
use std::fs::OpenOptions;
use std::io::{Read as IoRead, Seek};

/// `java.io.FileInputStream.initIDs()V`
pub(crate) fn init_ids() -> Result<()> {
    // todo: implement me
    Ok(())
}

/// `java.io.FileInputStream.open0(Ljava/lang/String;)V`
pub(crate) fn open0(obj_ref: i32, file_name_ref: i32) -> Result<()> {
    let file_name = get_utf8_string_by_ref(file_name_ref)?;
    match OpenOptions::new().read(true).open(&file_name) {
        Ok(file) => {
            let metadata = match file.metadata() {
                Ok(m) => m,
                Err(e) => {
                    set_pending_io_exception(&e.to_string())?;
                    return Ok(());
                }
            };
            if metadata.is_dir() {
                set_pending_file_not_found_exception(file_name_ref, "Is a directory")?;
                return Ok(());
            }

            PlatformFile::set_raw_id(obj_ref, file, FileInputStream)
        }
        Err(e) => {
            let message = e.to_string();
            set_pending_file_not_found_exception(file_name_ref, &message)?;
            Ok(())
        }
    }
}

/// `java.io.FileInputStream.length0()J`
pub(crate) fn length0(obj_ref: i32) -> Result<i64> {
    let len = length(obj_ref, FileInputStream)?.unwrap_or(-1);
    Ok(len)
}

/// `java.io.FileInputStream.position0()J`
pub(crate) fn position0(obj_ref: i32) -> Result<i64> {
    let Some(mut file) = get_by_raw_id(obj_ref, FileInputStream)? else {
        return Ok(-1);
    };
    let pos = match file.stream_position() {
        Ok(p) => p,
        Err(e) => {
            set_pending_io_exception(&e.to_string())?;
            return Ok(-1);
        }
    };
    Ok(pos as i64)
}

/// `java.io.FileInputStream.available0()I`
pub(crate) fn available0(obj_ref: i32) -> Result<i32> {
    let Some(mut file) = get_by_raw_id(obj_ref, FileInputStream)? else {
        return Ok(0);
    };
    let current_pos = match file.stream_position() {
        Ok(p) => p,
        Err(e) => {
            set_pending_io_exception(&e.to_string())?;
            return Ok(0);
        }
    };
    let metadata = match file.metadata() {
        Ok(m) => m,
        Err(e) => {
            set_pending_io_exception(&e.to_string())?;
            return Ok(0);
        }
    };
    let file_size = metadata.len();
    if file_size < current_pos {
        return Ok(0);
    }
    let available = file_size - current_pos;
    let available = if available > i32::MAX as u64 {
        i32::MAX
    } else {
        available as i32
    };
    Ok(available)
}

/// `java.io.FileInputStream.readBytes([BII)I`
pub(crate) fn read_bytes(obj_ref: i32, bytes_ref: i32, off: i32, len: i32) -> Result<i32> {
    let Some(mut file) = get_by_raw_id(obj_ref, FileInputStream)? else {
        return Ok(-1);
    };
    if len == 0 {
        return Ok(0);
    }

    let mut buffer = vec![0u8; len as usize];
    let read_bytes = match file.read(&mut buffer) {
        Ok(n) => n,
        Err(e) => {
            set_pending_io_exception(&e.to_string())?;
            return Ok(-1);
        }
    };
    if read_bytes == 0 {
        return Ok(-1);
    }
    let mut array = HEAP.get_entire_array(bytes_ref)?;

    for i in 0..read_bytes {
        array.set_value(off + i as i32, vec![buffer[i] as i8 as i32])?;
    }

    HEAP.set_entire_array(bytes_ref, array)?;

    Ok(read_bytes as i32)
}

/// `java.io.FileInputStream.read()I`
pub(crate) fn read0(obj_ref: i32) -> Result<i32> {
    let Some(mut file) = get_by_raw_id(obj_ref, FileInputStream)? else {
        return Ok(0);
    };
    let mut buffer = [0u8; 1];
    let read_bytes = match file.read(&mut buffer) {
        Ok(n) => n,
        Err(e) => {
            set_pending_io_exception(&e.to_string())?;
            return Ok(0);
        }
    };
    if read_bytes == 0 {
        return Ok(-1);
    }
    Ok(buffer[0] as i32)
}

/// `java.io.FileInputStream.isRegularFile0(Ljava/io/FileDescriptor;)Z`
pub(crate) fn is_regular_file0(_this: i32, fd_ref: i32) -> Result<bool> {
    let Some(file) = PlatformFile::get_by_fd(fd_ref)? else {
        return Ok(false);
    };
    let metadata = match file.metadata() {
        Ok(m) => m,
        Err(e) => {
            set_pending_io_exception(&e.to_string())?;
            return Ok(false);
        }
    };

    Ok(metadata.is_file())
}
