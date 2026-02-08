use crate::vm::error::Result;
use crate::vm::heap::heap::HEAP;
use rand::rngs::SysRng;
use rand::TryRng;

pub(crate) fn native_generate_seed_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let byte_array_ref = args[0];
    let ret = native_generate_seed(byte_array_ref)?;
    Ok(vec![if ret { 1 } else { 0 }])
}

fn native_generate_seed(byte_array_ref: i32) -> Result<bool> {
    let mut raw_data = HEAP.get_entire_raw_data_mut(byte_array_ref)?;
    SysRng.try_fill_bytes(raw_data.as_mut_slice())?;
    Ok(true)
}
