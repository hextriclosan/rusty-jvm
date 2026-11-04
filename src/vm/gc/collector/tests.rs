use super::*;
use crate::vm::gc::layout::{InstanceFields, InstanceLayout};
use crate::vm::gc::references::REFERENCE;
use crate::vm::gc::test_allocations::{measure, measure_bytes};
use crate::vm::heap::construction::InstancePlan;
use crate::vm::jni::local_frames::NativeFrame;
use crate::vm::method_area::field::{FieldInfo, FieldValue};
use crate::vm::safepoint::{Coordinator, Snapshot};
use crate::vm::stack::rooted_slots::RootedSlots;
use crate::vm::stack::slot::Slot;
use crate::vm::stack::stack_frame::{StackFrame, StackFrames};
use std::sync::{mpsc::channel, Arc};
use std::time::Duration;

fn array(heap: &Heap, descriptor: &str, values: &[i32]) -> i32 {
    heap.create_array_with_values(descriptor, values, &mut RootedSlots::new())
        .unwrap()
}

fn reference(heap: &Heap, kind: &str, target: i32) -> i32 {
    let fields = [
        ("referent", "Ljava/lang/Object;"),
        ("queue", "Ljava/lang/ref/ReferenceQueue;"),
        ("next", "Ljava/lang/ref/Reference;"),
        ("discovered", "Ljava/lang/ref/Reference;"),
    ];
    let metadata: Vec<_> = fields
        .iter()
        .map(|(name, descriptor)| {
            Arc::new(FieldInfo::new(
                descriptor.parse().unwrap(),
                0,
                REFERENCE.into(),
                (*name).into(),
            ))
        })
        .collect();
    let storage: InstanceFields = [
        (
            REFERENCE.into(),
            metadata
                .iter()
                .map(|field| {
                    (
                        field.name().into(),
                        FieldValue::new(field.type_descriptor().clone()).unwrap(),
                    )
                })
                .collect(),
        ),
        (kind.into(), Default::default()),
    ]
    .into_iter()
    .collect();
    let layout = Arc::new(
        InstanceLayout::new([(REFERENCE.into(), metadata), (kind.into(), vec![])]).unwrap(),
    );
    let object = heap
        .create_instance(
            InstancePlan::new(usize::MAX, Arc::new(storage), layout).unwrap(),
            &mut RootedSlots::new(),
        )
        .unwrap();
    heap.set_reference_link(object, "referent", target).unwrap();
    object
}

fn cycle(heap: &Heap, roots: &[i32], pending: &PendingList, cause: Cause) -> Result<Stats> {
    let coordinator = Arc::new(Coordinator::new());
    let owner = coordinator.startup(0);
    coordinator.resume(owner);
    let operation = coordinator
        .request_collection(
            owner,
            Snapshot {
                roots: roots.to_vec(),
            },
        )
        .unwrap();
    let mut snapshot: Vec<_> = operation.root_refs().collect();
    snapshot.push(pending.snapshot()?);
    let result = collect_snapshot(heap, &snapshot, pending, cause, &operation);
    if result.is_ok() {
        operation.complete();
    }
    result
}

fn chain(heap: &Heap, pending: &PendingList) -> Vec<i32> {
    let mut result = Vec::new();
    let mut node = pending.snapshot().unwrap();
    while node != 0 {
        assert!(!result.contains(&node), "pending chain must be acyclic");
        result.push(node);
        node = heap
            .get_object_field_value(node, REFERENCE, "discovered")
            .unwrap()[0];
    }
    result
}

