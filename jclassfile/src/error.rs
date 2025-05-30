use crate::error::ErrorKind::{InvalidInput, Io};
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
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &*self.0 {
            Io(err) => write!(f, "I/O Error: {err}"),
            InvalidInput(descr) => write!(f, "InvalidInput Error: {descr}"),

            ErrorKind::__Nonexhaustive => unreachable!(),
        }
    }
}

impl StdError for Error {}

#[derive(Debug)]
pub(crate) enum ErrorKind {
    Io(io::Error),
    InvalidInput(String),

    #[doc(hidden)]
    __Nonexhaustive,
}
