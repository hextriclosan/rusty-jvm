use crate::vm::error::Result;
use crate::vm::helper::i64_to_vec;

static mut NEXT_TID_OFFSET: i64 = 3; // todo: should have volatile semantics
pub(crate) fn get_next_threadid_offset_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    let offset = get_next_threadid_offset()?;
    Ok(i64_to_vec(offset))
}
fn get_next_threadid_offset() -> Result<i64> {
    Ok(&raw const NEXT_TID_OFFSET as i64)
}
