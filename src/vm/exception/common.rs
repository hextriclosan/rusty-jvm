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
        throw_exception_with_ref(exception_instance_ref[0], stack_frames)?;

    trace!("<THROWING> -> exception_name={exception_name}, found_exception_handler={found_exception_handler}");
    Ok(())
}

pub fn throw_exception_with_ref(
    throwable: Slot,
    stack_frames: &mut StackFrames,
) -> Result<(String, i16)> {
    if !throwable.is_ref() {
        return Err(Error::new_execution(
            "throwable must retain its producer reference tag",
        ));
    }
    // This owner exists before handler lookup can re-enter Java or any frame is popped.
    let ownership = Error::uncaught_exception(throwable);
    let throwable_ref = throwable.value();
    let exception_name = HEAP.get_instance_name(throwable_ref)?;
    trace!("<THROWING> -> about to throw: throwable_ref={throwable_ref}, exception_name={exception_name}");
    let found_exception_handler = unwind_stack(throwable_ref, stack_frames, ownership)?;

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

fn unwind_stack(
    throwable_ref: i32,
    stack_frames: &mut StackFrames,
    ownership: Error,
) -> Result<i16> {
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
                enter_handler(stack_frame, exception_handler as i16, throwable_ref)?;
                return Ok(exception_handler as i16);
            }
            None => {
                stack_frames.propagate_exception();
            }
        }
    }

    Err(ownership)
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

    #[test]
    fn ref_transfer_uncaught_exception_owns_root_after_all_frames_pop() {
        use crate::vm::jni::java_thread::JavaThread;
        use crate::vm::stack::stack_frame::observe_frame_pops;
        use std::cell::Cell;
        use std::rc::Rc;

        // Empty handler tables only need a heap identity; no class lookup or type matching.
        // The Java integration fixture covers real Throwable construction and handlers.
        let throwable = HEAP.create_array("[Ljava/lang/Object;", 0);
        let mut source = StackFrame::for_test(0, 1);
        source.push(Slot::Ref(throwable)).unwrap();
        let mut frames = StackFrames::new(vec![source]);
        let observed = Rc::new(Cell::new(false));
        let seen = Rc::clone(&observed);

        let error = observe_frame_pops(
            move |remaining| {
                assert!(remaining.is_empty());
                assert!(JavaThread::temp_root_refs().contains(&throwable));
                seen.set(true);
            },
            || throw_exception_with_ref(Slot::Ref(throwable), &mut frames).unwrap_err(),
        );

        assert!(observed.get());
        assert_eq!(error.throwable_ref(), Some(throwable));
        assert!(JavaThread::temp_root_refs().contains(&throwable));
        JavaThread::try_set_pending_exception(error.throwable_ref().unwrap()).unwrap();
        drop(error);
        assert_eq!(JavaThread::take_pending_exception(), Some(throwable));
        assert!(JavaThread::temp_root_refs().is_empty());
    }
}
