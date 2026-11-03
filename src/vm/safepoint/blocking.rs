use super::{current, event, managed, Event, Reason, COORDINATOR};
use crate::vm::jni::java_thread::JavaThread;
use parking_lot::{Condvar, Mutex, MutexGuard, WaitTimeoutResult};
use std::marker::PhantomData;
use std::rc::Rc;
use std::time::Duration;

/// A Safe interval. It carries no Rust lock and is confined to the owning OS thread.
pub(crate) struct Blocking {
    id: Option<u64>,
    _thread_bound: PhantomData<Rc<()>>,
}

impl Blocking {
    pub(crate) fn enter(reason: Reason) -> Self {
        let id = current();
        if let Some(id) = id {
            COORDINATOR.park(id, JavaThread::snapshot_roots(), reason);
        }
        Self {
            id,
            _thread_bound: PhantomData,
        }
    }

    /// A condvar has already reacquired this owned guard. If a stop won the race,
    /// drop that ACTUAL guard before waiting, then acquire and recheck; never repeat
    /// the original condvar sleep, which could consume a notification twice.
    pub(crate) fn resume_with<'a, T>(
        mut self,
        mutex: &'a Mutex<T>,
        mut guard: MutexGuard<'a, T>,
    ) -> MutexGuard<'a, T> {
        if let Some(id) = self.id.take() {
            loop {
                event(Event::GuardReacquired);
                if COORDINATOR.try_resume(id) {
                    break;
                }
                drop(guard);
                COORDINATOR.wait_eligible(id);
                guard = mutex.lock();
            }
        }
        guard
    }
}

impl Drop for Blocking {
    fn drop(&mut self) {
        if let Some(id) = self.id {
            COORDINATOR.resume(id);
        }
    }
}

/// Used at every native-to-managed boundary BEFORE handle decoding. Nested JNI
/// calls remain Running; only the outer callback republishes on its way to native.
pub(crate) struct ManagedAccess {
    republish: Option<u64>,
    _thread_bound: PhantomData<Rc<()>>,
}

impl ManagedAccess {
    pub(crate) fn enter() -> Self {
        let republish = current().filter(|_| !managed());
        if let Some(id) = republish {
            COORDINATOR.resume(id);
        }
        Self {
            republish,
            _thread_bound: PhantomData,
        }
    }
}

impl Drop for ManagedAccess {
    fn drop(&mut self) {
        if let Some(id) = self.republish {
            COORDINATOR.park(id, JavaThread::snapshot_roots(), Reason::Native);
        }
    }
}

pub(crate) fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    if let Some(guard) = mutex.try_lock() {
        return guard;
    }
    let blocked = Blocking::enter(Reason::Blocking);
    let guard = mutex.lock();
    blocked.resume_with(mutex, guard)
}

pub(crate) fn wait<'a, T>(
    cv: &Condvar,
    mutex: &'a Mutex<T>,
    mut guard: MutexGuard<'a, T>,
) -> MutexGuard<'a, T> {
    let blocked = Blocking::enter(Reason::Blocking);
    cv.wait(&mut guard);
    blocked.resume_with(mutex, guard)
}

pub(crate) fn wait_for<'a, T>(
    cv: &Condvar,
    mutex: &'a Mutex<T>,
    mut guard: MutexGuard<'a, T>,
    duration: Duration,
) -> (MutexGuard<'a, T>, WaitTimeoutResult) {
    let blocked = Blocking::enter(Reason::Blocking);
    let result = cv.wait_for(&mut guard, duration);
    (blocked.resume_with(mutex, guard), result)
}
