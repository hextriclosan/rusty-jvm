use crate::execution_engine::opcode::*;
use crate::stack::stack_frame::StackFrame;
use tracing::trace;

pub(crate) fn process(code: u8, stack_frames: &mut Vec<StackFrame>) -> crate::error::Result<()> {
    let stack_frame = stack_frames.last_mut().unwrap();
    match code {
        I2L => {
            let value: i32 = stack_frame.pop();
            let result = value as i64;
            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("I2L -> {result}L");
        }
        I2F => {
            let value: i32 = stack_frame.pop();
            let result = value as f32;
            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("I2F -> {result}F");
        }
        I2D => {
            let value: i32 = stack_frame.pop();
            let result = value as f64;
            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("I2D -> {result}D");
        }
        L2I => {
            let value: i64 = stack_frame.pop();
            let result = value as i32;
            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("L2I -> {result}I");
        }
        L2F => {
            let value: i64 = stack_frame.pop();
            let result = value as f32;
            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("L2F -> {result}F");
        }
        L2D => {
            let value: i64 = stack_frame.pop();
            let result = value as f64;
            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("L2D -> {result}D");
        }
        F2I => {
            let value: f32 = stack_frame.pop();
            let result = value as i32;
            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("F2I -> {result}I");
        }
        F2L => {
            let value: f32 = stack_frame.pop();
            let result = value as i64;
            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("F2L -> {result}L");
        }
        F2D => {
            let value: f32 = stack_frame.pop();
            let result = value as f64;
            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("F2D -> {result}D");
        }
        D2I => {
            let value: f64 = stack_frame.pop();
            let result = value as i32;
            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("D2I -> {result}I");
        }
        D2L => {
            let value: f64 = stack_frame.pop();
            let result = value as i64;
            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("D2L -> {result}L");
        }
        I2B => {
            let value: i32 = stack_frame.pop();
            let result = value as i8 as i32;
            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("I2B -> {result}B");
        }
        I2C => {
            let value: i32 = stack_frame.pop();
            let result = value as u16 as i32;
            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("I2C -> {result}C");
        }
        I2S => {
            let value: i32 = stack_frame.pop();
            let result = value as i16 as i32;
            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("I2S -> {result}S");
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
