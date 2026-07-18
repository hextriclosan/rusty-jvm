use crate::vm::error::Result;

/// `jdk.internal.misc.Signal.findSignal0(Ljava/lang/String;)I`
pub(crate) fn find_signal0(_name_ref: i32) -> Result<i32> {
    Ok(0) // todo: implement me
}

/// `jdk.internal.misc.Signal.handle0(IJ)J`
pub(crate) fn handle0(_sig: i32, _native_h: i64) -> Result<i64> {
    Ok(0) // todo: implement me
}
