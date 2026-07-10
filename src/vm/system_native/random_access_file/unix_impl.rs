use crate::vm::error::Result;
use crate::vm::exception::pending_helpers::set_pending_file_not_found_exception;
use crate::vm::system_native::platform_file::{Mode, PlatformFile};
use crate::vm::system_native::random_access_file::RandomAccessFileMode;
use crate::vm::system_native::string::get_utf8_string_by_ref;
use nix::fcntl::{open, OFlag};
use std::path::Path;

pub(super) fn open0(obj_ref: i32, file_name_ref: i32, flags: RandomAccessFileMode) -> Result<()> {
    let file_name = get_utf8_string_by_ref(file_name_ref)?;

    let path = Path::new(&file_name);
    if path.is_dir() {
        set_pending_file_not_found_exception(
            file_name_ref,
            &format!("{file_name} is a directory"),
        )?;
        return Ok(());
    }

    let mut oflag = OFlag::empty();
    if flags.contains(RandomAccessFileMode::O_RDONLY) {
        oflag |= OFlag::O_RDONLY;
    }
    if flags.contains(RandomAccessFileMode::O_RDWR) {
        oflag |= OFlag::O_RDWR | OFlag::O_CREAT;
    }
    // fn open0(...) can't be platform-agnostic because of this O_SYNC and O_DSYNC
    if flags.contains(RandomAccessFileMode::O_SYNC) {
        oflag |= OFlag::O_SYNC;
    }
    if flags.contains(RandomAccessFileMode::O_DSYNC) {
        oflag |= OFlag::O_DSYNC;
    }

    let mode = nix::sys::stat::Mode::from_bits_truncate(0o666);
    let owned_fd = match open(path, oflag, mode) {
        Ok(fd) => fd,
        Err(e) => {
            let message = e.to_string();
            set_pending_file_not_found_exception(file_name_ref, &message)?;
            return Ok(());
        }
    };

    PlatformFile::set_raw_id(obj_ref, owned_fd, Mode::RandomAccessFile)?;
    Ok(())
}
