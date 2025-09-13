use crate::vm::error::{Error, Result};
use crate::vm::exception::helpers::throw_null_pointer_exception_with_message;
use crate::vm::exception::throwing_result::ThrowingResult;
use crate::vm::execution_engine::common::last_frame_mut;
use crate::vm::execution_engine::opcode::*;
use crate::vm::heap::heap::with_heap_read_lock;
use crate::vm::stack::stack_frame::{StackFrame, StackFrames};
use crate::vm::stack::stack_value::StackValue;
use crate::{throw_and_return, unwrap_or_return_err, unwrap_result_or_return_default};
use std::fmt::Display;
use tracing::trace;

pub(crate) fn process(code: u8, stack_frames: &mut StackFrames) -> Result<()> {
    match code {
        ILOAD => handle_pos_and_load::<i32>(last_frame_mut(stack_frames)?, "ILOAD "),
        LLOAD => handle_pos_and_load::<i64>(last_frame_mut(stack_frames)?, "LLOAD "),
        FLOAD => handle_pos_and_load::<f32>(last_frame_mut(stack_frames)?, "FLOAD "),
        DLOAD => handle_pos_and_load::<f64>(last_frame_mut(stack_frames)?, "DLOAD "),
        ALOAD => handle_pos_and_load::<i32>(last_frame_mut(stack_frames)?, "ALOAD "),
        ILOAD_0 | ILOAD_1 | ILOAD_2 | ILOAD_3 => {
            handle_load::<i32, _>(last_frame_mut(stack_frames)?, code - ILOAD_0, "ILOAD_")
        }
        LLOAD_0 | LLOAD_1 | LLOAD_2 | LLOAD_3 => {
            handle_load::<i64, _>(last_frame_mut(stack_frames)?, code - LLOAD_0, "LLOAD_")
        }
        FLOAD_0 | FLOAD_1 | FLOAD_2 | FLOAD_3 => {
            handle_load::<f32, _>(last_frame_mut(stack_frames)?, code - FLOAD_0, "FLOAD_")
        }
        DLOAD_0 | DLOAD_1 | DLOAD_2 | DLOAD_3 => {
            handle_load::<f64, _>(last_frame_mut(stack_frames)?, code - DLOAD_0, "DLOAD_")
        }
        ALOAD_0 | ALOAD_1 | ALOAD_2 | ALOAD_3 => {
            handle_load::<i32, _>(last_frame_mut(stack_frames)?, code - ALOAD_0, "ALOAD_")
        }
        IALOAD => {
            let result = handle_array_load::<i32>(stack_frames, "IALOAD");
            Ok(unwrap_result_or_return_default!(result, ()))
        }
        LALOAD => {
            let result = handle_array_load::<i64>(stack_frames, "LALOAD");
            Ok(unwrap_result_or_return_default!(result, ()))
        }
        FALOAD => {
            let result = handle_array_load::<f32>(stack_frames, "FALOAD");
            Ok(unwrap_result_or_return_default!(result, ()))
        }
        DALOAD => {
            let result = handle_array_load::<f64>(stack_frames, "DALOAD");
            Ok(unwrap_result_or_return_default!(result, ()))
        }
        AALOAD => {
            let result = handle_array_load::<i32>(stack_frames, "AALOAD");
            Ok(unwrap_result_or_return_default!(result, ()))
        }
        BALOAD => {
            let result = handle_array_load::<i32>(stack_frames, "BALOAD");
            Ok(unwrap_result_or_return_default!(result, ()))
        }
        CALOAD => {
            let result = handle_array_load::<i32>(stack_frames, "CALOAD");
            Ok(unwrap_result_or_return_default!(result, ()))
        }
        SALOAD => {
            let result = handle_array_load::<i32>(stack_frames, "SALOAD");
            Ok(unwrap_result_or_return_default!(result, ()))
        }
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
    stack_frames: &mut StackFrames,
    name_starts: &str,
) -> ThrowingResult<()> {
    let (arrayref, index) = {
        let stack_frame = unwrap_or_return_err!(last_frame_mut(stack_frames));
        let index: i32 = stack_frame.pop();
        let arrayref: i32 = stack_frame.pop();
        (arrayref, index)
    };
    if arrayref == 0 {
        let array_type = unwrap_or_return_err!(type_by_aload(name_starts));
        throw_and_return!(throw_null_pointer_exception_with_message(
            &format!("Cannot load from {array_type} array because \"<VALUE>\" is null"),
            stack_frames
        ))
    }
    let raw_value = with_heap_read_lock(|heap| heap.get_array_value(arrayref, index));
    let raw_value = unwrap_or_return_err!(raw_value);

    let value: T = T::from_vec(&raw_value);

    let stack_frame = unwrap_or_return_err!(last_frame_mut(stack_frames));
    unwrap_or_return_err!(stack_frame.push(value));
    stack_frame.incr_pc();
    trace!("{name_starts} -> arrayref={arrayref}, index={index}, value={value}");

    ThrowingResult::ok(())
}

fn type_by_aload(aload: &str) -> Result<&str> {
    match aload {
        "IALOAD" => Ok("int"),
        "LALOAD" => Ok("long"),
        "FALOAD" => Ok("float"),
        "DALOAD" => Ok("double"),
        "AALOAD" => Ok("object"),
        "BALOAD" => Ok("byte"),
        "CALOAD" => Ok("char"),
        "SALOAD" => Ok("short"),
        _ => Err(Error::new_execution(&format!(
            "Unknown array load type: {aload}"
        ))),
    }
}
