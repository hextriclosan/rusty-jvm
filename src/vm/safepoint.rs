//! One authority for managed access, immutable root publication and VM operations.
//! The mutex is never an operation token: it is released before scanning or running Java.

mod blocking;
mod coordinator;
#[cfg(test)]
pub(crate) mod tests;

use crate::vm::jni::java_thread::JavaThread;
use crate::vm::stack::rooted_slots::RootedSlots;
use crate::vm::stack::slot::Slot;
use crate::vm::stack::stack_frames_util::{StackElement, StackFramesUtil};
pub(crate) use blocking::{lock, wait, wait_for, Blocking, ManagedAccess};
pub(crate) use coordinator::{Coordinator, Operation, Reason, Snapshot};
use std::cell::Cell;
#[cfg(test)]
use std::cell::RefCell;
use std::sync::{Arc, LazyLock};

pub(crate) static COORDINATOR: LazyLock<Arc<Coordinator>> =
    LazyLock::new(|| Arc::new(Coordinator::new()));

thread_local! {
    static PARTICIPANT: Cell<Option<u64>> = const { Cell::new(None) };
    // Unlike a counter local to Engine::execute, this survives all nested invocations.
    static BYTECODES: Cell<u32> = const { Cell::new(0) };
    // Mirrors this participant's Running/Safe state. Only the owning thread ever transitions
    // itself, so the mirror is exact, and it keeps every JNI entry off the coordinator mutex.
    static MANAGED: Cell<bool> = const { Cell::new(false) };
    #[cfg(test)]
    static OBSERVER: RefCell<Option<Box<dyn Fn(Event)>>> = RefCell::new(None);
}

pub(crate) fn current() -> Option<u64> {
    PARTICIPANT.with(Cell::get)
}

fn managed() -> bool {
    MANAGED.with(Cell::get)
}

fn set_managed(value: bool) {
    MANAGED.with(|managed| managed.set(value));
}

/// Bootstrap registers before the first managed allocation, not after Thread construction.
pub(crate) fn attach() {
    assert!(current().is_none(), "participant already attached");
    let id = COORDINATOR.startup(0);
    claim(id);
}

pub(crate) fn claim(id: u64) {
    PARTICIPANT.with(|participant| {
        assert!(participant.replace(Some(id)).is_none());
    });
    COORDINATOR.resume(id);
    event(Event::StartupClaimed);
}

pub(crate) fn bind(thread_ref: i32) {
    if let Some(id) = current() {
        COORDINATOR.bind(id, thread_ref);
    }
}

pub(crate) fn retire() {
    if let Some(id) = current() {
        COORDINATOR.park(id, JavaThread::snapshot_roots(), Reason::Retirement);
        COORDINATOR.retire(id);
        PARTICIPANT.with(|participant| participant.set(None));
    }
}

/// Run only host work here: copy managed inputs before entry and apply outputs
/// after return. All guards acquired by the closure must die inside the closure.
pub(crate) fn blocking<T>(work: impl FnOnce() -> T) -> T {
    let _safe = Blocking::enter(Reason::Blocking);
    work()
}

#[inline]
pub(crate) fn dispatch_bytecode() {
    let due = BYTECODES.with(|counter| {
        let next = counter.get().wrapping_add(1);
        counter.set(next);
        next & 4095 == 0
    });
    if due {
        poll();
    }
}

pub(crate) fn poll() {
    if !COORDINATOR.requested() {
        return;
    }
    let Some(id) = current() else { return };
    // Capture only on the owner. No pointer into another thread's frames is inspected.
    let dump = COORDINATOR
        .dump_request(id)
        .map(|dump_id| (dump_id, StackFramesUtil::collect_current_stack(1024).ok()));
    COORDINATOR.park(id, JavaThread::snapshot_roots(), Reason::Safepoint);
    if let Some((dump_id, stack)) = dump {
        COORDINATOR.publish_dump(id, dump_id, stack);
    }
    COORDINATOR.resume(id);
}

/// The caller allocates the destination before requesting the operation. Class references
/// are transferred before the operation ends and the caller becomes Running atomically.
pub(crate) fn dump_stack(
    thread_ref: i32,
    destination: &mut RootedSlots,
) -> Option<Vec<StackElement>> {
    let id = current()?;
    COORDINATOR.dump(id, thread_ref, JavaThread::snapshot_roots(), |frames| {
        destination.replace(
            frames
                .iter()
                .map(|frame| Slot::Ref(frame.class_ref()))
                .collect(),
        );
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Event {
    SnapshotPublished,
    Election,
    GuardReacquired,
    Resumed,
    StartupClaimed,
    Retired,
    NativeFramePop,
}

#[inline]
pub(crate) fn event(event: Event) {
    #[cfg(test)]
    OBSERVER.with(|observer| {
        if let Some(observer) = observer.borrow().as_ref() {
            observer(event);
        }
    });
    #[cfg(not(test))]
    let _ = event;
}

#[cfg(test)]
pub(crate) fn observe(observer: impl Fn(Event) + 'static) {
    OBSERVER.with(|slot| *slot.borrow_mut() = Some(Box::new(observer)));
}
