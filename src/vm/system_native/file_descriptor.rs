use crate::vm::error::Result;
use crate::vm::system_native::platform_file::PlatformFile;

/// `java.io.FileDescriptor.initIDs()V`
pub(crate) fn init_ids() -> Result<()> {
    // todo: implement me
    Ok(())
}

/// `java.io.FileDescriptor.close0()V`
pub(crate) fn file_descriptor_close0(fd_ref: i32) -> Result<()> {
    PlatformFile::close(fd_ref)?;
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
