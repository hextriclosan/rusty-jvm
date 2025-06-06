use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::common::last_frame_mut;
use crate::vm::execution_engine::opcode::*;
use crate::vm::stack::stack_frame::StackFrames;
use tracing::trace;

pub(crate) fn process(code: u8, stack_frames: &mut StackFrames) -> Result<()> {
    let stack_frame = last_frame_mut(stack_frames)?;
    match code {
        POP => {
            let _value: i32 = stack_frame.pop();

            stack_frame.incr_pc();
            trace!("POP");
        }
        POP2 => {
            let _value: i32 = stack_frame.pop();
            let _value: i32 = stack_frame.pop();

            stack_frame.incr_pc();
            trace!("POP2");
        }
        DUP => {
            let value: i32 = stack_frame.pop();
            stack_frame.push(value)?;
            stack_frame.push(value)?;

            stack_frame.incr_pc();
            trace!("DUP -> value={value}");
        }
        DUP_X1 => {
            let value1: i32 = stack_frame.pop();
            let value2: i32 = stack_frame.pop();
            stack_frame.push(value1)?;
            stack_frame.push(value2)?;
            stack_frame.push(value1)?;

            stack_frame.incr_pc();
            trace!("DUP_X1 -> value1={value1}, value2={value2}, value1={value1}");
        }
        DUP_X2 => {
            let value1: i32 = stack_frame.pop();
            let value2: i32 = stack_frame.pop();
            let value3: i32 = stack_frame.pop();
            stack_frame.push(value1)?;
            stack_frame.push(value3)?;
            stack_frame.push(value2)?;
            stack_frame.push(value1)?;

            stack_frame.incr_pc();
            trace!("DUP_X2 -> value1={value1}, value2={value2}, value3={value3}, value1={value1}");
        }
        DUP2 => {
            let value1: i32 = stack_frame.pop();
            let value2: i32 = stack_frame.pop();
            stack_frame.push(value2)?;
            stack_frame.push(value1)?;
            stack_frame.push(value2)?;
            stack_frame.push(value1)?;

            stack_frame.incr_pc();
            trace!("DUP2 -> value1={value1}, value2={value2}");
        }
        DUP2_X1 => {
            let value1: i32 = stack_frame.pop();
            let value2: i32 = stack_frame.pop();
            let value3: i32 = stack_frame.pop();
            stack_frame.push(value2)?;
            stack_frame.push(value1)?;
            stack_frame.push(value3)?;
            stack_frame.push(value2)?;
            stack_frame.push(value1)?;

            stack_frame.incr_pc();
            trace!("DUP2_X1 -> value1={value1}, value2={value2}, value3={value3}");
        }
        DUP2_X2 => {
            let value1: i32 = stack_frame.pop();
            let value2: i32 = stack_frame.pop();
            let value3: i32 = stack_frame.pop();
            let value4: i32 = stack_frame.pop();
            stack_frame.push(value2)?;
            stack_frame.push(value1)?;
            stack_frame.push(value4)?;
            stack_frame.push(value3)?;
            stack_frame.push(value2)?;
            stack_frame.push(value1)?;

            stack_frame.incr_pc();
            trace!(
                "DUP2_X2 -> value1={value1}, value2={value2}, value3={value3}, value4={value4}"
            );
        }
        SWAP => {
            let value1: i32 = stack_frame.pop();
            let value2: i32 = stack_frame.pop();
            stack_frame.push(value1)?;
            stack_frame.push(value2)?;

            stack_frame.incr_pc();
            trace!("SWAP -> value1={value1}, value2={value2}");
        }
        _ => {
            return Err(Error::new_execution(&format!(
                "Unknown stack opcode: {}",
                code
            )));
        }
    }

    Ok(())
}
