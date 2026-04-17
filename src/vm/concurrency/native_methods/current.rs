//! Purpose: Implements `java.lang.Thread.currentThread()`.
//!
//! Implementation Details:
//! Retrieves the currently executing thread's object reference directly from the
//! Tokio task-local storage context, avoiding global thread state lookups.
//! During the early JVM boot sequence, the task-local context might be 0, 
//! in which case it safely falls back to the global `MethodArea`.

use crate::vm::error::Result;
use crate::vm::stack::stack_frame::StackFrames;

/// Returns the JVM heap reference to the currently executing `Thread` object.
///
/// Java signature: `public static native Thread currentThread()`
pub async fn current_thread(_args: &[i32], _stack_frames: &mut StackFrames) -> Result<Vec<i32>> {
    // Extract the thread ID securely bound to this specific Tokio task.
    let mut thread_ref = crate::vm::concurrency::task_local::CURRENT_JAVA_THREAD_ID
        .try_with(|id| *id)
        .unwrap_or(0);
    
    // FIX: Task Local Shadowing during JVM Boot.
    // If the task-local is 0 (because we are in the middle of bootstrapping the 
    // primordial thread), we MUST fall back to the global MethodArea which receives 
    // the thread ID mid-execution!
    if thread_ref == 0 {
        thread_ref = crate::vm::method_area::method_area::with_method_area(|area| {
            area.system_thread_id().unwrap_or(0)
        });
    }
    
    Ok(vec![thread_ref])
}