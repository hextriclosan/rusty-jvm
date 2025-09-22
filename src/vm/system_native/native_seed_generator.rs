use crate::vm::error::Result;
use crate::vm::heap::heap::with_heap_write_lock;
use rand::rand_core::OsRng;
use rand::TryRngCore;

pub(crate) fn native_generate_seed_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let byte_array_ref = args[0];
    let ret = native_generate_seed(byte_array_ref)?;
    Ok(vec![if ret { 1 } else { 0 }])
}

fn native_generate_seed(byte_array_ref: i32) -> Result<bool> {
    with_heap_write_lock(|h| {
        let mut raw_data = h.get_entire_raw_data_mut(byte_array_ref)?;
        OsRng.try_fill_bytes(raw_data.as_mut_slice())?;
        Ok(true)
    })
}
