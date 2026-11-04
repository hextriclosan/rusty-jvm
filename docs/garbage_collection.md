# Garbage collection

`rusty-jvm` uses a precise, nonmoving, stop-the-world mark-sweep collector. It reclaims
unreachable heap entries without relocating live objects. Internal object references remain
positive `i32` heap IDs (`0` is null); reclaimed IDs are never reused. JNI references are distinct,
opaque pointer-width tokens.

The collector is designed around four contracts. These describe the current implementation, not
future interfaces.

## C1. One coordinator owns thread transitions and VM operations

The safepoint coordinator serializes diagnostic dumps and collection operations. A collection
waits for every registered mutator to either publish an owner-collected root snapshot or retire.
The elected operation remains exclusive during snapshotting, tracing, reference processing and
sweep. Threads park cooperatively at interpreter and native boundaries; external native calls
publish roots and enter the safe state before libffi execution.

The coordinator's mutex protects transitions and operation ownership, but is not held while the
collector scans the heap. Managed locks that could be needed by another participant are released
before waiting to resume. A diagnostic timeout cannot authorize collection with missing roots.

## C2. Precise roots and destination-first transfer

Each thread snapshots its own live frame segments, typed reference slots, temporary roots, pending
exceptions, current thread, JNI locals and buffer leases. Global snapshots include class mirrors
and reference statics, initialized reflection metadata, classloader/module state, strong JNI
globals, active thread and monitor state, the pending-reference head, emergency OOM state and the
intern cache. The collector does not inspect another thread's live stack.

Reference layouts are derived from field descriptors before object publication. Primitive slots
are not treated as references, and the distinguished `Reference.referent` field is processed
separately from ordinary strong fields. Tracing is iterative and bounded by the current heap
entries, not by the largest ID ever issued. A failed root snapshot, invalid edge or failed
preflight aborts that cycle before weak clearing or sweeping; a failed cycle is skipped rather than
retried indefinitely.

Reference processing retains soft-reference closure for explicit and threshold collections.
Pressure collection may clear eligible soft referents. Eligible weak and phantom references are
cleared and published exactly once through the pending chain; the JDK reference handler owns
delivery. Finalization is not implemented: final references do not trigger finalizers.

Classes are never unloaded. Interned strings are strong roots, including strings added at runtime
with `String.intern()`. Reflection, LDC and invokedynamic caches are also strong roots. The
reflection/LDC/indy caches are tied to loaded-class metadata, but runtime interning can grow with
program input. Reclamation therefore bounds objects not retained by these permanent roots; it is
not an unqualified bound on every reachable object.

## C3. JNI identities and conversions

Every JNI local alias, global reference and weak global is an independent pointer-width opaque
token; token zero represents null. Tokens resolve to internal `i32` heap IDs and are never reused.
Local tokens belong to JNI frames, global tokens root their targets, and weak-global tokens remain
identifiable after collection clears their target. Object arguments and returns are converted at
the JNI boundary rather than cast as heap IDs. JNI local frames have a maximum capacity of 65,536
references. Capacity and token exhaustion report out-of-memory instead of aliasing an old handle.

The JNI support code explicitly rejects targets without 64-bit pointers because its `jmethodID`
encoding uses a 64-bit layout. Local x86-64 and s390x CI therefore do not verify a 32-bit target;
the latter provides configured big-endian coverage, not 32-bit ABI coverage.

## C4. Allocation admission and accounting

`-Xmx` bounds the managed-heap accounting policy described in
[managed-heap-accounting.md](managed-heap-accounting.md); it does **not** measure process memory
or RSS. Policy v1 estimates aligned owned storage for heap entries, arrays, object field storage
and intern-map entries. Referenced children and shared class metadata are not charged again.
Collector scratch is accounted separately and its pre-reserved traversal storage scales with the
current number of allocated objects.

All managed allocations use checked reservation and recorded-charge accounting. Pressure may
collect and retry admission once; there is no unlimited bypass. `Runtime.totalMemory()` and
`maxMemory()` report the configured logical limit, while `freeMemory()` is that limit minus
committed managed usage. The Linux default is one quarter of `/proc/meminfo`'s `MemTotal`; if
unavailable, and on other platforms, the default is 1 GiB.

The budget is not a promise that every process allocation is catchable. Native allocations,
copied JNI buffers, JDK metadata, Rust bookkeeping, thread stacks and libffi state are outside the
managed limit. Fallible payload reservations can become Java `OutOfMemoryError`, but infallible
library/table/`Arc` allocations and host allocator aborts cannot be converted into a catchable Java
error. An OOM can also represent an accounting overflow, invalid allocation layout or a skipped
pressure collection, not only exhaustion of the configured byte budget. Individual array payloads
are capped at `i32::MAX - 8` bytes independently of `-Xmx`.

Heap IDs and JNI tokens are checked finite identity spaces. Exhaustion is an error; identities
never wrap around or alias a prior object/reference. A configured heap limit too small to bootstrap
the permanently rooted emergency `OutOfMemoryError` fails VM startup.

## Options and logging

- `-Xmx<size>` sets the managed limit. Positive byte values and `k`, `m` or `g` suffixes are
  accepted, case-insensitively; arithmetic overflow and zero are rejected.
- `-XX:GCAllocThreshold=<count>` sets a positive fixed allocation-count trigger. The default is
  65,536 allocations and adapts to the live-object count after collection.
- `-XX:+GcStressAtSafepoints` requests collection at interpreter safepoints and rooted native
  boundaries.
- `-Xlog:gc` enables collection and skipped-cycle messages on stderr, each marked with `[gc]`.
  Without that option, GC output is suppressed and stderr remains empty in the default logging
  configuration. `RUST_LOG` overrides the fallback tracing filter wholesale, so it can expose GC
  messages without `-Xlog:gc`.

## Known limitations

- Verified on Linux x86-64 with JDK 25 only. s390x (big-endian), Windows and macOS were not run
  for this work; those platforms are covered only by CI when it runs.
- No moving, generational or concurrent collection; no class unloading or finalization.
- Classes and selected caches are permanent roots. Runtime `String.intern()` can retain an
  input-dependent number of strings.
- JNI local frames are capped at 65,536 references. Finite heap and JNI identity spaces can
  exhaust, although exhaustion is checked and identities are not reused.
- The collector cannot bound RSS or make infallible host allocations recoverable. Very large
  individual arrays have the documented `i32::MAX - 8` payload cap.
- A native synchronized method invoked from bytecode acquires its implicit monitor before the
  native body and releases it on return or exception. Entry through `Executor` paths used by JNI
  `Call*Method` and reflection fails before native execution: `Executor::exec_jc` requests a
  bytecode stack frame, but native methods have no code context. These unsupported paths do not
  execute the method unlocked.
- An internal VM error that returns early from the interpreter can bypass cleanup of remaining
  frames and leave their synchronized monitors held; ordinary Java exception unwinding releases
  them.
- The VM implements only a subset of `java.lang.reflect.Array` native accessors; reflective uses
  of unimplemented typed `getX`/`setX` accessors can fail with a missing native symbol.
- 32-bit targets fail to compile: `src/vm/jni/utils.rs` has a `compile_error!` for targets
  without 64-bit pointers because its `jmethodID` encoding uses a 64-bit layout.
- Classes initialized before and after startup can currently receive different `Module` objects
  rather than the defining loader's shared unnamed module.

See [threading.md](threading.md) for the safepoint and monitor model, and
[architecture.md](architecture.md) for the collector's position in the VM.
