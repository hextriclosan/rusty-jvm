use crate::error::Error;
use crate::execution_engine::opcode::*;
use crate::execution_engine::ops_comparison_processor::branch1arg;
use crate::execution_engine::ops_load_processor::handle_load;
use crate::execution_engine::ops_math_processor::increment;
use crate::execution_engine::ops_store_processor::handle_store;
use crate::stack::sack_value::StackValue;
use crate::stack::stack_frame::{StackFrame, StackFrames};
use std::fmt::Display;

pub(crate) fn process(code: u8, stack_frames: &mut StackFrames) -> crate::error::Result<()> {
    let stack_frame = stack_frames.last_mut().unwrap();
    match code {
        WIDE => {
            let opcode = stack_frame.extract_one_byte();
            match opcode {
                ILOAD => handle_pos_and_load::<i32>(stack_frame, "WIDE ILOAD "),
                LLOAD => handle_pos_and_load::<i64>(stack_frame, "WIDE LLOAD "),
                FLOAD => handle_pos_and_load::<f32>(stack_frame, "WIDE FLOAD "),
                DLOAD => handle_pos_and_load::<f64>(stack_frame, "WIDE DLOAD "),
                ALOAD => handle_pos_and_load::<i32>(stack_frame, "WIDE ALOAD "),
                ISTORE => handle_pos_and_store::<i32>(stack_frame, "WIDE ISTORE "),
                LSTORE => handle_pos_and_store::<i64>(stack_frame, "WIDE LSTORE "),
                FSTORE => handle_pos_and_store::<f32>(stack_frame, "WIDE FSTORE "),
                DSTORE => handle_pos_and_store::<f64>(stack_frame, "WIDE DSTORE "),
                ASTORE => handle_pos_and_store::<i32>(stack_frame, "WIDE ASTORE "),
                IINC => increment(
                    stack_frame,
                    |sf| sf.extract_two_bytes() as usize,
                    |sf| sf.extract_two_bytes(),
                    "WIDE IINC",
                ),
                _ => {
                    return Err(Error::new_execution(&format!(
                        "Error executing WIDE opcode {opcode}"
                    )))
                }
            }
        }
        IFNULL => branch1arg(|a| a == 0, stack_frame, "IFNULL"),
        IFNONNULL => branch1arg(|a| a != 0, stack_frame, "IFNONNULL"),
        _ => {
            return Err(Error::new_execution(&format!(
                "Unknown extended opcode: {}",
                code
            )));
        }
    }

    Ok(())
}

fn handle_pos_and_load<T: StackValue + Display + Copy>(
    stack_frame: &mut StackFrame,
    name_starts: &str,
) {
    let pos = stack_frame.extract_two_bytes();
    handle_load::<T, _>(stack_frame, pos as usize, name_starts);
}

fn handle_pos_and_store<T: StackValue + Display + Copy>(
    stack_frame: &mut StackFrame,
    name_starts: &str,
) {
    let pos = stack_frame.extract_two_bytes();
    handle_store::<T, _>(stack_frame, pos as usize, name_starts);
}
