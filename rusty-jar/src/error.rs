use snafu::Snafu;
use std::path::PathBuf;

/// Represents errors that can occur while working with JAR files.
#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum JarError {
    #[snafu(display("I/O error for path '{path}': {source}", path = path.display()))]
    Io {
        #[snafu(source)]
        source: std::io::Error,
        path: PathBuf,
    },
    #[snafu(display("Zip error : {source}"))]
    Zip {
        #[snafu(source)]
        source: zip::result::ZipError,
    },
    #[snafu(display("Entry is absent: {entry_name}"))]
    EntryNotFound { entry_name: String },
}

/// Type alias for a Result type that uses JarError for error handling.
pub type Result<T> = std::result::Result<T, JarError>;

impl From<zip::result::ZipError> for JarError {
    fn from(value: zip::result::ZipError) -> Self {
        JarError::Zip { source: value }
    }
}
