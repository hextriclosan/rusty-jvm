use crate::vm::error::Error;

pub enum ThrowingResult<T> {
    Result(crate::vm::error::Result<T>),
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

#[macro_export]
macro_rules! unwrap_or_return_err {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(e) => return ThrowingResult::err(e),
        }
    };
}

#[macro_export]
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

#[macro_export]
macro_rules! unwrap_result_or_return_default {
    ($expr:expr, $default:expr) => {{
        match $expr {
            ThrowingResult::Result(result) => result?,
            ThrowingResult::ExceptionThrown => return Ok($default),
        }
    }};
}
