use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::common::last_frame_mut;
use crate::vm::execution_engine::opcode::*;
use crate::vm::execution_engine::ops_comparison_processor::branch1arg;
use crate::vm::execution_engine::ops_load_processor::handle_load;
use crate::vm::execution_engine::ops_math_processor::increment;
use crate::vm::execution_engine::ops_store_processor::handle_store;
use crate::vm::heap::heap::with_heap_write_lock;
use crate::vm::method_area::cpool_helper::CPoolHelperTrait;
use crate::vm::method_area::method_area::with_method_area;
use crate::vm::stack::stack_frame::{StackFrame, StackFrames};
use crate::vm::stack::stack_value::StackValue;
use std::fmt::Display;
use tracing::trace;

pub(crate) fn process(
    code: u8,
    current_class_name: &str,
    stack_frames: &mut StackFrames,
) -> Result<()> {
    let stack_frame = last_frame_mut(stack_frames)?;
    match code {
        WIDE => {
            let opcode = stack_frame.extract_one_byte();
            match opcode {
                ILOAD => handle_pos_and_load::<i32>(stack_frame, "WIDE ILOAD ")?,
                LLOAD => handle_pos_and_load::<i64>(stack_frame, "WIDE LLOAD ")?,
                FLOAD => handle_pos_and_load::<f32>(stack_frame, "WIDE FLOAD ")?,
                DLOAD => handle_pos_and_load::<f64>(stack_frame, "WIDE DLOAD ")?,
                ALOAD => handle_pos_and_load::<i32>(stack_frame, "WIDE ALOAD ")?,
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
        MULTIANEWARRAY => {
            let class_index = stack_frame.extract_two_bytes();
            let dimension_number = stack_frame.extract_one_byte();

            let rc = with_method_area(|method_area| method_area.get(current_class_name))?;
            let cpool_helper = rc.cpool_helper();
            let class_name = cpool_helper
                .get_class_name(class_index as u16)
                .ok_or_else(|| {
                    Error::new_execution(&format!(
                        "Error getting class name by index {class_index}"
                    ))
                })?;

            let dimentions = (0..dimension_number)
                .map(|_| stack_frame.pop())
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect::<Vec<_>>();

            let root_array_ref = create_n_array(&dimentions, &class_name, 0)?;
            stack_frame.push(root_array_ref)?;

            stack_frame.incr_pc();
            trace!("MULTIANEWARRAY -> type={class_name}, dimension_number={dimension_number}, root_array_ref={root_array_ref}");
        }
        IFNULL => branch1arg(|a| a == 0, stack_frame, "IFNULL"),
        IFNONNULL => branch1arg(|a| a != 0, stack_frame, "IFNONNULL"),
        GOTO_W => {
            let stack_frame = last_frame_mut(stack_frames)?;
            let offset = stack_frame.get_four_bytes_ahead();
            stack_frame.advance_pc_wide(offset);
            trace!("GOTO_W -> offset={offset}");
        }
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
) -> Result<()> {
    let pos = stack_frame.extract_two_bytes();
    handle_load::<T, _>(stack_frame, pos as usize, name_starts)
}

fn handle_pos_and_store<T: StackValue + Display + Copy>(
    stack_frame: &mut StackFrame,
    name_starts: &str,
) {
    let pos = stack_frame.extract_two_bytes();
    handle_store::<T, _>(stack_frame, pos as usize, name_starts);
}

fn create_n_array(dimensions: &[i32], signature: &str, current_level: usize) -> Result<i32> {
    let current_length = dimensions[current_level];
    let current_signature = &signature[current_level..];
    let arrayref =
        with_heap_write_lock(|heap| heap.create_array(current_signature, current_length));

    if current_level < dimensions.len() - 1 {
        for i in 0..current_length {
            let next_ref = create_n_array(dimensions, signature, current_level + 1)?;
            with_heap_write_lock(|heap| heap.set_array_value(arrayref, i, vec![next_ref]))?;
        }
    }

    Ok(arrayref)
}
