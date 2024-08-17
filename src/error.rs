use std::{error::Error as StdError, io, result};
use std::fmt::{Debug, Display, Formatter};

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
        write!(f, "{}", self)
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