#[test]
fn gc_collect_preserves_live_cycles_and_reclaims_dead_cycles_and_primitive_bits() {
    // Given a live cycle, a dead cycle, and an integer equal to a dead heap ID.
    let heap = Heap::default();
    let pending = PendingList::new();
    let live = array(&heap, "[Ljava/lang/Object;", &[0]);
    let child = array(&heap, "[Ljava/lang/Object;", &[live]);
    heap.set_array_value(live, 0, vec![child]).unwrap();
    let dead = array(&heap, "[Ljava/lang/Object;", &[0]);
    heap.set_array_value(dead, 0, vec![dead]).unwrap();
    let bits = array(&heap, "[I", &[dead]);
    let before = heap.memory().1;
    // When the actual traversal runs under an Operation.
    let stats = cycle(&heap, &[live, bits], &pending, Cause::Explicit).unwrap();
    // Then only typed edges retain targets and exact recorded charges are refunded.
    assert_eq!(stats.live, 3);
    assert!(heap.contains(live) && heap.contains(child) && heap.contains(bits));
    assert!(!heap.contains(dead));
    assert_eq!(before - heap.memory().1, stats.reclaimed);
}

#[test]
fn gc_collect_traces_deep_chains_without_native_recursion() {
    let heap = Heap::default();
    let pending = PendingList::new();
    let mut root = 0;
    for _ in 0..25_000 {
        root = array(&heap, "[Ljava/lang/Object;", &[root]);
    }
    let stats = cycle(&heap, &[root], &pending, Cause::Explicit).unwrap();
    assert_eq!(stats.live, 25_000);
    assert_eq!(stats.reclaimed, 0);
}

#[test]
fn gc_collect_duplicate_array_edges_do_not_expand_scratch() {
    let heap = Heap::default();
    let pending = PendingList::new();
    let target = array(&heap, "[I", &[7]);
    let fanout = array(&heap, "[Ljava/lang/Object;", &vec![target; 100_000]);
    // Repeated slots are deduplicated while the single array guard is held.
    let stats = cycle(&heap, &[fanout], &pending, Cause::Explicit).unwrap();
    assert_eq!(stats.live, 2);
    assert!(stats.scratch_bytes < 1024);
    assert!(heap.contains(target));
}

#[test]
fn gc_collect_rejects_jdk_reference_descriptor_drift_before_publication() {
    let fields = [
        ("referent", "Ljava/lang/Object;"),
        ("queue", "I"),
        ("next", "Ljava/lang/ref/Reference;"),
        ("discovered", "Ljava/lang/ref/Reference;"),
    ];
    let metadata = fields
        .iter()
        .map(|(name, descriptor)| {
            Arc::new(FieldInfo::new(
                descriptor.parse().unwrap(),
                0,
                REFERENCE.into(),
                (*name).into(),
            ))
        })
        .collect();
    assert!(InstanceLayout::new([(REFERENCE.into(), metadata)]).is_err());
}

#[test]
fn gc_collect_nested_soft_closure_reaches_fixed_point_and_pressure_clears_it() {
    let heap = Heap::default();
    let pending = PendingList::new();
    let leaf = array(&heap, "[I", &[123]);
    let inner = reference(&heap, "java/lang/ref/SoftReference", leaf);
    let outer = reference(&heap, "java/lang/ref/SoftReference", inner);
    let weak = reference(&heap, "java/lang/ref/WeakReference", leaf);
    let stats = cycle(&heap, &[outer, weak], &pending, Cause::Explicit).unwrap();
    assert_eq!(stats.live, 4);
    assert_eq!(stats.discovered, 0);
    assert_eq!(
        heap.get_object_field_value(weak, REFERENCE, "referent")
            .unwrap(),
        vec![leaf]
    );
    let pressure = cycle(&heap, &[outer, weak], &pending, Cause::Pressure).unwrap();
    assert_eq!(pressure.live, 2);
    assert_eq!(pressure.discovered, 2);
    assert!(!heap.contains(inner) && !heap.contains(leaf));
    assert_eq!(chain(&heap, &pending).len(), 2);
}

