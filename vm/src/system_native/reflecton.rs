use crate::helper::strip_nest_host;
use crate::method_area::method_area::with_method_area;
use crate::stack::stack_frame::StackFrames;
use crate::stack::stack_frames_util::StackFramesUtil;

pub(crate) fn reflection_get_caller_class_wrp(
    _args: &[i32],
    stack_frames: &StackFrames,
) -> crate::error::Result<Vec<i32>> {
    let class_ref = get_caller_class(stack_frames)?;
    Ok(vec![class_ref])
}

fn get_caller_class(stack_frames: &StackFrames) -> crate::error::Result<i32> {
    let caller_name = StackFramesUtil::get_caller_class_name(stack_frames)?;
    let reflection_ref =
        with_method_area(|method_area| method_area.load_reflection_class(&caller_name))?;

    Ok(reflection_ref)
}

pub(crate) fn reflection_get_class_access_flags_wrp(
    args: &[i32],
) -> crate::error::Result<Vec<i32>> {
    let class_ref = args[0];
    let flags = get_class_access_flags(class_ref)?;
    Ok(vec![flags])
}
fn get_class_access_flags(class_ref: i32) -> crate::error::Result<i32> {
    let class_name =
        with_method_area(|method_area| method_area.get_from_reflection_table(class_ref))?;

    let flags =
        with_method_area(|method_area| Ok(method_area.get(&class_name)?.access_flags() as i32));
    flags
}

pub(crate) fn reflection_are_nest_mates_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let current_class_ref = args[0];
    let member_class_ref = args[1];

    let mates = are_nest_mates(current_class_ref, member_class_ref)?;
    Ok(vec![if mates { 1 } else { 0 }])
}
fn are_nest_mates(current_class_ref: i32, member_class_ref: i32) -> crate::error::Result<bool> {
    let current_class_name =
        with_method_area(|area| area.get_from_reflection_table(current_class_ref))?;
    let member_class_name =
        with_method_area(|area| area.get_from_reflection_table(member_class_ref))?;

    let current_class_nest_host =
        strip_nest_host(&current_class_name).unwrap_or(current_class_name.as_str());

    let member_class_nest_host =
        strip_nest_host(&member_class_name).unwrap_or(member_class_name.as_str());

    Ok(current_class_nest_host == member_class_nest_host)
}
