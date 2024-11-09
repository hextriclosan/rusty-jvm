use crate::execution_engine::opcode::*;
use crate::stack::stack_frame::StackFrame;
use tracing::trace;

pub(crate) fn process(code: u8, stack_frames: &mut Vec<StackFrame>) -> crate::error::Result<()> {
    let stack_frame = stack_frames.last_mut().unwrap();
    match code {
        I2L => {
            let value = stack_frame.pop() as i64;

            stack_frame.push_i64(value);

            stack_frame.incr_pc();
            trace!("I2L -> {value}L");
        }
        I2F => {
            let value = stack_frame.pop() as f32;

            stack_frame.push_f32(value);

            stack_frame.incr_pc();
            trace!("I2F -> {value}F");
        }
        I2D => {
            let value = stack_frame.pop() as f64;

            stack_frame.push_i64(value.to_bits() as i64);

            stack_frame.incr_pc();
            trace!("I2D -> {value}D");
        }
        L2I => {
            let value = stack_frame.pop_i64() as i32;

            stack_frame.push(value);

            stack_frame.incr_pc();
            trace!("L2I -> {value}I");
        }
        L2F => {
            let value = stack_frame.pop_i64() as f32;

            stack_frame.push_f32(value);

            stack_frame.incr_pc();
            trace!("L2F -> {value}F");
        }
        L2D => {
            let value = stack_frame.pop_i64() as f64;

            stack_frame.push_f64(value);

            stack_frame.incr_pc();
            trace!("L2D -> {value}D");
        }
        F2I => {
            let value = stack_frame.pop_f32() as i32;

            stack_frame.push(value);

            stack_frame.incr_pc();
            trace!("F2I -> {value}I");
        }
        F2L => {
            let value = stack_frame.pop_f32();

            stack_frame.push_i64(value as i64);

            stack_frame.incr_pc();
            trace!("F2L -> {value}L");
        }
        F2D => {
            let value = stack_frame.pop_f32() as f64;

            stack_frame.push_f64(value);

            stack_frame.incr_pc();
            trace!("F2D -> {value}D");
        }
        D2I => {
            let value = stack_frame.pop_f64();

            stack_frame.push(value as i32);

            stack_frame.incr_pc();
            trace!("D2I -> {value}I");
        }
        D2L => {
            let value = stack_frame.pop_f64();

            stack_frame.push_i64(value as i64);

            stack_frame.incr_pc();
            trace!("D2L -> {value}L");
        }
        I2B => {
            let value = stack_frame.pop() as i8;

            stack_frame.push(value as i32);

            stack_frame.incr_pc();
            trace!("I2B -> {value}B");
        }
        I2C => {
            let value = stack_frame.pop() as u16;

            stack_frame.push(value as i32);

            stack_frame.incr_pc();
            trace!("I2C -> {value}C");
        }
        I2S => {
            let value = stack_frame.pop() as i16;

            stack_frame.push(value as i32);

            stack_frame.incr_pc();
            trace!("I2S -> {value}S");
        }
        _ => {
            return Err(crate::error::Error::new_execution(&format!(
                "Unknown conversion opcode: {}",
                code
            )));
        }
    }

    Ok(())
}
