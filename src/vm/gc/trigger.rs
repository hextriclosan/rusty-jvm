//! Triggers never run with a reservation, admission mutex or heap shard held.
use super::collector::{self, Cause};
use super::get;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

static ENABLED: AtomicBool = AtomicBool::new(false);
static ALLOCATIONS: AtomicU64 = AtomicU64::new(0);
static THRESHOLD: AtomicU64 = AtomicU64::new(65_536);

pub(crate) fn enable() {
    if let Some(config) = get() {
        THRESHOLD.store(config.gc_alloc_threshold, Ordering::Relaxed);
    }
    ENABLED.store(true, Ordering::Release);
}

pub(crate) fn enabled() -> bool {
    ENABLED.load(Ordering::Acquire)
}

pub(crate) fn allocated() {
    ALLOCATIONS
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |count| {
            Some(count.saturating_add(1))
        })
        .ok();
}

pub(crate) fn before_allocation() {
    if enabled() && ALLOCATIONS.load(Ordering::Relaxed) >= THRESHOLD.load(Ordering::Relaxed) {
        // A skipped cycle also consumes this threshold attempt. Do not turn a
        // persistently unavailable snapshot into a collection on every allocation.
        ALLOCATIONS.store(0, Ordering::Relaxed);
        collector::collect(Cause::Threshold);
    }
}

pub(crate) fn at_safepoint() {
    if enabled() && get().is_some_and(|config| config.stress_at_safepoints) {
        collector::collect(Cause::Stress);
    }
}

pub(crate) fn completed(live: usize) {
    ALLOCATIONS.store(0, Ordering::Relaxed);
    if let Some(config) = get() {
        if !config.fixed_threshold {
            THRESHOLD.store(
                config
                    .gc_alloc_threshold
                    .max(u64::try_from(live).unwrap_or(u64::MAX)),
                Ordering::Relaxed,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gc_collect_default_threshold_adapts_to_live_objects() {
        crate::vm::gc::install(&crate::Arguments::default()).unwrap();
        enable();
        allocated();
        completed(131_072);
        assert_eq!(THRESHOLD.load(Ordering::Relaxed), 131_072);
        assert_eq!(ALLOCATIONS.load(Ordering::Relaxed), 0);
        completed(10);
        assert_eq!(THRESHOLD.load(Ordering::Relaxed), 65_536);
    }

    #[test]
    fn gc_collect_explicit_threshold_remains_fixed_after_collection() {
        let arguments = crate::Arguments::new(
            String::new(),
            vec![],
            vec![],
            vec![],
            Default::default(),
            vec![],
            vec!["-XX:GCAllocThreshold=19".into()],
            None,
            false,
        );
        crate::vm::gc::install(&arguments).unwrap();
        enable();
        completed(131_072);
        assert_eq!(THRESHOLD.load(Ordering::Relaxed), 19);
    }
}
