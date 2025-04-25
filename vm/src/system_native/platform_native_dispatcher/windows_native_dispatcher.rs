use winapi::shared::minwindef::{DWORD, FALSE, LPCVOID, LPVOID, MAX_PATH, TRUE};
use winapi::um::errhandlingapi::GetLastError;
use crate::error::Error;
use winapi::um::fileapi::{GetFileAttributesExW, GetLogicalDrives, GetVolumeInformationW, GetVolumePathNameW, SetEndOfFile};
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::minwinbase::{GetFileExInfoStandard, SECURITY_ATTRIBUTES};
use winapi::um::processthreadsapi::{GetCurrentProcess, GetCurrentThread, OpenProcessToken, OpenThreadToken};
use winapi::um::securitybaseapi::GetFileSecurityW;
use winapi::um::winbase::{FormatMessageW, FORMAT_MESSAGE_FROM_SYSTEM};
use winapi::um::winnt::{HANDLE, LPCWSTR, PHANDLE, PSECURITY_DESCRIPTOR};
use crate::execution_engine::string_pool_helper::StringPoolHelper;
use crate::heap::heap::with_heap_write_lock;
use crate::helper::{i32toi64, i64_to_vec};

pub fn get_file_attributes_ex0_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let lp_file_name_addr = i32toi64(args[1], args[0]);
    let output_addr = i32toi64(args[3], args[2]);
    get_file_attributes_ex0(lp_file_name_addr, output_addr)?;

    Ok(vec![])
}
fn get_file_attributes_ex0(lp_file_name_addr: i64, output_addr: i64) -> crate::error::Result<()> {
    eprintln!("!!! get_file_attributes_ex0: lp_file_name_addr = {lp_file_name_addr}, output_addr = {output_addr}");
    unsafe {
        let x = lp_file_name_addr as usize as LPCWSTR;
        let mut len = 0;
        while *x.offset(len) != 0 {
            len += 1;
        }
        let slice = std::slice::from_raw_parts(x, len as usize);
        let string = String::from_utf16_lossy(slice);
        eprintln!("Content of x: {}", string);
        let res = GetFileAttributesExW(x, GetFileExInfoStandard, output_addr as usize as LPVOID);
        if res == 0 {
            let error_code = GetLastError();

            let message = [0u16; 256];

            let len = FormatMessageW(
                FORMAT_MESSAGE_FROM_SYSTEM,
                0 as LPCVOID,
                error_code,
                0,
                message.as_ptr() as *mut _,
                255,
                0 as *mut _,
            );

            eprintln!(
                "!!! get_file_attributes_ex0: {error_code} {len} {:?}",
                String::from_utf16_lossy(&message)
            );
            return Err(crate::error::Error::new_execution(
                "GetFileAttributesExW failed",
            ));
        }
    };

    Ok(())
}

pub fn get_file_security0_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let lp_file_name_addr = i32toi64(args[1], args[0]);
    let requested_information = args[2];
    let p_security_descriptor_addr = i32toi64(args[4], args[3]);
    let n_length = args[5];

    let result = get_file_security0(
        lp_file_name_addr,
        requested_information,
        p_security_descriptor_addr,
        n_length,
    )?;

    Ok(vec![result])
}
fn get_file_security0(
    lp_file_name_addr: i64,
    requested_information: i32,
    p_security_descriptor_addr: i64,
    n_length: i32,
) -> crate::error::Result<i32> {
    eprintln!("!!! get_file_security0: lp_file_name_addr = {lp_file_name_addr}, requested_information = {requested_information}, p_security_descriptor_addr = {p_security_descriptor_addr}, n_length = {n_length}");
    unsafe {
        let x = lp_file_name_addr as usize as LPCWSTR;
        {
            let mut len = 0;
            while *x.offset(len) != 0 {
                len += 1;
            }
            let slice = std::slice::from_raw_parts(x, len as usize);
            let string = String::from_utf16_lossy(slice);
            eprintln!("!!! get_file_security0 Content of x: {}", string);
        }
        let length_needed = 0 as DWORD;
        let res = GetFileSecurityW(
            x,
            requested_information as DWORD,
            p_security_descriptor_addr as LPVOID,
            n_length as DWORD,
            length_needed as *mut _,
        );
        if res == 0 {
            let error_code = GetLastError();

            let message = [0u16; 256];

            let len = FormatMessageW(
                FORMAT_MESSAGE_FROM_SYSTEM,
                0 as LPCVOID,
                error_code,
                0,
                message.as_ptr() as *mut _,
                255,
                0 as *mut _,
            );

            eprintln!(
                "!!! get_file_security0: {error_code} {:?}",
                String::from_utf16_lossy(&message)
            );
            // return Err(crate::error::Error::new_execution(
            //     "GetFileSecurityW failed",
            // ));
        }
    };

    Ok(1) // fixme
}

