//! Purpose: Implements the native backend for `java.lang.Thread.sleep0()` and `sleepNanos0()`.
//!
//! Implementation Details:
//! Maps the Java thread sleep instructions directly to `tokio::time::sleep`.
//! Because `rusty-jvm` uses Tokio as its underlying scheduler, ALL Java threads 
//! (both standard and virtual) are backed by lightweight Tokio tasks. 
//! This `.await` safely yields the task, allowing the underlying OS thread 
//! to execute other Java threads until the sleep duration elapses.

use crate::vm::error::Result;
use crate::vm::helper::i32toi64;
use crate::vm::stack::stack_frame::StackFrames;
use std::time::Duration;

/// Pauses the current virtual thread for the specified duration in milliseconds.
///
/// Java signature: `private static native void sleep0(long millis)`
pub async fn sleep0(args: &[i32], _stack_frames: &mut StackFrames) -> Result<Vec<i32>> {
    // Reconstruct the 64-bit Java `long` from two 32-bit registers.
    let millis = i32toi64(args[1], args[0]);

    if millis > 0 {
        // Yield control to the Tokio scheduler asynchronously.
        tokio::time::sleep(Duration::from_millis(millis as u64)).await;
    } else if millis == 0 {
        // A sleep of 0 milliseconds is conventionally treated as a hint to yield.
        tokio::task::yield_now().await;
    }

    // Return empty vector for `void` Java return type.
    Ok(vec![])
}

/// Pauses the current virtual thread for the specified duration in nanoseconds.
///
/// Java signature: `private static native void sleepNanos0(long nanos)`
pub async fn sleep_nanos0(args: &[i32], _stack_frames: &mut StackFrames) -> Result<Vec<i32>> {
    let nanos = i32toi64(args[1], args[0]);

    if nanos > 0 {
        tokio::time::sleep(Duration::from_nanos(nanos as u64)).await;
    } else if nanos == 0 {
        tokio::task::yield_now().await;
    }

    Ok(Vec::new())
}