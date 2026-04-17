//! Purpose: Entry point for the JVM concurrency module.
//!
//! Implementation Details:
//! This module encapsulates all Tokio-related thread scheduling, task-local state,
//! and thread management. It re-exports submodules to keep the core execution
//! engine decoupled from the underlying asynchronous runtime implementation.

pub mod native_methods;
pub mod task_local;
pub mod thread_manager;

/// Synchronously waits for an asynchronous JVM operation.
/// Safely bridges the synchronous JVM legacy code with the async Executor
/// without blocking the Tokio runtime.
pub fn block_on_async<F: std::future::Future>(future: F) -> F::Output {
    // Read the current Java thread ID before stepping out of the async context.
    // If we are not in an async context (e.g. early JVM boot), fallback to the primordial thread.
    let current_thread = crate::vm::concurrency::task_local::CURRENT_JAVA_THREAD_ID
        .try_with(|id| *id)
        .unwrap_or_else(|_| {
            crate::vm::method_area::method_area::with_method_area(|area| {
                area.system_thread_id().unwrap_or(0)
            })
        });

    let f = async move {
        crate::vm::concurrency::task_local::CURRENT_JAVA_THREAD_ID
            .scope(current_thread, future)
            .await
    };

    match tokio::runtime::Handle::try_current() {
        Ok(handle) => {
            // We are inside a Tokio runtime. We must use `block_in_place` to safely block 
            // the worker thread without starving other asynchronous tasks.
            tokio::task::block_in_place(move || handle.block_on(f))
        }
        Err(_) => {
            // We are entirely outside of a Tokio runtime context (e.g., during JVM prelude/shutdown).
            // We spin up a lightweight multi-thread runtime to execute this logic so that any
            // deeply nested block_on_async calls can safely use block_in_place.
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(2) // Minimal footprint for boot/shutdown fallback
                .enable_all()
                .build()
                .expect("Failed to create temporary Tokio runtime")
                .block_on(f)
        }
    }
}