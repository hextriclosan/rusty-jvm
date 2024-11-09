use crate::error::Error;
use crate::execution_engine::opcode::*;
use crate::stack::stack_frame::StackFrame;
use tracing::trace;

pub(crate) fn process(
    code: u8,
    stack_frames: &mut Vec<StackFrame>,
) -> crate::error::Result<Option<Vec<i32>>> {
    let stack_frame = stack_frames.last_mut().unwrap();
    match code {
        GOTO => {
            let offset = stack_frame.get_two_bytes_ahead();
            stack_frame.advance_pc(offset);
            trace!("GOTO -> offset={offset}");
        }
        LOOKUPSWITCH => {
            let key: i32 = stack_frame.pop();
            let instruction_pc = stack_frame.pc() as i16;
            stack_frame.adjust_pc_to_4();

            let default_offset = stack_frame.extract_four_bytes() as i16;
            let npairs = stack_frame.extract_four_bytes();

            let mut match_found = false;
            for _ in 0..npairs {
                let case_key = stack_frame.extract_four_bytes();
                let offset = stack_frame.extract_four_bytes() as i16;

                if key == case_key {
                    let current_pc = stack_frame.pc() as i16;
                    stack_frame.advance_pc(offset + instruction_pc - current_pc);
                    match_found = true;
                    break;
                }
            }

            if !match_found {
                let current_pc = stack_frame.pc() as i16;
                stack_frame.advance_pc(default_offset + instruction_pc - current_pc);
            }

            trace!("LOOKUPSWITCH -> default_offset={default_offset}, npairs={npairs}");
        }
        TABLESWITCH => {
            let index: i32 = stack_frame.pop();
            let instruction_pc = stack_frame.pc() as i16;
            stack_frame.adjust_pc_to_4();

            let default_offset = stack_frame.extract_four_bytes() as i16;
            let low = stack_frame.extract_four_bytes();
            let high = stack_frame.extract_four_bytes();

            let offset_table = (low..=high)
                .map(|_| stack_frame.extract_four_bytes())
                .collect::<Vec<_>>();

            let offset = offset_table
                .get((index - low) as usize)
                .map_or(default_offset, |offset| *offset as i16);
            let current_pc = stack_frame.pc() as i16;
            stack_frame.advance_pc(offset + instruction_pc - current_pc);

            trace!("TABLESWITCH -> default_offset={default_offset}, low={low}, high={high}");
        }
        IRETURN => {
            let ret: i32 = stack_frame.pop();
            stack_frames.pop();
            let _frame = stack_frames
                .last_mut()
                .ok_or(Error::new_execution("Error getting stack last value"))?
                .push(ret);
            trace!("IRETURN -> ret={ret}");
        }
        LRETURN => {
            let ret: i64 = stack_frame.pop();

            stack_frames.pop();
            let frame = stack_frames
                .last_mut()
                .ok_or(Error::new_execution("Error getting stack last value"))?;

            frame.push(ret);
            trace!("LRETURN -> ret={ret}");
        }
        FRETURN => {
            let ret: f32 = stack_frame.pop();
            stack_frames.pop();
            stack_frames
                .last_mut()
                .ok_or(Error::new_execution("Error getting stack last value"))?
                .push(ret);
            trace!("FRETURN -> ret={ret}");
        }
        DRETURN => {
            let ret: f64 = stack_frame.pop();

            stack_frames.pop();
            let frame = stack_frames
                .last_mut()
                .ok_or(Error::new_execution("Error getting stack last value"))?;

            frame.push(ret);

            trace!("DRETURN -> ret={ret}");
        }
        ARETURN => {
            let objref: i32 = stack_frame.pop();

            stack_frames.pop();
            stack_frames
                .last_mut()
                .ok_or(Error::new_execution("Error getting stack last value"))?
                .push(objref);
            trace!("ARETURN -> objref={objref}");
        }
        RETURN => {
            trace!("RETURN -> stack_frame.locals={:?}", stack_frame.locals());
            let last_value = Some(
                stack_frames
                    .last()
                    .ok_or(Error::new_execution("Error getting stack last value"))?
                    .locals()
                    .to_vec(),
            );

            stack_frames.pop();

            return Ok(last_value);
        }
        _ => {
            return Err(Error::new_execution(&format!(
                "Unknown control opcode: {}",
                code
            )));
        }
    }

    Ok(None)
}
