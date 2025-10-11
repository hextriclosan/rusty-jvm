use crate::vm::error::Result;
use crate::vm::exception::helpers::throw_ioexception;
use crate::vm::exception::throwing_result::ThrowingResult;
use crate::vm::helper::{get_fd, i32toi64, i64_to_vec};
use crate::vm::stack::stack_frame::StackFrames;
use crate::{throw_and_return, unwrap_or_return_err, unwrap_result_or_return_default};
use nix::sys::stat::fstat;
use nix::sys::uio::pread;
use nix::unistd::{read, write};
use std::os::fd::BorrowedFd;
use std::slice::{from_raw_parts, from_raw_parts_mut};

const IOS_EOF: i32 = -1;
const IOS_UNAVAILABLE: i32 = -2;
const IOS_INTERRUPTED: i32 = -3;

pub fn unix_file_dispatcher_impl_write0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let fd_ref = args[0];
    let address = i32toi64(args[2], args[1]);
    let len = args[3];

    let result =
        unwrap_result_or_return_default!(write0(fd_ref, address, len, stack_frames), vec![]);
    Ok(vec![result])
}
fn write0(
    fd_ref: i32,
    address: i64,
    len: i32,
    stack_frames: &mut StackFrames,
) -> ThrowingResult<i32> {
    let address = address as usize as *const u8;
    let buf = unsafe { from_raw_parts(address, len as usize) };

    let fd = unwrap_or_return_err!(get_fd(fd_ref));
    let fd = unsafe { BorrowedFd::borrow_raw(fd) };
    let result = match write(fd, buf) {
        Ok(written) => written as i32,
        Err(errno) => match errno {
            nix::errno::Errno::EAGAIN => IOS_UNAVAILABLE,
            nix::errno::Errno::EINTR => IOS_INTERRUPTED,
            _ => {
                throw_and_return!(throw_ioexception("Write failed", stack_frames))
            }
        },
    };

    ThrowingResult::ok(result)
}

pub fn unix_file_dispatcher_impl_read0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let fd_ref = args[0];
    let address = i32toi64(args[2], args[1]);
    let len = args[3];

    let result =
        unwrap_result_or_return_default!(read0(fd_ref, address, len, stack_frames), vec![]);
    Ok(vec![result])
}
fn read0(
    fd_ref: i32,
    address: i64,
    len: i32,
    stack_frames: &mut StackFrames,
) -> ThrowingResult<i32> {
    let address = address as usize as *mut u8;
    let buf = unsafe { from_raw_parts_mut(address, len as usize) };

    let fd = unwrap_or_return_err!(get_fd(fd_ref));
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
                throw_and_return!(throw_ioexception("Read failed", stack_frames))
            }
        },
    };

    ThrowingResult::ok(result)
}

pub fn unix_file_dispatcher_impl_pread0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let fd_ref = args[0];
    let address = i32toi64(args[2], args[1]);
    let len = args[3];
    let position = i32toi64(args[5], args[4]);

    let result = unwrap_result_or_return_default!(
        pread0(fd_ref, address, len, position, stack_frames),
        vec![]
    );
    Ok(vec![result])
}
fn pread0(
    fd_ref: i32,
    address: i64,
    len: i32,
    position: i64,
    stack_frames: &mut StackFrames,
) -> ThrowingResult<i32> {
    let address = address as usize as *mut u8;
    let buf = unsafe { from_raw_parts_mut(address, len as usize) };

    let fd = unwrap_or_return_err!(get_fd(fd_ref));
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
                throw_and_return!(throw_ioexception("Read failed", stack_frames))
            }
        },
    };

    ThrowingResult::ok(result)
}

pub fn unix_file_dispatcher_impl_size0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let fd_ref = args[0];

    let result = unwrap_result_or_return_default!(size0(fd_ref, stack_frames), vec![]);
    Ok(i64_to_vec(result))
}
fn size0(fd_ref: i32, stack_frames: &mut StackFrames) -> ThrowingResult<i64> {
    let fd = unwrap_or_return_err!(get_fd(fd_ref));
    let fd = unsafe { BorrowedFd::borrow_raw(fd) };
    let result = match fstat(fd) {
        Ok(stat) => stat.st_size as i64,
        Err(errno) if matches!(errno, nix::errno::Errno::EINTR) => IOS_INTERRUPTED as i64,
        Err(errno) => {
            throw_and_return!(throw_ioexception(
                &format!("Size failed: {}", errno.to_string()),
                stack_frames
            ))
        }
    };

    ThrowingResult::ok(result)
}
