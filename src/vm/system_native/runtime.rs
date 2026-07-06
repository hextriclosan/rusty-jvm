use crate::vm::error::Result;

/// `java.lang.Runtime.maxMemory()J`
pub(crate) fn max_memory(_this: i32) -> Result<i64> {
    Ok(i64::MAX) // todo: use meaningful value, maybe use `sysinfo` crate to get the actual memory size
}

/// `java.lang.Runtime.availableProcessors()I`
pub(crate) fn available_processors(_this: i32) -> Result<i32> {
    let available_parallelism = std::thread::available_parallelism()?;
    Ok(available_parallelism.get() as i32)
}
