use crate::vm::error::Result;
use crate::vm::helper::i64_to_vec;
use crate::vm::system_native::platform_file::PlatformFile;

pub(crate) fn file_descriptor_close0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let fd_ref = args[0];

    PlatformFile::close(fd_ref)?;
    Ok(vec![])
}

pub(crate) fn get_handle_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let fd = args[0];

    let handle = PlatformFile::get_handle(fd)?;

    Ok(i64_to_vec(handle))
}
