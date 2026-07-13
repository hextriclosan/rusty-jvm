use crate::vm::error::Result;
use crate::vm::method_area::method_area::with_method_area;

/// `java/lang/Thread.registerNatives()V`
pub(crate) fn register_natives() -> Result<()> {
    Ok(())
}

/// `java/lang/Thread.currentThread()Ljava/lang/Thread;`
pub(crate) fn current_thread() -> Result<i32> {
    let thread_id = with_method_area(|method_area| {
        method_area.system_thread_id() // since we do not spawn threads, primordial system thread is returned here
    })?;
    Ok(thread_id)
}

/// `java/lang/Thread.currentCarrierThread()Ljava/lang/Thread;`
pub(crate) fn current_carrier_thread() -> Result<i32> {
    current_thread() //todo: use current carrier thread here (no matter what it is)
}

/// `java/lang/Thread.holdsLock(Ljava/lang/Object;)Z`
pub(crate) fn holds_lock(_object_ref: i32) -> Result<bool> {
    Ok(true) // todo: implement me
}

static mut NEXT_TID_OFFSET: i64 = 3; // todo: should have volatile semantics
/// `java/lang/Thread.getNextThreadIdOffset()J`
pub(crate) fn get_next_threadid_offset() -> Result<i64> {
    Ok(&raw const NEXT_TID_OFFSET as i64) // todo: `NEXT_TID_OFFSET` should have volatile semantics
}

/// `java/lang/Thread.setPriority0(I)V`
pub(crate) fn set_priority0(_this: i32, _new_priority: i32) -> Result<()> {
    Ok(()) // todo: implement me
}

/// `java/lang/Thread.start0()V`
pub(crate) fn start0() -> Result<()> {
    Ok(()) // todo: implement me
}
