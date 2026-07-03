use crate::vm::error::Result;
use crate::vm::heap::heap::HEAP;
use crate::vm::jni::set_pending_internal_error;
use crc32fast::Hasher;
use jni_sys::{jarray, jclass, jint, JNIEnv};

pub(crate) extern "system" fn java_util_zip_crc32_updatebytes0_wrp(
    _env: *mut JNIEnv,
    _class: jclass,
    crc: jint,
    b: jarray,
    off: jint,
    len: jint,
) -> jint {
    let crc = crc as i32;
    let byte_array_ref = b as i32;
    let off = off as i32;
    let len = len as i32;

    match updatebytes0(crc, byte_array_ref, off, len) {
        Ok(updated_crc) => updated_crc as jint,
        Err(e) => {
            set_pending_internal_error(&e.to_string());
            0
        }
    }
}

fn updatebytes0(crc: i32, byte_array_ref: i32, off: i32, len: i32) -> Result<i32> {
    let mut hasher = Hasher::new_with_initial(crc as u32);

    let byte_array = HEAP.get_entire_raw_data(byte_array_ref)?;
    let byte_slice = &byte_array[off as usize..(off + len) as usize];
    hasher.update(byte_slice);

    Ok(hasher.finalize() as i32)
}
