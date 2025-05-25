use crate::error::Error;
use crate::stack::stack_frame::{StackFrame, StackFrames};

/// Retrieves a mutable reference to the last `StackFrame` in the provided `StackFrames`.
///
/// # Arguments
/// * `stack_frames` - A mutable reference to the `StackFrames` collection.
///
/// # Returns
/// * `Ok(&mut StackFrame)` - A mutable reference to the last `StackFrame` if it exists.
/// * `Err(Error)` - An error if the `StackFrames` collection is empty.
///
/// # Errors
/// Returns an execution error with the message "No stack frame"
/// if the `StackFrames` collection is empty.
pub(crate) fn last_frame_mut(
    stack_frames: &mut StackFrames,
) -> crate::error::Result<&mut StackFrame> {
    stack_frames
        .last_mut()
        .ok_or_else(|| Error::new_execution("No stack frame"))
}

/// Performs storing current program counter (PC) to the exception program counter (ex_pc) in the last `StackFrame`.
///
/// # Arguments
/// * `stack_frames` - A mutable reference to the `StackFrames` collection.
///
/// # Returns
/// * `Ok(())` - If the operation is successful.
/// * `Err(Error)` - If there is no `StackFrame` in the `StackFrames` collection.
///
/// # Errors
/// Propagates the error from `last_frame_mut` if the `StackFrames` collection is empty.
pub(crate) fn store_ex_pc(stack_frames: &mut StackFrames) -> crate::error::Result<()> {
    let stack_frame = last_frame_mut(stack_frames)?;
    stack_frame.store_ex_pc();
    Ok(())
}