#[test]
fn gc_collect_corrupt_strong_edge_aborts_before_clearing_or_sweep() {
    let heap = Heap::default();
    let pending = PendingList::new();
    let target = array(&heap, "[I", &[7]);
    let weak = reference(&heap, "java/lang/ref/WeakReference", target);
    let bad = array(&heap, "[Ljava/lang/Object;", &[i32::MAX]);
    let before = heap.memory().1;
    assert!(cycle(&heap, &[bad, weak], &pending, Cause::Explicit).is_err());
    assert_eq!(heap.memory().1, before);
    assert!(heap.contains(target) && heap.contains(bad));
    assert_eq!(
        heap.get_object_field_value(weak, REFERENCE, "referent")
            .unwrap(),
        vec![target]
    );
    assert_eq!(pending.snapshot().unwrap(), 0);
}

#[test]
fn gc_collect_pending_batches_survive_three_epochs_and_nulls_are_never_rediscovered() {
    let _serial = safepoint::tests::SERIAL.lock().unwrap();
    safepoint::attach();
    let first_target = array(&HEAP, "[I", &[1]);
    let first = reference(&HEAP, "java/lang/ref/WeakReference", first_target);
    {
        let _root = RootedSlots::from_slots(vec![Slot::Ref(first)]);
        assert_eq!(collect(Cause::Explicit).unwrap().discovered, 1);
    }
    // The FIRST node is now owned solely by gc/roots.rs's VM pending root.
    let second_target = array(&HEAP, "[I", &[2]);
    let second = reference(&HEAP, "java/lang/ref/PhantomReference", second_target);
    {
        let _root = RootedSlots::from_slots(vec![Slot::Ref(second)]);
        assert_eq!(collect(Cause::Explicit).unwrap().discovered, 1);
    }
    assert_eq!(collect(Cause::Explicit).unwrap().discovered, 0);
    let nodes = chain(&HEAP, &PENDING);
    assert_eq!(nodes.len(), 2);
    assert!(nodes.contains(&first) && nodes.contains(&second));
    assert!(!HEAP.contains(first_target) && !HEAP.contains(second_target));
    // Native-frame publication precedes head removal, and survives the next stop.
    {
        let _frame = NativeFrame::enter().unwrap();
        let head = PENDING.take();
        assert_eq!(head, nodes[0]);
        assert_eq!(PENDING.snapshot().unwrap(), 0);
        collect(Cause::Explicit).unwrap();
        assert!(nodes.iter().all(|&node| HEAP.contains(node)));
    }
    collect(Cause::Explicit).unwrap();
    assert!(nodes.iter().all(|&node| !HEAP.contains(node)));
    safepoint::retire();
}

#[test]
fn gc_collect_explicit_clear_and_final_references_are_not_delivered() {
    let heap = Heap::default();
    let pending = PendingList::new();
    let target = array(&heap, "[I", &[5]);
    let final_ref = reference(&heap, "java/lang/ref/FinalReference", target);
    let cleared = reference(&heap, "java/lang/ref/WeakReference", 0);
    let phantom = reference(&heap, "java/lang/ref/PhantomReference", 0);
    let stats = cycle(
        &heap,
        &[final_ref, cleared, phantom],
        &pending,
        Cause::Pressure,
    )
    .unwrap();
    assert_eq!(stats.discovered, 0);
    assert_eq!(stats.live, 4);
    assert_eq!(pending.snapshot().unwrap(), 0);
}

#[test]
fn gc_collect_failed_pending_snapshot_skips_once_and_releases_the_operation() {
    let _serial = safepoint::tests::SERIAL.lock().unwrap();
    safepoint::attach();
    let dead = array(&HEAP, "[I", &[1]);
    let before = HEAP.memory().1;
    {
        let _held = PENDING.for_collection().unwrap();
        assert!(collect(Cause::Explicit).is_none());
        assert!(!COORDINATOR.requested());
        assert!(COORDINATOR.running(safepoint::current().unwrap()));
        assert_eq!(HEAP.memory().1, before);
        assert!(HEAP.contains(dead));
    }
    assert!(collect(Cause::Explicit).is_some());
    assert!(!HEAP.contains(dead));
    safepoint::retire();
}

