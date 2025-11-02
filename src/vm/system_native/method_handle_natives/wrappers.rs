use crate::vm::error::Result;
use crate::vm::heap::heap::with_heap_write_lock;
use crate::vm::helper::i64_to_vec;
use crate::vm::method_area::method_area::with_method_area;
use crate::vm::stack::stack_frame::StackFrames;
use crate::vm::system_native::method_handle_natives::invocation::invoke_exact;
use crate::vm::system_native::method_handle_natives::member_name::MemberName;
use crate::vm::system_native::method_handle_natives::native_accessor::{
    native_accessor_invoke0, native_accessor_newinstance0,
};
use crate::vm::system_native::method_handle_natives::offsets::{
    get_field_offset, get_static_field_offset,
};
use crate::vm::system_native::method_handle_natives::resolution::{member_name_init, resolve};
use crate::vm::system_native::method_handle_natives::var_handle::{
    var_handle_compare_and_set, var_handle_get, var_handle_set,
};

pub(crate) fn method_handle_natives_init_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let member_name_ref = args[0];
    let obj_ref = args[1];
    member_name_init(member_name_ref, obj_ref)?;

    Ok(vec![])
}

pub(crate) fn method_handle_natives_resolve_wrp(args: &[i32]) -> Result<Vec<i32>> {
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
) -> Result<Vec<i32>> {
    let handle_ref = args[0];
    let method_args = &args[1..];
    invoke_exact(handle_ref, method_args, stack_frames)?;
    Ok(vec![])
}

pub(crate) fn method_handle_invoke_basic_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    method_handle_invoke_exact_wrp(args, stack_frames) // TODO: implement real invokeBasic
}

pub(crate) fn method_handle_natives_object_field_offset_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let member_name_ref = args[0];
    let offset = get_field_offset(member_name_ref)?;
    Ok(i64_to_vec(offset))
}

pub(crate) fn method_handle_invoke_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    // The invoke method should handle type conversions and argument adaptations that invokeExact does not perform,
    // making this a temporary placeholder that could cause incorrect behavior.
    // TODO: implement real invoke
    method_handle_invoke_exact_wrp(args, stack_frames)
}

pub(crate) fn method_handle_natives_static_field_offset_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let member_name_ref = args[0];
    let offset = get_static_field_offset(member_name_ref)?;
    Ok(i64_to_vec(offset))
}

pub(crate) fn method_handle_natives_static_field_base_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let member_name_ref = args[0];
    let base_ref = static_field_base(member_name_ref)?;
    Ok(vec![base_ref])
}
fn static_field_base(member_name_ref: i32) -> Result<i32> {
    let member_name = MemberName::new(member_name_ref)?;
    let class_name = member_name.class_name();
    let class_ref = with_method_area(|area| area.load_reflection_class(class_name))?;
    Ok(class_ref)
}

// By not setting a value for `box` we break the check loop on the Java side so, just bypassing the check
pub(crate) fn method_handle_natives_get_named_con_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let _what = args[0];
    let _box_ref = args[1];
    Ok(vec![0])
}

pub(crate) fn method_handle_natives_get_member_vm_info_wrp(args: &[i32]) -> Result<Vec<i32>> {
    // returns {vmindex,vmtarget}
    let member_name_ref = args[0];
    let member_name = MemberName::new(member_name_ref)?;
    let array_ref = member_name.get_member_vm_info()?;
    Ok(vec![array_ref])
}

pub(crate) fn set_call_site_target_normal_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let call_site_ref = args[0];
    let method_handle_ref = args[1];

    set_call_site_target_normal(call_site_ref, method_handle_ref)?;

    Ok(vec![])
}
fn set_call_site_target_normal(call_site_ref: i32, method_handle_ref: i32) -> Result<()> {
    with_heap_write_lock(|heap| {
        let call_site_name = heap.get_instance_name(call_site_ref)?;
        heap.set_object_field_value(
            call_site_ref,
            &call_site_name,
            "target",
            vec![method_handle_ref],
        )
    })
}

pub(crate) fn native_accessor_invoke0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let method_ref = args[0];
    let obj_ref = args[1];
    let args_ref = args[2];
    let ret_obj_ref = native_accessor_invoke0(method_ref, obj_ref, args_ref)?;

    Ok(vec![ret_obj_ref])
}

pub(crate) fn native_accessor_newinstance0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let constructor_ref = args[0];
    let args_ref = args[1];
    let ret_obj_ref = native_accessor_newinstance0(constructor_ref, args_ref)?;

    Ok(vec![ret_obj_ref])
}

pub(crate) fn var_handle_set_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let handle_ref = args[0];
    let args_to_set = &args[1..];
    var_handle_set(handle_ref, args_to_set)?;
    Ok(vec![])
}

pub(crate) fn var_handle_get_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let handle_ref = args[0];
    let args_to_get = &args[1..];
    var_handle_get(handle_ref, args_to_get)
}

pub(crate) fn var_handle_compare_and_set_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let handle_ref = args[0];
    let args_to_process = &args[1..];

    var_handle_compare_and_set(handle_ref, args_to_process)
}
