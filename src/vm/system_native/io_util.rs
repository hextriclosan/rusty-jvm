use crate::vm::error::Result;
use crate::vm::helper::i64_to_vec;

pub(crate) fn iov_max_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    let max = iov_max()?;
    Ok(vec![max])
}
#[cfg(windows)]
fn iov_max() -> Result<i32> {
    Ok(16)
}
#[cfg(unix)]
fn iov_max() -> Result<i32> {
    use nix::libc::{sysconf, _SC_IOV_MAX};

    let result = unsafe { sysconf(_SC_IOV_MAX) };
    let max = if result == -1 { 16 } else { result as i32 };
    Ok(max)
}

pub(crate) fn writev_max_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    let max = writev_max();
    Ok(i64_to_vec(max))
}
#[cfg(windows)]
fn writev_max() -> i64 {
    i64::MAX
}
#[cfg(unix)]
fn writev_max() -> i64 {
    i32::MAX as i64
}