#[test]
fn gc_collect_pending_wait_returns_running_after_collection_publishes() {
    let _serial = safepoint::tests::SERIAL.lock().unwrap();
    let (published, publication) = channel();
    let (done, completion) = channel();
    let worker = std::thread::spawn(move || {
        safepoint::attach();
        safepoint::observe(move |event| {
            if event == safepoint::Event::SnapshotPublished {
                published.send(()).unwrap();
            }
        });
        PENDING.wait();
        done.send(COORDINATOR.running(safepoint::current().unwrap()))
            .unwrap();
        safepoint::retire();
    });
    publication.recv_timeout(Duration::from_secs(5)).unwrap();
    safepoint::attach();
    assert!(!PENDING.has_pending()); // pairs with the waiter's condvar unlock
    let target = array(&HEAP, "[I", &[1]);
    let weak = reference(&HEAP, "java/lang/ref/WeakReference", target);
    let root = RootedSlots::from_slots(vec![Slot::Ref(weak)]);
    assert_eq!(collect(Cause::Explicit).unwrap().discovered, 1);
    // The collector must be Safe while waiting for worker retirement too.
    safepoint::blocking(|| {
        assert!(completion.recv_timeout(Duration::from_secs(5)).unwrap());
        worker.join().unwrap();
    });
    drop(root);
    safepoint::retire();
}

#[test]
fn gc_collect_prunes_idle_monitors_but_keeps_owned_identity() {
    let _serial = safepoint::tests::SERIAL.lock().unwrap();
    safepoint::attach();
    let before = crate::vm::monitor::record_count();
    let idle = [
        array(&HEAP, "[I", &[]),
        array(&HEAP, "[I", &[]),
        array(&HEAP, "[I", &[]),
    ];
    let owned = array(&HEAP, "[I", &[]);
    for reference in idle {
        crate::vm::monitor::holds_lock(reference);
    }
    crate::vm::monitor::enter(owned);
    assert_eq!(crate::vm::monitor::record_count(), before + idle.len() + 1);
    collect(Cause::Explicit).unwrap();
    assert_eq!(crate::vm::monitor::record_count(), before + 1);
    for reference in idle {
        assert!(!HEAP.contains(reference));
        assert!(!crate::vm::monitor::has_record(reference));
    }
    assert!(HEAP.contains(owned));
    assert!(crate::vm::monitor::holds_lock(owned));
    crate::vm::monitor::exit(owned).unwrap();
    collect(Cause::Explicit).unwrap();
    assert!(!crate::vm::monitor::has_record(owned));
    assert_eq!(crate::vm::monitor::record_count(), before);
    safepoint::retire();
}

#[test]
fn gc_collect_keeps_synchronized_monitor_after_receiver_slot_overwrite() {
    use crate::vm::safepoint;
    use crate::vm::stack::stack_frame::{StackFrame, StackFrames};

    let _serial = safepoint::tests::SERIAL.lock().unwrap();
    safepoint::attach();
    let mut allocation_roots = RootedSlots::new();
    let monitor = HEAP.create_array("[I", 0, &mut allocation_roots).unwrap();
    let identity_hash = crate::vm::system_native::object::identity_hashcode(monitor).unwrap();
    let mut frame = StackFrame::for_test(1, 0);
    frame.set_synchronized_method(true, false);
    frame.set_local(0, Slot::Ref(monitor));
    frame.set_synchronized_monitor(monitor);
    frame.set_local_raw(0, 99);
    drop(allocation_roots);
    let mut frames = StackFrames::new(vec![frame]);
    let _frames = JavaThread::register_stack_frames(&mut frames);

    collect(Cause::Explicit).unwrap();

    assert!(HEAP.contains(monitor));
    assert_eq!(
        crate::vm::system_native::object::identity_hashcode(monitor).unwrap(),
        identity_hash
    );
    safepoint::retire();
}

