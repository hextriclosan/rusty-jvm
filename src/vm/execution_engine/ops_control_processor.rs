use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::common::{last_frame_mut, push_slots};
use crate::vm::execution_engine::opcode::*;
use crate::vm::stack::rooted_slots::RootedSlots;
use crate::vm::stack::stack_frame::StackFrames;
use tracing::trace;

pub(crate) fn process(
    code: u8,
    stack_frames: &mut StackFrames,
    destination: &mut RootedSlots,
) -> Result<()> {
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
        IRETURN | FRETURN | ARETURN => perform_return(stack_frames, destination, 1)?,
        LRETURN | DRETURN => perform_return(stack_frames, destination, 2)?,
        RETURN => {
            destination.replace(vec![]);
            let (_, monitor_owned) = stack_frames.exit_frame()?;
            if !monitor_owned {
                return crate::vm::exception::common::construct_exception_and_throw(
                    "java/lang/IllegalMonitorStateException",
                    "<init>:()V",
                    &[],
                    stack_frames,
                );
            }
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

fn perform_return(
    stack_frames: &mut StackFrames,
    destination: &mut RootedSlots,
    width: usize,
) -> Result<()> {
    let stack_frame = last_frame_mut(stack_frames)?;
    let slots = (0..width).map(|_| stack_frame.pop_slot()).collect();
    destination.replace(slots);

    let (_, monitor_owned) = stack_frames.exit_frame()?;
    if !monitor_owned {
        destination.replace(vec![]);
        return crate::vm::exception::common::construct_exception_and_throw(
            "java/lang/IllegalMonitorStateException",
            "<init>:()V",
            &[],
            stack_frames,
        );
    }
    // Publish into the caller before releasing the transfer destination.
    if let Some(next_frame) = stack_frames.last_mut() {
        push_slots(next_frame, destination)?;
        destination.replace(vec![]);
    }

    Ok(())
}
