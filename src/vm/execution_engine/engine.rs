//! Purpose: Core bytecode execution engine for the JVM.
//!
//! Implementation Details:
//! This module implements the main interpreter loop. It continually pops the next
//! opcode from the current stack frame and dispatches it to the appropriate processor.
//! As part of the transition to Tokio, this engine operates asynchronously, allowing
//! instructions (like method invocations that map to blocking native calls) to yield
//! execution to the Tokio scheduler without blocking the underlying OS thread.

use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::{
    ops_comparison_processor, ops_constant_processor, ops_control_processor,
    ops_conversion_processor, ops_extended_processor, ops_load_processor, ops_math_processor,
    ops_reference_processor, ops_stack_processor, ops_store_processor,
};
use crate::vm::stack::stack_frame::{StackFrame, StackFrames};
use crate::vm::stack::stack_frames_util::StackFramesUtil;
use tracing::{span, trace, Level};

pub(crate) struct Engine {}

impl Engine {
    /// Starts the asynchronous execution of a JVM stack frame.
    ///
    /// # Arguments
    /// * `initial_stack_frame` - The root frame of the call stack.
    /// * `reason` - A debug string describing why this execution loop was started.
    ///
    /// # Returns
    /// A `Result` containing the vector of return values (empty for `void`).
    pub(crate) async fn execute(initial_stack_frame: StackFrame, reason: &str) -> Result<Vec<i32>> {
        trace!("@@@ Entering execute: {reason}");

        let mut stack_frames = StackFrames::new(vec![initial_stack_frame]);
        let mut last_value = vec![];
        
        while !stack_frames.is_empty() {
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

            // Dispatch the opcode to the appropriate processor based on its byte value.
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
                    // The reference processor handles method invocations (INVOKEVIRTUAL, etc.).
                    // Because these invocations may ultimately call asynchronous native methods 
                    // (like Thread.sleep), this processor must be awaited.
                    ops_reference_processor::process(code, &class, &mut stack_frames).await?;
                }
                196u8..=201u8 => {
                    ops_extended_processor::process(code, &class, &mut stack_frames)?;
                }
                _ => unreachable!("{}", format!("xxx = {}", code)),
            }
        }

        Ok(last_value)
    }
}