use crate::error::Error;
use crate::execution_engine::opcode::*;
use crate::helper::i32toi64;
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
                    let value = stack_frame.get_local(index);
                    stack_frame.push(value);
                    stack_frame.incr_pc();
                    trace!("WIDE ILOAD -> index={index}, value={value}");
                }
                LLOAD => {
                    let index = stack_frame.extract_two_bytes() as usize;

                    let (low, high, value) = stack_frame.get_two_bytes_from_local(index);

                    stack_frame.push(low);
                    stack_frame.push(high);

                    stack_frame.incr_pc();
                    trace!("WIDE LLOAD -> index={index}, value={value}");
                }
                FLOAD => {
                    let index = stack_frame.extract_two_bytes() as usize;
                    let value = stack_frame.get_local(index);
                    stack_frame.push(value);
                    stack_frame.incr_pc();
                    trace!(
                        "WIDE FLOAD -> index={index}, value={}",
                        f32::from_bits(value as u32)
                    );
                }
                DLOAD => {
                    let index = stack_frame.extract_two_bytes() as usize;
                    let (low, high, value) = stack_frame.get_two_bytes_from_local(index);

                    stack_frame.push(low);
                    stack_frame.push(high);

                    stack_frame.incr_pc();
                    trace!(
                        "WIDE DLOAD -> index={index}, value={}",
                        f64::from_bits(value as u64)
                    );
                }
                ALOAD => {
                    let index = stack_frame.extract_two_bytes() as usize;
                    let value = stack_frame.get_local(index);
                    stack_frame.push(value);
                    stack_frame.incr_pc();
                    trace!("WIDE ALOAD -> index={index}, value={value}");
                }
                ISTORE => {
                    let index = stack_frame.extract_two_bytes() as usize;
                    let value = stack_frame.pop();
                    stack_frame.set_local(index, value);
                    stack_frame.incr_pc();
                    trace!("WIDE ISTORE -> index={index}, value={value}");
                }
                LSTORE => {
                    let index = stack_frame.extract_two_bytes() as usize;
                    let high = stack_frame.pop();
                    let low = stack_frame.pop();

                    stack_frame.set_local(index, low);
                    stack_frame.set_local(index + 1, high);

                    stack_frame.incr_pc();
                    let value = i32toi64(high, low);
                    trace!("WIDE LSTORE -> index={index}, value={value}");
                }
                FSTORE => {
                    let index = stack_frame.extract_two_bytes() as usize;
                    let value = stack_frame.pop();
                    stack_frame.set_local(index, value);
                    stack_frame.incr_pc();
                    trace!(
                        "WIDE FSTORE -> index={index}, value={}",
                        f32::from_bits(value as u32)
                    );
                }
                DSTORE => {
                    let index = stack_frame.extract_two_bytes() as usize;

                    let high = stack_frame.pop();
                    let low = stack_frame.pop();
                    stack_frame.set_local(index, low);
                    stack_frame.set_local(index + 1, high);

                    stack_frame.incr_pc();
                    let value = ((high as i64) << 32) | (low as i64);
                    trace!(
                        "WIDE DSTORE -> index={index}, value={}",
                        f64::from_bits(value as u64)
                    );
                }
                ASTORE => {
                    let index = stack_frame.extract_two_bytes() as usize;
                    let obj_ref = stack_frame.pop();
                    stack_frame.set_local(index, obj_ref);
                    stack_frame.incr_pc();
                    trace!("WIDE ASTORE -> index={index}, obj_ref={obj_ref}");
                }
                IINC => {
                    let index = stack_frame.extract_two_bytes() as u16 as usize;
                    let const_val = stack_frame.extract_two_bytes();

                    let current_val = stack_frame.get_local(index);
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
            let value = stack_frame.pop();
            let offset = stack_frame.get_two_bytes_ahead();
            stack_frame.advance_pc(if value == 0 { offset } else { 3 });
            trace!("IFNULL -> value={value}, offset={offset}");
        }
        IFNONNULL => {
            //todo: this one is opposite to IFNULL ops code
            let value = stack_frame.pop();
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
