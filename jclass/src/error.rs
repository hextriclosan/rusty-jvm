use crate::error::ErrorKind::{InvalidInput, Io};
use std::fmt::{Debug, Display, Formatter};
use std::{error::Error as StdError, io, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error(Box<ErrorKind>);

impl Error {
    pub(crate) fn new(error_kind: ErrorKind) -> Self {
        Self(Box::new(error_kind))
    }

    pub(crate) fn new_io(kind: io::ErrorKind, error: &str) -> Self {
        Self::new(ErrorKind::Io(io::Error::new(kind, error)))
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }

    pub fn into_kind(self) -> ErrorKind {
        *self.0
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
pub enum ErrorKind {
    Io(io::Error),
    InvalidInput(String),

    #[doc(hidden)]
    __Nonexhaustive,
}
