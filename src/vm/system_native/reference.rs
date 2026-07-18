use crate::vm::error::Result;

/// `java.lang.ref.Reference.clear0()V`
pub(crate) fn clear0(_this: i32) -> Result<()> {
    Ok(()) // todo: this should be implemented with GC
}

/// `java.lang.ref.Reference.refersTo0(Ljava/lang/Object;)Z`
pub(crate) fn refers_to0(_this: i32, _o: i32) -> Result<bool> {
    Ok(false) // todo: this should be implemented with GC
}
