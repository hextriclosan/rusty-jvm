use super::*;
use crate::vm::gc::budget::{aligned, bytes, AllocError, Budget, CHARGE_POLICY_VERSION};
use crate::vm::gc::test_allocations::measure;
use std::sync::{mpsc::channel, Arc, Barrier};
use std::time::Duration;

fn bounded(max: u64) -> Heap {
    let heap = Heap::default();
    heap.admission.lock().budget = Budget::new(max, true);
    heap
}

fn charge() -> u64 {
    ArrayLayout::new("[I", 2).unwrap().charge("[I").unwrap()
}

#[test]
fn gc_budget_exact_equality_commits_and_publishes_root() {
    let heap = bounded(charge());
    let mut destination = RootedSlots::new();
    let reference = heap.create_array("[I", 2, &mut destination).unwrap();
    let admission = heap.admission.lock();
    assert_eq!(CHARGE_POLICY_VERSION, 1);
    assert_eq!(admission.budget.used, charge());
    assert_eq!(admission.budget.reserved, 0);
    assert_eq!(destination[0], Slot::Ref(reference));
    assert!(JavaThread::temp_root_refs().contains(&reference));
}

#[test]
fn gc_budget_reservations_are_not_used_and_cancel_refunds() {
    let heap = bounded(charge());
    let reservation = heap.reserve(charge()).unwrap();
    assert_eq!(heap.admission.lock().budget.used, 0);
    assert_eq!(heap.admission.lock().budget.reserved, charge());
    reservation.cancel().unwrap();
    assert_eq!(heap.admission.lock().budget.reserved, 0);
    let reservation = heap.reserve(charge()).unwrap();
    drop(reservation);
    assert_eq!(heap.admission.lock().budget.reserved, 0);
}

#[test]
fn gc_budget_sweep_subtracts_recorded_charge_not_reservation() {
    let heap = bounded(charge() + 64);
    let mut destination = RootedSlots::new();
    destination.prepare_single_ref().unwrap();
    let reservation = heap.reserve(charge() + 64).unwrap();
    let array = Array::new("[I", 2).unwrap();
    let reference = reservation.commit(Arr(array), &mut destination).unwrap();
    assert_eq!(heap.admission.lock().budget.used, charge());
    assert_eq!(heap.remove_recorded(reference).unwrap(), charge());
    assert_eq!(heap.admission.lock().budget.used, 0);
    assert_eq!(heap.admission.lock().budget.reserved, 0);
    assert!(heap.data.get(reference).is_none());
    assert_eq!(
        heap.remove_recorded(reference),
        Err(AllocError::InvalidLayout)
    );
}

#[test]
fn gc_budget_overflow_rejection_preserves_all_counters() {
    let heap = bounded(u64::MAX);
    heap.admission.lock().budget.used = u64::MAX - 1;
    let held = heap.reserve(1).unwrap();
    assert!(matches!(heap.reserve(1), Err(AllocError::Arithmetic)));
    assert_eq!(heap.admission.lock().budget.used, u64::MAX - 1);
    assert_eq!(heap.admission.lock().budget.reserved, 1);
    drop(held);
    assert_eq!(aligned(u64::MAX), Err(AllocError::Arithmetic));
    assert_eq!(bytes(usize::MAX, 8), Err(AllocError::Arithmetic));
}

#[test]
fn gc_budget_commit_and_refund_arithmetic_is_checked() {
    let mut budget = Budget::new(u64::MAX, true);
    budget.used = u64::MAX;
    budget.reserved = 1;
    assert_eq!(budget.commit(1, 1), Err(AllocError::Arithmetic));
    assert_eq!(budget.used, u64::MAX);
    assert_eq!(budget.reserved, 1);
    assert_eq!(budget.cancel(2), Err(AllocError::Arithmetic));
    assert_eq!(budget.reserved, 1);
    budget.used = 0;
    assert_eq!(budget.reclaim(1), Err(AllocError::Arithmetic));
    assert_eq!(budget.used, 0);
}

#[test]
fn gc_budget_two_allocators_compete_for_one_remaining_slot() {
    let heap = Arc::new(bounded(charge() * 2));
    heap.create_array("[I", 2, &mut RootedSlots::new()).unwrap();
    let start = Arc::new(Barrier::new(3));
    let (attempted, attempts) = channel();
    let mut workers = Vec::new();
    let mut releases = Vec::new();
    for _ in 0..2 {
        let heap = Arc::clone(&heap);
        let start = Arc::clone(&start);
        let attempted = attempted.clone();
        let (release, gate) = channel();
        releases.push(release);
        workers.push(std::thread::spawn(move || {
            let mut destination = RootedSlots::new();
            destination.prepare_single_ref().unwrap();
            start.wait();
            let reservation = heap.reserve(charge());
            attempted.send(reservation.is_ok()).unwrap();
            // Hold even a successful reservation until BOTH attempts are observed.
            gate.recv_timeout(Duration::from_secs(5)).unwrap();
            match reservation {
                Ok(reservation) => {
                    reservation.commit(Arr(Array::new("[I", 2).unwrap()), &mut destination)
                }
                Err(error) => Err(error),
            }
        }));
    }
    start.wait();
    let first = attempts.recv_timeout(Duration::from_secs(5));
    let second = attempts.recv_timeout(Duration::from_secs(5));
    let reserved = heap.admission.lock().budget.reserved;
    for release in releases {
        release.send(()).unwrap();
    }
    let results: Vec<_> = workers
        .into_iter()
        .map(|worker| worker.join().unwrap())
        .collect();
    assert_eq!(
        [first.unwrap(), second.unwrap()]
            .into_iter()
            .filter(|ok| *ok)
            .count(),
        1,
        "only one allocator may reserve the remaining slot"
    );
    assert_eq!(reserved, charge());
    assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 1);
    assert!(results.contains(&Err(AllocError::Capacity)));
    assert_eq!(heap.admission.lock().budget.used, charge() * 2);
    assert_eq!(heap.admission.lock().budget.reserved, 0);
}

