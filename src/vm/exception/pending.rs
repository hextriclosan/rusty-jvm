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
//! quietly". This module's `Throws<T>` type encodes the three possible outcomes in a single return type:
//! - `Ok(Some(value))` means "success, return this value"
//! - `Ok(None)` means "a Java exception was thrown, bail out quietly"
//! - `Err(...)` means "VM-level failure, propagate as `?`"

use crate::vm::error::Result;

/// A computation that may either return a value, throw a Java exception, or fail with a VM error.
pub type Throws<T> = Result<Option<T>>;

/// Convenience constructor for the "Java exception was thrown" case.
///
/// Equivalent to `Ok(None)` but reads more clearly at call sites.
#[inline]
pub fn thrown<T>() -> Throws<T> {
    Ok(None)
}

#[macro_export]
#[doc(hidden)]
macro_rules! bail_thrown {
    ($throw:expr) => {{
        // Enforce that `$throw` is a Result<(), _> (i.e., a "throw Java exception" helper).
        let _res: ::std::result::Result<(), _> = $throw;
        _res?;
        return Ok(None);
    }};
}
