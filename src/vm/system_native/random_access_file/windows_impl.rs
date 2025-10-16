use crate::vm::exception::helpers::throw_file_not_found_exception;
use crate::vm::exception::throwing_result::ThrowingResult;
use crate::vm::stack::stack_frame::StackFrames;
use crate::vm::system_native::platform_file::{Mode, PlatformFile};
use crate::vm::system_native::platform_native_dispatcher::windows_helpers::get_last_error;
use crate::vm::system_native::platform_specific_files::wide_cstring::WideCString;
use crate::vm::system_native::random_access_file::RandomAccessFileMode;
use crate::vm::system_native::string::get_utf8_string_by_ref;
use crate::{throw_and_return, unwrap_or_return_err};
use std::os::windows::io::{FromRawHandle, OwnedHandle};
use std::path::Path;
use std::ptr::null_mut;
use winapi::um::fileapi::{CreateFileW, OPEN_ALWAYS, OPEN_EXISTING};
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::winbase::{FILE_FLAG_DELETE_ON_CLOSE, FILE_FLAG_WRITE_THROUGH};
use winapi::um::winnt::{
    FILE_ATTRIBUTE_NORMAL, FILE_SHARE_READ, FILE_SHARE_WRITE, GENERIC_READ, GENERIC_WRITE,
};

pub(super) fn open0(
    obj_ref: i32,
    file_name_ref: i32,
    mode: RandomAccessFileMode,
    stack_frames: &mut StackFrames,
) -> ThrowingResult<()> {
    let file_name = unwrap_or_return_err!(get_utf8_string_by_ref(file_name_ref));

    if Path::new(&file_name).is_dir() {
        throw_and_return!(throw_file_not_found_exception(
            file_name_ref,
            &format!("{file_name} is a directory"),
            stack_frames
        ))
    }

    let path = WideCString::new(&file_name);

    let mut access = 0;
    if mode.contains(RandomAccessFileMode::O_RDONLY) {
        access |= GENERIC_READ;
    }
    if mode.contains(RandomAccessFileMode::O_RDWR) {
        access |= GENERIC_READ | GENERIC_WRITE;
    }

    let sharing = FILE_SHARE_READ | FILE_SHARE_WRITE;
    let disposition = if mode.contains(RandomAccessFileMode::O_RDWR) {
        OPEN_ALWAYS
    } else {
        OPEN_EXISTING
    };
    let mut flags_and_attributes = FILE_ATTRIBUTE_NORMAL;
    if mode.intersects(RandomAccessFileMode::O_SYNC | RandomAccessFileMode::O_DSYNC) {
        flags_and_attributes |= FILE_FLAG_WRITE_THROUGH; // fn open0(...) can't be platform-agnostic because of this O_SYNC and O_DSYNC
    }
    if mode.contains(RandomAccessFileMode::O_TEMPORARY) {
        flags_and_attributes |= FILE_FLAG_DELETE_ON_CLOSE;
    }

    let handle = unsafe {
        CreateFileW(
            path.as_ptr(),
            access,
            sharing,
            null_mut(),
            disposition,
            flags_and_attributes,
            null_mut(),
        )
    };

    if handle == INVALID_HANDLE_VALUE {
        let last_error = unwrap_or_return_err!(get_last_error());
        throw_and_return!(throw_file_not_found_exception(
            file_name_ref,
            &format!("Could not open file: {last_error}"),
            stack_frames
        ));
    }

    let owned_handle = unsafe { OwnedHandle::from_raw_handle(handle as _) };
    unwrap_or_return_err!(PlatformFile::set_raw_id(
        obj_ref,
        owned_handle,
        Mode::RandomAccessFile
    ));
    ThrowingResult::ok(())
}
