# Design Decisions

This document captures the non-obvious architectural choices made in `rusty-jvm` and explains
the reasoning behind them.
Understanding *why* the code is structured as it is helps contributors make consistent decisions
when extending or refactoring.

---

## 1. `i32` as the universal object reference

**Decision:** Every object reference on the heap, on the operand stack, and in local variables is
represented as a plain `i32`.

**Reasoning:**
The JVM Specification ([JVMS §2.6.2][jvms-2.6.2]) defines the operand stack as a sequence of
*values*, each occupying one or two *slots* of 32 bits.
Object references are one-slot values, so an `i32` is the natural Rust type - it requires no
boxing and is copy-able without unsafe code.

Keeping references as `i32` also means:
- The stack, local-variable array, and heap can all be `Vec<i32>`,
  which are safe, allocation-efficient, and trivially serialisable for debugging.
- 0 maps cleanly to the JVM `null` reference.
- No pointer arithmetic or lifetime juggling is needed at this stage of the project.

The trade-off is that `i32` keys in the heap `DashMap` are not *unforgeable* - any code that
can fabricate an integer can produce a seemingly valid reference.
This is acceptable because the only code that touches the heap is the trusted VM core, not
untrusted user code.
When a garbage collector is added, compacting GC will require changing this to indirect handles
anyway, so the abstraction surface is deliberately kept thin.

---

## 2. Vtable stored as `OnceCell<IndexMap<String, Arc<JavaMethod>>>`

**Decision:** Each `JavaClass` holds its vtable in a `OnceCell<IndexMap<String, Arc<JavaMethod>>>`,
initialised lazily on first virtual dispatch.

**Reasoning:**

### Why `OnceCell` (lazy initialisation)?
Class files are loaded on demand, and the full class hierarchy needed to build a vtable may not
yet be available at the moment a class is first parsed.
`OnceCell` lets us defer construction until the first `invokevirtual` / `invokeinterface` that
needs the table, at which point all super-classes and super-interfaces are guaranteed to be loaded.
There is no need for a separate "link" phase as in HotSpot.

### Why `IndexMap` (keyed by signature string)?
A vtable keyed by `(name, descriptor)` string has two advantages over a positional `Vec`:

1. **Interface default methods** - interfaces can contribute concrete methods to a class's vtable
   without requiring a fixed slot assignment agreed upon at compile time.
   A name-keyed map lets us overlay defaults from multiple interfaces without index conflicts.

2. **O(1) lookup** - `IndexMap` provides hash-map lookup speed while preserving insertion order,
   which simplifies debugging and makes the vtable deterministic across runs.

The vtable is built by `MethodArea::build_vtable` following the priority order mandated by
[JVMS §5.4.5][jvms-5.4.5]:
1. Inherit the parent class's vtable.
2. Overlay interface default methods.
3. Insert (or override) the class's own non-abstract methods.

---

## 3. Object references on the heap are `i32` keys, not raw pointers

**Decision:** The heap is a `DashMap<i32, HeapValue>` where keys are auto-incremented integers,
not raw `*mut` pointers.

**Reasoning:**
Using raw pointers would require `unsafe` throughout the codebase and would make the heap
vulnerable to use-after-free if any reference were dangling.
Because `rusty-jvm` does not yet implement a garbage collector, objects live for the entire
duration of the program; an indirect key-based map is therefore both safe and sufficient.

A `DashMap` was chosen over a `Mutex<HashMap>` to allow concurrent read access from multiple
native-method calls without a global lock.
The auto-incrementing key (`AutoDashMapI32`) starts at 1 to ensure 0 is permanently reserved
as the null reference.

When a real GC is introduced, the `i32` keys will naturally evolve into an *object-handle table*
(as used by, e.g., the JNI local/global reference model), without requiring changes to any code
that currently holds an `i32` reference.

---

## 4. Static native method dispatch via a lookup table

**Decision:** JDK native methods are dispatched through a static `HashMap` in
`system_native_table.rs` that maps a single string key in the format
`"java/lang/System:nanoTime:()J"` (that is, `internal/class/Name:methodName:descriptor`) to a
Rust function pointer.

**Reasoning:**
The alternative - loading the JDK's own native libraries (`.so`/`.dll`) at runtime - is
impractical because those libraries call back into a full HotSpot-compatible `JNIEnv`,
which would require implementing the entire JNI ABI before a single `println` could work.

The static table lets us bootstrap the standard library by implementing only the native methods
that are actually exercised by the tests, adding more incrementally.
For *user-supplied* native libraries (i.e. library code not in the JDK), the `libffi`-based
dynamic dispatcher is used instead, which does implement the JNI ABI.

---

## 5. Separate sub-crates for reusable components

**Decision:** Parsing and descriptor logic is factored into standalone published crates:
`jclassfile`, `jdescriptor`, `jimage-rs`, and `jniname`.

**Reasoning:**
These components have well-defined, narrow responsibilities and clear public APIs that are
useful independently of a full JVM runtime.
Publishing them separately:
- Enforces clean boundaries - the main crate cannot silently depend on internal details of the
  parser.
- Allows the community to reuse e.g. `jclassfile` for tooling (disassemblers, linters, IDEs)
  without pulling in the entire VM.
- Gives the components their own versioning, so breaking changes in the parser do not force a
  major bump of the main crate.

[jvms-2.6.2]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.6.2
[jvms-5.4.5]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html#jvms-5.4.5
