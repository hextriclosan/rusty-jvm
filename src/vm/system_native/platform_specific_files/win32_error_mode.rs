use crate::vm::error::Result;
use crate::vm::helper::{i32toi64, i64_to_vec};
use winapi::shared::minwindef::UINT;
use winapi::um::errhandlingapi::SetErrorMode;

pub fn set_error_mode_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let mode = i32toi64(args[1], args[0]);
    let result = set_error_mode(mode);

    Ok(i64_to_vec(result))
}
fn set_error_mode(mode: i64) -> i64 {
    unsafe { SetErrorMode(mode as UINT) as i64 }
}
