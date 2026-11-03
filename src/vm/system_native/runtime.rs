use crate::vm::error::Result;
use once_cell::sync::Lazy;
use sysinfo::{MemoryRefreshKind, RefreshKind, System};
use tracing::warn;

/// The upper heap bound HotSpot picks by default (`-XX:MaxRAMPercentage=25`): a quarter of the
/// physical memory, or of the cgroup memory limit on Linux if that is lower.
static MAX_MEMORY: Lazy<i64> = Lazy::new(|| {
    let refresh_kind = RefreshKind::nothing().with_memory(MemoryRefreshKind::nothing().with_ram());
    let system = match System::new_with_specifics(refresh_kind) {
        Ok(system) => system,
        Err(e) => {
            warn!("Failed to get the physical memory size: {e}");
            return i64::MAX;
        }
    };
    let physical_memory = match system.cgroup_limits() {
        Some(limits) => system.total_memory().min(limits.total_memory),
        None => system.total_memory(),
    };
    i64::try_from(physical_memory / 4).unwrap_or(i64::MAX)
});

/// `java.lang.Runtime.maxMemory()J`
pub(crate) fn max_memory(_this: i32) -> Result<i64> {
    Ok(*MAX_MEMORY)
}

/// `java.lang.Runtime.availableProcessors()I`
pub(crate) fn available_processors(_this: i32) -> Result<i32> {
    let available_parallelism = std::thread::available_parallelism()?;
    Ok(available_parallelism.get() as i32)
}

/// `java.lang.Runtime.totalMemory()J`
pub(crate) fn total_memory(_this: i32) -> Result<i64> {
    // Not above maxMemory(), as callers rely on that
    Ok(*MAX_MEMORY) // todo: implement me with GC
}

/// `java.lang.Runtime.freeMemory()J`
pub(crate) fn free_memory(_this: i32) -> Result<i64> {
    // Not above totalMemory(), as callers rely on that
    Ok(*MAX_MEMORY) // todo: implement me with GC
}
