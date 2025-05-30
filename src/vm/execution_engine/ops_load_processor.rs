use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::common::last_frame_mut;
use crate::vm::execution_engine::opcode::*;
use crate::vm::heap::heap::with_heap_read_lock;
use crate::vm::stack::stack_frame::{StackFrame, StackFrames};
use crate::vm::stack::stack_value::StackValue;
use std::fmt::Display;
use tracing::trace;

pub(crate) fn process(code: u8, stack_frames: &mut StackFrames) -> Result<()> {
    let stack_frame = last_frame_mut(stack_frames)?;
    match code {
        ILOAD => handle_pos_and_load::<i32>(stack_frame, "ILOAD "),
        LLOAD => handle_pos_and_load::<i64>(stack_frame, "LLOAD "),
        FLOAD => handle_pos_and_load::<f32>(stack_frame, "FLOAD "),
        DLOAD => handle_pos_and_load::<f64>(stack_frame, "DLOAD "),
        ALOAD => handle_pos_and_load::<i32>(stack_frame, "ALOAD "),
        ILOAD_0 | ILOAD_1 | ILOAD_2 | ILOAD_3 => {
            handle_load::<i32, _>(stack_frame, code - ILOAD_0, "ILOAD_")
        }
        LLOAD_0 | LLOAD_1 | LLOAD_2 | LLOAD_3 => {
            handle_load::<i64, _>(stack_frame, code - LLOAD_0, "LLOAD_")
        }
        FLOAD_0 | FLOAD_1 | FLOAD_2 | FLOAD_3 => {
            handle_load::<f32, _>(stack_frame, code - FLOAD_0, "FLOAD_")
        }
        DLOAD_0 | DLOAD_1 | DLOAD_2 | DLOAD_3 => {
            handle_load::<f64, _>(stack_frame, code - DLOAD_0, "DLOAD_")
        }
        ALOAD_0 | ALOAD_1 | ALOAD_2 | ALOAD_3 => {
            handle_load::<i32, _>(stack_frame, code - ALOAD_0, "ALOAD_")
        }
        IALOAD => handle_array_load::<i32>(stack_frame, "IALOAD"),
        LALOAD => handle_array_load::<i64>(stack_frame, "LALOAD"),
        FALOAD => handle_array_load::<f32>(stack_frame, "FALOAD"),
        DALOAD => handle_array_load::<f64>(stack_frame, "DALOAD"),
        AALOAD => handle_array_load::<i32>(stack_frame, "AALOAD"),
        BALOAD => handle_array_load::<i32>(stack_frame, "BALOAD"),
        CALOAD => handle_array_load::<i32>(stack_frame, "CALOAD"),
        SALOAD => handle_array_load::<i32>(stack_frame, "SALOAD"),
        _ => Err(Error::new_execution(&format!(
            "Unknown load opcode: {}",
            code
        ))),
    }
}

fn handle_pos_and_load<T: StackValue + Display + Copy>(
    stack_frame: &mut StackFrame,
    name_starts: &str,
) -> Result<()> {
    let pos = stack_frame.extract_one_byte();
    handle_load::<T, _>(stack_frame, pos, name_starts)
}

pub(crate) fn handle_load<T: StackValue + Display + Copy, POS: Display + Copy>(
    stack_frame: &mut StackFrame,
    pos: POS,
    name_starts: &str,
) -> Result<()>
where
    usize: From<POS>,
{
    let value: T = stack_frame.get_local(pos.into());
    stack_frame.push(value)?;

    stack_frame.incr_pc();
    trace!("{name_starts}{pos} -> value={value}");
    Ok(())
}

fn handle_array_load<T: StackValue + Display + Copy>(
    stack_frame: &mut StackFrame,
    name_starts: &str,
) -> Result<()> {
    let index = stack_frame.pop();
    let arrayref: i32 = stack_frame.pop();
    let raw_value = with_heap_read_lock(|heap| heap.get_array_value(arrayref, index))?;

    let value: T = T::from_vec(&raw_value);

    stack_frame.push(value)?;
    stack_frame.incr_pc();
    trace!("{name_starts} -> arrayref={arrayref}, index={index}, value={value}");

    Ok(())
}
