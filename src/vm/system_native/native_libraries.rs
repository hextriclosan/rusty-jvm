use crate::vm::error::Result;
use crate::vm::stack::stack_frame::StackFrames;
use crate::vm::system_native::string::get_utf8_string_by_ref;
use tracing::{enabled, trace};

pub(crate) fn find_builtin_lib_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    if enabled!(tracing::Level::TRACE) {
        let name_ref = args[0];
        let name = get_utf8_string_by_ref(name_ref, stack_frames)?;
        trace!("findBuiltinLib: {name}");
    }

    Ok(vec![0]) // we don't have static libraries, so we always return null
}
