use crate::vm::error::ErrorKind::{
    ClassFile, ConstantPool, Execution, Io, Native, UncaughtException,
};
use iana_time_zone::GetTimezoneError;
use jdescriptor::DescriptorError;
use jimage_rs::error::JImageError;
use miniz_oxide::MZError;
use num_enum::TryFromPrimitiveError;
use rand::rand_core::OsError;
use std::array::TryFromSliceError;
use std::error::Error as StdError;
use std::ffi::{NulError, OsString};
use std::fmt::{Debug, Display, Formatter};
use std::num::TryFromIntError;
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

    pub(crate) fn uncaught_exception() -> Error {
        Self::new(UncaughtException)
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }

    pub fn into_kind(self) -> ErrorKind {
        *self.0
    }

    /// Returns `true` if the error represents an unhandled Java exception.
    pub fn is_uncaught_exception(&self) -> bool {
        matches!(*self.0, UncaughtException)
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
            UncaughtException => write!(f, "Uncaught Exception"),

            ErrorKind::__Nonexhaustive => unreachable!(),
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::new(Io(error))
    }
}

impl From<jclassfile::error::Error> for Error {
    fn from(error: jclassfile::error::Error) -> Self {
        Error::new(ClassFile(error.to_string()))
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

impl From<JImageError> for Error {
    fn from(error: JImageError) -> Self {
        Error::new_execution(&format!("JImageError: {error}"))
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

impl<T: num_enum::TryFromPrimitive> From<TryFromPrimitiveError<T>> for Error {
    fn from(value: TryFromPrimitiveError<T>) -> Self {
        Error::new_execution(&format!("TryFromPrimitiveError: {value}"))
    }
}

impl From<OsString> for Error {
    fn from(value: OsString) -> Self {
        Error::new_execution(&format!(
            "Failed to convert OsString value {value:?} to String"
        ))
    }
}

impl From<OsError> for Error {
    fn from(value: OsError) -> Self {
        Error::new_execution(&format!("OsError: {value}"))
    }
}

impl From<TryFromIntError> for Error {
    fn from(value: TryFromIntError) -> Self {
        Error::new_execution(&format!("TryFromIntError: {value}"))
    }
}

impl From<NulError> for Error {
    fn from(value: NulError) -> Self {
        Error::new_execution(&format!("NulError: {value}"))
    }
}

impl From<MZError> for Error {
    fn from(error: MZError) -> Self {
        Error::new_execution(&format!("MZError: {error:?}"))
    }
}

impl From<TryFromSliceError> for Error {
    fn from(error: TryFromSliceError) -> Self {
        Error::new_execution(&format!("TryFromSliceError: {error}"))
    }
}

impl From<GetTimezoneError> for Error {
    fn from(error: GetTimezoneError) -> Self {
        Error::new_execution(&format!("GetTimezoneError: {error}"))
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
    UncaughtException,

    #[doc(hidden)]
    __Nonexhaustive,
}
