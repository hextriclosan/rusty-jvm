use super::*;
use crate::vm::jni::buffer_leases;
use crate::vm::jni::local_frames::NativeFrame;
use crate::vm::stack::rooted_slots::RootedSlots;
use crate::vm::stack::slot::Slot;
use jni_sys::{JNI_ABORT, JNI_COMMIT};

fn record() -> HandleRecord {
    HandleRecord {
        kind: ReferenceKind::Global,
        target: 1,
        owner_thread: None,
        frame: None,
    }
}

#[test]
fn gc_roots_jni_locals_leases_exceptions_transfers_and_strong_globals_are_complete() {
    let native = NativeFrame::enter().unwrap();
    let references: Vec<_> = (0..10)
        .map(|_| {
            HEAP.create_array(
                "[I",
                0,
                &mut crate::vm::stack::rooted_slots::RootedSlots::new(),
            )
            .unwrap()
        })
        .collect();
    let local = new_local(references[0]);
    let global_local = new_local(references[1]);
    let global = new_reference(global_local, ReferenceKind::Global);
    delete_reference(global_local, ReferenceKind::Local);
    let weak_local = new_local(references[2]);
    let weak = new_reference(weak_local, ReferenceKind::WeakGlobal);
    delete_reference(weak_local, ReferenceKind::Local);
    let lease = buffer_leases::copy(references[3], &[], align_of::<i32>());
    assert!(!lease.is_null());
    JavaThread::force_set_pending_exception(references[4]);
    JavaThread::set_scoped_value_cache(references[5]);
    let temporary =
        JavaThread::hold_temp_roots(vec![Slot::Ref(references[6]), Slot::Value(references[9])]);
    let transfer = RootedSlots::from_slots(vec![Slot::Ref(references[7])]);
    let exception = crate::vm::error::Error::uncaught_exception(Slot::Ref(references[8]));
    let emergency = HEAP
        .create_array(
            "[I",
            0,
            &mut crate::vm::stack::rooted_slots::RootedSlots::new(),
        )
        .unwrap();
    EMERGENCY_OOM.set(emergency).unwrap();

    let current = JavaThread::snapshot_roots().roots;
    let globals = global_roots().unwrap();

    for index in [0, 3, 4, 5, 6, 7, 8] {
        assert!(
            current.contains(&references[index]),
            "missing category {index}"
        );
    }
    assert!(!current.contains(&references[1]));
    assert!(!current.contains(&references[2]));
    assert!(!current.contains(&references[9]));
    assert!(globals.contains(&references[1]));
    assert!(globals.contains(&emergency));
    assert!(!globals.contains(&references[2]));
    delete_reference(local, ReferenceKind::Local);
    delete_reference(global, ReferenceKind::Global);
    delete_reference(weak, ReferenceKind::WeakGlobal);
    buffer_leases::release(lease, references[3], None);
    JavaThread::take_pending_exception();
    JavaThread::set_scoped_value_cache(0);
    drop(exception);
    drop(transfer);
    drop(temporary);
    drop(native);
    assert!(JavaThread::snapshot_roots().roots.is_empty());
}

#[test]
fn gc_roots_snapshot_failure_aborts_operation_without_partial_roots_or_deadlock() {
    use crate::vm::gc::roots;
    use crate::vm::safepoint::{self, COORDINATOR};
    let _serial = safepoint::tests::SERIAL.lock().unwrap();
    safepoint::attach();
    let owner = safepoint::current().unwrap();
    let operation = COORDINATOR
        .request_collection(owner, JavaThread::snapshot_roots())
        .unwrap();
    let guard = table();

    let failed = roots::snapshot(&operation);

    assert!(failed.is_err());
    drop(guard);
    drop(operation);
    assert!(COORDINATOR.running(owner));
    let next = COORDINATOR
        .request_collection(owner, JavaThread::snapshot_roots())
        .unwrap();
    assert!(roots::snapshot(&next).is_ok());
    next.complete();
    safepoint::retire();
}

#[test]
fn jni_handles_exhaustion_never_reuses_deleted_tokens() {
    let mut table = HandleTable::new();
    table.next = Some(usize::MAX - 1);
    let first = table.insert(record()).unwrap();
    table.records.remove(&first);
    let last = table.insert(record()).unwrap();
    assert_eq!(last, usize::MAX);
    assert_eq!(table.insert(record()), Err(HandleError::OutOfMemory));
    assert_eq!(table.resolve(first), Err(HandleError::InvalidReference));
    assert_eq!(table.resolve(last), Ok(1));
    assert_eq!(table.resolve(0), Ok(0));
}

