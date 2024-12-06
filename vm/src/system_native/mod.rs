pub(crate) mod class;
pub(crate) mod file_descriptor;
pub(crate) mod file_output_stream;
pub(crate) mod object;
pub(crate) mod reflecton;
pub(crate) mod string;
pub(crate) mod system;
pub(crate) mod system_props_raw;
pub(crate) mod thread;
pub(crate) mod unsafe_;

//// Platform-specific file operations.
#[cfg(unix)]
mod unix_file;
#[cfg(windows)]
mod windows_file;

#[cfg(unix)]
pub use unix_file::PlatformFile;
#[cfg(windows)]
pub use windows_file::PlatformFile;
////
