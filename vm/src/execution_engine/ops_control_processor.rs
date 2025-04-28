use crate::error::Error;
use crate::execution_engine::common::last_frame_mut;
use crate::execution_engine::opcode::*;
use crate::stack::stack_value::StackValue;
use crate::stack::stack_frame::StackFrames;
use std::fmt::Display;
use tracing::trace;

pub(crate) fn process(code: u8, stack_frames: &mut StackFrames) -> crate::error::Result<()> {
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
        IRETURN => perform_return::<i32>(stack_frames, "IRETURN")?,
        LRETURN => perform_return::<i64>(stack_frames, "LRETURN")?,
        FRETURN => perform_return::<f32>(stack_frames, "FRETURN")?,
        DRETURN => perform_return::<f64>(stack_frames, "DRETURN")?,
        ARETURN => perform_return::<i32>(stack_frames, "ARETURN")?,
        RETURN => {
            stack_frames.pop();
            trace!("RETURN");
        }
        _ => {
            return Err(Error::new_execution(&format!(
                "Unknown control opcode: {}",
                code
            )));
        }
    }

    Ok(())
}

fn perform_return<T: StackValue + Copy + Display>(
    stack_frames: &mut StackFrames,
    name: &str,
) -> crate::error::Result<()> {
    let stack_frame = last_frame_mut(stack_frames)?;
    let result: T = stack_frame.pop();

    stack_frames.pop();
    let next_frame = stack_frames
        .last_mut()
        .ok_or(Error::new_execution("Error getting stack last value"))?;
    next_frame.push(result)?;

    trace!("{name} -> {result}");
    Ok(())
}
