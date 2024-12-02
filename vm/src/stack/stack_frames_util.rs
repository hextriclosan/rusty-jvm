use crate::stack::stack_frame::StackFrames;

pub struct StackFramesUtil {}

impl StackFramesUtil {
    pub fn get_caller_class_name(stack_frames: &StackFrames) -> crate::error::Result<String> {
        let nearest_class_name = stack_frames
            .last()
            .ok_or_else(|| crate::error::Error::new_execution("stack frames is empty"))?
            .current_class_name();

        let class_name = stack_frames
            .iter()
            .rev()
            .skip(1)
            .map(|frame| frame.current_class_name())
            .find_map(|current_class_name| {
                if current_class_name != nearest_class_name {
                    Some(current_class_name)
                } else {
                    None // Continue searching
                }
            })
            .unwrap_or(nearest_class_name);

        Ok(class_name.to_string())
    }
}
