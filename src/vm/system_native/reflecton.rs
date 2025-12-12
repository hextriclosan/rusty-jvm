use crate::vm::error::Result;
use crate::vm::helper::{clazz_ref, klass, strip_nest_host};
use crate::vm::stack::stack_frame::StackFrames;
use crate::vm::stack::stack_frames_util::StackFramesUtil;

pub(crate) fn reflection_get_caller_class_wrp(
    _args: &[i32],
    stack_frames: &StackFrames,
) -> Result<Vec<i32>> {
    let class_ref = get_caller_class(stack_frames)?;
    Ok(vec![class_ref])
}

fn get_caller_class(stack_frames: &StackFrames) -> Result<i32> {
    let caller_name = StackFramesUtil::get_caller_class_name(stack_frames)?;
    clazz_ref(&caller_name)
}

pub(crate) fn reflection_get_class_access_flags_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let class_ref = args[0];
    let flags = get_class_access_flags(class_ref)?;
    Ok(vec![flags])
}
fn get_class_access_flags(class_ref: i32) -> Result<i32> {
    let klass = klass(class_ref)?;
    let flags = klass.class_modifiers().bits() as i32;
    Ok(flags)
}

pub(crate) fn reflection_are_nest_mates_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let current_class_ref = args[0];
    let member_class_ref = args[1];

    let mates = are_nest_mates(current_class_ref, member_class_ref)?;
    Ok(vec![if mates { 1 } else { 0 }])
}
fn are_nest_mates(current_class_ref: i32, member_class_ref: i32) -> Result<bool> {
    let current_klass = klass(current_class_ref)?;
    let member_klass = klass(member_class_ref)?;
    let current_class_name = current_klass.this_class_name();
    let member_class_name = member_klass.this_class_name();

    let current_class_nest_host =
        strip_nest_host(&current_class_name).unwrap_or(current_class_name.as_str());

    let member_class_nest_host =
        strip_nest_host(&member_class_name).unwrap_or(member_class_name.as_str());

    Ok(current_class_nest_host == member_class_nest_host)
}