#[test]
fn gc_collect_synchronized_method_monitor_field_is_a_root() {
    let _serial = safepoint::tests::SERIAL.lock().unwrap();
    safepoint::attach();
    let monitor_ref = array(&HEAP, "[I", &[]);
    let _dead = array(&HEAP, "[I", &[7]);

    crate::vm::monitor::enter(monitor_ref);
    let mut frame = StackFrame::for_test(1, 0);
    frame.set_local_raw(0, 0);
    frame.set_synchronized_method(true, false);
    frame.set_synchronized_monitor(monitor_ref);
    let mut frames = StackFrames::new(vec![frame]);
    let _frames = JavaThread::register_stack_frames(&mut frames);

    // The real monitor owner root is intentionally removed so this frame field
    // is the only collector-visible edge to the still-owned object.
    JavaThread::unroot_monitor(monitor_ref);
    let stats = collect(Cause::Explicit).unwrap();
    assert!(stats.reclaimed > 0);
    assert!(HEAP.contains(monitor_ref));
    assert!(crate::vm::monitor::holds_lock(monitor_ref));

    // Restore the monitor bookkeeping pair before the normal exit path.
    JavaThread::root_monitor(monitor_ref);
    crate::vm::monitor::exit(monitor_ref).unwrap();
    safepoint::retire();
}

#[test]
fn gc_collect_blocked_worker_local_reference_survives_real_cycle() {
    use crate::vm::safepoint::{self, Blocking, Event, Reason};
    use crate::vm::stack::slot::Slot;
    use crate::vm::stack::stack_frame::{StackFrame, StackFrames};
    use std::sync::mpsc::channel;
    use std::time::Duration;

    let _serial = safepoint::tests::SERIAL.lock().unwrap();
    safepoint::attach();
    let (events, observed) = channel::<String>();
    let (published, publication) = channel();
    let (release, released) = channel();
    let (survived, survival) = channel();
    let worker_events = events.clone();
    let worker = std::thread::spawn(move || {
        safepoint::attach();
        let mut rooted = RootedSlots::new();
        let only_root = HEAP.create_array("[I", 1, &mut rooted).unwrap();
        let _unreachable = HEAP.create_array("[I", 1, &mut RootedSlots::new()).unwrap();
        let mut frame = StackFrame::for_test(1, 0);
        frame.set_local(0, Slot::Ref(only_root));
        drop(rooted);
        let mut frames = StackFrames::new(vec![frame]);
        let _frames = JavaThread::register_stack_frames(&mut frames);
        safepoint::observe(move |event| {
            if event == Event::SnapshotPublished {
                worker_events.send(format!("worker:{event:?}")).unwrap();
                published.send(()).unwrap();
            } else if matches!(event, Event::Resumed | Event::GuardReacquired) {
                worker_events.send(format!("worker:{event:?}")).unwrap();
            }
        });

        let blocked = Blocking::enter(Reason::Blocking);
        released
            .recv_timeout(Duration::from_secs(5))
            .expect("test wake was not delivered");
        drop(blocked);
        let retained = frames
            .last_mut()
            .map(|frame| frame.get_local::<Slot>(0))
            .is_some_and(|slot| slot.is_ref() && HEAP.get_array_len(slot.value()).is_ok());
        survived.send(retained).unwrap();
        safepoint::retire();
    });

    publication
        .recv_timeout(Duration::from_secs(5))
        .expect("worker did not publish its blocked root snapshot");
    events.send("request:collect".to_string()).unwrap();
    safepoint::observe({
        let owner_events = events.clone();
        move |event| {
            if matches!(
                event,
                Event::SnapshotPublished | Event::Election | Event::Resumed
            ) {
                owner_events.send(format!("owner:{event:?}")).unwrap();
            }
        }
    });
    let stats = collect(Cause::Explicit).expect("collection rendezvous was skipped");
    assert!(
        stats.reclaimed > 0,
        "the cycle reclaimed no unreachable object"
    );
    assert!(
        survival.try_recv().is_err(),
        "worker woke before the explicit wake"
    );
    events.send("wake:release".to_string()).unwrap();
    release.send(()).unwrap();
    assert!(
        survival
            .recv_timeout(Duration::from_secs(5))
            .expect("worker failed to resume after wake"),
        "blocked worker's only-root frame reference was reclaimed"
    );
    worker.join().unwrap();

    let mut order = Vec::new();
    while order.len() < 7 {
        order.push(
            observed
                .recv_timeout(Duration::from_secs(5))
                .expect("missing rendezvous hook evidence"),
        );
    }
    assert_eq!(
        order,
        [
            "worker:SnapshotPublished",
            "request:collect",
            "owner:SnapshotPublished",
            "owner:Election",
            "owner:Resumed",
            "wake:release",
            "worker:Resumed",
        ],
        "unexpected snapshot/request/wake/resume ordering"
    );
    safepoint::retire();
}

