use crate::vm::error::Result;
use crate::vm::helper::i64_to_vec;
use crate::vm::system_native::string::get_utf8_string_by_ref;
use nix::libc::{pathconf, _PC_NAME_MAX};
use std::ffi::CString;

pub(crate) fn get_name_max0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let _filesystem_impl_ref = args[0];
    let string_ref = args[1];

    let name_max = get_name_max0(string_ref)?;
    Ok(i64_to_vec(name_max))
}
fn get_name_max0(string_ref: i32) -> Result<i64> {
    const NAME_MAX_FALLBACK: i64 = 255;

    let string_content = get_utf8_string_by_ref(string_ref)?;
    let c_string = CString::new(&*string_content)?;
    let name_max = unsafe { pathconf(c_string.as_ptr(), _PC_NAME_MAX) };

    Ok(if name_max != -1 {
        name_max
    } else {
        NAME_MAX_FALLBACK
    })
}
