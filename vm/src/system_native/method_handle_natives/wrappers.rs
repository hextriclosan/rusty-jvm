use crate::helper::i64_to_vec;
use crate::method_area::method_area::with_method_area;
use crate::stack::stack_frame::StackFrames;
use crate::system_native::method_handle_natives::invocation::invoke_exact;
use crate::system_native::method_handle_natives::member_name::MemberName;
use crate::system_native::method_handle_natives::offsets::{
    get_field_offset, get_static_field_offset,
};
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

pub(crate) fn method_handle_invoke_basic_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> crate::error::Result<Vec<i32>> {
    method_handle_invoke_exact_wrp(args, stack_frames) // TODO: implement real invokeBasic
}

pub(crate) fn method_handle_natives_object_field_offset_wrp(
    args: &[i32],
) -> crate::error::Result<Vec<i32>> {
    let member_name_ref = args[0];
    let offset = get_field_offset(member_name_ref)?;
    Ok(i64_to_vec(offset))
}

pub(crate) fn method_handle_natives_static_field_offset_wrp(
    args: &[i32],
) -> crate::error::Result<Vec<i32>> {
    let member_name_ref = args[0];
    let offset = get_static_field_offset(member_name_ref)?;
    Ok(i64_to_vec(offset))
}

pub(crate) fn method_handle_natives_static_field_base_wrp(
    args: &[i32],
) -> crate::error::Result<Vec<i32>> {
    let member_name_ref = args[0];
    let base_ref = static_field_base(member_name_ref)?;
    Ok(vec![base_ref])
}
fn static_field_base(member_name_ref: i32) -> crate::error::Result<i32> {
    let member_name = MemberName::new(member_name_ref)?;
    let class_name = member_name.class_name();
    let class_ref = with_method_area(|area| area.load_reflection_class(class_name))?;
    Ok(class_ref)
}

// By not setting a value for `box` we break the check loop on the Java side so, just bypassing the check
pub(crate) fn method_handle_natives_get_named_con_wrp(
    args: &[i32],
) -> crate::error::Result<Vec<i32>> {
    let _what = args[0];
    let _box_ref = args[1];
    Ok(vec![0])
}

pub(crate) fn method_handle_natives_get_member_vm_info_wrp(
    args: &[i32],
) -> crate::error::Result<Vec<i32>> {
    // returns {vmindex,vmtarget}
    let member_name_ref = args[0];
    let member_name = MemberName::new(member_name_ref)?;
    let array_ref = member_name.get_member_vm_info(member_name_ref)?;
    Ok(vec![array_ref])
}
