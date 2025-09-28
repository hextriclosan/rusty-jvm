use crate::vm::error::{Error, Result};

pub enum ThrowingResult<T> {
    Result(Result<T>),
    ExceptionThrown,
}

impl<T> ThrowingResult<T> {
    pub fn ok(val: T) -> Self {
        ThrowingResult::Result(Ok(val))
    }

    pub fn err(e: Error) -> Self {
        ThrowingResult::Result(Err(e))
    }

    pub fn thrown() -> Self {
        ThrowingResult::ExceptionThrown
    }
}

/// Unwraps a `Result<T, E>` and returns early as `ThrowingResult::err(E)` if it's an `Err`.
///
/// # Panics / Constraints
/// Must be used inside a function returning `ThrowingResult<T>`.
#[macro_export]
#[doc(hidden)]
macro_rules! unwrap_or_return_err {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(e) => return ThrowingResult::err(e.into()),
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! throw_and_return {
    ($expr:expr) => {{
        // Force type-checking: this will only compile if $expr is Result<(), E>
        let _res: ::std::result::Result<(), _> = $expr;
        match _res {
            Ok(()) => return ThrowingResult::thrown(),
            Err(e) => return ThrowingResult::err(e),
        }
    }};
}

/// Unwraps a `ThrowingResult<T>`:
/// - Returns the inner `T` if `ThrowingResult::Result(Ok(t))`
/// - Propagates error if `ThrowingResult::Result(Err(e))`
/// - Returns `Ok($default)` if `ThrowingResult::ExceptionThrown`
///
/// # Constraints
/// Must be used inside a function returning `Result<T, E>`.
#[macro_export]
#[doc(hidden)]
macro_rules! unwrap_result_or_return_default {
    ($expr:expr, $default:expr) => {{
        match $expr {
            ThrowingResult::Result(result) => result?,
            ThrowingResult::ExceptionThrown => return Ok($default),
        }
    }};
}
