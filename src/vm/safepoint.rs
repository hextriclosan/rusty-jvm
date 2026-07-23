//! Cooperative safepoints.
//!
//! To read another thread's call stack safely, that thread must be paused at a consistent point —
//! not mid-instruction mutating its frames. The interpreter loop [`poll`](Safepoint::poll)s a cheap
//! per-thread flag every iteration; when a dumper raises it, the target thread reaches the poll,
//! **collects its own stack** (safe: it walks its own thread-local frames), hands the result to the
//! dumper, and parks until released. Only owned frame data crosses threads — never a live pointer
//! into another thread's frames.
//!
//! This is deliberately minimal: a target that is *not* running the interpreter (blocked in a
//! native such as `Object.wait`/`park`) never reaches the poll, so a dump of it times out and
//! yields nothing. Pausing blocked threads for inspection, and stop-the-world use for a future GC,
//! build on the same registry and flag.

use crate::vm::stack::stack_frames_util::{StackElement, StackFramesUtil};
use dashmap::DashMap;
use parking_lot::{Condvar, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, LazyLock};
use std::time::{Duration, Instant};

/// Cap on frames captured per dump — a guard against a pathologically deep stack.
const MAX_DUMP_DEPTH: usize = 1024;
/// How long a dumper waits for the target to reach the poll before giving up (e.g. it is blocked in
/// a native and will not reach a safepoint).
const DUMP_TIMEOUT: Duration = Duration::from_millis(2000);

/// Per-thread safepoint state. Held both in the thread's `JavaThread` (for polling) and in
/// [`REGISTRY`] keyed by the thread's `java.lang.Thread` reference (for targeting by a dumper).
pub(crate) struct Safepoint {
    requested: AtomicBool,
    rendezvous: Mutex<Rendezvous>,
    cv: Condvar,
}

#[derive(Default)]
struct Rendezvous {
    /// `Some` once the target has reached the poll and published its stack (empty on a collection
    /// error); consumed by the dumper.
    collected: Option<Vec<StackElement>>,
    /// Set by the dumper to let the parked target resume.
    release: bool,
}

impl Safepoint {
    pub(crate) fn new() -> Arc<Self> {
        Arc::new(Self {
            requested: AtomicBool::new(false),
            rendezvous: Mutex::new(Rendezvous::default()),
            cv: Condvar::new(),
        })
    }

    /// Cheap check run by the interpreter; no-op unless a dump was requested. `Relaxed` is enough:
    /// the flag is only a wake-up hint, and the actual stack handoff is synchronized by the mutex —
    /// a slightly delayed observation just means the dumper waits a touch longer.
    #[inline]
    pub(crate) fn poll(&self) {
        if self.requested.load(Ordering::Relaxed) {
            self.reach();
        }
    }

    /// Slow path: publish our own stack and park until the dumper releases us.
    #[cold]
    fn reach(&self) {
        let stack = StackFramesUtil::collect_current_stack(MAX_DUMP_DEPTH).unwrap_or_default();
        let mut r = self.rendezvous.lock();
        r.collected = Some(stack);
        self.cv.notify_all();
        while !r.release {
            self.cv.wait(&mut r);
        }
        r.collected = None;
        r.release = false;
        self.requested.store(false, Ordering::Relaxed);
    }
}

/// Live threads' safepoints, keyed by `java.lang.Thread` reference.
static REGISTRY: LazyLock<DashMap<i32, Arc<Safepoint>>> = LazyLock::new(DashMap::new);
/// Serializes dumpers so at most one safepoint request is outstanding at a time.
static DUMP_GATE: Mutex<()> = Mutex::new(());

/// Publishes a thread's safepoint so it can be targeted by [`dump_stack`]. Called when the thread's
/// identity is bound.
pub(crate) fn register(thread_ref: i32, safepoint: Arc<Safepoint>) {
    REGISTRY.insert(thread_ref, safepoint);
}

/// Removes a terminated thread's safepoint.
pub(crate) fn unregister(thread_ref: i32) {
    REGISTRY.remove(&thread_ref);
}

/// Drives `thread_ref` to a safepoint and returns its stack (newest frame first). Returns `None` if
/// the thread is not registered or does not reach a safepoint within [`DUMP_TIMEOUT`] (e.g. it is
/// blocked in a native and never polls).
pub(crate) fn dump_stack(thread_ref: i32) -> Option<Vec<StackElement>> {
    let safepoint = REGISTRY.get(&thread_ref).map(|e| Arc::clone(e.value()))?;
    let _gate = DUMP_GATE.lock();

    let mut r = safepoint.rendezvous.lock();
    // Start each dump from a clean rendezvous. A previous request that timed out (the target was
    // blocked in a native and never reached the poll) leaves `release == true`; if the target later
    // reaches the poll it would then see `release` already set, immediately clear `collected`, and
    // skip parking — so this dumper would miss the published stack and time out as well. Resetting
    // here (before raising `requested`) guarantees the target parks until *this* dumper releases it.
    r.release = false;
    r.collected = None;
    safepoint.requested.store(true, Ordering::Relaxed);

    let start = Instant::now();
    while r.collected.is_none() {
        let elapsed = start.elapsed();
        if elapsed >= DUMP_TIMEOUT {
            break;
        }
        safepoint.cv.wait_for(&mut r, DUMP_TIMEOUT - elapsed);
    }
    let result = r.collected.take();

    // Release the target if it parked; also clear the flag so a target that never arrived (timeout)
    // does not park on a stale request later.
    r.release = true;
    safepoint.requested.store(false, Ordering::Relaxed);
    safepoint.cv.notify_all();
    result
}
