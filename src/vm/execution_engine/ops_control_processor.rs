use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::common::last_frame_mut;
use crate::vm::execution_engine::opcode::*;
use crate::vm::stack::stack_frame::StackFrames;
use crate::vm::stack::stack_value::StackValue;
use std::fmt::Display;
use tracing::trace;

pub(crate) fn process(code: u8, stack_frames: &mut StackFrames) -> Result<Vec<i32>> {
    let mut last_value = vec![];
    match code {
        GOTO => {
            let stack_frame = last_frame_mut(stack_frames)?;
            let offset = stack_frame.get_two_bytes_ahead();
            stack_frame.advance_pc(offset);
            trace!("GOTO -> offset={offset}");
        }
        LOOKUPSWITCH => {
            let stack_frame = last_frame_mut(stack_frames)?;
            let key: i32 = stack_frame.pop();
            let instruction_pc = stack_frame.pc() as i16;
            stack_frame.adjust_pc_to_4();

            let default_offset = stack_frame.extract_four_bytes() as i16;
            let npairs = stack_frame.extract_four_bytes();

            let mut matched_offset = None;
            for _ in 0..npairs {
                let case_key = stack_frame.extract_four_bytes();
                let offset = stack_frame.extract_four_bytes() as i16;

                if key == case_key {
                    //todo: add me to cache
                    matched_offset = Some(offset);
                    break;
                }
            }

            let offset = matched_offset.unwrap_or(default_offset);
            let current_pc = stack_frame.pc() as i16;
            stack_frame.advance_pc(offset + instruction_pc - current_pc);

            trace!("LOOKUPSWITCH -> offset={offset}, npairs={npairs}");
        }
        TABLESWITCH => {
            let stack_frame = last_frame_mut(stack_frames)?;
            let key_index: i32 = stack_frame.pop();
            let instruction_pc = stack_frame.pc() as i16;
            stack_frame.adjust_pc_to_4();

            let default_offset = stack_frame.extract_four_bytes();
            let from = stack_frame.extract_four_bytes();
            let to = stack_frame.extract_four_bytes();

            let offset_table = (from..=to)
                .map(|_| stack_frame.extract_four_bytes())
                .collect::<Vec<_>>(); //todo: add me to cache

            let offset = offset_table
                .get((key_index - from) as usize)
                .copied()
                .unwrap_or(default_offset);
            let current_pc = stack_frame.pc() as i16;
            stack_frame.advance_pc(offset as i16 + instruction_pc - current_pc);

            trace!("TABLESWITCH -> offset={offset}, from={from}, to={to}");
        }
        IRETURN => last_value = perform_return::<i32>(stack_frames, "IRETURN")?,
        LRETURN => last_value = perform_return::<i64>(stack_frames, "LRETURN")?,
        FRETURN => last_value = perform_return::<f32>(stack_frames, "FRETURN")?,
        DRETURN => last_value = perform_return::<f64>(stack_frames, "DRETURN")?,
        ARETURN => last_value = perform_return::<i32>(stack_frames, "ARETURN")?,
        RETURN => {
            last_value = vec![];
            stack_frames.exit_frame();
            trace!("RETURN");
        }
        _ => {
            return Err(Error::new_execution(&format!(
                "Unknown control opcode: {}",
                code
            )));
        }
    }

    Ok(last_value)
}

fn perform_return<T: StackValue + Copy + Display>(
    stack_frames: &mut StackFrames,
    name: &str,
) -> Result<Vec<i32>> {
    let stack_frame = last_frame_mut(stack_frames)?;
    let result: T = stack_frame.pop();

    stack_frames.exit_frame();
    // if we have next frame - populate return value to it
    if let Some(next_frame) = stack_frames.last_mut() {
        next_frame.push(result)?;
    }

    trace!("{name} -> {result}");
    Ok(result.to_vec())
}
