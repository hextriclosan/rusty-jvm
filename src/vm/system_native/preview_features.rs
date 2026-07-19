use crate::vm::error::Result;

/// `jdk.internal.misc.PreviewFeatures.isPreviewEnabled()Z`
pub(crate) fn is_preview_enabled() -> Result<bool> {
    Ok(false) // todo: implement me
}
