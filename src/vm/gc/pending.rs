//! The JDK handler owns `next`; the VM pending chain exclusively uses `discovered`.
use crate::vm::error::{Error, Result};
use crate::vm::jni::handles;
use crate::vm::safepoint;
use parking_lot::{Condvar, Mutex, MutexGuard};
use std::sync::LazyLock;

pub(crate) static PENDING: LazyLock<PendingList> = LazyLock::new(PendingList::new);

pub(crate) struct PendingList {
    head: Mutex<i32>,
    available: Condvar,
}

impl PendingList {
    pub(crate) const fn new() -> Self {
        Self {
            head: Mutex::new(0),
            available: Condvar::new(),
        }
    }

    pub(crate) fn snapshot(&self) -> Result<i32> {
        Ok(*self.for_collection()?)
    }

    pub(crate) fn for_collection(&self) -> Result<MutexGuard<'_, i32>> {
        self.head
            .try_lock()
            .ok_or_else(|| Error::new_execution("Reference pending list locked during collection"))
    }

    pub(crate) fn notify(&self) {
        self.available.notify_all();
    }

    pub(crate) fn has_pending(&self) -> bool {
        *safepoint::lock(&self.head) != 0
    }

    pub(crate) fn wait(&self) {
        let mut head = safepoint::lock(&self.head);
        while *head == 0 {
            head = safepoint::wait(&self.available, &self.head, head);
        }
        // wait() resumed with an owned guard and rechecked its predicate. This
        // returns Running, with no Rust lock held across a coordinator wait.
    }

    pub(crate) fn take(&self) -> i32 {
        let mut head = safepoint::lock(&self.head);
        // Destination-first: create the handler's native-frame root BEFORE
        // removing the VM root. If handle capacity is exhausted, leave it pending.
        if *head != 0 && handles::new_local(*head).is_null() {
            return 0;
        }
        std::mem::take(&mut *head)
    }
}
