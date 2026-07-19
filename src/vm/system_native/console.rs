use crate::vm::error::Result;

/// `java.io.Console.istty()Z`
pub(crate) fn istty() -> Result<bool> {
    Ok(true) // todo: implement me
}
