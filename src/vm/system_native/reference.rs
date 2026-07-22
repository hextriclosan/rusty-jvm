use crate::vm::error::Result;
use crate::vm::heap::heap::HEAP;

const REFERENCE: &str = "java/lang/ref/Reference";
const REFERENT: &str = "referent";

/// `java.lang.ref.Reference.clear0()V`
///
/// Breaks the reference by nulling its `referent`. This is the explicit `Reference.clear()` path
/// (also used by `ThreadLocalMap.remove`/`WeakReference.clear`); it needs no GC — after it, `get()`
/// yields null and `refersTo0` no longer matches the former referent (it then matches only a null
/// argument, as `refersTo(null)` on a cleared reference does).
pub(crate) fn clear0(this: i32) -> Result<()> {
    HEAP.set_object_field_value(this, REFERENCE, REFERENT, vec![0])
}

/// `java.lang.ref.Reference.refersTo0(Ljava/lang/Object;)Z`
///
/// True when this reference still points at `o`. Reading the `referent` field directly is correct
/// without GC (the referent only changes via `clear`); this is what `ThreadLocalMap.getEntry` uses
/// to match a key, so a correct answer here is what makes `ThreadLocal` work.
pub(crate) fn refers_to0(this: i32, o: i32) -> Result<bool> {
    let referent = HEAP.get_object_field_value(this, REFERENCE, REFERENT)?[0];
    Ok(referent == o)
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
