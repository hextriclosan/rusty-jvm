use crate::vm::error::Result;
use crate::vm::exception::pending_helpers::set_pending_io_exception;
use crate::vm::system_native::platform_file_dispatcher::mmap_registry::MmapVariant;

/// `java.nio.MappedMemoryUtils.registerNatives()V`
pub(crate) fn register_natives() -> Result<()> {
    Ok(()) // todo: implement me
}

/// `java.nio.MappedMemoryUtils.force0(Ljava/io/FileDescriptor;JJ)V`
pub(crate) fn force0(_fd_ref: i32, address: i64, length: i64) -> Result<()> {
    if let Err(e) = MmapVariant::flush(address, length as usize) {
        set_pending_io_exception(&format!("Forcing mapped memory to storage failed: {}", e))?;
    }

    Ok(())
}
