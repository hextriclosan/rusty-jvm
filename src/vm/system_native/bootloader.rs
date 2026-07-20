use crate::vm::error::Result;

/// `jdk.internal.loader.BootLoader.setBootLoaderUnnamedModule0(Ljava/lang/Module;)V`
pub(crate) fn set_bootloader_unnamed_module0(_module_ref: i32) -> Result<()> {
    Ok(())
}