#[test]
fn gc_collect_scratch_high_water_tracks_current_objects_not_lifetime_keys() {
    let heap = Heap::default();
    let pending = PendingList::new();
    let live = array(&heap, "[I", &[42]);
    let coordinator = Arc::new(Coordinator::new());
    let owner = coordinator.startup(0);
    coordinator.resume(owner);
    // Warm process-global monitor/handle bookkeeping outside the measurement.
    cycle(&heap, &[live], &pending, Cause::Explicit).unwrap();
    let mut previous = None;
    for epoch in 0..32 {
        for _ in 0..128 {
            array(&heap, "[I", &[9]);
        }
        let operation = coordinator
            .request_collection(owner, Snapshot { roots: vec![live] })
            .unwrap();
        let ((result, allocations), bytes) = measure_bytes(|| {
            measure(usize::MAX, || {
                collect_snapshot(&heap, &[live], &pending, Cause::Explicit, &operation)
            })
        });
        let stats = result.unwrap();
        operation.complete();
        assert_eq!(stats.before, 129);
        assert_eq!(stats.live, 1);
        assert_eq!(
            allocations, 4,
            "only preflight allocates, not trace or sweep"
        );
        assert!(bytes <= stats.scratch_bytes);
        if let Some(previous) = previous {
            assert_eq!(bytes, previous, "scratch must not grow with lifetime keys");
        }
        previous = Some(bytes);
        eprintln!(
            "gc_collect scratch epoch={epoch} objects={} allocator_bytes={bytes} bound={}",
            stats.before, stats.scratch_bytes
        );
    }
    let operation = coordinator
        .request_collection(owner, Snapshot { roots: vec![live] })
        .unwrap();
    let (result, bytes) =
        measure_bytes(|| collect_snapshot(&heap, &[live], &pending, Cause::Explicit, &operation));
    assert_eq!(result.unwrap().before, 1);
    assert!(
        bytes < previous.unwrap(),
        "the previous high water is not retained"
    );
    operation.complete();
}

#[test]
fn gc_collect_scratch_preflight_failure_leaves_heap_and_budget_unchanged() {
    let heap = Heap::default();
    let pending = PendingList::new();
    let live = array(&heap, "[I", &[1]);
    let dead = array(&heap, "[I", &[2]);
    let coordinator = Arc::new(Coordinator::new());
    let owner = coordinator.startup(0);
    coordinator.resume(owner);
    let operation = coordinator
        .request_collection(owner, Snapshot { roots: vec![live] })
        .unwrap();
    let before = heap.memory();
    let (result, _) = measure(0, || {
        collect_snapshot(&heap, &[live], &pending, Cause::Explicit, &operation)
    });
    assert!(result.is_err());
    assert_eq!(heap.memory(), before);
    assert!(heap.contains(live) && heap.contains(dead));
}

