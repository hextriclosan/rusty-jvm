use crate::vm::error::{Error, Result};
use crate::vm::exception::pending_helpers::{
    set_pending_interrupted_exception, set_pending_null_pointer_exception,
};
use crate::vm::execution_engine::executor::Executor;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::i64_to_vec;
use crate::vm::jni::java_thread::JavaThread;
use crate::vm::method_area::lookup;
use crate::vm::{monitor, threads};
use std::time::{Duration, Instant};
use tracing::error;

/// `java/lang/Thread.registerNatives()V`
pub(crate) fn register_natives() -> Result<()> {
    Ok(())
}

/// `java/lang/Thread.currentThread()Ljava/lang/Thread;`
pub(crate) fn current_thread() -> Result<i32> {
    JavaThread::current_thread().ok_or_else(|| {
        Error::new_execution("Thread.currentThread() called before the thread was attached")
    })
}

/// `java/lang/Thread.currentCarrierThread()Ljava/lang/Thread;`
pub(crate) fn current_carrier_thread() -> Result<i32> {
    current_thread() //todo: use current carrier thread here (no matter what it is)
}

/// `java/lang/Thread.holdsLock(Ljava/lang/Object;)Z`
pub(crate) fn holds_lock(object_ref: i32) -> Result<bool> {
    if object_ref == 0 {
        set_pending_null_pointer_exception()?;
        return Ok(false);
    }

    Ok(crate::vm::monitor::holds_lock(object_ref))
}

static mut NEXT_TID_OFFSET: i64 = 3; // todo: should have volatile semantics
/// `java/lang/Thread.getNextThreadIdOffset()J`
pub(crate) fn get_next_threadid_offset() -> Result<i64> {
    Ok(&raw const NEXT_TID_OFFSET as i64) // todo: `NEXT_TID_OFFSET` should have volatile semantics
}

/// `java/lang/Thread.setPriority0(I)V`
pub(crate) fn set_priority0(_this: i32, _new_priority: i32) -> Result<()> {
    Ok(()) // priorities are advisory; the OS scheduler decides. No-op.
}

/// `java/lang/Thread.sleepNanos0(J)V`
///
/// Sleeps the current thread for `nanos`, interruptibly: `Thread.interrupt` unparks it, and on wake
/// it observes the interrupt flag and throws `InterruptedException`. Backed by `park_timeout`, whose
/// permit also makes an interrupt that arrives just before the park take effect immediately.
pub(crate) fn sleep_nanos0(nanos: i64) -> Result<()> {
    if nanos <= 0 {
        return Ok(());
    }
    // Track elapsed against a start instant rather than computing `now + duration`: the latter can
    // panic on `Instant` overflow for the very large `nanos` a Java `long` permits (~292 years).
    let total = Duration::from_nanos(nanos as u64);
    let start = Instant::now();
    threads::set_current_thread_status(threads::thread_status::TIMED_WAITING);
    let result = loop {
        if monitor::take_current_interrupt() {
            break set_pending_interrupted_exception();
        }
        let remaining = total.saturating_sub(start.elapsed());
        if remaining.is_zero() {
            break Ok(());
        }
        std::thread::park_timeout(remaining);
    };
    threads::set_current_thread_status(threads::thread_status::RUNNABLE);
    result
}

/// `java/lang/Thread.yield0()V`
pub(crate) fn yield0() -> Result<()> {
    std::thread::yield_now();
    Ok(())
}

/// `java/lang/Thread.interrupt0()V`
///
/// `Thread.interrupt()` has already set the target's `interrupted` field; this only wakes the target
/// from a blocking operation so it observes the flag: `unpark` covers `LockSupport.park` and
/// `Thread.sleep`, and [`monitor::interrupt_waiter`] covers `Object.wait`.
pub(crate) fn interrupt0(this: i32) -> Result<()> {
    threads::unpark(this);
    monitor::interrupt_waiter(this);
    Ok(())
}

/// `java/lang/Thread.clearInterruptEvent()V`
///
/// Clears any VM-level pending interrupt event. We keep no such event (the `interrupted` field is
/// the sole source of truth), so this is a no-op.
pub(crate) fn clear_interrupt_event() -> Result<()> {
    Ok(())
}

