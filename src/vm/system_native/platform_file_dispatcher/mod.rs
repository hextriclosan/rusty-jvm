use crate::vm::error::Result;
use crate::vm::helper::i64_to_vec;

#[cfg(windows)]
pub mod windows_file_dispatcher;

#[cfg(unix)]
pub mod unix_file_dispatcher;

pub fn allocation_granularity0_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(i64_to_vec(allocation_granularity0()))
}
fn allocation_granularity0() -> i64 {
    page_size::get_granularity() as i64
}
