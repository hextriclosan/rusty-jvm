use crate::vm::safepoint;
use parking_lot::{Condvar, Mutex};
use std::collections::HashMap;
use std::thread::ThreadId;

/// Serializes publication per class name, not execution under a Rust lock. This
/// keeps concurrent loaders from returning different initialization state holders.
/// Claims may cover cache inspection and mirror publication only: class loading,
/// array component loading and lazy layout resolution must happen outside them.
#[derive(Debug, Default)]
pub(super) struct Loading {
    owners: Mutex<HashMap<String, ThreadId>>,
    cv: Condvar,
}

impl Loading {
    pub(super) fn claim<'a>(&'a self, name: &'a str) -> Claim<'a> {
        let thread = std::thread::current().id();
        let mut owners = safepoint::lock(&self.owners);
        loop {
            match owners.get(name) {
                Some(owner) if *owner == thread => {
                    return Claim {
                        loading: self,
                        name,
                        owns: false,
                    }
                }
                Some(_) => owners = safepoint::wait(&self.cv, &self.owners, owners),
                None => {
                    owners.insert(name.to_string(), thread);
                    return Claim {
                        loading: self,
                        name,
                        owns: true,
                    };
                }
            }
        }
    }
}

pub(super) struct Claim<'a> {
    loading: &'a Loading,
    name: &'a str,
    owns: bool,
}

impl Drop for Claim<'_> {
    fn drop(&mut self) {
        if self.owns {
            let mut owners = safepoint::lock(&self.loading.owners);
            owners.remove(self.name);
            self.loading.cv.notify_all();
        }
    }
}
