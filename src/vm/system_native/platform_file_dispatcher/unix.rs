use crate::vm::error::Result;
use crate::vm::exception::pending_helpers::set_pending_io_exception;
use crate::vm::helper::get_handle;
use crate::vm::system_native::platform_file::PlatformFile;
use nix::sys::stat::fstat;
use nix::sys::uio::pread;
use nix::unistd::{ftruncate, read, write};
use std::os::fd::BorrowedFd;
use std::slice::{from_raw_parts, from_raw_parts_mut};

const IOS_EOF: i32 = -1;
const IOS_UNAVAILABLE: i32 = -2;
const IOS_INTERRUPTED: i32 = -3;

/// `sun.nio.ch.UnixFileDispatcherImpl.write0(Ljava/io/FileDescriptor;JI)I`
pub(crate) fn write0(fd_ref: i32, address: i64, len: i32) -> Result<i32> {
    let address = address as usize as *const u8;
    let buf = unsafe { from_raw_parts(address, len as usize) };

    let fd = get_handle(fd_ref)?;
    let fd = unsafe { BorrowedFd::borrow_raw(fd) };
    let result = match write(fd, buf) {
        Ok(written) => written as i32,
        Err(errno) => match errno {
            nix::errno::Errno::EAGAIN => IOS_UNAVAILABLE,
            nix::errno::Errno::EINTR => IOS_INTERRUPTED,
            _ => {
                set_pending_io_exception(&format!("Write failed: {}", errno))?;
                return Ok(0);
            }
        },
    };

    Ok(result)
}

/// `sun.nio.ch.UnixFileDispatcherImpl.read0(Ljava/io/FileDescriptor;JI)I`
pub(crate) fn read0(fd_ref: i32, address: i64, len: i32) -> Result<i32> {
    let address = address as usize as *mut u8;
    let buf = unsafe { from_raw_parts_mut(address, len as usize) };

    let fd = (get_handle(fd_ref))?;
    let fd = unsafe { BorrowedFd::borrow_raw(fd) };
    let result = match read(fd, buf) {
        Ok(read) => {
            if read == 0 {
                IOS_EOF // EOF is -1 in javaland
            } else {
                read as i32
            }
        }
        Err(errno) => match errno {
            nix::errno::Errno::EAGAIN => IOS_UNAVAILABLE,
            nix::errno::Errno::EINTR => IOS_INTERRUPTED,
            _ => {
                set_pending_io_exception(&format!("Read failed: {}", errno))?;
                return Ok(0);
            }
        },
    };

    Ok(result)
}

/// `sun.nio.ch.UnixFileDispatcherImpl.pread0(Ljava/io/FileDescriptor;JIJ)I`
pub(crate) fn pread0(fd_ref: i32, address: i64, len: i32, position: i64) -> Result<i32> {
    let address = address as usize as *mut u8;
    let buf = unsafe { from_raw_parts_mut(address, len as usize) };

    let fd = (get_handle(fd_ref))?;
    let fd = unsafe { BorrowedFd::borrow_raw(fd) };
    let result = match pread(fd, buf, position) {
        Ok(read) => {
            if read == 0 {
                IOS_EOF // EOF is -1 in javaland
            } else {
                read as i32
            }
        }
        Err(errno) => match errno {
            nix::errno::Errno::EAGAIN => IOS_UNAVAILABLE,
            nix::errno::Errno::EINTR => IOS_INTERRUPTED,
            _ => {
                set_pending_io_exception(&format!("Read failed: {}", errno))?;
                return Ok(0);
            }
        },
    };

    Ok(result)
}

/// `sun.nio.ch.UnixFileDispatcherImpl.size0(Ljava/io/FileDescriptor;)J`
pub(crate) fn size0(fd_ref: i32) -> Result<i64> {
    let fd = (get_handle(fd_ref))?;
    let fd = unsafe { BorrowedFd::borrow_raw(fd) };
    let result = match fstat(fd) {
        Ok(stat) => stat.st_size,
        Err(errno) if matches!(errno, nix::errno::Errno::EINTR) => IOS_INTERRUPTED as i64,
        Err(errno) => {
            set_pending_io_exception(&format!("Size failed: {}", errno))?;
            return Ok(0);
        }
    };

    Ok(result)
}

/// `sun.nio.ch.UnixFileDispatcherImpl.truncate0(Ljava/io/FileDescriptor;J)I`
pub(crate) fn truncate0(fd_ref: i32, size: i64) -> Result<i32> {
    let Some(file) = PlatformFile::get_by_fd_pending(fd_ref)? else {
        return Ok(0);
    };

    let ret = match ftruncate(&*file, size) {
        Ok(_) => 0,
        Err(errno) if matches!(errno, nix::errno::Errno::EINTR) => IOS_INTERRUPTED,
        Err(errno) => {
            set_pending_io_exception(&format!("Truncate failed: {}", errno))?;
            return Ok(0);
        }
    };

    Ok(ret)
}
