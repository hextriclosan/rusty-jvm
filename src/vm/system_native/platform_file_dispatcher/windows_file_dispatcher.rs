use crate::vm::error::Result;
use crate::vm::exception::helpers::throw_ioexception;
use crate::vm::exception::throwing_result::ThrowingResult;
use crate::vm::helper::{get_handle, i32toi64, i64_to_vec};
use crate::vm::stack::stack_frame::StackFrames;
use crate::vm::system_native::platform_native_dispatcher::windows_helpers::get_last_error;
use crate::{throw_and_return, unwrap_or_return_err, unwrap_result_or_return_default};
use std::mem::zeroed;
use std::ptr::null_mut;
use winapi::shared::minwindef::{DWORD, LPCVOID};
use winapi::um::fileapi::WriteFile;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::minwinbase::OVERLAPPED;
use winapi::um::sysinfoapi::{GetSystemInfo, SYSTEM_INFO};
use winapi::um::winnt::HANDLE;

pub fn allocation_granularity0_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    let result = allocation_granularity0();

    Ok(i64_to_vec(result))
}
fn allocation_granularity0() -> i64 {
    let mut si: SYSTEM_INFO = unsafe { zeroed() };
    unsafe { GetSystemInfo(&mut si) }

    si.dwAllocationGranularity as i64
}

pub fn windows_file_dispatcher_write0_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let fd_ref = args[0];
    let address = i32toi64(args[2], args[1]);
    let len = args[3];
    let append = args[4] != 0;

    let ret = unwrap_result_or_return_default!(
        write0(fd_ref, address, len, append, stack_frames),
        vec![]
    );
    Ok(vec![ret])
}
fn write0(
    fd_ref: i32,
    address: i64,
    len: i32,
    append: bool,
    stack_frames: &mut StackFrames,
) -> ThrowingResult<i32> {
    let handle = unwrap_or_return_err!(get_handle(fd_ref, stack_frames));
    let handle = handle as usize as HANDLE;
    if handle == INVALID_HANDLE_VALUE {
        throw_and_return!(throw_ioexception("Invalid handle", stack_frames))
    }

    let mut ov: OVERLAPPED = unsafe { zeroed() };
    let mut written = 0 as DWORD;
    let lp_ov = if append {
        unsafe { ov.u.s_mut().Offset = 0xFFFFFFFF };
        unsafe { ov.u.s_mut().OffsetHigh = 0xFFFFFFFF };
        &mut ov
    } else {
        null_mut()
    };
    let result = unsafe {
        WriteFile(
            handle,             /* File handle to write */
            address as LPCVOID, /* pointer to the buffer */
            len as DWORD,       /* number of bytes to write */
            &mut written,       /* receives number of bytes written */
            lp_ov,              /* overlapped struct */
        )
    };

    if result == 0 {
        let error_msg = unwrap_or_return_err!(get_last_error());
        throw_and_return!(throw_ioexception(&error_msg, stack_frames))
    }

    ThrowingResult::ok(written as i32)
}
