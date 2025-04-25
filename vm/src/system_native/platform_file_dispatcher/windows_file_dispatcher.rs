use crate::error::Error;
use crate::heap::heap::with_heap_read_lock;
use crate::helper::{i32toi64, i64_to_vec, vec_to_i64};
use std::mem::zeroed;
use winapi::shared::minwindef::{DWORD, LPCVOID};
use winapi::um::fileapi::WriteFile;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::minwinbase::{LPOVERLAPPED, OVERLAPPED};
use winapi::um::sysinfoapi::{GetSystemInfo, SYSTEM_INFO};
use winapi::um::winnt::HANDLE;

pub fn allocation_granularity0_wrp(_args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let result = allocation_granularity0();

    Ok(i64_to_vec(result))
}
fn allocation_granularity0() -> i64 {
    let mut si: SYSTEM_INFO = unsafe { zeroed() };
    unsafe { GetSystemInfo(&mut si) }

    si.dwAllocationGranularity as i64
}

pub fn windows_file_dispatcher_write0_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let fd_ref = args[0];
    let address = i32toi64(args[2], args[1]);
    let len = args[3];
    let append = args[4] != 0;

    let result = write0(fd_ref, address, len, append)?;

    Ok(vec![result])
}
fn write0(fd_ref: i32, address: i64, len: i32, append: bool) -> crate::error::Result<i32> {
    let handle = with_heap_read_lock(|heap| {
        let raw = heap.get_object_field_value(fd_ref, "java/io/FileDescriptor", "handle")?;

        Ok::<i64, Error>(vec_to_i64(&raw))
    })?;

    let handle = handle as usize as HANDLE;
    if handle == INVALID_HANDLE_VALUE {
        return Err(Error::new_native(&"Invalid handle".to_string()));
    }

    let mut ov: OVERLAPPED = unsafe { zeroed() };
    let mut written = 0 as DWORD;
    let result = unsafe {
        let lp_ov: LPOVERLAPPED;
        if append {
            ov.u.s_mut().Offset = 0xFFFFFFFF;
            ov.u.s_mut().OffsetHigh = 0xFFFFFFFF;
            lp_ov = &mut ov;
        } else {
            lp_ov = 0 as LPOVERLAPPED;
        }

        WriteFile(
            handle,             /* File handle to write */
            address as LPCVOID, /* pointer to the buffer */
            len as DWORD,       /* number of bytes to write */
            &mut written,       /* receives number of bytes written */
            lp_ov,              /* overlapped struct */
        )
    };

    if result == 0 {
        return Err(Error::new_native(&"Write failed".to_string()));
    }

    Ok(written as i32)
}
