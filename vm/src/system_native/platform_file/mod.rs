#[cfg(unix)]
pub(crate) mod unix_file;
#[cfg(windows)]
pub(crate) mod windows_file;
#[cfg(unix)]
pub use unix_file::PlatformFile;
#[cfg(windows)]
pub use windows_file::PlatformFile;
