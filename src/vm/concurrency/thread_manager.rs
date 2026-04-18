//! Purpose: Tracks and manages the lifecycle of spawned Virtual Threads.
//!
//! Implementation Details:
//! Maintains a highly concurrent `DashMap` associating the JVM thread reference (an `i32` object ID)
//! with its corresponding Tokio `JoinHandle`. This registry allows the JVM to safely implement
//! `Thread.join()`, track active threads, and ensure clean JVM shutdown.

use dashmap::DashMap;
use std::sync::LazyLock;
use tokio::task::JoinHandle;

/// Global registry of running Java threads and their underlying Tokio task handles.
pub static THREAD_HANDLES: LazyLock<DashMap<i32, JoinHandle<()>>> =
    LazyLock::new(DashMap::default);

/// Registers a newly spawned Tokio task as a running Java thread.
/// 
/// # Arguments
/// * `thread_ref` - The heap reference to the `java.lang.Thread` object.
/// * `handle` - The Tokio `JoinHandle` representing the asynchronous execution task.
pub fn register_thread(thread_ref: i32, handle: JoinHandle<()>) {
    THREAD_HANDLES.insert(thread_ref, handle);
}

/// Removes a thread handle from the registry.
/// This should be called when the thread's execution successfully completes or terminates.
pub fn unregister_thread(thread_ref: i32) {
    THREAD_HANDLES.remove(&thread_ref);
}
