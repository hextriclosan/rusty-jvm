use crate::vm::error::Result;

/// `java.lang.ref.Reference.clear0()V`
pub(crate) fn clear0(_this: i32) -> Result<()> {
    Ok(()) // todo: this should be implemented with GC
}

/// `java.lang.ref.Reference.refersTo0(Ljava/lang/Object;)Z`
pub(crate) fn refers_to0(_this: i32, _o: i32) -> Result<bool> {
    Ok(false) // todo: this should be implemented with GC
}

/// `java.lang.ref.Reference.waitForReferencePendingList()V`
///
/// The Reference Handler daemon calls this to block until the GC enqueues pending references. With
/// no GC the list is never populated, so the daemon simply parks here forever — matching the
/// "block until a reference is pending" contract. It is a daemon, so this never delays VM shutdown.
pub(crate) fn wait_for_reference_pending_list() -> Result<()> {
    loop {
        std::thread::park();
    }
}
