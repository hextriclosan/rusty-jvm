use crate::error::ErrorKind::{Format, Io, Truncated};
use std::error::Error as StdError;
use std::fmt::{Debug, Display, Formatter};
use std::{io, result};

pub(crate) type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
/// Error type for the jclassfile library.
pub struct Error(Box<ErrorKind>);

impl Error {
    pub(crate) fn new(error_kind: ErrorKind) -> Self {
        Self(Box::new(error_kind))
    }

    pub(crate) fn new_io(kind: io::ErrorKind, error: &str) -> Self {
        Self::new(Io(io::Error::new(kind, error)))
    }

    /// A class file that fails format checking (JVMS §4.8).
    pub(crate) fn format(descr: impl Into<String>) -> Self {
        Self::new(Format(descr.into()))
    }

    /// A read of `size` bytes at `offset` that does not fit in the `len` bytes available.
    pub(crate) fn truncated(offset: usize, size: usize, len: usize) -> Self {
        Self::new(Truncated { offset, size, len })
    }

    pub(crate) fn is_truncated(&self) -> bool {
        matches!(*self.0, Truncated { .. })
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &*self.0 {
            Io(err) => write!(f, "I/O Error: {err}"),
            Format(descr) => write!(f, "Format Error: {descr}"),
            Truncated { offset, size, len } => write!(
                f,
                "Format Error: class file is truncated: {size} byte(s) expected at offset {offset}, but its length is {len} (JVMS §4.8)"
            ),

            ErrorKind::__Nonexhaustive => unreachable!(),
        }
    }
}

impl StdError for Error {}

#[derive(Debug)]
pub(crate) enum ErrorKind {
    Io(io::Error),
    Format(String),
    Truncated {
        offset: usize,
        size: usize,
        len: usize,
    },

    #[doc(hidden)]
    __Nonexhaustive,
}
