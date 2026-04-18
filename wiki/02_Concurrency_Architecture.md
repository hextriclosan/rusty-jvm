# `rusty-jvm` Concurrency Architecture

## 1. Philosophy & Core Concepts

### 1.1 The "All-Virtual" Threading Model
Historically, traditional Java Virtual Machines (like HotSpot) utilized a 1:1 threading model, where every `java.lang.Thread` was mapped directly to a heavy OS-level thread (e.g., a `pthread`). This approach severely limited scalability due to high memory overhead per stack and expensive context-switching during blocking I/O operations.

In `rusty-jvm`, we embrace an **"All-Virtual"** threading philosophy. Rather than requesting OS threads from the host, the JVM maps every Java thread—regardless of whether the Java code considers it a standard "Platform Thread" or a "Virtual Thread"—directly to a lightweight asynchronous `tokio` task. 

From the perspective of the executing Java bytecode, it is interacting with standard threads. From the perspective of the Rust host, it is simply spawning green threads. This allows `rusty-jvm` to seamlessly support massive concurrency out of the box, fulfilling the core promise of Java 21+ (Project Loom) without the need to maintain a custom, hand-written user-space scheduler.

### 1.2 The Tokio Scheduler (M:N Multiplexing)
By delegating thread scheduling to the `tokio` runtime, `rusty-jvm` employs an **M:N multiplexing model**. 

Millions of Java virtual threads (M) are multiplexed onto a small, fixed pool of Rust carrier OS threads (N)—typically equal to the number of CPU cores. Tokio uses a highly optimized work-stealing scheduler. When a Java thread encounters a blocking operation (such as `Thread.sleep` or a network call), the Tokio task `.await`s, saving its state machine to the heap and instantly yielding the underlying OS carrier thread to execute the next pending Java thread.

This architecture ensures maximum CPU utilization and prevents individual Java threads from deadlocking the host environment.

---

## 2. The Asynchronous Execution Engine

### 2.1 The "Viral Async" Transformation
To allow a Java thread to yield its OS carrier thread during a blocking operation, the core JVM execution pipeline must be entirely asynchronous. In Rust, asynchronous functions are "viral"—an `async` function can only be `.await`ed by another `async` function.

Therefore, the core bytecode interpreter loop (`Engine::execute`) acts as an asynchronous state machine:
```rust
pub(crate) async fn execute(initial_stack_frame: StackFrame, reason: &str) -> Result<Vec<i32>>
```
Because the `execute` loop holds exclusive ownership of the thread's `StackFrames`, yielding via `.await` safely parks the entire Java call stack in memory. This async context bubbles up through the method invocation chain (`Executor::invoke_non_static_method`, `invoker::invoke`) and down into the native method resolution table (`SYSTEM_NATIVE_TABLE`), culminating in native operations that can explicitly yield, such as `tokio::time::sleep().await`.

### 2.2 Bridging Sync and Async (`block_on_async`)
While the core execution engine is asynchronous, certain parts of the JVM—such as early bootstrapping (`prelude()`), shutdown hooks, and synchronous JNI boundary crossings—must operate in a synchronous context. We cannot blindly rewrite the entire JVM to be async, nor can we use simple blocking calls that would starve the Tokio worker pool.

To bridge this gap, `rusty-jvm` uses a custom `block_on_async` synchronization barrier. This function dynamically detects the current execution context:

1. **Inside an active Tokio Runtime:** It uses `tokio::task::block_in_place` to temporarily convert the current worker thread into a blocking thread, moving other queued tasks to a different worker to prevent scheduler starvation.
2. **Outside a Tokio Runtime (e.g., JVM Boot/Shutdown):** It spins up a temporary, lightweight `multi_thread` runtime (with minimal worker threads) to safely execute the future. This ensures that deeply nested synchronous native calls can still utilize `block_in_place` without causing single-thread deadlocks.

Additionally, `block_on_async` is responsible for mitigating **Task-Local Context Loss**. Because stepping out of an async block drops Tokio's task-local variables, the bridge captures the `CURRENT_JAVA_THREAD_ID` before blocking and explicitly re-injects it into the new inner runtime scope, ensuring that `Thread.currentThread()` resolves correctly across synchronous boundaries.

---

## 3. Thread-Safe Memory & State Management

### 3.1 The Concurrent Heap (`DashMap`)
In standard JVM implementations, managing the Java Heap across multiple threads requires complex garbage collection algorithms, memory barriers, and a Global Interpreter Lock (GIL) or fine-grained locks. 

In `rusty-jvm`, the heap (`vm/heap/heap.rs`) avoids a GIL by utilizing `DashMap`, a highly concurrent, lock-free (for reads) and shard-locked (for writes) hash map. Because each Java object or array is stored as an independent entry inside the `DashMap`, multiple Tokio tasks can simultaneously read from and write to different objects in the Java Heap without contending for a global lock. This provides massive parallel throughput for memory operations out of the box.

