# Commit Summary: `9e1ada4` — Tokio-backed Virtual Threads (Loom capabilities)

**Author:** AryanGold (co-authored by YaroslavMe)
**Date:** Sat Apr 18 2026
**Branch:** `tokio-virtual-threads`
**Diff stats:** 54 files changed, **+1510 / −434** lines

---

## 1. Overview

This commit fundamentally re-architects the `rusty-jvm` execution engine to be
**fully asynchronous**, mapping every `java.lang.Thread` (both classic
"platform" threads and Java 21 virtual threads) directly onto a lightweight
**Tokio task**. The result is an "All-Virtual" M:N threading model that
delivers Project Loom–style scalability without a hand-written user-space
scheduler.

---

## 2. Architectural Changes

### 2.1 Tokio runtime integration
- Added `tokio = "1.52.1"` (features `full`, `tracing`) to `Cargo.toml`.
- Java threads are multiplexed (M:N) onto a small pool of OS carrier threads
  via Tokio's work-stealing scheduler.

### 2.2 "Viral async" execution engine
Refactored the entire interpreter pipeline to be `async`:
- `Engine::execute` is now an `async` state machine that owns the thread's
  `StackFrames`, so `.await` safely parks the whole Java call stack.
- `Executor`, `invoker`, `invoke_dynamic_runner`, `ldc_resolution_manager`,
  `ops_reference_processor`, `static_init`, `string_pool_helper`, and the
  whole `system_native_table` were converted to `async fn` / boxed futures.
- JNI bridges (`instance_methods_impl`, `static_methods_impl`,
  `string_operations_impl`) and `MethodHandle` natives (`invocation`,
  `member_name`, `native_accessor`, `var_handle`, `wrappers`) all updated to
  the new async signature.

### 2.3 Sync ↔ Async bridge (`block_on_async`)
New helper in `src/vm/concurrency/mod.rs` that safely runs an async future
from synchronous code (JVM boot, shutdown, nested native calls):
- Inside a Tokio runtime → uses `tokio::task::block_in_place` +
  `Handle::block_on` to avoid worker starvation.
- Outside a Tokio runtime → spins up a minimal 2-worker multi-thread runtime
  as fallback, allowing further nested `block_in_place` calls.
- Captures `CURRENT_JAVA_THREAD_ID` before blocking and re-injects it into the
  inner scope to prevent task-local context loss.

---

## 3. New `vm::concurrency` Module

| File | Purpose |
|------|---------|
| `concurrency/mod.rs` | Module entry + `block_on_async` bridge. |
| `concurrency/task_local.rs` | Declares `CURRENT_JAVA_THREAD_ID` via `tokio::task_local!` (replaces OS TLS, which is unsafe under M:N). |
| `concurrency/thread_manager.rs` | Global `DashMap<i32, JoinHandle<()>>` registry of running virtual threads, with `register_thread` / `unregister_thread`. |
| `concurrency/native_methods/mod.rs` | Re-exports the four async native stubs below. |
| `concurrency/native_methods/start.rs` | `Thread.start0` → `tokio::spawn` of the `run()` interpreter loop, scoped under the Java thread ID. |
| `concurrency/native_methods/sleep.rs` | `Thread.sleep0` / `sleepNanos0` → `tokio::time::sleep` (or `yield_now` for 0). |
| `concurrency/native_methods/yield_thread.rs` | `Thread.yield0` → `tokio::task::yield_now()`. |
| `concurrency/native_methods/current.rs` | `Thread.currentThread()` → reads `CURRENT_JAVA_THREAD_ID`, falls back to `MethodArea::system_thread_id` during boot. |

The previous synchronous logic in `src/vm/system_native/thread.rs` was removed
(−13 lines).

---

## 4. Native Method Implementations

- **Asynchronous JNI stubs** for `Thread.start0`, `sleep0`, `sleepNanos0`,
  `yield0`, `currentThread`.
- **Daemon thread support**: `Object.wait0` and `Unsafe.park` for
  indefinite waits now `await std::future::pending::<()>()`, suspending
  the task forever with zero CPU and zero OS-thread cost
  (fixes 100% CPU spin from `ReferenceHandler` / `Finalizer`).
