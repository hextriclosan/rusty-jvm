use crate::vm::error::Result;
use crate::vm::exception::pending_helpers::set_pending_sync_failed_exception;
use crate::vm::system_native::platform_file::PlatformFile;

/// `java.io.FileDescriptor.initIDs()V`
pub(crate) fn init_ids() -> Result<()> {
    // todo: implement me
    Ok(())
}

/// `java.io.FileDescriptor.close0()V`
pub(crate) fn close0(fd_ref: i32) -> Result<()> {
    PlatformFile::close(fd_ref)?;
    Ok(())
}

pub(crate) fn sync0(fd_ref: i32) -> Result<()> {
    if let Err(error) = PlatformFile::sync(fd_ref) {
        set_pending_sync_failed_exception(&error.to_string())?;
    }
    Ok(())
}

/// `java.io.FileDescriptor.getHandle(I)J`
pub(crate) fn get_handle(fd: i32) -> Result<i64> {
    let handle = PlatformFile::get_handle(fd)?;
    Ok(handle)
}

/// `java.io.FileDescriptor.getAppend(I)Z`
pub(crate) fn get_append(fd: i32) -> Result<bool> {
    let append = PlatformFile::get_append(fd)?;
    Ok(append)
}
