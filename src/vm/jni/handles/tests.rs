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
fn jni_handles_weak_clearing_keeps_identity_and_kind() {
    let frame = NativeFrame::enter().unwrap();
    let target = HEAP.create_array("[I", 0);
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
    let target = HEAP.create_array("[I", 0);
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
    assert!(global_roots().contains(&target));
    delete_reference(global, ReferenceKind::Global);
}

#[test]
fn jni_handles_pop_publishes_fresh_parent_local_before_child_dies() {
    let native = NativeFrame::enter().unwrap();
    let target = HEAP.create_array("[I", 0);
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
    let target = HEAP.create_array("[I", 0);
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
    let target = HEAP.create_array_with_values("[I", &[7]);
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
    let target = HEAP.create_array("[J", 1);
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
