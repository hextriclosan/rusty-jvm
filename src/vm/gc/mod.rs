//! Precise, nonmoving mark-sweep collection, root enumeration and managed-heap admission.
//!
//! Collection runs under an exclusive safepoint operation and fails closed: incomplete roots
//! or invalid edges skip a cycle before sweep. See `docs/garbage_collection.md` for contracts,
//! accounting and supported limits.

pub(crate) mod budget;
pub(crate) mod collector;
mod config;
pub(crate) mod layout;
pub(crate) mod pending;
pub(crate) mod references;
pub(crate) mod roots;
pub(crate) mod trigger;

#[cfg(test)]
pub(crate) mod test_allocations;

pub(crate) use config::{get, install, logging_enabled};
