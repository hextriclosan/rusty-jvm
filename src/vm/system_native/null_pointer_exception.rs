use crate::vm::error::Result;

/// `java.lang.NullPointerException.getExtendedNPEMessage()Ljava/lang/String;`
pub(crate) fn get_extended_npe_message(_this: i32) -> Result<i32> {
    Ok(0) // todo: https://github.com/hextriclosan/rusty-jvm/issues/521
}