#[test]
fn jni_handles_capacity_failure_preserves_table_and_counter() {
    let mut table = HandleTable::new();
    table.next = Some(usize::MAX);
    assert_eq!(table.reserve(2), Err(HandleError::OutOfMemory));
    assert!(table.records.is_empty());
    assert_eq!(table.insert(record()), Ok(usize::MAX));
}

#[test]
fn gc_budget_jni_token_exhaustion_and_occupied_tokens_do_not_alias() {
    let mut table = HandleTable::new();
    table.next = Some(usize::MAX);
    assert_eq!(table.insert(record()), Ok(usize::MAX));
    assert_eq!(table.insert(record()), Err(HandleError::OutOfMemory));
    table.next = Some(usize::MAX);
    let mut replacement = record();
    replacement.target = 2;
    assert_eq!(table.insert(replacement), Err(HandleError::OutOfMemory));
    assert_eq!(table.resolve(usize::MAX), Ok(1));
}

#[test]
fn gc_budget_jni_exhaustion_installs_emergency_without_allocation() {
    use crate::vm::gc::test_allocations::{bootstrap_oom, measure};
    let emergency = bootstrap_oom();
    assert_eq!(JavaThread::get_pending_exception(), None);
    let (_, allocations) = measure(usize::MAX, || HandleError::OutOfMemory.set_pending());
    assert_eq!(allocations, 0);
    assert_eq!(JavaThread::take_pending_exception(), Some(emergency));
}

#[test]
fn jni_handles_weak_clearing_keeps_identity_and_kind() {
    let frame = NativeFrame::enter().unwrap();
    let target = HEAP
        .create_array(
            "[I",
            0,
            &mut crate::vm::stack::rooted_slots::RootedSlots::new(),
        )
        .unwrap();
    let local = new_local(target);
    let first = new_reference(local, ReferenceKind::WeakGlobal);
    let second = new_reference(local, ReferenceKind::WeakGlobal);
    let strong = new_reference(first, ReferenceKind::Global);
    assert_ne!(first, second);
    clear_weak_references(|object| object != target);
    assert_eq!(resolve(first), Some(0));
    assert_eq!(resolve(second), Some(0));
    assert!(matches!(
        reference_type(second),
        jobjectRefType::JNIWeakGlobalRefType
    ));
    assert!(new_reference(second, ReferenceKind::Local).is_null());
    assert_eq!(resolve(strong), Some(target));
    delete_reference(first, ReferenceKind::WeakGlobal);
    assert!(matches!(
        reference_type(first),
        jobjectRefType::JNIInvalidRefType
    ));
    assert!(matches!(
        reference_type(second),
        jobjectRefType::JNIWeakGlobalRefType
    ));
    delete_reference(second, ReferenceKind::WeakGlobal);
    delete_reference(strong, ReferenceKind::Global);
    drop(frame);
    assert!(local_roots().is_empty());
}

#[test]
fn jni_handles_locals_are_thread_owned_but_globals_are_shared() {
    let _frame = NativeFrame::enter().unwrap();
    let target = HEAP
        .create_array(
            "[I",
            0,
            &mut crate::vm::stack::rooted_slots::RootedSlots::new(),
        )
        .unwrap();
    let local = new_local(target);
    let global = new_reference(local, ReferenceKind::Global);
    let (local_token, global_token) = (local as usize, global as usize);
    std::thread::spawn(move || {
        assert!(matches!(
            reference_type(as_object(local_token)),
            jobjectRefType::JNIInvalidRefType
        ));
        assert_eq!(resolve(as_object(global_token)), Some(target));
    })
    .join()
    .unwrap();
    assert!(global_roots().unwrap().contains(&target));
    delete_reference(global, ReferenceKind::Global);
}

