//! Complete roots for one exclusive, completed rendezvous.
//!
//! No accessor here initializes managed metadata or calls Java. Errors invalidate the
//! whole snapshot: the caller must drop the operation and abort, never sweep a prefix.
//! Class unloading is disabled. Intern, reflection, LDC and indy caches remain strong;
//! their growth is separate from any future bounded-live-heap guarantee.
use crate::vm::error::Result;
use crate::vm::heap::heap::HEAP;
use crate::vm::jni::handles;
use crate::vm::method_area::{loaded_classes::CLASSES, method_area};
use crate::vm::safepoint::Operation;
use crate::vm::{threads, PLATFORM_CLASSLOADER_REF, SYSTEM_CLASSLOADER_REF, UNNAMED_MODULE_REF};

/// Only an elected, fully quiescent operation can request a complete snapshot.
/// Thread roots are owned values collected on their owners, never remote stack reads.
pub(crate) fn snapshot(operation: &Operation) -> Result<Vec<i32>> {
    let mut roots: Vec<_> = operation.root_refs().collect();
    roots.extend(CLASSES.root_refs()?);
    roots.extend(method_area::global_root_refs()?);
    roots.extend(PLATFORM_CLASSLOADER_REF.get().copied());
    roots.extend(SYSTEM_CLASSLOADER_REF.get().copied());
    roots.extend(UNNAMED_MODULE_REF.get().copied());
    roots.extend(handles::global_roots()?);
    roots.extend(HEAP.interned_refs());
    roots.extend(threads::parker_refs());
    // Owned, contended and waited monitor targets are in the owning thread's
    // published snapshot. WAIT_TARGETS keys are registered Thread roots. Do not
    // lock monitor state here or turn every idle MONITORS key into a permanent root.
    //
    // Diagnostic ownership cannot overlap this operation; dump results move to
    // RootedSlots before that ownership ends. No separate dump-table scan is needed.
    //
    // The head remains a root until the native handler handoff. `discovered`
    // is an ordinary strong edge, including across multiple undrained epochs.
    roots.push(super::pending::PENDING.snapshot()?);
    roots.retain(|&reference| reference != 0);
    Ok(roots)
}

#[cfg(test)]
mod tests;
