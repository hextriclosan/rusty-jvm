use crate::vm::error::{Error, Result};
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
        // Register this segment on the thread's stack chain for the whole interpreter loop so
        // stack-walking natives can traverse it (and older segments) without a parameter. The
        // guard pops it on return/unwind, keeping the chain correct across native re-entries.
        let _stack_frames_guard = JavaThread::register_stack_frames(&mut stack_frames);
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

            match code {
                0u8..=20u8 => {
                    ops_constant_processor::process(code, &class, &mut stack_frames)?;
                }
                21u8..=53u8 => {
                    ops_load_processor::process(code, &mut stack_frames)?;
                }
                54u8..=86u8 => {
                    ops_store_processor::process(code, &mut stack_frames)?;
                }
                87u8..=95u8 => {
                    ops_stack_processor::process(code, &mut stack_frames)?;
                }
                96u8..=132u8 => {
                    ops_math_processor::process(code, &mut stack_frames)?;
                }
                133u8..=147u8 => {
                    ops_conversion_processor::process(code, &mut stack_frames)?;
                }
                148u8..=166u8 => {
                    ops_comparison_processor::process(code, &mut stack_frames)?;
                }
                167u8..=177u8 => {
                    last_value = ops_control_processor::process(code, &mut stack_frames)?;
                }
                178u8..=195u8 => {
                    ops_reference_processor::process(code, &class, &mut stack_frames)?;
                }
                196u8..=201u8 => {
                    ops_extended_processor::process(code, &class, &mut stack_frames)?;
                }
                _ => unreachable!("{}", format! {"xxx = {}", code}),
            }
        }

        Ok(last_value)
    }
}