#[test]
fn jni_handles_pop_publishes_fresh_parent_local_before_child_dies() {
    let native = NativeFrame::enter().unwrap();
    let target = HEAP
        .create_array(
            "[I",
            0,
            &mut crate::vm::stack::rooted_slots::RootedSlots::new(),
        )
        .unwrap();
    let outer = new_local(target);
    JavaThread::with_jni_frames(|frames| frames.push(2, false)).unwrap();
    let child = new_local(target);
    delete_reference(outer, ReferenceKind::Local);
    let escaped = as_object(JavaThread::with_jni_frames(|frames| frames.pop(child)).unwrap());
    assert_ne!(child, escaped);
    assert!(matches!(
        reference_type(child),
        jobjectRefType::JNIInvalidRefType
    ));
    assert!(matches!(
        reference_type(outer),
        jobjectRefType::JNIInvalidRefType
    ));
    assert_eq!(resolve(escaped), Some(target));
    let result = RootedSlots::from_slots(vec![Slot::Ref(resolve(escaped).unwrap())]);
    drop(native);
    assert!(matches!(
        reference_type(escaped),
        jobjectRefType::JNIInvalidRefType
    ));
    assert!(JavaThread::temp_root_refs().contains(&target));
    drop(result);
    assert!(JavaThread::temp_root_refs().is_empty());
}

#[test]
fn jni_handles_failed_pop_preserves_child_and_parent_records() {
    let _native = NativeFrame::enter().unwrap();
    let target = HEAP
        .create_array(
            "[I",
            0,
            &mut crate::vm::stack::rooted_slots::RootedSlots::new(),
        )
        .unwrap();
    for _ in 0..65_536 {
        assert!(!new_local(target).is_null());
    }
    JavaThread::with_jni_frames(|frames| frames.push(1, false)).unwrap();
    let child = new_local(target);
    assert_eq!(
        JavaThread::with_jni_frames(|frames| frames.pop(child)),
        Err(HandleError::OutOfMemory)
    );
    assert_eq!(resolve(child), Some(target));
    assert_eq!(local_roots().len(), 65_537);
    assert_eq!(
        JavaThread::with_jni_frames(|frames| frames.pop(std::ptr::null_mut())),
        Ok(0)
    );
}

#[test]
fn jni_handles_leases_survive_local_pop_commit_and_abort() {
    let native = NativeFrame::enter().unwrap();
    let target = HEAP
        .create_array_with_values(
            "[I",
            &[7],
            &mut crate::vm::stack::rooted_slots::RootedSlots::new(),
        )
        .unwrap();
    JavaThread::with_jni_frames(|frames| frames.push(1, false)).unwrap();
    let local = new_local(target);
    let pointer = buffer_leases::copy(target, &7i32.to_ne_bytes(), align_of::<i32>());
    JavaThread::with_jni_frames(|frames| frames.pop(std::ptr::null_mut())).unwrap();
    assert!(matches!(
        reference_type(local),
        jobjectRefType::JNIInvalidRefType
    ));
    assert!(JavaThread::temp_root_refs().contains(&target));
    // SAFETY: the lease owns one aligned initialized i32 until the final release.
    unsafe { pointer.cast::<i32>().write(11) };
    buffer_leases::release(pointer, target, Some(JNI_COMMIT));
    assert_eq!(HEAP.get_array_value(target, 0).unwrap(), vec![11]);
    assert!(JavaThread::temp_root_refs().contains(&target));
    unsafe { pointer.cast::<i32>().write(19) };
    buffer_leases::release(pointer, target, Some(JNI_ABORT));
    assert_eq!(HEAP.get_array_value(target, 0).unwrap(), vec![11]);
    assert!(!JavaThread::temp_root_refs().contains(&target));
    drop(native);
}

#[test]
fn jni_handles_native_cleanup_discards_unreleased_buffers_and_frames() {
    let native = NativeFrame::enter().unwrap();
    let target = HEAP
        .create_array(
            "[J",
            1,
            &mut crate::vm::stack::rooted_slots::RootedSlots::new(),
        )
        .unwrap();
    let pointer = buffer_leases::copy(target, &0i64.to_ne_bytes(), align_of::<i64>());
    assert!(!pointer.is_null());
    JavaThread::with_jni_frames(|frames| frames.push(1, false)).unwrap();
    let local = new_local(target);
    JavaThread::force_set_pending_exception(target);
    drop(native);
    assert!(matches!(
        reference_type(local),
        jobjectRefType::JNIInvalidRefType
    ));
    assert!(JavaThread::with_jni_leases(|leases| leases.is_empty()));
    assert_eq!(JavaThread::take_pending_exception(), Some(target));
    assert!(local_roots().is_empty());
}
