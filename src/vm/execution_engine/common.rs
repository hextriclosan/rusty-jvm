use crate::vm::error::{Error, Result};
use crate::vm::helper::{get_length, is_reference};
use crate::vm::stack::slot::Slot;
use crate::vm::stack::stack_frame::{StackFrame, StackFrames};
use jdescriptor::TypeDescriptor;

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
pub(crate) fn last_frame_mut(stack_frames: &mut StackFrames) -> Result<&mut StackFrame> {
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
pub(crate) fn store_ex_pc(stack_frames: &mut StackFrames) -> Result<()> {
    let stack_frame = last_frame_mut(stack_frames)?;
    stack_frame.store_ex_pc();
    Ok(())
}

/// Pushes a value held as raw chunks — a field read, a native method's result — onto the operand
/// stack, tagging it as a reference when `type_descriptor` says it is one.
///
/// Raw `Vec<i32>`s carry no tag of their own, so the declared type is the only thing that can supply
/// it. Chunks go on in reverse so that a `long`'s or `double`'s halves land in the order the
/// interpreter's wide reads expect; a reference is a single chunk and so is unaffected by the order.
pub(crate) fn push_typed(
    stack_frame: &mut StackFrame,
    type_descriptor: &TypeDescriptor,
    raw: &[i32],
) -> Result<()> {
    push_slots(stack_frame, &typed_slots(type_descriptor, raw)?)
}

/// Tags raw storage at a producer whose declared type is exact, never at a transfer.
pub(crate) fn typed_slots(type_descriptor: &TypeDescriptor, raw: &[i32]) -> Result<Vec<Slot>> {
    let width = if matches!(type_descriptor, TypeDescriptor::Void) {
        0
    } else {
        get_length(type_descriptor)? as usize
    };
    if raw.len() != width {
        return Err(Error::new_execution(&format!(
            "result width {} disagrees with {type_descriptor}",
            raw.len()
        )));
    }
    Ok(raw
        .iter()
        .map(|&value| {
            if is_reference(type_descriptor) {
                Slot::Ref(value)
            } else {
                Slot::Value(value)
            }
        })
        .collect())
}

/// Result chunks are in pop order (high, low for category 2).
pub(crate) fn push_slots(stack_frame: &mut StackFrame, slots: &[Slot]) -> Result<()> {
    slots
        .iter()
        .rev()
        .try_for_each(|slot| stack_frame.push(*slot))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::helper::i64_to_vec;

    #[test]
    fn should_tag_an_object_valued_push() {
        let mut frame = StackFrame::for_test(0, 1);
        let descriptor = TypeDescriptor::Object("java/lang/String".to_string());

        push_typed(&mut frame, &descriptor, &[7]).unwrap();

        assert_eq!(frame.ref_slots().collect::<Vec<_>>(), vec![7]);
    }

    #[test]
    fn should_tag_an_array_valued_push() {
        let mut frame = StackFrame::for_test(0, 1);
        let descriptor = TypeDescriptor::Array(Box::new(TypeDescriptor::Integer), 1);

        push_typed(&mut frame, &descriptor, &[7]).unwrap();

        assert_eq!(frame.ref_slots().collect::<Vec<_>>(), vec![7]);
    }

    #[test]
    fn should_leave_a_primitive_push_untagged() {
        let mut frame = StackFrame::for_test(0, 1);

        push_typed(&mut frame, &TypeDescriptor::Integer, &[7]).unwrap();

        assert_eq!(frame.ref_slots().count(), 0);
        assert_eq!(frame.pop::<i32>(), 7);
    }

    /// Wide values occupy two slots, and the halves must land in the order the interpreter's `long`
    /// reads expect. The round trip is what pins the chunk ordering.
    #[test]
    fn should_push_both_halves_of_a_wide_value_untagged() {
        let mut frame = StackFrame::for_test(0, 2);
        let value = i64::MIN + 12345;

        push_typed(&mut frame, &TypeDescriptor::Long, &i64_to_vec(value)).unwrap();

        assert_eq!(frame.ref_slots().count(), 0);
        assert_eq!(frame.pop::<i64>(), value);
    }

    #[test]
    fn should_push_nothing_for_void() {
        let mut frame = StackFrame::for_test(0, 1);

        push_typed(&mut frame, &TypeDescriptor::Void, &[]).unwrap();

        assert_eq!(frame.ref_slots().count(), 0);
    }

    /// A reference with no value behind it is a bug in the caller, not something to paper over by
    /// pushing whatever happens to be there.
    #[test]
    fn should_reject_a_reference_with_no_value() {
        let mut frame = StackFrame::for_test(0, 1);
        let descriptor = TypeDescriptor::Object("java/lang/String".to_string());

        assert!(push_typed(&mut frame, &descriptor, &[]).is_err());
    }
}
