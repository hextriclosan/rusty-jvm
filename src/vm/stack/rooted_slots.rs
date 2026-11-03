use crate::vm::gc::budget::{AllocError, AllocResult};
use crate::vm::stack::slot::Slot;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

thread_local! {
    static TRANSFERS: RefCell<Vec<Weak<RefCell<Vec<Slot>>>>> = const { RefCell::new(Vec::new()) };
}

/// An owned, published destination for an argument, result or exception transfer.
///
/// Unlike a temporary LIFO scope, a transfer can outlive the Rust call that fills it.
/// Create the destination before releasing the source frame, fill it without a
/// safepoint, and keep it until the receiving frame, field or pending slot is set.
/// The Rc makes both the value and its registration thread-bound.
#[derive(Debug)]
pub(crate) struct RootedSlots {
    slots: Vec<Slot>,
    published: Rc<RefCell<Vec<Slot>>>,
}

impl RootedSlots {
    pub(crate) fn new() -> Self {
        let published = Rc::new(RefCell::new(Vec::new()));
        TRANSFERS.with(|roots| roots.borrow_mut().push(Rc::downgrade(&published)));
        Self {
            slots: Vec::new(),
            published,
        }
    }

    pub(crate) fn from_slots(slots: Vec<Slot>) -> Self {
        let mut destination = Self::new();
        destination.replace(slots);
        destination
    }

    pub(crate) fn replace(&mut self, slots: Vec<Slot>) {
        self.published.borrow_mut().clone_from(&slots);
        self.slots = slots;
    }

    /// Prepare before admission so commit only writes already-owned storage.
    pub(crate) fn prepare_single_ref(&mut self) -> AllocResult<()> {
        if self.slots.is_empty() {
            self.slots
                .try_reserve_exact(1)
                .map_err(|_| AllocError::HostAllocation)?;
        }
        let mut published = self.published.borrow_mut();
        if published.is_empty() {
            published
                .try_reserve_exact(1)
                .map_err(|_| AllocError::HostAllocation)?;
        }
        Ok(())
    }

    pub(crate) fn publish_single_ref(&mut self, reference: i32) {
        let mut published = self.published.borrow_mut();
        assert!(self.slots.capacity() >= 1 && published.capacity() >= 1);
        self.slots.clear();
        published.clear();
        self.slots.push(Slot::Ref(reference));
        published.push(Slot::Ref(reference));
    }

    /// Raw bits are only for a consumer that already knows the type (heap or JNI).
    pub(crate) fn raw(&self) -> Vec<i32> {
        self.slots.iter().map(|slot| slot.value()).collect()
    }

    pub(crate) fn root_refs() -> Vec<i32> {
        TRANSFERS.with(|roots| {
            roots
                .borrow()
                .iter()
                .filter_map(Weak::upgrade)
                .flat_map(|slots| {
                    slots
                        .borrow()
                        .iter()
                        .filter(|slot| slot.is_ref())
                        .map(|slot| slot.value())
                        .collect::<Vec<_>>()
                })
                .collect()
        })
    }
}

impl Deref for RootedSlots {
    type Target = [Slot];

    fn deref(&self) -> &Self::Target {
        &self.slots
    }
}

impl Drop for RootedSlots {
    fn drop(&mut self) {
        let registration = Rc::downgrade(&self.published);
        TRANSFERS.with(|roots| {
            roots
                .borrow_mut()
                .retain(|root| !root.ptr_eq(&registration));
        });
    }
}
