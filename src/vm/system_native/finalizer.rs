use crate::vm::error::Result;

/// `java.lang.ref.Finalizer.isFinalizationEnabled()Z`
pub(crate) fn is_finalization_enabled() -> Result<bool> {
    Ok(false) // todo: this should be implemented with GC
}
