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
