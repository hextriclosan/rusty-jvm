#[cfg(unix)]
pub(crate) mod unix_file;
#[cfg(windows)]
pub(crate) mod windows_file;

use strum::{AsRefStr, EnumString};
#[cfg(unix)]
pub use unix_file::PlatformFile;
#[cfg(windows)]
pub use windows_file::PlatformFile;

#[derive(EnumString, AsRefStr, Debug)]
pub enum Mode {
    #[strum(serialize = "java/io/FileInputStream")]
    Read,
    #[strum(serialize = "java/io/FileOutputStream")]
    Write,
    #[strum(serialize = "java/io/RandomAccessFile")]
    ReadWrite,
}
