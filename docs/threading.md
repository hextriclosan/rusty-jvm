# Threading and concurrency

> Status: **implemented**. Platform threads spawn and run; `synchronized`,
> `wait`/`notify`/`notifyAll`, `join`, `sleep`, `yield`, `interrupt`, `LockSupport`,
> `ThreadLocal`, atomic `Unsafe` CAS, `Thread.getState()`, and stack traces (own
> thread, and other running threads via cooperative safepoints) all work for real
> multithreaded programs. Known boundaries are listed in §12.

## 1. The model

Each Java **platform thread is backed by one OS thread** (`std::thread`). One
logical Java thread therefore maps to exactly one Rust `ThreadId`, which is what
identifies the owner of a monitor, the target of an unpark, and so on. There are
no virtual threads (yet).

Two kinds of state have to be kept straight:

- **Shared VM state** — already concurrency-safe and untouched by this work:
  `HEAP` (`LazyLock<Heap>`, a `DashMap` inside), `CLASSES`
  (`LazyLock<LoadedClasses>`, `DashMap` + atomics), `METHOD_AREA`
  (`OnceCell<MethodArea>`), and class initialization, which already takes a
  per-class lock (`JavaClass::static_fields_init_state().lock()`, the JVMS §5.5
  init lock).
- **Per-thread state** — held in a `thread_local!` `JavaThread`
  (`src/vm/jni/java_thread.rs`): the `JNIEnv` function-table pointer, the pending
  exception, the current `java.lang.Thread` reference, and the chain of live
  `StackFrames` segments. Because every OS thread gets its own `JavaThread`, a
  freshly spawned thread can run the interpreter with no extra plumbing.

The interpreter loop (`Engine::execute`) owns its `StackFrames` per invocation and
is entirely per-thread; nothing about it is shared. That is the property the whole
milestone rests on.

## 2. Per-thread infrastructure (`vm/threads.rs`)

A small module owns the VM-side bookkeeping for live threads:

- **`eetop` tokens.** `Thread.isAlive()` is defined as `eetop != 0`, so every
  started thread is stamped with a unique non-zero token on start and cleared to
  `0` on termination. (The value is otherwise opaque; HotSpot stores a native
  pointer here.)
- **Non-daemon join registry.** The `JoinHandle` of every non-daemon thread is
  recorded so the VM can wait for all of them before exiting (§3).
- **Parker registry.** A `DashMap<thread_ref, std::thread::Thread>` maps each
  thread to its OS handle so `LockSupport.unpark` can wake a specific thread (§8).
- **Thread-status writer.** `set_thread_status` writes the JVMTI-encoded
  `Thread$FieldHolder.threadStatus` that backs `getState()` (§6).

## 3. Spawning, run, and shutdown (`system_native/thread.rs`)

`Thread.start()` (Java) calls the `start0()` native, which:

1. reads the daemon flag,
2. sets `eetop` (alive) and `threadStatus = RUNNABLE` **synchronously, before
   spawning** — so `isAlive()`/`getState()` are correct the instant `start()`
   returns and a second `start()` is rejected (`Thread.start` checks
   `threadStatus != 0`),
3. `std::thread::spawn`s the worker and registers its parker eagerly (from the
   parent, so an unpark can never race ahead of the child's first park),
4. records the handle for shutdown if the thread is non-daemon.

The spawned body (`run_thread`) binds the thread's identity, resolves `run()`
**virtually** on the object's actual class (honoring subclass overrides), and
dispatches it. On return it routes any uncaught exception through the *offending*
thread's own `Thread.dispatchUncaughtException` (so the message reads
`Exception in thread "<name>"` with the right name), runs `Thread.exit()`, clears
`eetop`, sets `threadStatus = TERMINATED`, wakes any joiners (§4), and unregisters
its parker.

**Shutdown.** Per the Java Language Specification ([JLS §12.8][jls-12.8]) the
process stays alive until the last non-daemon thread dies. After `main` returns,
the VM calls `join_all_non_daemon()` (which
loops until the registry drains, so threads spawned *by* other threads are awaited
too) before running shutdown hooks — regardless of how `main` itself finished.
Daemon threads are not tracked and are abandoned at process exit, matching Java
semantics. If `start0` fails to spawn the OS thread, the "started" markers
(`eetop`, `threadStatus`) are rolled back so the thread returns cleanly to `NEW`.

## 4. Object monitors (`vm/monitor.rs`)

