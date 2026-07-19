use crate::vm::error::Result;

/// `jdk.internal.vm.ContinuationSupport.isSupported0()Z`
pub(crate) fn is_supported0() -> Result<bool> {
    Ok(false) // We do not support Loom continuations (yet)
}
