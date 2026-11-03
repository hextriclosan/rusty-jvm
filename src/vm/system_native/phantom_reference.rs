use crate::vm::error::Result;

/// `java.lang.ref.PhantomReference.clear0()V`
pub(crate) fn clear0(this: i32) -> Result<()> {
    super::reference::clear0(this)
}

pub(crate) fn refers_to0(this: i32, object: i32) -> Result<bool> {
    super::reference::refers_to0(this, object)
}
