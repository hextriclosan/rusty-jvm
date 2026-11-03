use super::{event, set_managed, Event};
use crate::vm::stack::stack_frames_util::StackElement;
use parking_lot::{Condvar, Mutex};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

const DUMP_TIMEOUT: Duration = Duration::from_secs(2);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Reason {
    Startup,
    Safepoint,
    Blocking,
    Native,
    Operation,
    Retirement,
}

#[derive(Debug, Default)]
pub(crate) struct Snapshot {
    /// Read by the tests here and by the root scan task 7 adds; nothing in the VM
    /// consumes it yet, because no collection happens in this unit.
    #[allow(dead_code)]
    pub(crate) roots: Vec<i32>,
}

#[derive(Debug)]
enum ThreadState {
    Running,
    Safe {
        reason: Reason,
        snapshot: Arc<Snapshot>,
    },
    Retired,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Phase {
    Open,
    Dump {
        id: u64,
        owner: u64,
        target: u64,
        deadline: Instant,
    },
    Stopping {
        epoch: u64,
        owner: u64,
    },
    Collecting {
        epoch: u64,
        owner: u64,
    },
}

struct Record {
    thread_ref: i32,
    state: ThreadState,
}

struct State {
    threads: HashMap<u64, Record>,
    phase: Phase,
    next_id: u64,
    next_operation: u64,
    completed_epoch: u64,
    dump_result: Option<Option<Vec<StackElement>>>,
}

pub(crate) struct Coordinator {
    state: Mutex<State>,
    cv: Condvar,
    requested: AtomicBool,
}

impl Coordinator {
    pub(crate) fn new() -> Self {
        Self {
            state: Mutex::new(State {
                threads: HashMap::new(),
                phase: Phase::Open,
                next_id: 0,
                next_operation: 0,
                completed_epoch: 0,
                dump_result: None,
            }),
            cv: Condvar::new(),
            requested: AtomicBool::new(false),
        }
    }

    pub(crate) fn requested(&self) -> bool {
        self.requested.load(Ordering::Relaxed)
    }

    pub(crate) fn startup(&self, thread_ref: i32) -> u64 {
        let mut state = self.state.lock();
        state.next_id = state
            .next_id
            .checked_add(1)
            .expect("participant identity exhausted");
        let id = state.next_id;
        state.threads.insert(
            id,
            Record {
                thread_ref,
                state: ThreadState::Safe {
                    reason: Reason::Startup,
                    snapshot: Arc::new(Snapshot {
                        roots: vec![thread_ref],
                    }),
                },
            },
        );
        self.cv.notify_all();
        id
    }

    pub(crate) fn bind(&self, id: u64, thread_ref: i32) {
        let mut state = self.state.lock();
        let record = state.threads.get_mut(&id).expect("registered participant");
        assert!(matches!(record.state, ThreadState::Running));
        record.thread_ref = thread_ref;
    }

    pub(crate) fn cancel_startup(&self, id: u64) {
        let mut state = self.state.lock();
        assert!(matches!(
            state.threads[&id].state,
            ThreadState::Safe {
                reason: Reason::Startup,
                ..
            }
        ));
        // A startup escrow can be withdrawn even during a stop: any snapshot already
        // acquired by the owner is an Arc, and the spawning parent still roots the object.
        state.threads.remove(&id);
        self.cv.notify_all();
    }

    #[cfg(test)]
    pub(crate) fn running(&self, id: u64) -> bool {
        matches!(self.state.lock().threads[&id].state, ThreadState::Running)
    }

    pub(crate) fn park(&self, id: u64, snapshot: Snapshot, reason: Reason) {
        let mut state = self.state.lock();
        let record = state.threads.get_mut(&id).expect("registered participant");
        assert!(matches!(record.state, ThreadState::Running));
        record.state = ThreadState::Safe {
            reason,
            snapshot: Arc::new(snapshot),
        };
        set_managed(false);
        self.cv.notify_all();
        drop(state);
        event(Event::SnapshotPublished);
    }

    fn eligible(state: &State, id: u64) -> bool {
        match state.phase {
            Phase::Open => true,
            Phase::Dump { owner, target, .. } => {
                id != owner && (id != target || state.dump_result.is_none())
            }
            Phase::Stopping { .. } | Phase::Collecting { .. } => false,
        }
    }

    pub(crate) fn try_resume(&self, id: u64) -> bool {
        let mut state = self.state.lock();
        if !Self::eligible(&state, id) {
            return false;
        }
        let record = state.threads.get_mut(&id).expect("registered participant");
        assert!(matches!(record.state, ThreadState::Safe { .. }));
        record.state = ThreadState::Running;
        set_managed(true);
        drop(state);
        event(Event::Resumed);
        true
    }

    /// Waits without consuming the snapshot or becoming Running. A blocking-lock
    /// caller reacquires its real guard and then checks try_resume again.
    pub(crate) fn wait_eligible(&self, id: u64) {
        let mut state = self.state.lock();
        while !Self::eligible(&state, id) {
            self.cv.wait(&mut state);
        }
    }

    pub(crate) fn resume(&self, id: u64) {
        while !self.try_resume(id) {
            self.wait_eligible(id);
        }
    }

    pub(crate) fn retire(&self, id: u64) {
        let mut state = self.state.lock();
        while !matches!(state.phase, Phase::Open) {
            self.cv.wait(&mut state);
        }
        let record = state.threads.get_mut(&id).expect("registered participant");
        assert!(matches!(record.state, ThreadState::Safe { .. }));
        record.state = ThreadState::Retired;
        state.threads.remove(&id);
        self.cv.notify_all();
        drop(state);
        event(Event::Retired);
    }