/// `java/lang/Thread.ensureMaterializedForStackWalk(Ljava/lang/Object;)V`
///
/// A reachability/materialization barrier the VM treats as a no-op; called at the top of every
/// `Thread.run()` to keep the task object live for stack walks.
pub(crate) fn ensure_materialized_for_stack_walk(_o: i32) -> Result<()> {
    Ok(())
}

/// `java/lang/Thread.start0()V`
///
/// Spawns a real OS thread for `this` and returns to the Java `Thread.start()` caller immediately.
/// The thread is marked alive (`eetop != 0`) *before* spawning so `isAlive()` is true as soon as
/// `start()` returns, mirroring HotSpot.
pub(crate) fn start0(this: i32) -> Result<()> {
    // Daemon status is fixed once a thread is started; read it now, on the starting thread.
    let is_daemon =
        Executor::invoke_non_static_method("java/lang/Thread", "isDaemon:()Z", this, &[])?
            .first()
            .is_some_and(|&v| v != 0);

    set_eetop(this, threads::next_eetop())?;
    // Mark RUNNABLE synchronously, before spawning, so getState() is correct the moment start()
    // returns and a second start() is rejected (Thread.start checks threadStatus != 0).
    threads::set_thread_status(this, threads::thread_status::RUNNABLE);

    let thread_ref = this;
    let handle = std::thread::Builder::new()
        .spawn(move || run_thread(thread_ref))
        .map_err(|e| {
            // Roll back the "started" markers so a thread that never launched is left back in NEW:
            // eetop cleared so isAlive() is false, and threadStatus reset so getState() reads NEW and
            // Thread.start() can be retried (it rejects threadStatus != 0).
            let _ = set_eetop(this, 0);
            threads::set_thread_status(this, threads::thread_status::NEW);
            Error::new_execution(&format!("failed to spawn OS thread: {e}"))
        })?;

    // Register the parker eagerly, from the parent, before the child can be unparked (race-free).
    threads::register_parker(this, handle.thread().clone());

    if !is_daemon {
        threads::register_non_daemon(handle);
    }
    Ok(())
}

/// Body executed on the freshly-spawned OS thread: bind its identity, run `Thread.run()`
/// (virtually), dispatch any uncaught exception, then mark the thread terminated.
fn run_thread(this: i32) {
    JavaThread::set_current_thread(this);

    match invoke_run(this) {
        Ok(_) => {}
        Err(e) if e.is_uncaught_exception() => {
            if let Some(throwable_ref) = e.throwable_ref() {
                if let Err(dispatch_err) =
                    crate::vm::invoke_uncaught_exception_handler(throwable_ref)
                {
                    error!("failed to dispatch uncaught exception on thread: {dispatch_err}");
                }
            }
        }
        Err(e) => error!("thread terminated with internal error: {e}"),
    }

    // Java-level cleanup while the thread is still nominally alive, then clear `eetop` so that
    // `isAlive()`/`join()` observe termination.
    if let Err(e) = Executor::invoke_non_static_method("java/lang/Thread", "exit:()V", this, &[]) {
        error!("Thread.exit() failed: {e}");
    }
    if let Err(e) = set_eetop(this, 0) {
        error!("failed to clear eetop on thread exit: {e}");
    }
    threads::set_thread_status(this, threads::thread_status::TERMINATED);
    // Wake any joiners: Thread.join sits in `synchronized(thread) { while (isAlive()) wait(); }`,
    // and isAlive() is now false (eetop cleared above), so a notify lets them observe termination.
    crate::vm::monitor::wake_all_on_exit(this);
    threads::unregister_parker(this);
}

/// Resolves `run:()V` on the thread object's actual class (virtual dispatch, honoring subclass
/// overrides) and invokes it.
fn invoke_run(this: i32) -> Result<Vec<i32>> {
    let actual_class = HEAP.get_instance_name(this)?;
    let run_method = lookup::lookup_method(&actual_class, "run:()V")?.ok_or_else(|| {
        Error::new_execution(&format!(
            "run:()V not found for thread class {actual_class}"
        ))
    })?;
    Executor::invoke_non_static_method(run_method.class_name(), "run:()V", this, &[])
}

/// Writes `Thread.eetop` (a `long`), whose non-zero-ness is what `Thread.isAlive()` reports.
fn set_eetop(this: i32, value: i64) -> Result<()> {
    HEAP.set_object_field_value(this, "java/lang/Thread", "eetop", i64_to_vec(value))
}
