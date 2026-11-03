//! Precise, nonmoving mark/sweep under the coordinator's exclusive Operation.
use super::budget::AllocError;
use super::pending::{PendingList, PENDING};
use super::references::{Kind, Reference};
use super::{roots, trigger};
use crate::vm::error::Result;
use crate::vm::heap::heap::{Heap, HEAP};
use crate::vm::jni::{handles, java_thread::JavaThread};
use crate::vm::safepoint::{self, Operation, COORDINATOR};
use std::collections::HashSet;
use std::mem::size_of;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Cause {
    Explicit,
    Threshold,
    Stress,
    Pressure,
}

#[derive(Debug)]
pub(crate) struct Stats {
    pub(crate) before: usize,
    pub(crate) live: usize,
    pub(crate) reclaimed: u64,
    pub(crate) discovered: usize,
    pub(crate) scratch_bytes: usize,
}

struct Trace {
    marked: HashSet<i32>,
    work: Vec<i32>,
    references: Vec<Reference>,
    limit: usize,
}

impl Trace {
    fn new(limit: usize) -> Result<Self> {
        let mut trace = Self {
            marked: HashSet::new(),
            work: Vec::new(),
            references: Vec::new(),
            limit,
        };
        trace
            .marked
            .try_reserve(limit)
            .map_err(|_| AllocError::HostAllocation)?;
        trace
            .work
            .try_reserve_exact(limit)
            .map_err(|_| AllocError::HostAllocation)?;
        trace
            .references
            .try_reserve_exact(limit)
            .map_err(|_| AllocError::HostAllocation)?;
        Ok(trace)
    }

    fn mark(&mut self, reference: i32) -> Result<()> {
        if reference == 0 || self.marked.contains(&reference) {
            return Ok(());
        }
        // Even corrupted edges cannot grow scratch past preflight. Each valid
        // object enters the worklist once, irrespective of edge count or key age.
        if reference < 0 || self.marked.len() == self.limit {
            return Err(AllocError::InvalidLayout.into());
        }
        self.marked.insert(reference);
        self.work.push(reference);
        Ok(())
    }

    fn close(&mut self, heap: &Heap, pressure: bool) -> Result<()> {
        while let Some(object) = self.work.pop() {
            let reference = heap.trace_edges(object, |edge| self.mark(edge))?;
            if let Some(reference) = reference {
                if reference.referent != 0 && !heap.contains(reference.referent) {
                    return Err(AllocError::InvalidLayout.into());
                }
                match reference.kind {
                    Kind::Soft if !pressure => self.mark(reference.referent)?,
                    // Finalization is disabled. Never discover a FinalReference,
                    // or free a referent that Java could still read through one.
                    Kind::Final | Kind::Other => self.mark(reference.referent)?,
                    Kind::Soft | Kind::Weak | Kind::Phantom => {}
                }
                self.references.push(reference);
            }
        }
        // Retained soft edges join this same worklist, including newly reached
        // soft references, so closure reaches a fixed point without recursion.
        Ok(())
    }

    fn scratch_bytes(&self) -> usize {
        // HashSet exposes element capacity, not bucket bytes. Report a conservative
        // bucket/control-byte bound; tests also measure actual allocator requests.
        let hash_bytes = self
            .marked
            .capacity()
            .saturating_mul(2)
            .saturating_add(16)
            .saturating_mul(size_of::<i32>() + 1);
        hash_bytes
            + self.work.capacity() * size_of::<i32>()
            + self.references.capacity() * size_of::<Reference>()
    }
}

/// One attempt, with skipped-cycle policy for ANY snapshot/preflight/trace failure.
/// Dropping an incomplete Operation opens the coordinator without reporting a
/// successful epoch. No retry loop can spin on a lock held by a Safe thread.
pub(crate) fn collect(cause: Cause) -> Option<Stats> {
    let owner = safepoint::current()?;
    let operation = COORDINATOR.request_collection_for(
        owner,
        JavaThread::snapshot_roots(),
        cause == Cause::Pressure,
    )?;
    let cause = if operation.pressure() {
        Cause::Pressure
    } else {
        cause
    };
    let result = roots::snapshot(&operation)
        .and_then(|snapshot| collect_snapshot(&HEAP, &snapshot, &PENDING, cause, &operation));
    match result {
        Ok(stats) => {
            trigger::completed(stats.live);
            tracing::info!(target: "gc",
                "[gc] {:?}: {} -> {} objects, reclaimed={} bytes, pending={}, scratch<={} bytes",
                cause, stats.before, stats.live, stats.reclaimed, stats.discovered, stats.scratch_bytes);
            operation.complete();
            Some(stats)
        }
        Err(error) => {
            tracing::info!(target: "gc", "[gc] {:?}: skipped: {}", cause, error);
            None
        }
    }
}

/// Unit seams use the same traversal and real heap, still requiring an Operation.
/// The snapshot is the ONLY source of roots; never consult live mutator state here.
pub(crate) fn collect_snapshot(
    heap: &Heap,
    snapshot: &[i32],
    pending: &PendingList,
    cause: Cause,
    _operation: &Operation,
) -> Result<Stats> {
    let entries = heap.collection_entries()?;
    let mut trace = Trace::new(entries.len())?;
    let mut pending_head = pending.for_collection()?;
    for &root in snapshot {
        trace.mark(root)?;
    }
    trace.close(heap, cause == Cause::Pressure)?;
    // Marking has checked every live ID under its heap guard. Check the remaining
    // entries before any mutation, rather than acquiring every live shard twice.
    for &(reference, _) in &entries {
        if !trace.marked.contains(&reference) && !heap.contains(reference) {
            return Err(AllocError::InvalidLayout.into());
        }
    }

    // All roots, strong edges, referents and accounting have now been validated.
    // Nothing below allocates scratch or takes admission while a heap guard lives.
    trace.references.retain(|reference| {
        reference.active()
            && !trace.marked.contains(&reference.referent)
            && matches!(reference.kind, Kind::Soft | Kind::Weak | Kind::Phantom)
    });
    // Weak JNI identities survive; only their target is cleared.
    handles::clear_weak_references(|reference| trace.marked.contains(&reference));
    for reference in &trace.references {
        heap.set_reference_link(reference.object, "referent", 0)?;
        heap.set_reference_link(reference.object, "discovered", *pending_head)?;
        *pending_head = reference.object;
    }
    let mut reclaimed = 0;
    for &(reference, _) in &entries {
        if !trace.marked.contains(&reference) {
            reclaimed += heap.remove_recorded(reference)?;
        }
    }
    crate::vm::monitor::prune_idle(|reference| trace.marked.contains(&reference));
    if !trace.references.is_empty() {
        pending.notify();
    }
    Ok(Stats {
        before: entries.len(),
        live: trace.marked.len(),
        reclaimed,
        discovered: trace.references.len(),
        scratch_bytes: trace.scratch_bytes() + entries.capacity() * size_of::<(i32, u64)>(),
    })
}

#[cfg(test)]
mod tests;
