use crate::helper::i64_to_vec;
use crate::stack::stack_frame::StackFrames;
use crate::system_native::method_handle_natives::invocation::invoke_exact;
use crate::system_native::method_handle_natives::offsets::get_field_offset;
use crate::system_native::method_handle_natives::resolution::resolve;

pub(crate) fn method_handle_natives_resolve_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let member_mame_ref = args[0];
    let caller_class_ref = args[1];
    let lookup_mode = args[2];
    let speculative_resolve = args[3] != 0;
    let member_mame_ref = resolve(
        member_mame_ref,
        caller_class_ref,
        lookup_mode,
        speculative_resolve,
    )?;
    Ok(vec![member_mame_ref])
}

pub(crate) fn method_handle_invoke_exact_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> crate::error::Result<Vec<i32>> {
    let handle_ref = args[0];
    let method_args = &args[1..];
    invoke_exact(handle_ref, method_args, stack_frames)?;
    Ok(vec![])
}

pub(crate) fn method_handle_natives_object_field_offset_wrp(
    args: &[i32],
) -> crate::error::Result<Vec<i32>> {
    let member_name_ref = args[0];
    let offset = get_field_offset(member_name_ref)?;
    Ok(i64_to_vec(offset))
}
