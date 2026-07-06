use crate::vm::error::Result;

pub(crate) fn before_halt() -> Result<()> {
    // todo: implement me
    Ok(())
}

pub(crate) fn halt0(status: i32) -> Result<()> {
    std::process::exit(status); // fixme: by doing this we skip destructors and other shutdown hooks, later it might be an issue
}
