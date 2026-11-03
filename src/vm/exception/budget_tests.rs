use super::*;
use crate::vm::gc::budget::AllocError;
use crate::vm::gc::test_allocations::{bootstrap_oom, measure};
use crate::vm::jni::{handles, java_thread::JavaThread};
use crate::vm::stack::stack_frame::ExceptionTable;
use jclassmodel::{exception_table::ExceptionTableRecord, CatchType};
use std::collections::BTreeMap;
use std::sync::Arc;

#[test]
fn gc_budget_emergency_error_install_transfer_and_unwind_allocate_nothing() {
    let emergency = bootstrap_oom();
    assert_eq!(
        HEAP.get_instance_name(emergency).unwrap(),
        "java/lang/OutOfMemoryError"
    );
    assert!(handles::global_roots().unwrap().contains(&emergency));
    let table = ExceptionTable::new(vec![
        ExceptionTableRecord::new(0, 1, 5, CatchType::Class("java/lang/Exception".into())),
        ExceptionTableRecord::new(
            0,
            1,
            7,
            CatchType::Class("java/lang/VirtualMachineError".into()),
        ),
    ]);
    let caller = StackFrame::new(
        Arc::new("catch:()V".into()),
        0,
        1,
        Arc::new(vec![0]),
        Arc::new("Test".into()),
        Arc::new(BTreeMap::new()),
        Arc::new(table),
    );
    let mut frames = StackFrames::new(vec![caller, StackFrame::for_test(0, 1)]);
    // Initialize thread-local storage before measuring failure delivery.
    assert_eq!(JavaThread::get_pending_exception(), None);

    let (result, allocations) = measure(usize::MAX, || {
        let failure = Error::from(AllocError::Capacity);
        let reference = failure.throwable_ref();
        handles::install_emergency_oom();
        let pending = JavaThread::take_pending_exception_rooted().unwrap();
        let handler = throw_exception_with_ref(pending.slot(), &mut frames);
        (reference, handler)
    });

    assert_eq!(
        allocations, 0,
        "emergency delivery must not touch the Rust allocator"
    );
    assert_eq!(result.0, Some(emergency));
    assert_eq!(result.1.unwrap().1, 7);
    assert_eq!(
        frames.last_mut().unwrap().pop::<Slot>(),
        Slot::Ref(emergency)
    );
    assert_eq!(JavaThread::get_pending_exception(), None);
    println!("emergency install + pending transfer + handler unwind: allocations={allocations}");
}

#[test]
fn gc_budget_emergency_uncaught_and_first_wins_paths_allocate_nothing() {
    let emergency = bootstrap_oom();
    JavaThread::try_set_pending_exception(73).unwrap();
    let mut frames = StackFrames::new(vec![StackFrame::for_test(0, 1)]);
    let (failure, allocations) = measure(usize::MAX, || {
        handles::install_emergency_oom();
        throw_exception_with_ref(Slot::Ref(emergency), &mut frames).unwrap_err()
    });
    assert_eq!(allocations, 0);
    assert_eq!(failure.throwable_ref(), Some(emergency));
    assert!(frames.is_empty());
    assert_eq!(JavaThread::take_pending_exception(), Some(73));
}
