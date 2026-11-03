use crate::vm::error::ErrorKind::{
    Allocation, ClassFile, ConstantPool, Execution, Io, JniExceptionAlreadyPending, Native,
    UncaughtException,
};
use crate::vm::gc::budget::AllocError;
use crate::vm::stack::rooted_slots::RootedSlots;
use crate::vm::stack::slot::Slot;
use iana_time_zone::GetTimezoneError;
use jdescriptor::DescriptorError;
use jimage_rs::error::JImageError;
use miniz_oxide::MZError;
use num_enum::TryFromPrimitiveError;
use rand::rngs::SysError;
use std::array::TryFromSliceError;
use std::env::VarError;
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
pub struct Error(ErrorKind);

/// Owns the published root while an uncaught Java exception crosses Rust calls.
#[derive(Debug)]
pub struct RootedThrowable {
    root: ThrowableRoot,
}

#[derive(Debug)]
enum ThrowableRoot {
    Owned(RootedSlots),
    Permanent([Slot; 1]),
}

impl RootedThrowable {
    pub(crate) fn new(throwable: Slot) -> Self {
        let root = if crate::vm::jni::handles::emergency_oom_ref() == Some(throwable.value()) {
            ThrowableRoot::Permanent([throwable])
        } else {
            ThrowableRoot::Owned(RootedSlots::from_slots(vec![throwable]))
        };
        Self { root }
    }

    fn reference(&self) -> i32 {
        self.slot().value()
    }

    pub(crate) fn slot(&self) -> Slot {
        match &self.root {
            ThrowableRoot::Owned(root) => root[0],
            ThrowableRoot::Permanent(slot) => slot[0],
        }
    }
}

impl Error {
    pub(crate) fn new(error_kind: ErrorKind) -> Self {
        Self(error_kind)
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

    pub(crate) fn uncaught_exception(throwable: Slot) -> Error {
        Self::new(UncaughtException(RootedThrowable::new(throwable)))
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }

    pub fn into_kind(self) -> ErrorKind {
        self.0
    }

    /// Returns `true` if the error represents an unhandled Java exception.
    pub fn is_uncaught_exception(&self) -> bool {
        self.throwable_ref().is_some()
    }

    /// If this error represents an uncaught Java exception, return the throwable heap reference.
    pub fn throwable_ref(&self) -> Option<i32> {
        match &self.0 {
            UncaughtException(throwable) => Some(throwable.reference()),
            Allocation(_) => crate::vm::jni::handles::emergency_oom_ref(),
            _ => None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Allocation(error) => write!(f, "Allocation Error: {error}"),
            Io(err) => write!(f, "I/O Error: {err}"),
            ClassFile(err) => write!(f, "ClassFile Error: {err}"),
            ConstantPool(descr) => write!(f, "ConstantPool Error: {descr}"),
            Execution(descr) => write!(f, "Execution Error: {descr}"),
            Native(descr) => write!(f, "Native Call Error: {descr}"),
            UncaughtException(throwable) => {
                write!(
                    f,
                    "Uncaught Exception (throwable_ref={})",
                    throwable.reference()
                )
            }
            JniExceptionAlreadyPending(throwable_ref) => write!(
                f,
                "JNI already has a pending exception (existing throwable_ref={throwable_ref}"
            ),

            ErrorKind::__Nonexhaustive => unreachable!(),
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::new(Io(error))
    }
}

impl From<jclassmodel::Error> for Error {
    fn from(error: jclassmodel::Error) -> Self {
        let message = match StdError::source(&error) {
            Some(source) => format!("{}: {source}", error.message()),
            None => error.message().to_string(),
        };

        match error.kind() {
            jclassmodel::ErrorKind::ClassFile => Error::new(ClassFile(message)),
            jclassmodel::ErrorKind::ConstantPool => Error::new(ConstantPool(message)),
            _ => Error::new_execution(&message),
        }
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

impl From<SysError> for Error {
    fn from(value: SysError) -> Self {
        Error::new_execution(&format!("SysError: {value}"))
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

impl From<whoami::Error> for Error {
    fn from(error: whoami::Error) -> Self {
        Error::new_execution(&format!("whoami::Error: {error}"))
    }
}

impl From<VarError> for Error {
    fn from(error: VarError) -> Self {
        Error::new_execution(&format!("VarError: {error}"))
    }
}

impl StdError for Error {}

impl From<AllocError> for Error {
    fn from(error: AllocError) -> Self {
        if let AllocError::NegativeLength(length) = error {
            // Validation occurs before any array reservation or payload construction.
            // Negatives keep the existing JNI decimal message convention.
            use crate::vm::execution_engine::executor::Executor;
            use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
            use crate::vm::stack::stack_value::StackValueKind;
            let exception =
                StringPoolHelper::get_string(&length.to_string()).and_then(|message| {
                    Executor::invoke_args_constructor(
                        "java/lang/NegativeArraySizeException",
                        "<init>:(Ljava/lang/String;)V",
                        &[StackValueKind::Ref(message)],
                        None,
                    )
                });
            return match exception {
                Ok(throwable) => Self::uncaught_exception(throwable[0]),
                Err(error) => error,
            };
        }
        Self::new(Allocation(error))
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    Allocation(AllocError),
    Io(io::Error),
    ClassFile(String),
    ConstantPool(String),
    Execution(String),
    Native(String),
    UncaughtException(RootedThrowable),
    JniExceptionAlreadyPending(i32),

    #[doc(hidden)]
    __Nonexhaustive,
}
