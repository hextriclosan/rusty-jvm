//! Purpose: Implements `java.lang.Thread.yield0()`.
//!
//! Implementation Details:
//! Triggers a cooperative context switch by calling `tokio::task::yield_now()`,
//! moving the current task to the back of the scheduler's execution queue.

use crate::vm::error::Result;
use crate::vm::stack::stack_frame::StackFrames;

/// Hints to the scheduler that the current thread is willing to yield its current use of a processor.
///
/// Java signature: `private native void yield0()`
pub async fn yield0(_args: &[i32], _stack_frames: &mut StackFrames) -> Result<Vec<i32>> {
    // Explicitly yield the Tokio task, allowing pending tasks on the same OS thread to progress.
    tokio::task::yield_now().await;
    
    Ok(vec![])
}