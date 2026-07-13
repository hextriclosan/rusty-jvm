use crate::vm::error::Result;
use crate::vm::heap::heap::HEAP;
use rand::rngs::SysRng;
use rand::TryRng;

/// `sun.security.provider.NativeSeedGenerator.nativeGenerateSeed([B)Z`
pub(crate) fn native_generate_seed(byte_array_ref: i32) -> Result<bool> {
    let mut raw_data = HEAP.get_entire_raw_data_mut(byte_array_ref)?;
    SysRng.try_fill_bytes(raw_data.as_mut_slice())?;
    Ok(true)
}
