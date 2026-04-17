//! Purpose: Encapsulates asynchronous native method implementations for threading.
//!
//! Implementation Details:
//! Exposes the individual native method implementations so they can be registered
//! in the global `system_native_table.rs`. These functions share a common signature
//! returning a `Result<Vec<i32>>` asynchronously.

pub mod current;
pub mod sleep;
pub mod start;
pub mod yield_thread;
