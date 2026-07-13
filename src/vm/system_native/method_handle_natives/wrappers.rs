use crate::vm::error::Result;
use crate::vm::stack::stack_frame::StackFrames;
use crate::vm::system_native::method_handle_natives::invocation::invoke_exact;
use crate::vm::system_native::method_handle_natives::native_accessor::{
    native_accessor_invoke0, native_accessor_newinstance0,
};
use crate::vm::system_native::method_handle_natives::var_handle::{
    var_handle_compare_and_set, var_handle_get, var_handle_set,
};

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

pub(crate) fn method_handle_invoke_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    // The invoke method should handle type conversions and argument adaptations that invokeExact does not perform,
    // making this a temporary placeholder that could cause incorrect behavior.
    // TODO: implement real invoke
    method_handle_invoke_exact_wrp(args, stack_frames)
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
