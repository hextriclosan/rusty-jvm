use super::*;
use crate::vm::heap::heap::HEAP;
use crate::vm::method_area::java_class::{InitState, InnerState};
use std::sync::mpsc::{channel, Receiver};
use std::sync::Mutex;
use std::time::Duration;

pub(crate) static SERIAL: Mutex<()> = Mutex::new(());
const BOUND: Duration = Duration::from_secs(5);

fn receive<T>(receiver: &Receiver<T>) -> T {
    receiver
        .recv_timeout(BOUND)
        .expect("bounded coordinator event")
}

fn roots(reference: i32) -> Snapshot {
    Snapshot {
        roots: vec![reference],
    }
}

#[test]
fn gc_coordinator_simultaneous_requests_share_one_complete_epoch() {
    // Given two running requesters and an already-Safe startup record.
    let coordinator = Arc::new(Coordinator::new());
    let first = coordinator.startup(11);
    let second = coordinator.startup(22);
    let startup = coordinator.startup(33);
    coordinator.resume(first);
    coordinator.resume(second);
    let (events, observed) = channel();
    let (finish, allowed) = channel();
    let allowed = Arc::new(Mutex::new(allowed));
    let mut workers = Vec::new();
    for (id, root) in [(first, 11), (second, 22)] {
        let coordinator = Arc::clone(&coordinator);
        let events = events.clone();
        let allowed = Arc::clone(&allowed);
        workers.push(std::thread::spawn(move || {
            observe(move |event| {
                events.send(event).unwrap();
            });
            let elected = if let Some(operation) = coordinator.request_collection(id, roots(root))
            {
                let mut snapshot_roots: Vec<_> = operation
                    .snapshots
                    .iter()
                    .flat_map(|snapshot| snapshot.roots.iter().copied())
                    .collect();
                snapshot_roots.sort();
                assert_eq!(snapshot_roots, vec![11, 22, 33]);
                receive(&allowed.lock().unwrap());
                operation.complete();
                true
            } else {
                false
            };
            coordinator.park(id, roots(root), Reason::Retirement);
            coordinator.retire(id);
            elected
        }));
    }
    // When both requests have published, permit the single elected operation to finish.
    let mut publications = 0;
    let mut elections = 0;
    while publications != 2 || elections != 1 {
        match receive(&observed) {
            Event::SnapshotPublished => publications += 1,
            Event::Election => elections += 1,
            _ => {}
        }
    }
    finish.send(()).unwrap();
    // Then the loser accepts the completed epoch, rather than nesting ownership.
    let winners = workers
        .into_iter()
        .map(|worker| usize::from(worker.join().unwrap()))
        .sum::<usize>();
    assert_eq!(winners, 1);
    coordinator.retire(startup);
}

#[test]
fn gc_coordinator_saved_snapshot_survives_later_epochs_and_abort() {
    // Given a blocked participant whose snapshot has no live stack to rescan.
    let coordinator = Arc::new(Coordinator::new());
    let owner = coordinator.startup(11);
    let blocked = coordinator.startup(22);
    coordinator.resume(owner);
    // When one stop is aborted and another completes without resuming the blocked thread.
    let first = coordinator.request_collection(owner, roots(11)).unwrap();
    let saved = first
        .snapshots
        .iter()
        .find(|snapshot| snapshot.roots == vec![22])
        .unwrap()
        .clone();
    drop(first);
    let second = coordinator.request_collection(owner, roots(11)).unwrap();
    let repeated = second
        .snapshots
        .iter()
        .find(|snapshot| snapshot.roots == vec![22])
        .unwrap();
    // Then the coordinator reuses the immutable, complete owner-produced value.
    assert!(Arc::ptr_eq(&saved, repeated));
    second.complete();
    coordinator.park(owner, roots(11), Reason::Retirement);
    coordinator.retire(owner);
    coordinator.retire(blocked);
}

#[test]
fn gc_coordinator_dump_timeout_does_not_cancel_a_collection() {
    // Given a blocked target, a dumper, and a collection requester.
    let coordinator = Arc::new(Coordinator::new());
    let target = coordinator.startup(33);
    let dumper = coordinator.startup(11);
    let collector = coordinator.startup(22);
    coordinator.resume(dumper);
    coordinator.resume(collector);
    let (elected, election) = channel();
    let dump_coordinator = Arc::clone(&coordinator);
    let dump = std::thread::spawn(move || {
        observe(move |event| {
            if event == Event::Election {
                elected.send(()).unwrap();
            }
        });
        let started = std::time::Instant::now();
        let result = dump_coordinator.dump(dumper, 33, roots(11), |_| panic!("blocked target"));
        assert!(result.is_none());
        assert!(started.elapsed() >= Duration::from_secs(2));
        dump_coordinator.park(dumper, roots(11), Reason::Retirement);
        dump_coordinator.retire(dumper);
    });
    receive(&election);
    let stale_dump = coordinator.dump_request(target).unwrap();
    // When a request overlaps the dump, it publishes Safe and waits for ownership.
    let operation = coordinator
        .request_collection(collector, roots(22))
        .unwrap();
    // Then a late publication for the timed-out dump cannot release this GC epoch.
    coordinator.publish_dump(target, stale_dump, Some(vec![]));
    assert!(!coordinator.try_resume(target));
    assert!(operation
        .snapshots
        .iter()
        .any(|snapshot| snapshot.roots == vec![33]));
    operation.complete();
    dump.join().unwrap();
    coordinator.park(collector, roots(22), Reason::Retirement);
    coordinator.retire(collector);
    coordinator.retire(target);
}