#[test]
fn gc_budget_key_exhaustion_never_wraps_or_reuses_swept_key() {
    let heap = bounded(u64::MAX);
    heap.admission.lock().next = Some(i32::MAX);
    let last = heap.create_array("[I", 2, &mut RootedSlots::new()).unwrap();
    assert_eq!(last, i32::MAX);
    heap.remove_recorded(last).unwrap();
    assert!(matches!(
        heap.reserve(charge()),
        Err(AllocError::IdentityExhausted)
    ));
    assert_eq!(heap.admission.lock().next, None);
    assert!(heap.data.get(0).is_none());
    assert!(heap.data.get(i32::MIN).is_none());
}

#[test]
fn gc_budget_occupied_and_nonpositive_keys_are_rejected() {
    let heap = bounded(u64::MAX);
    let reference = heap
        .create_array_with_values("[I", &[17, 23], &mut RootedSlots::new())
        .unwrap();
    for key in [reference, 0, -1] {
        heap.admission.lock().next = Some(key);
        assert!(matches!(
            heap.reserve(charge()),
            Err(AllocError::IdentityExhausted)
        ));
    }
    assert_eq!(heap.get_array_value(reference, 1).unwrap(), vec![23]);
    assert_eq!(heap.admission.lock().budget.used, charge());
}

#[test]
fn gc_budget_host_payload_failure_cancels_before_error_delivery() {
    let heap = bounded(charge());
    let mut destination = RootedSlots::new();
    destination.prepare_single_ref().unwrap();
    let (result, attempts) = measure(0, || heap.create_array("[I", 2, &mut destination));
    assert!(matches!(
        result.unwrap_err().kind(),
        crate::vm::error::ErrorKind::Allocation(AllocError::HostAllocation)
    ));
    assert_eq!(attempts, 1);
    assert_eq!(heap.admission.lock().budget.used, 0);
    assert_eq!(heap.admission.lock().budget.reserved, 0);
    assert!(destination.is_empty());
}

#[test]
fn gc_budget_clone_uses_admission_before_copying_payload() {
    let heap = bounded(charge());
    let source = heap
        .create_array_with_values("[I", &[17, 23], &mut RootedSlots::new())
        .unwrap();
    let mut destination = RootedSlots::new();
    let result = heap.clone_instance(source, &mut destination);
    assert!(matches!(
        result.unwrap_err().kind(),
        crate::vm::error::ErrorKind::Allocation(AllocError::Capacity)
    ));
    assert!(destination.is_empty());
    assert_eq!(heap.admission.lock().budget.used, charge());
    heap.admission.lock().budget.max = charge() * 2;
    heap.clone_instance(source, &mut destination).unwrap();
    assert_eq!(
        heap.get_array_value(destination[0].value(), 1).unwrap(),
        vec![23]
    );
    assert_eq!(heap.admission.lock().budget.used, charge() * 2);
}

#[test]
fn gc_budget_intern_growth_is_charged_and_replacement_does_not_grow() {
    let charge = charge::intern("key".len()).unwrap();
    let heap = bounded(charge);
    heap.put_const_string_ref("key", 1).unwrap();
    assert_eq!(heap.put_const_string_ref("key", 2).unwrap(), Some(1));
    assert_eq!(heap.admission.lock().budget.used, charge);
    assert!(heap.put_const_string_ref("other", 3).is_err());
    assert_eq!(heap.get_const_string_ref("key"), Some(2));
    assert_eq!(heap.get_const_string_ref("other"), None);
}

#[test]
fn gc_budget_prepared_destination_publication_does_not_allocate() {
    let mut destination = RootedSlots::new();
    destination.prepare_single_ref().unwrap();
    let (_, allocations) = measure(usize::MAX, || destination.publish_single_ref(71));
    assert_eq!(allocations, 0);
    assert_eq!(&*destination, &[Slot::Ref(71)]);
    assert!(JavaThread::temp_root_refs().contains(&71));
}

#[test]
fn gc_budget_bootstrap_is_bounded_before_uniform_accounting_only_runtime() {
    let heap = bounded(charge());
    heap.create_array("[I", 2, &mut RootedSlots::new()).unwrap();
    assert!(matches!(heap.reserve(charge()), Err(AllocError::Capacity)));
    assert_eq!(heap.admission.lock().budget.used, charge());
    heap.finish_emergency_bootstrap().unwrap();
    heap.create_array_with_values("[I", &[3, 5], &mut RootedSlots::new())
        .unwrap();
    assert_eq!(heap.admission.lock().budget.used, charge() * 2);
    assert_eq!(heap.admission.lock().budget.reserved, 0);
}
