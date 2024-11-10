use crate::error::Error;
use crate::execution_engine::opcode::*;
use crate::stack::stack_frame::StackFrame;
use tracing::trace;

pub(crate) fn process(code: u8, stack_frames: &mut Vec<StackFrame>) -> crate::error::Result<()> {
    let stack_frame = stack_frames.last_mut().unwrap();
    match code {
        WIDE => {
            stack_frame.incr_pc();
            let opcode = stack_frame.get_bytecode_byte();
            match opcode {
                ILOAD => {
                    let index = stack_frame.extract_two_bytes() as usize;
                    let value: i32 = stack_frame.get_local(index);
                    stack_frame.push(value);
                    stack_frame.incr_pc();
                    trace!("WIDE ILOAD -> index={index}, value={value}");
                }
                LLOAD => {
                    let index = stack_frame.extract_two_bytes() as usize;
                    let value: i64 = stack_frame.get_local(index);
                    stack_frame.push(value);

                    stack_frame.incr_pc();
                    trace!("WIDE LLOAD -> index={index}, value={value}");
                }
                FLOAD => {
                    let index = stack_frame.extract_two_bytes() as usize;
                    let value: f32 = stack_frame.get_local(index);
                    stack_frame.push(value);
                    stack_frame.incr_pc();
                    trace!("WIDE FLOAD -> index={index}, value={value}");
                }
                DLOAD => {
                    let index = stack_frame.extract_two_bytes() as usize;
                    let value: f64 = stack_frame.get_local(index);
                    stack_frame.push(value);

                    stack_frame.incr_pc();
                    trace!("WIDE DLOAD -> index={index}, value={value}");
                }
                ALOAD => {
                    let index = stack_frame.extract_two_bytes() as usize;
                    let value: i32 = stack_frame.get_local(index);
                    stack_frame.push(value);
                    stack_frame.incr_pc();
                    trace!("WIDE ALOAD -> index={index}, value={value}");
                }
                ISTORE => {
                    let index = stack_frame.extract_two_bytes() as usize;
                    let value: i32 = stack_frame.pop();
                    stack_frame.set_local(index, value);
                    stack_frame.incr_pc();
                    trace!("WIDE ISTORE -> index={index}, value={value}");
                }
                LSTORE => {
                    let index = stack_frame.extract_two_bytes() as usize;
                    let value: i64 = stack_frame.pop();
                    stack_frame.set_local(index, value);

                    stack_frame.incr_pc();
                    trace!("WIDE LSTORE -> index={index}, value={value}");
                }
                FSTORE => {
                    let index = stack_frame.extract_two_bytes() as usize;
                    let value: f32 = stack_frame.pop();
                    stack_frame.set_local(index, value);
                    stack_frame.incr_pc();
                    trace!("WIDE FSTORE -> index={index}, value={value}");
                }
                DSTORE => {
                    let index = stack_frame.extract_two_bytes() as usize;
                    let value: f64 = stack_frame.pop();
                    stack_frame.set_local(index, value);

                    stack_frame.incr_pc();
                    trace!("WIDE DSTORE -> index={index}, value={value}");
                }
                ASTORE => {
                    let index = stack_frame.extract_two_bytes() as usize;
                    let obj_ref: i32 = stack_frame.pop();
                    stack_frame.set_local(index, obj_ref);
                    stack_frame.incr_pc();
                    trace!("WIDE ASTORE -> index={index}, obj_ref={obj_ref}");
                }
                IINC => {
                    let index = stack_frame.extract_two_bytes() as u16 as usize;
                    let const_val = stack_frame.extract_two_bytes();

                    let current_val: i32 = stack_frame.get_local(index);
                    let new_val = current_val + const_val as i32;
                    stack_frame.set_local(index, new_val);

                    stack_frame.incr_pc();
                    trace!("WIDE IINC -> {current_val} + {const_val} = {new_val}");
                }
                _ => {
                    return Err(Error::new_execution(&format!(
                        "Error executing WIDE opcode {opcode}"
                    )))
                }
            }
        }
        IFNULL => {
            //todo: this one is opposite to IFNE ops code
            let value: i32 = stack_frame.pop();
            let offset = stack_frame.get_two_bytes_ahead();
            stack_frame.advance_pc(if value == 0 { offset } else { 3 });
            trace!("IFNULL -> value={value}, offset={offset}");
        }
        IFNONNULL => {
            //todo: this one is opposite to IFNULL ops code
            let value: i32 = stack_frame.pop();
            let offset = stack_frame.get_two_bytes_ahead();
            stack_frame.advance_pc(if value != 0 { offset } else { 3 });
            trace!("IFNONNULL -> value={value}, offset={offset}");
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
