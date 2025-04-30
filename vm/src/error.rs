use crate::error::ErrorKind::{ClassFile, ConstantPool, Execution, Io, Native};
use jdescriptor::DescriptorError;
use std::error::Error as StdError;
use std::ffi::OsString;
use std::fmt::{Debug, Display, Formatter};
use std::string::{FromUtf16Error, FromUtf8Error};
use std::sync::PoisonError;
use std::time::SystemTimeError;
use std::{io, result};

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

    pub(crate) fn new_native(descr: &str) -> Error {
        Self::new(Native(String::from(descr)))
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
            Native(descr) => write!(f, "Native Call Error: {descr}"),

            ErrorKind::__Nonexhaustive => unreachable!(),
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::new(Io(error))
    }
}

impl From<SystemTimeError> for Error {
    fn from(error: SystemTimeError) -> Self {
        Error::new_execution(&format!("SystemTimeError: {error}"))
    }
}

impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Self {
        Error::new_execution(&format!("FromUtf8Error: {error}"))
    }
}

impl From<FromUtf16Error> for Error {
    fn from(error: FromUtf16Error) -> Self {
        Error::new_execution(&format!("FromUtf16Error: {error}"))
    }
}

impl From<DescriptorError> for Error {
    fn from(error: DescriptorError) -> Self {
        Error::new_execution(&format!("DescriptorError: {error}"))
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::new_execution(&format!("serde_json::Error: {error}"))
    }
}

impl<T> From<PoisonError<T>> for Error {
    fn from(error: PoisonError<T>) -> Self {
        Error::new_execution(&format!("PoisonError: {error}"))
    }
}

impl From<OsString> for Error {
    fn from(value: OsString) -> Self {
        Error::new_execution(&format!(
            "Failed to convert OsString value {value:?} to String"
        ))
    }
}

impl StdError for Error {}

#[derive(Debug)]
pub enum ErrorKind {
    Io(io::Error),
    ClassFile(String),
    ConstantPool(String),
    Execution(String),
    Native(String),

    #[doc(hidden)]
    __Nonexhaustive,
}
