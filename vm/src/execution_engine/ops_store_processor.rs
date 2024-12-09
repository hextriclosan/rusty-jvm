use crate::execution_engine::common::last_frame_mut;
use crate::execution_engine::opcode::*;
use crate::heap::heap::with_heap_write_lock;
use crate::stack::sack_value::StackValue;
use crate::stack::stack_frame::{StackFrame, StackFrames};
use std::fmt::Display;
use tracing::trace;

pub(crate) fn process(code: u8, stack_frames: &mut StackFrames) -> crate::error::Result<()> {
    let stack_frame = last_frame_mut(stack_frames)?;
    match code {
        ISTORE => handle_pos_and_store::<i32>(stack_frame, "ISTORE "),
        LSTORE => handle_pos_and_store::<i64>(stack_frame, "LSTORE "),
        FSTORE => handle_pos_and_store::<f32>(stack_frame, "FSTORE "),
        DSTORE => handle_pos_and_store::<f64>(stack_frame, "DSTORE "),
        ASTORE => handle_pos_and_store::<i32>(stack_frame, "ASTORE "),
        ISTORE_0 | ISTORE_1 | ISTORE_2 | ISTORE_3 => {
            handle_store::<i32, _>(stack_frame, code - ISTORE_0, "ISTORE_")
        }
        LSTORE_0 | LSTORE_1 | LSTORE_2 | LSTORE_3 => {
            handle_store::<i64, _>(stack_frame, code - LSTORE_0, "LSTORE_")
        }
        FSTORE_0 | FSTORE_1 | FSTORE_2 | FSTORE_3 => {
            handle_store::<f32, _>(stack_frame, code - FSTORE_0, "FSTORE_")
        }
        DSTORE_0 | DSTORE_1 | DSTORE_2 | DSTORE_3 => {
            handle_store::<f64, _>(stack_frame, code - DSTORE_0, "DSTORE_")
        }
        ASTORE_0 | ASTORE_1 | ASTORE_2 | ASTORE_3 => {
            handle_store::<i32, _>(stack_frame, code - ASTORE_0, "ASTORE_")
        }
        IASTORE => handle_array_store::<i32>(stack_frame, "IASTORE")?,
        LASTORE => handle_array_store::<i64>(stack_frame, "LASTORE")?,
        FASTORE => handle_array_store::<f32>(stack_frame, "FASTORE")?,
        DASTORE => handle_array_store::<f64>(stack_frame, "DASTORE")?,
        AASTORE => handle_array_store::<i32>(stack_frame, "AASTORE")?,
        BASTORE => handle_array_store::<i32>(stack_frame, "BASTORE")?,
        CASTORE => handle_array_store::<i32>(stack_frame, "CASTORE")?,
        SASTORE => handle_array_store::<i32>(stack_frame, "SASTORE")?,
        _ => {
            return Err(crate::error::Error::new_execution(&format!(
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
    stack_frame: &mut StackFrame,
    name_starts: &str,
) -> crate::error::Result<()> {
    let value: T = stack_frame.pop();
    let index = stack_frame.pop();
    let arrayref: i32 = stack_frame.pop();
    let raw_value = value.to_vec();
    with_heap_write_lock(|heap| heap.set_array_value(arrayref, index, raw_value))?;

    stack_frame.incr_pc();
    trace!("{name_starts} -> arrayref={arrayref}, index={index}, value={value}");

    Ok(())
}
