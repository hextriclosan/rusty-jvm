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

## Activation and limits of this increment

Emergency `OutOfMemoryError` bootstrap uses the configured limit through common
admission from the first heap entry: a too-small limit fails startup before an
over-budget payload is built. Once the emergency throwable and ancestry metadata
are prepared, one startup checkpoint switches **all** runtime paths to
accounting-only mode. Task 6 continues recording every managed allocation, without
partially enforcing the runtime limit on selected callers. Bounded test heaps
exercise the real admission mechanism. Task 7 replaces that accounting-only phase
with pressure collection and shared runtime enforcement together. Java memory
reporting is also task 7's work.

This measures the **managed heap budget, not process resident memory (RSS)**.
Native allocations and copied JNI buffers, JDK metadata, Rust root bookkeeping,
thread stacks and libffi state are outside it. Collector scratch is separate and
must be measured by the collector. Fallible payload and bookkeeping reservations
can report failure; infallible library/table/Arc allocation and host allocator
aborts are not made catchable by this policy.

Intern, reflection, LDC and invokedynamic caches remain strong roots. No
unqualified bounded-live-set claim follows: future reclamation bounds apply to
objects not reachable from these caches. Reflection/LDC/indy retention follows
loaded-class metadata; runtime `String.intern()` can grow with program input.
Intern retention is not an exemption from the future managed admission limit.
Task 6 alone proves neither repeated-cycle reclamation nor collector scratch bounds.