#[test]
fn gc_coordinator_dump_handoff_precedes_managed_resume() {
    let coordinator = Arc::new(Coordinator::new());
    let target = coordinator.startup(22);
    let owner = coordinator.startup(11);
    coordinator.resume(target);
    coordinator.resume(owner);
    let (elected, election) = channel();
    let worker_coordinator = Arc::clone(&coordinator);
    let worker = std::thread::spawn(move || {
        receive(&election);
        let dump = worker_coordinator.dump_request(target).unwrap();
        worker_coordinator.park(target, roots(22), Reason::Safepoint);
        worker_coordinator.publish_dump(
            target,
            dump,
            Some(vec![StackElement::new(91, 0, 0, 0, false)]),
        );
        worker_coordinator.resume(target);
        worker_coordinator.park(target, roots(22), Reason::Retirement);
        worker_coordinator.retire(target);
    });
    observe(move |event| {
        if event == Event::Election {
            elected.send(()).unwrap();
        }
    });
    let mut destination = RootedSlots::new();
    let result = coordinator
        .dump(owner, 22, roots(11), |frames| {
            destination.replace(
                frames
                    .iter()
                    .map(|frame| Slot::Ref(frame.class_ref()))
                    .collect(),
            );
        })
        .unwrap();
    assert_eq!(result[0].class_ref(), 91);
    assert_eq!(destination[0], Slot::Ref(91));
    assert!(coordinator.running(owner));
    OBSERVER.with(|slot| slot.borrow_mut().take());
    coordinator.park(owner, roots(11), Reason::Retirement);
    coordinator.retire(owner);
    worker.join().unwrap();
}

#[test]
fn gc_coordinator_class_initialization_is_recursive_and_contended() {
    let _serial = SERIAL.lock().unwrap();
    attach();
    let state = Arc::new(InitState::default());
    assert_eq!(state.begin(), Ok(true));
    assert_eq!(state.begin(), Ok(false));
    let (published, publication) = channel();
    let other = Arc::clone(&state);
    let child = std::thread::spawn(move || {
        attach();
        observe(move |event| {
            if event == Event::SnapshotPublished {
                published.send(()).unwrap();
            }
        });
        assert_eq!(other.begin(), Ok(false));
        OBSERVER.with(|slot| slot.borrow_mut().take());
        retire();
    });
    // The subscribed publication, not elapsed time, proves the child is contending.
    receive(&publication);
    state.finish(None);
    child.join().unwrap();
    assert_eq!(state.get_inner_state(), InnerState::Initialized);
    retire();
}

#[test]
fn gc_coordinator_failed_class_keeps_its_cause_root() {
    let state = InitState::default();
    let cause = HEAP
        .create_array(
            "[I",
            0,
            &mut crate::vm::stack::rooted_slots::RootedSlots::new(),
        )
        .unwrap();
    assert_eq!(state.begin(), Ok(true));
    state.finish(Some(cause));
    assert_eq!(state.begin(), Err(cause));
    assert_eq!(state.begin(), Err(cause));
    assert_eq!(
        state.get_inner_state(),
        InnerState::Failed { cause_root: cause }
    );
}

#[test]
fn gc_coordinator_startup_claim_and_retirement_obey_active_epoch() {
    let _serial = SERIAL.lock().unwrap();
    attach();
    let object = HEAP
        .create_array(
            "[I",
            0,
            &mut crate::vm::stack::rooted_slots::RootedSlots::new(),
        )
        .unwrap();
    let child = COORDINATOR.startup(object);
    let cancelled = COORDINATOR.startup(object);
    COORDINATOR.cancel_startup(cancelled);
    let (ready, entered) = channel();
    let (claimed, claim_event) = channel();
    let (exit, exit_event) = channel();
    let operation = COORDINATOR
        .request_collection(current().unwrap(), JavaThread::snapshot_roots())
        .unwrap();
    let worker = std::thread::spawn(move || {
        observe(move |event| {
            if event == Event::StartupClaimed {
                claimed.send(()).unwrap();
            }
        });
        ready.send(()).unwrap();
        claim(child);
        JavaThread::set_current_thread(object);
        retire();
        exit.send(()).unwrap();
    });
    receive(&entered);
    assert!(!COORDINATOR.running(child));
    assert_eq!(
        operation
            .snapshots
            .iter()
            .filter(|snapshot| snapshot.roots == vec![object])
            .count(),
        1
    );
    operation.complete();
    receive(&claim_event);
    receive(&exit_event);
    worker.join().unwrap();
    retire();
}