pub fn get_current_process_wrp(_args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let process_id = get_current_process()?;

    Ok(i64_to_vec(process_id))
}
fn get_current_process() -> crate::error::Result<i64> {
    let handle = unsafe { GetCurrentProcess() as usize as i64 };

    Ok(handle)
}

pub fn get_current_thread_wrp(_args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let process_id = get_current_process()?;

    Ok(i64_to_vec(process_id))
}
fn get_current_thread() -> crate::error::Result<i64> {
    let handle = unsafe { GetCurrentThread() as usize as i64 };

    Ok(handle)
}

pub fn open_process_token_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let process = i32toi64(args[1], args[0]);
    let desired_access = args[2];

    let h_token = open_process_token(process, desired_access)?;

    Ok(i64_to_vec(h_token))
}
fn open_process_token(process: i64, desired_access: i32) -> crate::error::Result<i64> {
    let handle = unsafe {
        let h_token = 0i64 as usize as PHANDLE;
        let result =
            OpenProcessToken(process as usize as HANDLE, desired_access as DWORD, h_token);
        if result == 0 {
            let error_code = GetLastError();
            let message = [0u16; 256];

            let len = FormatMessageW(
                FORMAT_MESSAGE_FROM_SYSTEM,
                0 as LPCVOID,
                error_code,
                0,
                message.as_ptr() as *mut _,
                255,
                0 as *mut _,
            );

            eprintln!(
                "!!! open_process_token: {error_code} {len} {:?}",
                String::from_utf16_lossy(&message)
            );
            // return Err(crate::error::Error::new_execution(
            //     "OpenProcessToken failed",
            // ));
        }

        h_token
    };

    Ok(handle as usize as i64)
}

pub fn open_thread_token_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let thread = i32toi64(args[1], args[0]);
    let desired_access = args[2];
    let open_as_self = args[3] != 0;

    let h_token = open_thread_token(thread, desired_access, open_as_self)?;

    Ok(i64_to_vec(h_token))
}
fn open_thread_token(
    thread: i64,
    desired_access: i32,
    open_as_self: bool,
) -> crate::error::Result<i64> {
    let handle = unsafe {
        let h_thread = thread as usize as HANDLE;
        let h_token = 0i64 as usize as PHANDLE;
        let open_as_self = if open_as_self { TRUE } else { FALSE };

        let result = OpenThreadToken(h_thread, desired_access as DWORD, open_as_self, h_token);

        if result == 0 {
            let error_code = GetLastError();
            let message = [0u16; 256];

            let len = FormatMessageW(
                FORMAT_MESSAGE_FROM_SYSTEM,
                0 as LPCVOID,
                error_code,
                0,
                message.as_ptr() as *mut _,
                255,
                0 as *mut _,
            );

            eprintln!(
                "!!! open_thread_token: {error_code} {len} {:?}",
                String::from_utf16_lossy(&message)
            );
            // return Err(crate::error::Error::new_execution(
            //     "OpenProcessToken failed",
            // ));
        }

        h_token
    };

    //Ok(handle as usize as i64)
    Ok(-1) // fixme
}

pub fn close_handle_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let handle = i32toi64(args[1], args[0]);
    close_handle(handle)?;

    Ok(vec![])
}
fn close_handle(handle: i64) -> crate::error::Result<()> {
    let result = unsafe {
        CloseHandle(handle as usize as HANDLE) // fixme errorhandling
    };

    eprintln!("!!! close_handle: {result}");

    Ok(())
}