#[test]
fn gc_collect_pressure_retries_common_admission_without_overspending() {
    let _serial = safepoint::tests::SERIAL.lock().unwrap();
    let dead = array(&HEAP, "[I", &[1]);
    let charge = HEAP.memory().1;
    HEAP.set_limit_for_test(charge);
    trigger::enable();
    let start = Arc::new(std::sync::Barrier::new(3));
    let (attempted, attempts) = channel();
    let mut releases = Vec::new();
    let mut workers = Vec::new();
    for _ in 0..2 {
        let start = Arc::clone(&start);
        let attempted = attempted.clone();
        let (release, gate) = channel();
        releases.push(release);
        workers.push(std::thread::spawn(move || {
            start.wait();
            safepoint::attach();
            let mut root = RootedSlots::new();
            let result = HEAP.create_array_with_values("[I", &[9], &mut root);
            safepoint::blocking(|| {
                attempted.send(result.is_ok()).unwrap();
                gate.recv_timeout(Duration::from_secs(5)).unwrap();
            });
            safepoint::retire();
            result.map_err(|error| {
                matches!(
                    error.kind(),
                    crate::vm::error::ErrorKind::Allocation(AllocError::Capacity)
                )
            })
        }));
    }
    start.wait();
    let first = attempts.recv_timeout(Duration::from_secs(5));
    let second = attempts.recv_timeout(Duration::from_secs(5));
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
        1
    );
    assert!(results.iter().any(|result| matches!(result, Err(true))));
    assert_eq!(HEAP.memory(), (charge, charge));
    assert!(!HEAP.contains(dead));
}

#[test]
fn gc_collect_pressure_contender_upgrades_an_ordinary_epoch_before_joining() {
    let _serial = safepoint::tests::SERIAL.lock().unwrap();
    let target = array(&HEAP, "[I", &vec![1; 1024]);
    let soft = reference(&HEAP, "java/lang/ref/SoftReference", target);
    HEAP.set_limit_for_test(HEAP.memory().1);
    trigger::enable();
    let (ready, running) = channel();
    let (allocate, allocation_gate) = channel();
    let (parked, publication) = channel();
    let allocator = std::thread::spawn(move || {
        safepoint::attach();
        ready.send(()).unwrap();
        allocation_gate
            .recv_timeout(Duration::from_secs(5))
            .unwrap();
        let announced = std::cell::Cell::new(false);
        safepoint::observe(move |event| {
            if event == safepoint::Event::SnapshotPublished && !announced.replace(true) {
                parked.send(()).unwrap();
            }
        });
        let result = HEAP
            .create_array("[I", 1024, &mut RootedSlots::new())
            .is_ok();
        safepoint::retire();
        result
    });
    running.recv_timeout(Duration::from_secs(5)).unwrap();
    let (elected, election) = channel();
    let (proceed, election_gate) = channel();
    let owner = std::thread::spawn(move || {
        safepoint::attach();
        let _root = RootedSlots::from_slots(vec![Slot::Ref(soft)]);
        safepoint::observe(move |event| {
            if event == safepoint::Event::Election {
                elected.send(()).unwrap();
                election_gate.recv_timeout(Duration::from_secs(5)).unwrap();
            }
        });
        collect(Cause::Explicit).unwrap();
        safepoint::retire();
    });
    election.recv_timeout(Duration::from_secs(5)).unwrap();
    allocate.send(()).unwrap();
    publication.recv_timeout(Duration::from_secs(5)).unwrap();
    proceed.send(()).unwrap();
    owner.join().unwrap();
    assert!(allocator.join().unwrap(),
        "pressure must not accept a completed epoch that retained the only reclaimable soft target");
    assert!(!HEAP.contains(target));
}

#[test]
fn gc_collect_aborted_pressure_is_not_forgotten_by_an_ordinary_successor() {
    let coordinator = Arc::new(Coordinator::new());
    let owner = coordinator.startup(0);
    coordinator.resume(owner);
    let aborted = coordinator
        .request_collection_for(owner, Snapshot::default(), true)
        .unwrap();
    assert!(aborted.pressure());
    drop(aborted);
    let successor = coordinator
        .request_collection(owner, Snapshot::default())
        .unwrap();
    assert!(successor.pressure());
    successor.complete();
    let ordinary = coordinator
        .request_collection(owner, Snapshot::default())
        .unwrap();
    assert!(!ordinary.pressure());
    ordinary.complete();
}
