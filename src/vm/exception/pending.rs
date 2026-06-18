//! Helpers for the "set-pending-Java-exception then return a default" pattern that mirrors the
//! JNI native code style (see `vm::jni::class_operations_impl::find_class`).
//!
//! ## The problem this module solves
//!
//! Some VM-internal Rust functions need to *both*:
//! 1. throw a Java exception (which, in this VM, immediately mutates the calling
//!    [`StackFrames`](crate::vm::stack::stack_frame::StackFrames) so the next bytecode dispatch
//!    runs the matching `catch` handler), **and**
//! 2. signal to their direct Rust caller that the normal post-call work (pushing a return value
//!    onto the operand stack, copying out parameters, ...) **must not happen** because control
//!    has already been transferred.
//!
//! Plain `Result<T, E>` doesn't capture the second outcome cleanly: the caller can't tell whether
//! `Err(...)` means "VM-level failure, propagate as `?`" or "Java exception, just bail out
//! quietly". Earlier this module shipped a custom `ThrowingResult<T>` enum with three variants
//! to encode it. We've replaced that with the much more idiomatic [`Throws<T>`] type alias plus
//! small helpers (`thrown()` / `bail_thrown!`) — see below.
//!
//! ## The replacement
//!
//! | Outcome                                           | Encoding                       |
//! |---------------------------------------------------|--------------------------------|
//! | success with value `v`                            | `Ok(Some(v))`                  |
//! | Java exception thrown (stack frames already unwound) | `Ok(None)`                  |
//! | VM-internal error (bug, OOM, IO error in the VM, …) | `Err(e)`                     |
//!
//! At call sites this composes naturally:
//!
//! ```ignore
//! // Inside a fn returning `Throws<X>`:
//! let Some(value) = some_throws_call(stack_frames)? else { return Ok(None); };
//! //                                              ^ propagates VM errors
//! //              ^ destructures the `Some` / short-circuits on `None`
//! ```
//!
//! Inside a `*_wrp` function returning `Result<Vec<i32>>`, the same pattern picks a sentinel:
//! ```ignore
//! let Some(value) = some_throws_call(stack_frames)? else { return Ok(vec![]); };
//! ```

use crate::vm::error::Result;

/// A computation that may either return a value, throw a Java exception, or fail with a VM error.
///
/// See the [module-level docs](self) for the encoding.
pub type Throws<T> = Result<Option<T>>;

/// Convenience constructor for the "Java exception was thrown" case.
///
/// Equivalent to `Ok(None)` but reads more clearly at call sites.
#[inline]
pub fn thrown<T>() -> Throws<T> {
    Ok(None)
}

#[macro_export]
macro_rules! bail_thrown {
    ($throw:expr) => {{
        $throw?;
        return Ok(None);
    }};
}
