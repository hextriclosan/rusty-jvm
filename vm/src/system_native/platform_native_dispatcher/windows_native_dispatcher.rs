use crate::error::Error;
use winapi::um::fileapi::GetLogicalDrives;

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