Every object can be a monitor. One is inflated lazily on first use and keyed by
the object's heap reference in a `DashMap` (objects never move — no GC — so the
`i32` key is stable). A monitor is a **reentrant** lock (owner `ThreadId` +
recursion count) plus a condition queue:

```rust
struct MonitorState { owner: Option<ThreadId>, count: u32, waiters: u32, to_wake: u32 }
struct ObjectMonitor { state: Mutex<MonitorState>, cv: Condvar }   // parking_lot
```

- **`monitorenter` / `monitorexit`** acquire/release reentrantly.
- **`Object.wait(timeout)`** fully releases the monitor (remembering the recursion
  depth), blocks on the condvar until notified/timed-out/interrupted, then
  re-acquires and restores the depth. `notify`/`notifyAll` hand out wake tokens via
  the `to_wake` counter; every waiter re-checks its condition on wake, so a
  broadcast that also disturbs threads contending to enter is merely inefficient,
  never incorrect. Spurious wakeups are permitted by the contract.
- **`Thread.holdsLock`** reports whether the caller owns the monitor.
- **`join()`** needs no special native: it is Java that sits in
  `synchronized(thread) { while (isAlive()) wait(); }`, and a terminating thread
  calls `wake_all_on_exit` (after clearing `eetop`) so joiners observe termination
  — mirroring HotSpot's `lock.notify_all` in `JavaThread::exit`.

`wait`/`notify` share a single condvar between the entry set and the wait set;
callers own the monitor when they call these (else `IllegalMonitorStateException`).

## 5. Interrupts

Interrupts follow the JDK's split exactly: `Thread.interrupt()` (Java) sets the
`interrupted` field itself, then calls `interrupt0()`, whose only job is to **wake**
the target so it observes the flag. `interrupt0` therefore does `unpark(target)`
(covers `LockSupport.park` and `Thread.sleep`) plus `monitor::interrupt_waiter`
(covers `Object.wait`, by targeting the exact monitor the thread registered when it
began waiting).

The blocking primitives check-and-clear the flag on wake and throw
`InterruptedException`:

- The flag is read+cleared atomically (`take_current_interrupt`) via a single
  compare-and-exchange on the `interrupted` field — the same entry lock
  `interrupt()`'s write takes — so an interrupt racing the clear is never lost.
- `Object.wait` checks on entry and each wake; on interrupt it **re-acquires the
  monitor first** (per spec) and then throws.
- `Thread.sleep` (`sleepNanos0`) parks toward a deadline computed with
  `Instant::elapsed` (no `Instant + Duration` overflow for the huge values a `long`
  permits) and throws if interrupted.

## 6. Thread state (`getState`)

`getState()` reads `Thread$FieldHolder.threadStatus`, decoded by
`jdk.internal.misc.VM.toThreadState`. The VM maintains it across the lifecycle
using that exact JVMTI bit encoding:

| State | Value | Set when |
|---|---|---|
| `NEW` | `0x00` | field default / spawn rollback |
| `RUNNABLE` | `0x05` | at `start0` (before spawn) and for the main thread |
| `BLOCKED` | `0x401` | contending for a monitor (`enter`, and re-entry after `wait`) |
| `WAITING` | `0x11` | `Object.wait()` with no timeout |
| `TIMED_WAITING` | `0x21` | `Object.wait(t)` and `Thread.sleep` |
| `TERMINATED` | `0x02` | thread exit |

`BLOCKED` is written only when a monitor is actually contended, so the uncontended
lock path is untouched. A waiter's full sequence is
`RUNNABLE → WAITING/TIMED_WAITING → BLOCKED (re-acquiring) → RUNNABLE`, matching
HotSpot (since `notify` is usually issued while still holding the lock, the
re-acquire window is real).

## 7. Atomicity and the memory model

Heap field reads/writes go through `DashMap` shard locks and static fields through
a `RwLock`; both carry acquire/release ordering, so ordinary cross-thread
visibility already holds. The one thing that needs more than a single locked
access is **compare-and-swap**, which must be atomic across its read-compare-write:

- `Heap::compare_and_exchange_object_field` / `..._array_by_raw_offset` do the whole
  CAS under a single entry lock.
- `FieldValue::compare_and_exchange` does the same for static fields, under the
  field's own write lock.
- `Unsafe.compareAndSet{Int,Long,Reference}` / `compareAndExchangeLong` are built on
  these (including the `java/lang/Class` static-field case).

