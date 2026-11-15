use crate::vm::error::{Error, Result};
use crate::vm::exception::common::throw_exception_with_ref;
use crate::vm::execution_engine::{
    ops_comparison_processor, ops_constant_processor, ops_control_processor,
    ops_conversion_processor, ops_extended_processor, ops_load_processor, ops_math_processor,
    ops_reference_processor, ops_stack_processor, ops_store_processor,
};
use crate::vm::jni::java_thread::JavaThread;
use crate::vm::stack::stack_frame::{StackFrame, StackFrames};
use crate::vm::stack::stack_frames_util::StackFramesUtil;
use tracing::{span, trace, Level};

/// How often (in bytecodes) the interpreter polls its safepoint. A power of two so the modulo is a
/// cheap mask; small enough that a dump request is serviced within microseconds.
const SAFEPOINT_POLL_INTERVAL: u32 = 4096;

pub(crate) struct Engine {}

impl Engine {
    pub(crate) fn execute(initial_stack_frame: StackFrame, reason: &str) -> Result<Vec<i32>> {
        trace!("@@@ Entering execute: {reason}");

        let mut stack_frames = StackFrames::new(vec![initial_stack_frame]);
        JavaThread::frame_pushed(); // the initial frame, pushed by no `new_frame`
                                    // Register this segment on the thread's stack chain for the whole interpreter loop so
                                    // stack-walking natives can traverse it (and older segments) without a parameter. The
                                    // guard pops it on return/unwind, keeping the chain correct across native re-entries.
        let _stack_frames_guard = JavaThread::register_stack_frames(&mut stack_frames);

        let result = Self::interpret(&mut stack_frames);
        if result.is_err() {
            // An error discards this segment, frames and all, instead of unwinding it: an uncaught
            // exception coming out of a nested segment (a failing class initializer, say) or an
            // internal error. Nothing pops those frames, so the monitors their `synchronized`
            // methods hold would stay owned by a thread that may be about to die, with every other
            // thread that needs them blocked for good.
            stack_frames.release_held_monitors();
        }
        result
    }

    /// The interpreter loop over `stack_frames`, until its last frame returns or an error, a
    /// Java exception no frame of the segment handles among them, ends it.
    fn interpret(stack_frames: &mut StackFrames) -> Result<Vec<i32>> {
        // Grabbed once per invocation, not per instruction. The safepoint is polled only every
        // `SAFEPOINT_POLL_INTERVAL` bytecodes (a cheap local-counter test each iteration); that
        // bounds time-to-safepoint to a few microseconds while keeping the hot loop's overhead to a
        // register increment, not an atomic load, on every instruction.
        let safepoint = JavaThread::safepoint();
        let mut poll_counter: u32 = 0;
        let mut last_value = vec![];
        while !stack_frames.is_empty() {
            poll_counter = poll_counter.wrapping_add(1);
            if poll_counter % SAFEPOINT_POLL_INTERVAL == 0 {
                safepoint.poll();
            }
            // Stamp the instruction's start pc so that, if this opcode raises an implicit
            // NullPointerException, the throwable's backtrace records the faulting bytecode index
            // (JEP 358) even though the handler advances `pc` while reading operands.
            //
            // And forget the call site the frame may have saved: a frame that dispatches an
            // instruction has no call of its own in flight, whatever ended the last one (a return,
            // a handler entered here, a native that came back, an `invokedynamic` resolved on the
            // side). Left behind, that site would locate an exception this instruction raises
            // itself at the earlier call, and so at the wrong handler and line.
            {
                let frame = stack_frames
                    .last_mut()
                    .ok_or(Error::new_execution("Error getting stack frame"))?;
                let pc = frame.pc();
                frame.set_bci(pc);
                frame.reset_ex_pc();
            }
            let (class, code, pc, line_numbers) = {
                let frame = stack_frames
                    .last()
                    .ok_or(Error::new_execution("Error getting stack frame"))?;
                (
                    frame.current_class_name().to_string(),
                    frame.get_bytecode_byte(),
                    frame.pc(),
                    frame.line_numbers(),
                )
            };
            let instruction_line_num = StackFramesUtil::extract_line_number(line_numbers, pc);
            let span = span!(Level::TRACE, "", "{class}:{instruction_line_num}");
            let _entered = span.enter();

            let outcome = match code {
                0u8..=20u8 => ops_constant_processor::process(code, &class, stack_frames),
                21u8..=53u8 => ops_load_processor::process(code, stack_frames),
                54u8..=86u8 => ops_store_processor::process(code, stack_frames),
                87u8..=95u8 => ops_stack_processor::process(code, stack_frames),
                96u8..=132u8 => ops_math_processor::process(code, stack_frames),
                133u8..=147u8 => ops_conversion_processor::process(code, stack_frames),
                148u8..=166u8 => ops_comparison_processor::process(code, stack_frames),
                167u8..=177u8 => ops_control_processor::process(code, stack_frames)
                    .map(|value| last_value = value),
                178u8..=195u8 => ops_reference_processor::process(code, &class, stack_frames),
                196u8..=201u8 => ops_extended_processor::process(code, &class, stack_frames),
                _ => unreachable!("{}", format! {"xxx = {}", code}),
            };
            if let Err(error) = outcome {
                // A Java exception that no frame of a nested segment handled comes back as an
                // error: out of the class initialization an instruction triggered, out of the Java
                // code a native or an `invokedynamic` bootstrap called, and so on. It is thrown
                // here, at the instruction that caused it, so that this segment's frames get their
                // turn at handling it; past the last of them, it goes on to the segment below the
                // same way. An empty stack means this segment has just unwound for it itself.
                // Anything but a Java exception is the VM's own failure and ends the run.
                let Some(throwable_ref) = error.throwable_ref() else {
                    return Err(error);
                };
                if stack_frames.is_empty() {
                    return Err(error);
                }
                throw_exception_with_ref(throwable_ref, stack_frames)?;
            }
        }

        Ok(last_value)
    }
}
