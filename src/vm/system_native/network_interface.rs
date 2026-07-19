use crate::vm::error::Result;

/// `java.net.NetworkInterface.init()V`
pub(crate) fn init() -> Result<()> {
    Ok(()) // todo: implement me
}

/// `java.net.NetworkInterface.getAll()[Ljava/net/NetworkInterface;`
pub(crate) fn get_all() -> Result<i32> {
    Ok(0) // fixme: https://github.com/hextriclosan/rusty-jvm/issues/539
}