    pub(crate) fn dump_request(&self, id: u64) -> Option<u64> {
        match self.state.lock().phase {
            Phase::Dump {
                id: dump, target, ..
            } if target == id => Some(dump),
            _ => None,
        }
    }

    pub(crate) fn publish_dump(&self, id: u64, dump: u64, frames: Option<Vec<StackElement>>) {
        let mut state = self.state.lock();
        if matches!(state.phase, Phase::Dump { id: active, target, .. }
            if active == dump && target == id)
        {
            state.dump_result = Some(frames);
            self.cv.notify_all();
        }
    }

    pub(crate) fn dump(
        &self,
        owner: u64,
        thread_ref: i32,
        roots: Snapshot,
        handoff: impl FnOnce(&[StackElement]),
    ) -> Option<Vec<StackElement>> {
        self.park(owner, roots, Reason::Operation);
        let mut state = self.state.lock();
        while !matches!(state.phase, Phase::Open) {
            self.cv.wait(&mut state);
        }
        let target = state
            .threads
            .iter()
            .find_map(|(&id, record)| (record.thread_ref == thread_ref).then_some(id));
        let Some(target) = target else {
            state
                .threads
                .get_mut(&owner)
                .expect("registered owner")
                .state = ThreadState::Running;
            set_managed(true);
            return None;
        };
        state.next_operation = state
            .next_operation
            .checked_add(1)
            .expect("operation identity exhausted");
        let id = state.next_operation;
        let deadline = Instant::now() + DUMP_TIMEOUT;
        state.phase = Phase::Dump {
            id,
            owner,
            target,
            deadline,
        };
        state.dump_result = None;
        self.requested.store(true, Ordering::Relaxed);
        drop(state);
        event(Event::Election);
        let mut state = self.state.lock();
        while state.dump_result.is_none() {
            let now = Instant::now();
            if now >= deadline {
                break;
            }
            self.cv.wait_for(&mut state, deadline - now);
        }
        let result = state.dump_result.take().flatten();
        // Handoff performs only owner-local root publication, never Java/heap work.
        // Keep the operation exclusive until the caller's destination exists.
        if let Some(frames) = &result {
            handoff(frames);
        }
        assert!(matches!(state.phase, Phase::Dump { id: active, .. } if active == id));
        state.phase = Phase::Open;
        state
            .threads
            .get_mut(&owner)
            .expect("registered owner")
            .state = ThreadState::Running;
        set_managed(true);
        self.requested.store(false, Ordering::Relaxed);
        self.cv.notify_all();
        drop(state);
        event(Event::Resumed);
        result
    }

    /// Stop machinery only; task 7 supplies the scanner. No timeout or diagnostic
    /// frame limit can satisfy this all-participant predicate.
    #[allow(dead_code)] // Runtime caller arrives with the collector; exercised by real thread tests.
    pub(crate) fn request_collection(
        self: &Arc<Self>,
        owner: u64,
        roots: Snapshot,
    ) -> Option<Operation> {
        let observed = self.state.lock().completed_epoch;
        self.park(owner, roots, Reason::Operation);
        let mut state = self.state.lock();
        loop {
            if state.completed_epoch > observed {
                drop(state);
                self.resume(owner);
                return None;
            }
            if matches!(state.phase, Phase::Open) {
                break;
            }
            self.cv.wait(&mut state);
        }
        state.next_operation = state
            .next_operation
            .checked_add(1)
            .expect("operation identity exhausted");
        let epoch = state.next_operation;
        state.phase = Phase::Stopping { epoch, owner };
        self.requested.store(true, Ordering::Relaxed);
        drop(state);
        event(Event::Election);
        let mut state = self.state.lock();
        while state
            .threads
            .values()
            .any(|record| matches!(record.state, ThreadState::Running))
        {
            self.cv.wait(&mut state);
        }
        let snapshots = state
            .threads
            .values()
            .filter_map(|record| match &record.state {
                ThreadState::Safe { snapshot, .. } => Some(Arc::clone(snapshot)),
                ThreadState::Retired => None,
                ThreadState::Running => unreachable!("rendezvous is incomplete"),
            })
            .collect();
        state.phase = Phase::Collecting { epoch, owner };
        drop(state);
        Some(Operation {
            coordinator: Arc::clone(self),
            owner,
            epoch,
            snapshots,
            completed: false,
        })
    }
}

/// Exclusive ownership, NOT a lock guard. Dropping an uncompleted operation aborts
/// it coherently; contenders must elect another epoch rather than accept a failed scan.
pub(crate) struct Operation {
    coordinator: Arc<Coordinator>,
    owner: u64,
    epoch: u64,
    /// The complete rendezvous: one immutable snapshot per live participant. Consumed by
    /// task 7's scanner; until then only the tests here read it.
    #[allow(dead_code)]
    pub(crate) snapshots: Vec<Arc<Snapshot>>,
    completed: bool,
}

impl Operation {
    #[allow(dead_code)] // Activated by task 7 after a successful scan.
    pub(crate) fn complete(mut self) {
        self.completed = true;
    }
}

impl Drop for Operation {
    fn drop(&mut self) {
        let mut state = self.coordinator.state.lock();
        assert_eq!(
            state.phase,
            Phase::Collecting {
                epoch: self.epoch,
                owner: self.owner
            }
        );
        if self.completed {
            state.completed_epoch = self.epoch;
        }
        state.phase = Phase::Open;
        self.coordinator.requested.store(false, Ordering::Relaxed);
        self.coordinator.cv.notify_all();
        drop(state);
        self.coordinator.resume(self.owner);
    }
}
