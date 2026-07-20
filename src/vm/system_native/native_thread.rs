use crate::vm::error::Result;

/// `sun.nio.ch.NativeThread.init()V`
#[cfg(unix)]
pub(crate) fn init() -> Result<()> {
    Ok(()) // todo: implement me
}

/// `sun.nio.ch.NativeThread.current0()J`
pub(crate) fn current0() -> Result<i64> {
    Ok(0) // todo: implement this (by 0 we say that the platform can not signal native threads)
}
