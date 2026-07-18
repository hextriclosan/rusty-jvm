use crate::vm::error::Result;

/// `sun.nio.ch.IOUtil.initIDs()V`
pub(crate) fn init_ids() -> Result<()> {
    // todo: implement me
    Ok(())
}

/// `sun.nio.ch.IOUtil.iovMax()I`
pub(crate) fn iov_max() -> Result<i32> {
    #[cfg(unix)]
    {
        use nix::libc::{sysconf, _SC_IOV_MAX};

        let result = unsafe { sysconf(_SC_IOV_MAX) };
        let max = if result == -1 { 16 } else { result as i32 };
        Ok(max)
    }
    #[cfg(windows)]
    Ok(16)
}

/// `sun.nio.ch.IOUtil.writevMax()J`
pub(crate) fn writev_max() -> Result<i64> {
    #[cfg(unix)]
    return Ok(i32::MAX as i64);
    #[cfg(windows)]
    return Ok(i64::MAX);
}