- **`Reference.refersTo0`** workaround: direct pointer compare on the
  `referent` field so `ThreadLocalMap` keeps working without a GC.
- **`Unsafe.freeMemory0`** is a no-op (intentional leak) to keep NIO
  `FileChannel` from crashing.

---

## 5. Bug Fixes

- **`Unsafe` static field offsets**: operations such as
  `compareAndSet*`, `getVolatile`, `putVolatile`, etc. were treating static
  field offsets (which start at `i32::MAX` and grow downward) as instance
  field offsets, causing memory corruption. Fixed in `system_native/unsafe_.rs`
  (+146 / −81 lines), with a global `CAS_LOCK` `Mutex` providing sequential
  consistency for emulated atomic operations on the `DashMap` heap.
- **Method resolution for `Thread.run()`**: `start0` now uses
  `lookup_method` to walk the class hierarchy, so subclasses such as
  `InnocuousForkJoinWorkerThread` resolve `run:()V` correctly.
- Misc fixes in `exception/common.rs`, `method_area/field.rs`,
  `method_area/java_method.rs`, `method_area/method_area.rs`,
  `system_native/class.rs`, `module.rs`, `perf.rs`,
  `native_image_buffer.rs`, `dispatcher/{invoke,utils}.rs`.

---

## 6. Tests Added

New integration tests covering the virtual-thread machinery
(`tests/integration_tests.rs` +76 lines, plus Java sources):

| Test data file | Scenario |
|----------------|----------|
| `VirtualThreadAtomicCounterExample.java` | Atomicity / CAS under contention. |
| `VirtualThreadCurrentThreadExample.java` | Correctness of `Thread.currentThread()` per task. |
| `VirtualThreadLocalExample.java` | `ThreadLocal` isolation across virtual threads. |
| `VirtualThreadMassiveSpawn.java` | Stress test spawning many virtual threads. |
| `VirtualThreadPingPong.java` | Cooperative yielding / hand-off. |
| `VirtualThreadProducerConsumer.java` | Blocking-queue style coordination. |
| `VirtualThreadSleepExample.java` | `Thread.sleep` correctness. |

---

## 7. Documentation

- **`wiki/02_Concurrency_Architecture.md`** (new, 96 lines): full design
  document covering the All-Virtual philosophy, M:N multiplexing, viral async
  transformation, the `block_on_async` bridge, `DashMap` heap, `CAS_LOCK`,
  task-local vs thread-local storage, thread lifecycle, and GC-related
  workarounds.
- **`README.md`**: added `cargo test --test integration_tests` instructions.
- **`.gitignore`**: minor entry added.

---

## 8. Files Changed (highlights)

```
src/vm/concurrency/**                              (new module, ~250 LOC)
src/vm/execution_engine/{engine,executor,invoker,
    invoke_dynamic_runner,ldc_resolution_manager,
    ops_reference_processor,static_init,
    string_pool_helper,system_native_table}.rs    (async refactor)
src/vm/jni/{instance_methods_impl,static_methods_impl,
    string_operations_impl}.rs                    (async)
src/vm/system_native/method_handle_natives/**     (async)
src/vm/system_native/unsafe_.rs                   (+146/−81, CAS_LOCK + offset fix)
src/vm/{mod,launcher,method_area/*}.rs            (runtime wiring)
tests/integration_tests.rs + 7 Java sources       (virtual-thread tests)
wiki/02_Concurrency_Architecture.md               (new design doc)
Cargo.{toml,lock}, README.md, .gitignore
```

---

## 9. Take-aways

- `rusty-jvm` now treats every Java thread as virtual, achieving
  Loom-class scalability "for free" by leveraging Tokio.
- The async refactor is pervasive ("viral") but isolated behind a single
  `block_on_async` shim for the remaining synchronous boot/shutdown paths.
- New concurrency primitives (`CURRENT_JAVA_THREAD_ID`, `THREAD_HANDLES`,
  `CAS_LOCK`) provide the safety guarantees that the absence of a real GC
  and hardware atomics would otherwise compromise.

