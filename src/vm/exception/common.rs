use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::common::last_frame_mut;
use crate::vm::execution_engine::executor::Executor;
use crate::vm::heap::heap::HEAP;
use crate::vm::stack::slot::Slot;
use crate::vm::stack::stack_frame::{StackFrame, StackFrames};
use crate::vm::stack::stack_value::StackValueKind;
use tracing::trace;

pub fn construct_exception_and_throw(
    exception_class_name: &str,
    constructor_signature: &str,
    args: &[StackValueKind],
    stack_frames: &mut StackFrames,
) -> Result<()> {
    let exception_instance_ref = Executor::invoke_args_constructor(
        exception_class_name,
        constructor_signature,
        args,
        Some(&format!(
            "construction of {exception_class_name}:{constructor_signature}({args:?}) instance"
        )),
    )?;

    let (exception_name, found_exception_handler) =
        throw_exception_with_ref(exception_instance_ref, stack_frames)?;

    trace!("<THROWING> -> exception_name={exception_name}, found_exception_handler={found_exception_handler}");
    Ok(())
}

pub fn throw_exception_with_ref(
    throwable_ref: i32,
    stack_frames: &mut StackFrames,
) -> Result<(String, i16)> {
    let exception_name = HEAP.get_instance_name(throwable_ref)?;
    trace!("<THROWING> -> about to throw: throwable_ref={throwable_ref}, exception_name={exception_name}");
    let found_exception_handler = unwind_stack(throwable_ref, stack_frames)?;

    enter_handler(
        last_frame_mut(stack_frames)?,
        found_exception_handler,
        throwable_ref,
    )?;

    Ok((exception_name, found_exception_handler))
}

/// Positions a frame at its exception handler: pc at the handler, operand stack discarded and
/// replaced by the throwable alone, per JVMS §2.10.
///
/// The throwable is pushed as a reference. It has to be: `clear_stack` has just discarded
/// everything else, so for as long as the handler runs this slot is the frame's only operand-stack
/// root, and an untagged one would hide the in-flight exception — and everything it references,
/// including its backtrace — from a root scan.
fn enter_handler(stack_frame: &mut StackFrame, handler_pc: i16, throwable_ref: i32) -> Result<()> {
    stack_frame.set_pc(handler_pc);
    stack_frame.clear_stack(); // according to JVM spec
    stack_frame.push(Slot::Ref(throwable_ref))
}

fn unwind_stack(throwable_ref: i32, stack_frames: &mut StackFrames) -> Result<i16> {
    let exception_name = HEAP.get_instance_name(throwable_ref)?;
    while !stack_frames.is_empty() {
        let stack_frame = last_frame_mut(stack_frames)?;
        let exception_table = stack_frame.exception_table();
        let pc = stack_frame.ex_pc() as u16;
        match exception_table.find_exception_handler(
            &exception_name,
            pc,
            stack_frame.method_name(),
            stack_frame.current_class_name(),
        )? {
            Some(exception_handler) => {
                return Ok(exception_handler as i16);
            }
            None => {
                stack_frames.propagate_exception();
            }
        }
    }

    Err(Error::uncaught_exception(throwable_ref))
}

#[cfg(test)]
mod tests {
    use super::*;

    const HANDLER_PC: i16 = 7;
    const THROWABLE: i32 = 42;

    fn frame_at_handler() -> StackFrame {
        let mut stack_frame = StackFrame::for_test(0, 4);
        // Operands the throwing instruction left behind, one of them a reference.
        stack_frame.push(Slot::Ref(11)).unwrap();
        stack_frame.push(1i32).unwrap();
        enter_handler(&mut stack_frame, HANDLER_PC, THROWABLE).unwrap();
        stack_frame
    }

    /// A catch handler begins with the throwable as its only operand, and it must be visible as a
    /// root for as long as the handler runs.
    #[test]
    fn should_leave_the_throwable_as_the_only_root() {
        assert_eq!(
            frame_at_handler().ref_slots().collect::<Vec<_>>(),
            vec![THROWABLE]
        );
    }

    #[test]
    fn should_hand_the_throwable_to_the_handler_as_a_reference() {
        assert_eq!(frame_at_handler().pop::<Slot>(), Slot::Ref(THROWABLE));
    }

    #[test]
    fn should_position_the_frame_at_the_handler() {
        assert_eq!(frame_at_handler().pc(), HANDLER_PC as usize);
    }
}
