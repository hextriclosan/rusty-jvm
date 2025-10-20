use crate::vm::error::Result;
use crate::vm::helper::i64_to_vec;
use crate::vm::method_area::method_area::with_method_area;

pub(crate) fn current_thread_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    let result = current_thread()?;

    Ok(vec![result])
}
fn current_thread() -> Result<i32> {
    let thread_id = with_method_area(|method_area| {
        method_area.system_thread_id() // since we do not spawn threads, primordial system thread is returned here
    })?;
    Ok(thread_id)
}

static mut NEXT_TID_OFFSET: i64 = 3; // todo: should have volatile semantics
pub(crate) fn get_next_threadid_offset_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    let offset = get_next_threadid_offset()?;
    Ok(i64_to_vec(offset))
}
fn get_next_threadid_offset() -> Result<i64> {
    Ok(&raw const NEXT_TID_OFFSET as i64)
}
