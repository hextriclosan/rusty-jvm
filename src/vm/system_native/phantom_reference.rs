use crate::vm::error::Result;

/// `java.lang.ref.PhantomReference.clear0()V`
pub(crate) fn clear0(_this: i32) -> Result<()> {
    Ok(()) // todo: this should be implemented with GC
}