pub fn get_volume_path_name0_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let address = i32toi64(args[1], args[0]);
    let string_ref = get_volume_path_name0(address)?;

    Ok(vec![string_ref])
}
fn get_volume_path_name0(address: i64) -> crate::error::Result<i32> {
    let volume_name = [0u16; MAX_PATH + 1];
    let result = unsafe {
        let lp_file_name = address as usize as LPCWSTR;

        GetVolumePathNameW(
            lp_file_name,
            volume_name.as_ptr() as *mut _,
            MAX_PATH as DWORD + 1,
        )
    };

    eprintln!("!!! get_volume_path_name0: {result}");

    let mut len = 0;
    while volume_name[len] != 0 {
        len += 1;
    }
    let slice = &volume_name[0..len];
    let string = String::from_utf16_lossy(&slice);

    eprintln!("!!! get_volume_path_name0: {string}");

    let string_ref = StringPoolHelper::get_string(string)?;

    Ok(string_ref)
}

pub fn get_volume_information0_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let address = i32toi64(args[1], args[0]);
    let volume_information_ref = args[2];
    get_volume_information0(address, volume_information_ref)?;

    Ok(vec![])
}
fn get_volume_information0(address: i64, volume_information_ref: i32) -> crate::error::Result<()> {
    let volume_name = [0u16; MAX_PATH + 1];
    let volume_serial_number = 0 as DWORD;
    let max_component_length = 0 as DWORD;
    let flags = 0 as DWORD;
    let filesystem_name = [0u16; MAX_PATH + 1];

    let result = unsafe {
        GetVolumeInformationW(
            address as usize as LPCWSTR,
            volume_name.as_ptr() as *mut _,
            MAX_PATH as DWORD + 1,
            volume_serial_number as *mut _,
            max_component_length as *mut _,
            flags as *mut _,
            filesystem_name.as_ptr() as *mut _,
            MAX_PATH as DWORD + 1,
        )
    };

    let mut len = 0;
    while volume_name[len] != 0 {
        len += 1;
    }
    let slice = &volume_name[0..len];
    let volume_name = String::from_utf16_lossy(&slice);

    let mut len = 0;
    while filesystem_name[len] != 0 {
        len += 1;
    }
    let slice = &filesystem_name[0..len];
    let filesystem_name = String::from_utf16_lossy(&slice);

    let volume_name_ref = StringPoolHelper::get_string(volume_name.clone())?;
    let filesystem_name_ref = StringPoolHelper::get_string(filesystem_name.clone())?;

    with_heap_write_lock(|heap| {
        heap.set_object_field_value(
            volume_information_ref,
            "sun/nio/fs/WindowsNativeDispatcher$VolumeInformation",
            "volumeName",
            vec![volume_name_ref],
        )?;
        heap.set_object_field_value(
            volume_information_ref,
            "sun/nio/fs/WindowsNativeDispatcher$VolumeInformation",
            "fileSystemName",
            vec![filesystem_name_ref],
        )?;

        heap.set_object_field_value(
            volume_information_ref,
            "sun/nio/fs/WindowsNativeDispatcher$VolumeInformation",
            "volumeSerialNumber",
            vec![volume_serial_number as i32],
        )?;
        heap.set_object_field_value(
            volume_information_ref,
            "sun/nio/fs/WindowsNativeDispatcher$VolumeInformation",
            "flags",
            vec![flags as i32],
        )?;
        Ok::<(), Error>(())
    })?;

    eprintln!("!!! get_volume_information0: result={result} volume_serial_number={volume_serial_number} max_component_length={max_component_length} flags={flags} filesystem_name={filesystem_name:?} volume_name={volume_name}");

    Ok(())
}

pub fn get_logical_drives_wrp(_args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let logical_drives = get_logical_drives()?;

    Ok(vec![logical_drives])
}
fn get_logical_drives() -> crate::error::Result<i32> {
    let handle = unsafe { GetLogicalDrives() };
    if handle == 0 {
        return Err(Error::new_execution("GetLogicalDrives failed"));
    }

    Ok(handle as i32)
}

