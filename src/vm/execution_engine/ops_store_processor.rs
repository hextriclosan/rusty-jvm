use crate::vm::error::{Error, Result};
use crate::vm::exception::helpers::check_array_access;
use crate::vm::execution_engine::common::last_frame_mut;
use crate::vm::execution_engine::opcode::*;
use crate::vm::heap::heap::HEAP;
use crate::vm::stack::slot::Slot;
use crate::vm::stack::stack_frame::{StackFrame, StackFrames};
use crate::vm::stack::stack_value::StackValue;
use std::fmt::Display;
use tracing::trace;

pub(crate) fn process(code: u8, stack_frames: &mut StackFrames) -> Result<()> {
    match code {
        ISTORE => handle_pos_and_store::<i32>(last_frame_mut(stack_frames)?, "ISTORE "),
        LSTORE => handle_pos_and_store::<i64>(last_frame_mut(stack_frames)?, "LSTORE "),
        FSTORE => handle_pos_and_store::<f32>(last_frame_mut(stack_frames)?, "FSTORE "),
        DSTORE => handle_pos_and_store::<f64>(last_frame_mut(stack_frames)?, "DSTORE "),
        ASTORE => handle_pos_and_store::<Slot>(last_frame_mut(stack_frames)?, "ASTORE "),
        ISTORE_0 | ISTORE_1 | ISTORE_2 | ISTORE_3 => {
            handle_store::<i32, _>(last_frame_mut(stack_frames)?, code - ISTORE_0, "ISTORE_")
        }
        LSTORE_0 | LSTORE_1 | LSTORE_2 | LSTORE_3 => {
            handle_store::<i64, _>(last_frame_mut(stack_frames)?, code - LSTORE_0, "LSTORE_")
        }
        FSTORE_0 | FSTORE_1 | FSTORE_2 | FSTORE_3 => {
            handle_store::<f32, _>(last_frame_mut(stack_frames)?, code - FSTORE_0, "FSTORE_")
        }
        DSTORE_0 | DSTORE_1 | DSTORE_2 | DSTORE_3 => {
            handle_store::<f64, _>(last_frame_mut(stack_frames)?, code - DSTORE_0, "DSTORE_")
        }
        ASTORE_0 | ASTORE_1 | ASTORE_2 | ASTORE_3 => {
            handle_store::<Slot, _>(last_frame_mut(stack_frames)?, code - ASTORE_0, "ASTORE_")
        }
        IASTORE => handle_array_store::<i32>(stack_frames, "IASTORE")?,
        LASTORE => handle_array_store::<i64>(stack_frames, "LASTORE")?,
        FASTORE => handle_array_store::<f32>(stack_frames, "FASTORE")?,
        DASTORE => handle_array_store::<f64>(stack_frames, "DASTORE")?,
        AASTORE => handle_array_store::<Slot>(stack_frames, "AASTORE")?,
        BASTORE => handle_array_store::<i32>(stack_frames, "BASTORE")?,
        CASTORE => handle_array_store::<i32>(stack_frames, "CASTORE")?,
        SASTORE => handle_array_store::<i32>(stack_frames, "SASTORE")?,
        _ => {
            return Err(Error::new_execution(&format!(
                "Unknown store opcode: {}",
                code
            )));
        }
    }

    Ok(())
}

fn handle_pos_and_store<T: StackValue + Display + Copy>(
    stack_frame: &mut StackFrame,
    name_starts: &str,
) {
    let pos = stack_frame.extract_one_byte();
    handle_store::<T, _>(stack_frame, pos, name_starts);
}

pub(crate) fn handle_store<T: StackValue + Display + Copy, POS: Display + Copy>(
    stack_frame: &mut StackFrame,
    pos: POS,
    name_starts: &str,
) where
    usize: From<POS>,
{
    let value: T = stack_frame.pop();
    stack_frame.set_local(pos.into(), value);

    stack_frame.incr_pc();
    trace!("{name_starts}{pos} -> value={value}");
}

fn handle_array_store<T: StackValue + Display + Copy>(
    stack_frames: &mut StackFrames,
    name_starts: &str,
) -> Result<()> {
    let (arrayref, index, value) = {
        let stack_frame = last_frame_mut(stack_frames)?;
        let value: T = stack_frame.pop();
        let index: i32 = stack_frame.pop();
        let arrayref: i32 = stack_frame.pop();
        (arrayref, index, value)
    };
    if !check_array_access(arrayref, index, stack_frames)? {
        return Ok(());
    }
    let raw_value = value.to_vec();
    HEAP.set_array_value(arrayref, index, raw_value)?;

    let stack_frame = last_frame_mut(stack_frames)?;
    stack_frame.incr_pc();
    trace!("{name_starts} -> arrayref={arrayref}, index={index}, value={value}");

    Ok(())
}
