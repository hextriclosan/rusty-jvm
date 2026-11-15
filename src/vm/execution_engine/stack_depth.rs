//! The limit of a thread's Java stack (JVMS §2.5.2): past [`MAX_FRAME_DEPTH`] frames, a call
//! raises `StackOverflowError` instead of pushing one more. Frames live on the heap here, not on
//! the OS thread's stack, so the limit is a count rather than a size: one in the range that
//! HotSpot's default 1 MB thread stack allows for small interpreted frames, so that runaway
//! recursion ends the same way, instead of in the VM eating memory until something else gives.

use crate::vm::jni::java_thread::{JavaThread, StackOverflowHandling};

pub(crate) const MAX_FRAME_DEPTH: usize = 10_000;

/// Whether the call about to push a frame has to raise `StackOverflowError` instead. The guard it
/// returns then suspends the limit for as long as it lives: raising the error takes frames of its
/// own, which must not overflow in turn. `None` as well while the limit is already suspended.
pub(crate) fn exhausted() -> Option<StackOverflowHandling> {
    if JavaThread::frame_depth() < MAX_FRAME_DEPTH {
        return None;
    }
    JavaThread::begin_stack_overflow_handling()
}
