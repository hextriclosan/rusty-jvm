use crate::error::Error;
use crate::execution_engine::{
    ops_comparison_processor, ops_control_processor, ops_conversion_processor,
    ops_extended_processor, ops_reference_processor,
};
use crate::execution_engine::{
    ops_constant_processor, ops_load_processor, ops_math_processor, ops_stack_processor,
    ops_store_processor,
};
use crate::stack::stack_frame::StackFrame;
use tracing::{span, trace, Level};

pub(crate) struct Engine {}

impl Engine {
    pub(crate) fn execute(
        initial_stack_frame: StackFrame,
        reason: &str,
    ) -> crate::error::Result<()> {
        trace!("@@@ Entering execute: {reason}");

        let mut stack_frames = vec![initial_stack_frame];
        while !stack_frames.is_empty() {
            let (class, code) = {
                let frame = stack_frames
                    .last()
                    .ok_or(Error::new_execution("Error getting stack frame"))?;
                (
                    frame.current_class_name().to_string(),
                    frame.get_bytecode_byte(),
                )
            };
            let span = span!(Level::TRACE, "", class);
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
                    ops_control_processor::process(code, &mut stack_frames)?;
                }
                178u8..=195u8 => {
                    ops_reference_processor::process(code, &class, &mut stack_frames)?;
                }
                196u8..=201u8 => {
                    ops_extended_processor::process(code, &mut stack_frames)?;
                }
                _ => unreachable!("{}", format! {"xxx = {}", code}),
            }
        }

        Ok(())
    }
}
