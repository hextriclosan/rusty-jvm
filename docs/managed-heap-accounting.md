# Managed heap accounting policy v1

The machine-readable version is `gc::budget::CHARGE_POLICY_VERSION = 1`, with a
compile-time assertion. Tasks 7 and 11 must use this policy rather than introduce
independent formulas. A formula change requires an explicit version change.

## Formula

All arithmetic is checked. Let `A(n) = (n + 7) & !7`, rounding up to eight bytes,
and `S(T) = size_of::<T>()` on the target. Policy v1 is an **owned-storage
estimate**, not allocator instrumentation.

- Heap entry header: `H = A(S(HeapValue) + 64)`. `HeapValue` includes the enum
  discriminants and inline String, Vec, IndexMap, lock and class-ID storage.
  The 64-byte allowance estimates heap-table and recorded-charge-table entry
  overhead. It does not claim to measure DashMap shard/bucket allocations.
- Array: `H + A(descriptor.capacity()) + A(data.capacity())`.
- Object: `H + A(outer.capacity() * (S((String, IndexMap<String, FieldValue>)) + M))`,
  plus, for each declaring-class segment,
  `A(class_name.capacity()) + A(inner.capacity() * (S((String, FieldValue)) + M))`,
  plus, for each field,
  `A(field_name.capacity()) + A(value.capacity() * S(i32))`.
  Here `M = 2 * S(usize) + 1` estimates the IndexMap hash/index bucket storage per
  capacity slot. Field-vector headers and locks are already in the inline entry
  size and are not counted twice.
- Strong intern-map entry:
  `A(S((String, i32)) + 64) + A(key.capacity())`. Replacement of an existing key
  keeps that key's storage and charge. These permanent auxiliary charges are
  not refunded by object sweep.

Referenced Java children are separate entries, not charged again through their
parents. Shared class templates and reference layouts are metadata, not per-object
payload. Map allowances approximate table overhead; actual hash-table load factors,
allocator headers, alignment slack and physical page commitment are not measured.

## Admission and ownership

A plan contains only dimensions or shared class metadata. It computes a charge
using requested exact capacities before allocating per-entry payload. Strings,
vectors and IndexMaps use fallible exact reservation. Commit checks the actual
owned capacities against that reservation and records the actual policy charge.
If a host collection over-reserves beyond the planned charge, the staged value
is discarded rather than publishing an undercharged entry.

One admission mutex serializes checked `used + reserved + requested` accounting,
nonreusing positive heap IDs, insertion and publication into a prepared caller-owned
root slot. A reservation owns a numeric debit, not a mutex guard. Cancellation
refunds the whole debit; commit transfers it to used bytes and refunds unused
slack. Sweep subtracts the stored charge, never a newly evaluated formula.
No polling, Java execution or coordinator wait is allowed with an unfinished
reservation. Task 7 must cancel before a pressure rendezvous and retry admission
afterwards.

Published fields and arrays have fixed sizes. Field writes and whole-array copyback
copy into existing storage and reject length changes. Mutable heap-array views are
slices, not vectors. There is consequently no published payload resize that can
bypass a delta reservation. Intern-key growth uses the same reservation mechanism.
Any future resizing API must reserve the positive charge delta before resizing.

The per-array payload implementation maximum is `i32::MAX - 8` bytes, independent
of `-Xmx`. Negative lengths, checked multiplication overflow and larger payloads
are rejected before allocation. Wide array initializer slices contain consecutive
high-word/low-word pairs; an odd slot count is invalid.

## Collection and admission

Emergency `OutOfMemoryError` bootstrap uses the configured limit through common
admission from the first heap entry: a too-small limit fails startup before an
over-budget payload is built. Once the emergency throwable and ancestry metadata
are permanently rooted, pressure collection is enabled; admission remains strict.
A capacity failure releases admission without constructing a payload, requests or
joins one pressure epoch, and retries the reservation atomically once. A skipped
cycle or insufficient reclaimed capacity returns the existing allocation error.
No reservation or heap shard guard crosses the rendezvous.
Pressure is published at the coordinator before the requester becomes Safe, so an
ordinary elected owner upgrades its policy before tracing if a pressure contender
joins it. A pressure requester cannot accept a soft-retaining ordinary epoch.

Collection retains the coordinator operation through root snapshot, iterative
tracing, reference processing and recorded-charge sweep. A failed snapshot,
scratch preflight or trace skips that cycle without sweeping. There is no retry
spin. Scratch is freshly pre-reserved from the current object count and dropped
after each cycle, rather than indexed by lifetime heap IDs. Repeated array edges
are deduplicated into this bounded worklist without allocating under heap guards.

Ordinary explicit and threshold collections retain the soft-reference closure;
capacity pressure may clear eligible soft references. Weak and phantom references
are cleared and published once through a null-terminated `discovered` chain whose
head is a snapshot root. The native handler handoff establishes a JNI local root
before clearing the head. Finalization remains disabled.

Runtime reports fixed logical `totalMemory = maxMemory = max` and
`freeMemory = max - used` from an admission-locked snapshot. Uncommitted
reservations are not used bytes. Java's signed `long` reporting saturates at
`Long.MAX_VALUE` for configured unsigned limits above that representable range.
The default allocation threshold adapts to the live object count; an explicit
`-XX:GCAllocThreshold=N` remains fixed. `-XX:+GcStressAtSafepoints` collects at
engine polls and rooted native call boundaries. Collector logging uses the `gc`
target and literal `[gc]` text; without opt-in logging, collection is quiet.

This measures the **managed heap budget, not process resident memory (RSS)**.
Native allocations and copied JNI buffers, JDK metadata, Rust root bookkeeping,
thread stacks and libffi state are outside it. Collector scratch is separate and
must be measured by the collector. Fallible payload and bookkeeping reservations
can report failure; infallible library/table/Arc allocation and host allocator
aborts are not made catchable by this policy.

Intern, reflection, LDC and invokedynamic caches remain strong roots. No
unqualified bounded-live-set claim follows: reclamation bounds apply to
objects not reachable from these caches. Reflection/LDC/indy retention follows
loaded-class metadata; runtime `String.intern()` can grow with program input.
Intern retention is not an exemption from the managed admission limit.