This is what makes lock-free code (`AtomicInteger`, `ForkJoinPool`'s control word,
`java.util.concurrent` locks) correct; a non-atomic CAS silently drops updates and
deadlocks such algorithms.

## 8. `LockSupport` (`Unsafe.park`/`unpark`)

`park`/`unpark` map directly onto `std::thread` park/unpark, whose permit
semantics are exactly `LockSupport`'s (an unpark grants a permit a later park
consumes). `unpark(thread)` looks the target up in the parker registry (§2) and
wakes it; `park` blocks the current thread (with an optional relative-nanos or
absolute-millis deadline). Spurious wakeups are allowed, as the contract requires.

## 9. `ThreadLocal` and references

`ThreadLocalMap.getEntry` matches a key with `Entry.refersTo(key)` →
`Reference.refersTo0`, so working `ThreadLocal` depends on correct reference
semantics. Those need no GC here:

- `Reference.refersTo0(o)` returns `referent == o` (read the `referent` field);
- `Reference.clear0()` nulls `referent`;
- `Reference.get()` reads `referent` directly (no native involved).

With these, `ThreadLocal` get/set/`remove` work with proper per-thread isolation.
Making references work also lets `Cleaner`s actually run — including ones that free
native memory via `Unsafe.freeMemory0`, which is implemented against a size-tracking
table so `allocateMemory0` blocks can be freed with the right `Layout`.

## 10. Testing

The behaviour is covered by focused integration tests plus one orchestrated
edge-case suite (`tests/test_data/*` under `samples.concurrency.threads`,
asserted in `tests/integration_tests.rs`): spawning + per-thread `currentThread`,
per-thread uncaught-exception dispatch, `join`, `synchronized` mutual exclusion,
`wait`/`notify`, `notifyAll` waking *all* waiters, `sleep`, interrupting a
sleeping/waiting thread, `getState` transitions, `ThreadLocal` isolation +
`remove`, current-thread `getStackTrace`, dumping a *running* thread's stack via a
safepoint, and a bounded `ThreadEdgeCasesDemo`
covering reentrant `wait` depth restore, interrupt-status semantics, timed-wait
timeout, `BLOCKED` observation, double-`start` rejection, and atomic CAS under
contention. Each edge-case scenario is deterministic and bounded — it throws a
diagnostic on regression rather than hanging.

## 11. Safepoints and cross-thread stack traces (`vm/safepoint.rs`)

Reading another thread's call stack safely requires that thread to be paused at a
consistent point, not mid-instruction mutating its frames. A **cooperative
safepoint** provides this: each thread holds a `Safepoint` (in its `JavaThread`,
also published in a global registry by thread ref), and the interpreter
[`poll`](vm/safepoint.rs)s a flag periodically (every ~4096 bytecodes — a local
counter test each iteration, so the hot loop pays only a register increment; the
poll is measurably free). When a *dumper* raises the flag, the target reaches the
poll, **collects its own stack** (safe — it walks its own thread-local frames),
hands the owned frame data to the dumper via a mutex, and parks until released.
Only owned data ever crosses threads — never a live pointer into another thread's
frames.

`Thread.getStackTrace()` on another thread uses this: `getStackTrace0` drives the
target to a safepoint, then turns the collected frames into `StackTraceElement`s
through the same backtrace path as `fillInStackTrace`. The current thread collects
directly. The same registry + flag are the foundation a future stop-the-world GC
would use to pause all threads for root scanning.

## 12. Known boundaries

- **Dumping a *blocked* thread.** The safepoint (§11) pauses threads that are
  *running* the interpreter. A thread blocked in a native (`Object.wait`, `park`,
  `sleep`) never reaches the poll, so a dump of it times out and yields an empty
  trace. `Thread.getAllStackTraces` / `dumpThreads` (all-thread enumeration) are
  likewise not wired up yet. Dumping running threads and the current thread works.
- **Parallel streams / `ForkJoinPool`** are *correct* (atomic CAS fixed the
  coordination) but too slow to run reliably under interpretation — the pool
  over-provisions one busy-spinning worker per core. The `should_support_java_streams`
  test is `#[ignore]`d for this reason (performance, not correctness).
- **Priorities** (`setPriority0`) are advisory no-ops.
- **`clearInterruptEvent`** is a no-op — the `interrupted` field is the sole
  interrupt source of truth.

[//]: # (links)
[jls-12.8]: https://docs.oracle.com/javase/specs/jls/se25/html/jls-12.html#jls-12.8
