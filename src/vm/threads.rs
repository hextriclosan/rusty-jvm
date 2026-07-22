use crate::vm::heap::heap::HEAP;
use crate::vm::jni::java_thread::JavaThread;
use dashmap::DashMap;
use parking_lot::Mutex;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::LazyLock;
use std::thread::JoinHandle;
use tracing::error;

/// `Thread.State` encodings written to `Thread$FieldHolder.threadStatus`, using the JVMTI bit
/// layout that `jdk.internal.misc.VM.toThreadState` decodes (`getState()` reads it). `ALIVE` (1) is
/// set for every live state so `isAlive`-style checks see it; `NEW` is the field's default of 0.
pub(crate) mod thread_status {
    pub(crate) const NEW: i32 = 0x00; // no bits set (the field's default)
    pub(crate) const RUNNABLE: i32 = 0x05; // ALIVE | RUNNABLE(4)
    pub(crate) const BLOCKED: i32 = 0x401; // ALIVE | BLOCKED_ON_MONITOR_ENTER(1024)
    pub(crate) const WAITING: i32 = 0x11; // ALIVE | WAITING_INDEFINITELY(16)
    pub(crate) const TIMED_WAITING: i32 = 0x21; // ALIVE | WAITING_WITH_TIMEOUT(32)
    pub(crate) const TERMINATED: i32 = 0x02; // TERMINATED(2)
}

/// Monotonic source of non-zero `Thread.eetop` tokens.
///
/// `Thread.isAlive()` is defined as `eetop != 0`, so every started thread is stamped with a unique
/// non-zero value on start and cleared back to `0` on termination. The concrete value is otherwise
/// opaque (HotSpot stores the native thread pointer here); we only need it to be non-zero and
/// distinct per thread.
static NEXT_EETOP: AtomicI64 = AtomicI64::new(1);

/// Join handles of live **non-daemon** platform threads.
///
/// Per the JVM spec the VM keeps running until the last non-daemon thread dies, so the main thread
/// awaits all of these (via [`join_all_non_daemon`]) before running shutdown hooks. Daemon threads
/// are intentionally *not* tracked: they are abandoned at process exit, matching Java semantics.
static NON_DAEMON_THREADS: LazyLock<Mutex<Vec<JoinHandle<()>>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));

/// Maps each live thread's `java.lang.Thread` reference to its OS thread handle, so `Unsafe.unpark`
/// can wake a specific thread. Rust's `std::thread` park/unpark implement exactly the permit-based
/// semantics of `LockSupport` (unpark grants a permit that a later park consumes), so the two map
/// directly. Every thread registers itself once it is attached (identity is set) and unregisters on
/// termination.
static PARKERS: LazyLock<DashMap<i32, std::thread::Thread>> = LazyLock::new(DashMap::new);

/// Returns the next non-zero `eetop` token for a starting thread.
pub(crate) fn next_eetop() -> i64 {
    NEXT_EETOP.fetch_add(1, Ordering::Relaxed)
}

/// Writes `status` (a [`thread_status`] value) into `thread_ref`'s `Thread$FieldHolder.threadStatus`,
/// which is what `Thread.getState()` reports. Best-effort: any failure to reach the field is ignored
/// (thread state is observational and must never break the operation that updates it).
pub(crate) fn set_thread_status(thread_ref: i32, status: i32) {
    if let Ok(holder) = HEAP.get_object_field_value(thread_ref, "java/lang/Thread", "holder") {
        if let Some(&holder_ref) = holder.first() {
            let _ = HEAP.set_object_field_value(
                holder_ref,
                "java/lang/Thread$FieldHolder",
                "threadStatus",
                vec![status],
            );
        }
    }
}

/// [`set_thread_status`] for the calling thread (e.g. entering `wait`/`sleep`/`blocked`).
pub(crate) fn set_current_thread_status(status: i32) {
    if let Some(thread_ref) = JavaThread::current_thread() {
        set_thread_status(thread_ref, status);
    }
}

/// Associates a `java.lang.Thread` reference with its OS thread handle so it can be unparked by
/// [`unpark`]. Registered *eagerly by the spawning thread* (from `JoinHandle::thread()`) before the
/// child starts running, so an `unpark` that races ahead of the child's first `park` is never lost.
pub(crate) fn register_parker(thread_ref: i32, handle: std::thread::Thread) {
    PARKERS.insert(thread_ref, handle);
}

/// Drops the parker mapping for a terminated thread.
pub(crate) fn unregister_parker(thread_ref: i32) {
    PARKERS.remove(&thread_ref);
}

/// Grants a parking permit to the thread identified by `thread_ref` (waking it if parked), or does
/// nothing if that thread is not currently registered. Backs `Unsafe.unpark` / `LockSupport.unpark`.
pub(crate) fn unpark(thread_ref: i32) {
    if let Some(handle) = PARKERS.get(&thread_ref) {
        handle.unpark();
    }
}

/// Records a non-daemon thread's join handle so the VM waits for it before exiting.
pub(crate) fn register_non_daemon(handle: JoinHandle<()>) {
    NON_DAEMON_THREADS.lock().push(handle);
}

/// Blocks until every non-daemon thread has terminated.
///
/// Called once from the main thread after `main` returns. Loops until the registry drains, so that
/// threads spawned *by* other non-daemon threads while we are joining are awaited too. A thread that
/// terminated by panicking is logged and otherwise ignored (a panic is already contained by the OS
/// thread boundary and cannot poison shared state — locks here are `parking_lot`).
pub(crate) fn join_all_non_daemon() {
    loop {
        let handles: Vec<JoinHandle<()>> = std::mem::take(&mut *NON_DAEMON_THREADS.lock());
        if handles.is_empty() {
            break;
        }
        for handle in handles {
            if let Err(panic) = handle.join() {
                error!("a non-daemon thread terminated by panicking: {panic:?}");
            }
        }
    }
}