### 3.2 Emulating Hardware Atomics (`CAS_LOCK`)
While `DashMap` provides thread safety for *individual* read or write operations, it cannot make a composite "check-then-act" sequence atomic. This causes the classic "Lost Update" race condition in parallel workloads (like Java Streams or `java.util.concurrent.atomic` classes).

In C++ or Rust, atomic operations (`std::atomic<T>`) map to hardware-level instructions (like `LOCK CMPXCHG` on x86). Because `rusty-jvm` emulates memory in software via a hash map, we cannot rely on CPU-level atomics. 

To guarantee **Sequential Consistency**, we introduced a global mutex (`CAS_LOCK`) in `unsafe_.rs`. This acts as a localized GIL strictly for `Unsafe` memory barriers. All `Unsafe.compareAndSet` (CAS) and `Unsafe.getVolatile` / `putVolatile` operations must acquire this lock. This ensures that a Tokio task can safely read an expected value and write an updated value to the emulated heap without another task modifying it mid-flight.

### 3.3 Task-Local Storage vs. Thread-Local Storage
In traditional systems programming (C/C++), developers use OS-level Thread Local Storage (`thread_local`) to attach context to the executing thread. In a Tokio M:N multiplexing architecture, using OS-level TLS is dangerous: a single OS carrier thread might execute Task A, suspend it, and immediately execute Task B.

To securely isolate Java Thread context (e.g., for `Thread.currentThread()`), `rusty-jvm` uses **Task-Local Storage** via the `tokio::task_local!` macro. 
*   **`CURRENT_JAVA_THREAD_ID`**: Binds the heap reference (an `i32`) of the executing `java.lang.Thread` object directly to the asynchronous Tokio task.
*   **Primordial Fallback**: During the early JVM boot sequence (before tasks are fully scoped), the task-local context might be empty (`0`). The `current_thread` native method detects this and safely falls back to reading the `system_thread_id` from the global `MethodArea`.

---

## 4. Thread Lifecycle & Native Method Mapping

### 4.1 Thread Creation & Spawning (`Thread.start0`)
When a Java application invokes `Thread.start()`, the JVM intercepts the internal `start0` native call. The sequence is as follows:
1.  The JVM retrieves the `java.lang.Thread` object from the heap.
2.  It resolves the `run:()V` method. (This resolution correctly walks the class hierarchy, supporting subclasses like Java's internal `InnocuousForkJoinWorkerThread`).
3.  It constructs a fresh `StackFrame`.
4.  Ownership of this frame is **moved** into a new Tokio task via `tokio::spawn`, satisfying Rust's strict memory safety rules and guaranteeing that call stacks cannot be corrupted across threads.
5.  The Tokio `JoinHandle` is recorded in a global `THREAD_HANDLES` registry to support future lifecycle operations like `Thread.join()`.

### 4.2 Cooperative Yielding & Sleeping
`rusty-jvm` maps Java's thread suspension instructions directly to Tokio's asynchronous time utilities.
*   **`Thread.sleep0` / `Thread.sleepNanos0`**: Maps to `tokio::time::sleep()`. This pauses the specific Java virtual thread for the requested duration without blocking the underlying OS carrier thread.
*   **`Thread.yield0`**: Maps to `tokio::task::yield_now()`. This triggers a cooperative context switch, explicitly yielding control back to the Tokio scheduler so other pending Java threads can progress.

### 4.3 Parking and Daemon Threads
The standard Java standard library automatically boots several internal daemon threads (such as the `ReferenceHandler` and `Finalizer`). When idle, these threads invoke native parking methods like `Object.wait0` or `Unsafe.park`.

If the JVM returned from these methods immediately, the background threads would spin in an infinite `while(true)` loop, consuming 100% of the CPU. Because the execution engine is asynchronous, `rusty-jvm` handles indefinite waits (e.g., `wait(0)`) by evaluating `std::future::pending::<()>().await`. This elegantly suspends the Tokio task forever, consuming zero CPU cycles and zero OS threads.

---

## 5. Mocking Missing JVM Subsystems

Because `rusty-jvm` is an actively developing runtime, certain advanced JVM subsystems (like the Garbage Collector and physical memory allocators) are not yet fully implemented. The concurrency module implements specific workarounds to ensure Java's standard library threads do not crash:

### 5.1 Garbage Collection Workarounds
*   **`WeakReference` Semantics (`Reference.refersTo0`)**: Modern Java's `ThreadLocalMap` relies on `WeakReference` entries to clean up stale thread locals. Since `rusty-jvm` does not have a Garbage Collector to clear weak references, memory is never collected. We override `refersTo0` to extract the `referent` field and do a direct pointer comparison, allowing `WeakReference` instances to safely act as strong references and ensuring `ThreadLocal` variables function normally.
*   **Intentional Memory Leaks (`Unsafe.freeMemory0`)**: High-performance I/O (like NIO `FileChannel`) allocates raw memory outside the JVM heap using `Unsafe.allocateMemory0`, and subsequently attempts to free it. We provide a no-op void stub for `Unsafe.freeMemory0`, intentionally leaking the memory to prevent the JVM from crashing during native DLL resolution.

