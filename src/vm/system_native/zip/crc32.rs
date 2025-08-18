use crate::vm::error::{Error, Result};
use crate::vm::heap::heap::with_heap_write_lock;
use crc32fast::Hasher;

pub(crate) fn java_util_zip_crc32_updatebytes0_wrp(
    args: &[i32],
) -> crate::vm::error::Result<Vec<i32>> {
    let crc = args[0];
    let byte_array_ref = args[1];
    let off = args[2];
    let len = args[3];

    let updated_crc = updatebytes0(crc, byte_array_ref, off, len)?;

    Ok(vec![updated_crc])
}

fn updatebytes0(crc: i32, byte_array_ref: i32, off: i32, len: i32) -> Result<i32> {
    let mut hasher = Hasher::new_with_initial(crc as u32);

    with_heap_write_lock(|h| {
        let byte_array = h.get_entire_raw_data(byte_array_ref)?;
        let byte_slice = &byte_array[off as usize..(off + len) as usize];
        hasher.update(byte_slice);
        Ok::<(), Error>(())
    })?;

    Ok(hasher.finalize() as i32)
}
