use std::error::Error as StdError;
use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;

/// What went wrong while building the model of a class file.
///
/// Deliberately coarse: callers generally cannot recover from any of these, they only report them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorKind {
    /// The class file could not be parsed at all.
    ClassFile,
    /// A constant-pool entry was absent, or held a different kind than the reference to it implied.
    ConstantPool,
    /// A field or method descriptor was not well formed.
    Descriptor,
    /// A structural expectation of the class file was not met (e.g. a non-abstract, non-native
    /// method without a `Code` attribute).
    Structure,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            ErrorKind::ClassFile => "ClassFile",
            ErrorKind::ConstantPool => "ConstantPool",
            ErrorKind::Descriptor => "Descriptor",
            ErrorKind::Structure => "Structure",
        };
        write!(f, "{text}")
    }
}

/// Error returned when a class file cannot be turned into a model.
///
/// The underlying parser error, when there is one, is reachable through
/// [`StdError::source`] but its concrete type is not part of this crate's API.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    message: String,
    source: Option<Box<dyn StdError + Send + Sync + 'static>>,
}

impl Error {
    pub(crate) fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            source: None,
        }
    }

    pub(crate) fn with_source(
        kind: ErrorKind,
        message: impl Into<String>,
        source: impl StdError + Send + Sync + 'static,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    pub(crate) fn constant_pool(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::ConstantPool, message)
    }

    pub(crate) fn structure(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Structure, message)
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} Error: {}", self.kind, self.message)
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &dyn StdError)
    }
}
