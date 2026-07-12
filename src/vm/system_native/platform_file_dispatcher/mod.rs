use crate::vm::error::Result;
use crate::vm::helper::get_handle;
use std::io::Seek;

#[cfg(windows)]
pub mod win;

#[cfg(target_os = "linux")]
pub mod linux;
pub(crate) mod mmap_registry;

#[cfg(unix)]
pub mod unix;

use crate::vm::exception::pending_helpers::set_pending_io_exception;
use crate::vm::system_native::platform_file::PlatformFile;
use crate::vm::system_native::platform_file_dispatcher::mmap_registry::MmapVariant;

/// `sun.nio.ch.UnixFileDispatcherImpl.allocationGranularity0()J`
/// `sun.nio.ch.FileDispatcherImpl.allocationGranularity0()J`
pub(crate) fn allocation_granularity0() -> Result<i64> {
    Ok(page_size::get_granularity() as i64)
}

/// `sun.nio.ch.UnixFileDispatcherImpl.map0(Ljava/io/FileDescriptor;IJJZ)J`
/// `sun.nio.ch.FileDispatcherImpl.map0(Ljava/io/FileDescriptor;IJJZ)J`
pub(crate) fn map0(
    fd_ref: i32,
    prot: i32,
    position: i64,
    length: i64,
    _is_sync: bool, // is not supported
) -> Result<i64> {
    #[cfg(windows)]
    let raw_handle = get_handle(fd_ref)? as std::os::windows::io::RawHandle;
    #[cfg(unix)]
    let raw_handle = get_handle(fd_ref)?;

    let addr = match MmapVariant::register(raw_handle, prot, position as u64, length as usize) {
        Ok(addr) => addr,
        Err(e) => {
            set_pending_io_exception(&format!("Memory mapping failed: {}", e))?;
            return Ok(0);
        }
    };

    Ok(addr)
}

/// `sun.nio.ch.UnixFileDispatcherImpl.isOther0(Ljava/io/FileDescriptor;)Z`
/// `sun.nio.ch.FileDispatcherImpl.isOther0(Ljava/io/FileDescriptor;)Z`
pub(crate) fn is_other0(fd_ref: i32) -> Result<bool> {
    let Some(file) = PlatformFile::get_by_fd(fd_ref)? else {
        return Ok(false);
    };

    let metadata = match file.metadata() {
        Ok(metadata) => metadata,
        Err(e) => {
            set_pending_io_exception(&format!("Failed to get file metadata: {}", e))?;
            return Ok(false);
        }
    };

    Ok(!metadata.is_file() && !metadata.is_dir() && !metadata.file_type().is_symlink())
}

/// `sun.nio.ch.UnixFileDispatcherImpl.seek0(Ljava/io/FileDescriptor;J)J`
/// `sun.nio.ch.FileDispatcherImpl.seek0(Ljava/io/FileDescriptor;J)J`
pub(crate) fn seek0(fd_ref: i32, offset: i64) -> Result<i64> {
    let Some(mut file) = PlatformFile::get_by_fd(fd_ref)? else {
        return Ok(0);
    };

    let current_offset = if offset < 0 {
        file.seek(std::io::SeekFrom::Current(0i64))
    } else {
        file.seek(std::io::SeekFrom::Start(offset as u64))
    };

    match current_offset {
        Ok(current_offset) => Ok(current_offset as i64),
        Err(e) => {
            set_pending_io_exception(&format!("Seek failed: {}", e))?;
            Ok(0)
        }
    }
}
