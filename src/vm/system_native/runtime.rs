use crate::vm::error::Result;
use crate::vm::gc::collector::{self, Cause};
use crate::vm::heap::heap::HEAP;

/// `java.lang.Runtime.maxMemory()J`
pub(crate) fn max_memory(_this: i32) -> Result<i64> {
    Ok(i64::try_from(HEAP.memory().0).unwrap_or(i64::MAX))
}

/// `java.lang.Runtime.availableProcessors()I`
pub(crate) fn available_processors(_this: i32) -> Result<i32> {
    let available_parallelism = std::thread::available_parallelism()?;
    Ok(available_parallelism.get() as i32)
}

/// `java.lang.Runtime.totalMemory()J`
pub(crate) fn total_memory(_this: i32) -> Result<i64> {
    max_memory(_this)
}

/// `java.lang.Runtime.freeMemory()J`
pub(crate) fn free_memory(_this: i32) -> Result<i64> {
    let (max, used) = HEAP.memory();
    Ok(i64::try_from(max - used).unwrap_or(i64::MAX))
}

/// An ordinary explicit collection retains soft referents.
pub(crate) fn gc(_this: i32) -> Result<()> {
    collector::collect(Cause::Explicit);
    Ok(())
}
