use crate::execution_engine::common::last_frame_mut;
use crate::execution_engine::opcode::*;
use crate::stack::sack_value::StackValue;
use crate::stack::stack_frame::{StackFrame, StackFrames};
use std::fmt::Display;
use tracing::trace;

pub(crate) fn process(code: u8, stack_frames: &mut StackFrames) -> crate::error::Result<()> {
    let stack_frame = last_frame_mut(stack_frames)?;
    match code {
        I2L => convert::<i32, i64>(stack_frame, |from| from.into(), "I2L"),
        I2F => convert::<i32, f32>(stack_frame, |from| from as f32, "I2F"),
        I2D => convert::<i32, f64>(stack_frame, |from| from as f64, "I2D"),
        L2I => convert::<i64, i32>(stack_frame, |from| from as i32, "L2I"),
        L2F => convert::<i64, f32>(stack_frame, |from| from as f32, "L2F"),
        L2D => convert::<i64, f64>(stack_frame, |from| from as f64, "L2D"),
        F2I => convert::<f32, i32>(stack_frame, |from| from as i32, "F2I"),
        F2L => convert::<f32, i64>(stack_frame, |from| from as i64, "F2L"),
        F2D => convert::<f32, f64>(stack_frame, |from| from as f64, "F2D"),
        D2I => convert::<f64, i32>(stack_frame, |from| from as i32, "D2I"),
        D2L => convert::<f64, i64>(stack_frame, |from| from as i64, "D2L"),
        I2B => convert::<i32, i32>(stack_frame, |from| from as i8 as i32, "I2B"),
        I2C => convert::<i32, i32>(stack_frame, |from| from as u16 as i32, "I2C"),
        I2S => convert::<i32, i32>(stack_frame, |from| from as i16 as i32, "I2S"),
        _ => Err(crate::error::Error::new_execution(&format!(
            "Unknown conversion opcode: {}",
            code
        ))),
    }
}

fn convert<From: StackValue + Copy + Display, To: StackValue + Copy + Display>(
    stack_frame: &mut StackFrame,
    convertor: impl Fn(From) -> To,
    name: &str,
) -> crate::error::Result<()> {
    let from: From = stack_frame.pop();
    let to = convertor(from);
    stack_frame.push(to)?;

    stack_frame.incr_pc();
    trace!("{name} -> {from}->{to}");
    Ok(())
}