pub fn create_directory0_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let address_ptr = i32toi64(args[1], args[0]);
    let sd_address_ptr = i32toi64(args[3], args[2]);
    create_directory0(address_ptr, sd_address_ptr)?;

    Ok(vec![])
}
fn create_directory0(address_ptr: i64, sd_address_ptr: i64) -> crate::error::Result<()> {
    unsafe {
        let address = address_ptr as usize as LPCWSTR;

        {
            let mut len = 0;
            while *address.offset(len) != 0 {
                len += 1;
            }
            let slice = std::slice::from_raw_parts(address, len as usize);
            let string = String::from_utf16_lossy(slice);
            eprintln!("!!! create_directory0 Content of address: {}", string);
        }

        //let security_attributes = SECURITY_ATTRIBUTES;
        //let lp_security_attributes = LPSECURITY_ATTRIBUTES;
        let sd_address = sd_address_ptr as usize as PSECURITY_DESCRIPTOR;

        let result = winapi::um::fileapi::CreateDirectoryW(address, 0 as *mut SECURITY_ATTRIBUTES);

        if result == 0 {
            let error_code = GetLastError();
            let message = [0u16; 256];

            let len = FormatMessageW(
                FORMAT_MESSAGE_FROM_SYSTEM,
                0 as LPCVOID,
                error_code,
                0,
                message.as_ptr() as *mut _,
                255,
                0 as *mut _,
            );

            eprintln!(
                "!!! create_directory0: {error_code} {len} {:?}",
                String::from_utf16_lossy(&message)
            );
            // return Err(crate::error::Error::new_execution(
            //     "create_directory0 failed",
            // ));
        }
    }

    Ok(())
}

pub fn create_file0_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    //long lpFileName,
    //int dwDesiredAccess,
    //int dwShareMode,
    //long lpSecurityAttributes,
    //int dwCreationDisposition,
    //int dwFlagsAndAttributes
    let filename_ptr = i32toi64(args[1], args[0]);
    let desired_access = args[2];
    let share_mode = args[3];
    let seq_attr_ptr = i32toi64(args[5], args[4]);
    let creation_disposition = args[6];
    let flags_and_attributes = args[7];

    let result = create_file0(
        filename_ptr,
        desired_access,
        share_mode,
        seq_attr_ptr,
        creation_disposition,
        flags_and_attributes,
    )?;

    Ok(i64_to_vec(result))
}
fn create_file0(
    filename_ptr: i64,
    desired_access: i32,
    share_mode: i32,
    seq_attr_ptr: i64,
    creation_disposition: i32,
    flags_and_attributes: i32,
) -> crate::error::Result<i64> {
    let result = unsafe {
        let filename = filename_ptr as usize as LPCWSTR;

        {
            let mut len = 0;
            while *filename.offset(len) != 0 {
                len += 1;
            }
            let slice = std::slice::from_raw_parts(filename, len as usize);
            let string = String::from_utf16_lossy(slice);
            eprintln!("!!! create_file0 Content of filename: {}", string);
        }

        let result = winapi::um::fileapi::CreateFileW(
            filename,
            desired_access as DWORD,
            share_mode as DWORD,
            0 as *mut SECURITY_ATTRIBUTES,
            creation_disposition as DWORD,
            flags_and_attributes as DWORD,
            0 as HANDLE,
        );

        if result == INVALID_HANDLE_VALUE {
            let error_code = GetLastError();
            let message = [0u16; 256];

            let len = FormatMessageW(
                FORMAT_MESSAGE_FROM_SYSTEM,
                0 as LPCVOID,
                error_code,
                0,
                message.as_ptr() as *mut _,
                255,
                0 as *mut _,
            );

            eprintln!(
                "!!! create_file0: {error_code} {len} {:?}",
                String::from_utf16_lossy(&message)
            );
            return Err(Error::new_execution("create_file0 failed"));
        }

        result
    };

    Ok(result as i64)
}

pub(crate) fn set_end_of_file_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let handle_ptr = i32toi64(args[1], args[0]);

    Ok(vec![])
}
fn set_end_of_file(handle_ptr: i64) -> crate::error::Result<()> {
    unsafe {
        let handle = handle_ptr as usize as HANDLE;

        if SetEndOfFile(handle) == 0 {
            return Err(Error::new_execution("set_end_of_file failed"));
        }
    }

    Ok(())
}
