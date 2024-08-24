use std::{io, result};
use std::fmt::{Debug, Display, Formatter};
use crate::error::ErrorKind::{ClassFile, ConstantPool, Execution, Io};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error(Box<ErrorKind>);

impl Error {
    pub(crate) fn new(error_kind: ErrorKind) -> Self {
        Self(Box::new(error_kind))
    }

    pub(crate) fn new_constant_pool(descr: &str) -> Self {
        Self::new(ConstantPool(String::from(descr)))
    }

    pub(crate) fn new_execution(descr: &str) -> Self {
        Self::new(Execution(String::from(descr)))
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
            ClassFile(err) => write!(f, "ClassFile Error: {err}"),
            ConstantPool(descr) => write!(f, "ConstantPool Error: {descr}"),
            Execution(descr) => write!(f, "Execution Error: {descr}"),

            ErrorKind::__Nonexhaustive => unreachable!(),
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::new(Io(error))
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    Io(io::Error),
    ClassFile(String),
    ConstantPool(String),
    Execution(String),

    #[doc(hidden)]
    __Nonexhaustive,
}
