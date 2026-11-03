use super::*;
use crate::vm::safepoint::{self, Event, COORDINATOR};
use std::cell::Cell;
use std::sync::mpsc::{channel, Receiver};

const BOUND: Duration = Duration::from_secs(5);
/// Strictly shorter than `BOUND`, so detecting a retained guard still leaves every other
/// bounded wait time to be released and the failure to surface as this test's assertion.
const DETECT: Duration = Duration::from_secs(2);

fn receive<T>(receiver: &Receiver<T>) -> T {
    receiver.recv_timeout(BOUND).expect("bounded monitor event")
}

#[test]
fn gc_coordinator_waiter_drops_actual_guard_for_third_contender() {
    let _serial = safepoint::tests::SERIAL.lock().unwrap();
    let object = HEAP.create_array("[I", 0);
    let (wait_published, waiting) = channel();
    let (wake_guard, woke) = channel();
    let (continue_wake, wake_gate) = channel();
    let (third_started, third_ready) = channel();
    let (start_third, third_gate) = channel();
    let (third_safe, third_publication) = channel();
    let (third_guard, third_acquired) = channel();
    let (collector_started, collector_ready) = channel();
    let (start_collection, collection_gate) = channel();
    let (collecting, collection_ready) = channel();
    let (finish_collection, collection_finish) = channel();
    let (done, completed) = channel();

    // Given a real monitor waiter, with observers installed before releasing its wait.
    let waiter_done = done.clone();
    let waiter = std::thread::spawn(move || {
        safepoint::attach();
        enter(object);
        let published = Cell::new(false);
        let reacquired = Cell::new(false);
        safepoint::observe(move |event| {
            if event == Event::SnapshotPublished && !published.replace(true) {
                wait_published.send(()).unwrap();
            }
            if event == Event::GuardReacquired && !reacquired.replace(true) {
                wake_guard.send(()).unwrap();
                receive(&wake_gate);
            }
        });
        wait(object, 0).unwrap();
        exit(object).unwrap();
        safepoint::retire();
        waiter_done.send(()).unwrap();
    });
    receive(&waiting);

    let contender_done = done.clone();
    let contender = std::thread::spawn(move || {
        safepoint::attach();
        third_started.send(()).unwrap();
        receive(&third_gate);
        let published = Cell::new(false);
        let acquired = Cell::new(false);
        safepoint::observe(move |event| {
            if event == Event::SnapshotPublished && !published.replace(true) {
                third_safe.send(()).unwrap();
            }
            if event == Event::GuardReacquired && !acquired.replace(true) {
                third_guard.send(()).unwrap();
            }
        });
        enter(object);
        exit(object).unwrap();
        safepoint::retire();
        contender_done.send(()).unwrap();
    });
    receive(&third_ready);

    let collector = std::thread::spawn(move || {
        safepoint::attach();
        collector_started.send(()).unwrap();
        receive(&collection_gate);
        let operation = COORDINATOR
            .request_collection(safepoint::current().unwrap(), JavaThread::snapshot_roots())
            .unwrap();
        assert!(operation
            .snapshots
            .iter()
            .any(|snapshot| snapshot.roots.contains(&object)));
        collecting.send(()).unwrap();
        receive(&collection_finish);
        operation.complete();
        safepoint::retire();
        done.send(()).unwrap();
    });
    receive(&collector_ready);

    // When notify wakes the waiter, let a third contender arrive before rendezvous.
    enter(object);
    notify(object).unwrap();
    exit(object).unwrap();
    receive(&woke);
    start_third.send(()).unwrap();
    receive(&third_publication);
    start_collection.send(()).unwrap();
    receive(&collection_ready);
    continue_wake.send(()).unwrap();

    // Then the third thread must acquire the actual state mutex DURING the stop.
    // This timeout only DETECTS; every ordering above is established by a subscribed
    // channel. The stop is released even on failure, so a retained guard fails this
    // assertion instead of deadlocking the suite.
    let acquired = third_acquired.recv_timeout(DETECT);
    finish_collection.send(()).unwrap();
    for _ in 0..3 {
        receive(&completed);
    }
    waiter.join().unwrap();
    contender.join().unwrap();
    collector.join().unwrap();
    assert!(
        acquired.is_ok(),
        "waiter retained the actual monitor guard across coordinator wait"
    );
}
