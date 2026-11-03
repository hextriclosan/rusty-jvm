use crate::vm::error::Result;
use crate::vm::gc::pending::PENDING;
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

/// `java.lang.ref.Reference.get0()Ljava/lang/Object;` (JDK 26+)
///
/// Managed access excludes collection while reading this field.
pub(crate) fn get0(this: i32) -> Result<i32> {
    Ok(HEAP.get_object_field_value(this, REFERENCE, REFERENT)?[0])
}

/// `java.lang.ref.Reference.refersTo0(Ljava/lang/Object;)Z`
///
/// True when this reference still points at `o`, including null after clearing.
pub(crate) fn refers_to0(this: i32, o: i32) -> Result<bool> {
    let referent = HEAP.get_object_field_value(this, REFERENCE, REFERENT)?[0];
    Ok(referent == o)
}

/// `java.lang.ref.Reference.waitForReferencePendingList()V`
///
/// Return Running when a batch is available; the JDK then takes and drains it.
pub(crate) fn wait_for_reference_pending_list() -> Result<()> {
    PENDING.wait();
    Ok(())
}

pub(crate) fn get_and_clear_reference_pending_list() -> Result<i32> {
    Ok(PENDING.take())
}

pub(crate) fn has_reference_pending_list() -> Result<bool> {
    Ok(PENDING.has_pending())
}
