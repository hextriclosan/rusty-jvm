use crate::error::Error;
use crate::stack::stack_frame::{StackFrame, StackFrames};

pub(crate) fn last_frame_mut(
    stack_frames: &mut StackFrames,
) -> crate::error::Result<&mut StackFrame> {
    stack_frames
        .last_mut()
        .ok_or_else(|| Error::new_execution("No stack frame to process comparison opcode"))
}
