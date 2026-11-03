use super::*;
use crate::vm::jni::java_thread::JavaThread;
use crate::vm::safepoint::{self, Blocking, Event, Reason, COORDINATOR};
use crate::vm::stack::rooted_slots::RootedSlots;
use crate::vm::stack::slot::Slot;
use crate::vm::stack::stack_frame::{StackFrame, StackFrames};
use std::sync::mpsc::channel;
use std::time::Duration;

#[test]
fn gc_roots_blocked_frames_all_segments_and_startup_escrow_are_complete() {
    let _serial = safepoint::tests::SERIAL.lock().unwrap();
    safepoint::attach();
    let startup_root = HEAP.create_array("[I", 0);
    let startup = COORDINATOR.startup(startup_root);
    let thread_root = HEAP.create_array("[I", 0);
    let deep_local = HEAP.create_array("[I", 0);
    let operand = HEAP.create_array("[I", 0);
    let nested_local = HEAP.create_array("[I", 0);
    let primitive_bits = HEAP.create_array("[I", 0);
    let temporary = HEAP.create_array("[I", 0);
    let transfer = HEAP.create_array("[I", 0);
    let (published, publication) = channel();
    let (release, released) = channel();
    let worker = std::thread::spawn(move || {
        safepoint::attach();
        JavaThread::set_current_thread(thread_root);
        let mut oldest = StackFrame::for_test(2, 2);
        oldest.set_local(0, Slot::Ref(deep_local));
        oldest.set_local(1, Slot::Value(primitive_bits));
        oldest.push(Slot::Ref(operand)).unwrap();
        oldest.push(Slot::Value(primitive_bits)).unwrap();
        let mut outer = StackFrames::new(vec![oldest]);
        for _ in 0..1100 {
            outer.new_frame(StackFrame::for_test(0, 0));
        }
        let _outer = JavaThread::register_stack_frames(&mut outer);
        let mut nested = StackFrame::for_test(1, 0);
        nested.set_local(0, Slot::Ref(nested_local));
        let mut inner = StackFrames::new(vec![nested]);
        let _inner = JavaThread::register_stack_frames(&mut inner);
        let _temp = JavaThread::hold_temp_roots(vec![Slot::Ref(temporary)]);
        let _transfer = RootedSlots::from_slots(vec![Slot::Ref(transfer)]);
        safepoint::observe(move |event| {
            if event == Event::SnapshotPublished {
                published.send(()).unwrap();
            }
        });
        let blocked = Blocking::enter(Reason::Blocking);
        released.recv_timeout(Duration::from_secs(5)).unwrap();
        drop(blocked);
        safepoint::retire();
    });
    publication.recv_timeout(Duration::from_secs(5)).unwrap();

    // Both epochs consume the same blocked owner snapshot, with no remote stack access.
    for _ in 0..2 {
        let operation = COORDINATOR
            .request_collection(safepoint::current().unwrap(), JavaThread::snapshot_roots())
            .unwrap();
        let roots = snapshot(&operation).unwrap();
        for root in [
            startup_root,
            thread_root,
            deep_local,
            operand,
            nested_local,
            temporary,
            transfer,
        ] {
            assert!(roots.contains(&root), "missing root {root}");
        }
        assert!(!roots.contains(&primitive_bits));
        operation.complete();
    }
    release.send(()).unwrap();
    worker.join().unwrap();
    COORDINATOR.cancel_startup(startup);
    safepoint::retire();
}

#[test]
fn gc_roots_global_loaders_interns_parkers_and_registry_identities_are_included() {
    let _serial = safepoint::tests::SERIAL.lock().unwrap();
    safepoint::attach();
    let references: Vec<_> = (0..6).map(|_| HEAP.create_array("[I", 0)).collect();
    PLATFORM_CLASSLOADER_REF.set(references[0]).unwrap();
    SYSTEM_CLASSLOADER_REF.set(references[1]).unwrap();
    UNNAMED_MODULE_REF.set(references[2]).unwrap();
    HEAP.put_const_string_ref("gc_roots_global", references[3]);
    threads::register_parker(references[4], std::thread::current());
    // Deliberately bind only the registry record, not the thread-local current Thread.
    COORDINATOR.bind(safepoint::current().unwrap(), references[5]);

    let operation = COORDINATOR
        .request_collection(safepoint::current().unwrap(), JavaThread::snapshot_roots())
        .unwrap();
    let roots = snapshot(&operation).unwrap();

    assert!(references.iter().all(|reference| roots.contains(reference)));
    operation.complete();
    threads::unregister_parker(references[4]);
    safepoint::retire();
}

#[test]
fn gc_roots_waited_monitor_survives_but_idle_monitor_is_not_a_root() {
    let _serial = safepoint::tests::SERIAL.lock().unwrap();
    let target = HEAP.create_array("[I", 0);
    let idle = HEAP.create_array("[I", 0);
    assert!(!crate::vm::monitor::holds_lock(idle));
    let (published, publication) = channel();
    let worker = std::thread::spawn(move || {
        safepoint::attach();
        crate::vm::monitor::enter(target);
        safepoint::observe(move |event| {
            if event == Event::SnapshotPublished {
                published.send(()).unwrap();
            }
        });
        crate::vm::monitor::wait(target, 0).unwrap();
        crate::vm::monitor::exit(target).unwrap();
        safepoint::retire();
    });
    publication.recv_timeout(Duration::from_secs(5)).unwrap();
    safepoint::attach();

    let operation = COORDINATOR
        .request_collection(safepoint::current().unwrap(), JavaThread::snapshot_roots())
        .unwrap();
    let roots = snapshot(&operation).unwrap();

    assert!(roots.contains(&target));
    assert!(!roots.contains(&idle));
    operation.complete();
    crate::vm::monitor::enter(target);
    crate::vm::monitor::notify(target).unwrap();
    crate::vm::monitor::exit(target).unwrap();
    safepoint::retire();
    worker.join().unwrap();
}
