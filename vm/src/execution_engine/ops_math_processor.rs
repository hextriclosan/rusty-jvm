use crate::execution_engine::opcode::*;
use crate::stack::stack_frame::StackFrame;
use tracing::trace;

pub(crate) fn process(code: u8, stack_frames: &mut Vec<StackFrame>) -> crate::error::Result<()> {
    let stack_frame = stack_frames.last_mut().unwrap();
    match code {
        IADD => {
            let b: i32 = stack_frame.pop();
            let a: i32 = stack_frame.pop();
            let result = a + b;
            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("IADD -> {a} + {b} = {result}");
        }
        LADD => {
            let b: i64 = stack_frame.pop();
            let a: i64 = stack_frame.pop();

            let result = a.wrapping_add(b);

            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("LADD -> {a} + {b} = {result}");
        }
        FADD => {
            let b: f32 = stack_frame.pop();
            let a: f32 = stack_frame.pop();
            let result = a + b;
            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("FADD -> {a} + {b} = {result}");
        }
        DADD => {
            let b: f64 = stack_frame.pop();
            let a: f64 = stack_frame.pop();

            let result = a + b;

            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("DADD -> {a} + {b} = {result}");
        }
        ISUB => {
            let b: i32 = stack_frame.pop();
            let a: i32 = stack_frame.pop();
            let result = a - b;
            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("ISUB -> {a} - {b} = {result}");
        }
        LSUB => {
            let b: i64 = stack_frame.pop();
            let a: i64 = stack_frame.pop();

            let result = a - b;

            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("LSUB -> {a} - {b} = {result}");
        }
        FSUB => {
            let b: f32 = stack_frame.pop();
            let a: f32 = stack_frame.pop();
            let result = a - b;
            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("FSUB -> {a} - {b} = {result}");
        }
        DSUB => {
            let b: f64 = stack_frame.pop();
            let a: f64 = stack_frame.pop();

            let result = a - b;

            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("DSUB -> {a} - {b} = {result}");
        }
        IMUL => {
            let b: i32 = stack_frame.pop();
            let a: i32 = stack_frame.pop();
            let result = a.wrapping_mul(b);
            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("IMUL -> {a} * {b} = {result}");
        }
        LMUL => {
            let b: i64 = stack_frame.pop();
            let a: i64 = stack_frame.pop();

            let result = b.wrapping_mul(a);

            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("LMUL -> {a} * {b} = {result}");
        }
        FMUL => {
            let b: f32 = stack_frame.pop();
            let a: f32 = stack_frame.pop();
            let result = a * b;
            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("FMUL -> {a} * {b} = {result}");
        }
        DMUL => {
            let b: f64 = stack_frame.pop();
            let a: f64 = stack_frame.pop();

            let result = a * b;

            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("DMUL -> {a} * {b} = {result}");
        }
        IDIV => {
            let b: i32 = stack_frame.pop();
            let a: i32 = stack_frame.pop();
            let result = a / b; //todo add check for ArithmeticException here
            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("IDIV -> {a} / {b} = {result}");
        }
        LDIV => {
            let b: i64 = stack_frame.pop();
            let a: i64 = stack_frame.pop();

            let result = a / b; //todo add check for ArithmeticException here

            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("LDIV -> {a} / {b} = {result}");
        }
        FDIV => {
            let b: f32 = stack_frame.pop();
            let a: f32 = stack_frame.pop();
            let result = a / b;
            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("FDIV -> {a} / {b} = {result}");
        }
        DDIV => {
            let b: f64 = stack_frame.pop();
            let a: f64 = stack_frame.pop();

            let result = a / b;

            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("DDIV -> {a} / {b} = {result}");
        }
        IREM => {
            let b: i32 = stack_frame.pop();
            let a: i32 = stack_frame.pop();
            let result = a % b;
            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("IREM -> {a} % {b} = {result}");
        }
        LREM => {
            let b: i64 = stack_frame.pop();
            let a: i64 = stack_frame.pop();

            let result = a % b;

            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("LREM -> {a} % {b} = {result}");
        }
        DREM => {
            let b: f64 = stack_frame.pop();
            let a: f64 = stack_frame.pop();

            let result = a % b;

            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("DREM -> {a} % {b} = {result}");
        }
        INEG => {
            let value: i32 = stack_frame.pop();
            let result = -value;
            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("INEG -> {result}");
        }
        LNEG => {
            let value: i64 = stack_frame.pop();
            let result = -value;
            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("LNEG -> {result}");
        }
        ISHL => {
            let b: i32 = stack_frame.pop();
            let a: i32 = stack_frame.pop();

            let b_trunc = b & 0b00011111;
            let result = a << b_trunc;
            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("ISHL -> {a} << {b} = {result}");
        }
        LSHL => {
            let b: i32 = stack_frame.pop();
            let a: i64 = stack_frame.pop();

            let b_trunc = b as u32 & 0b00111111u32;
            let result = a << b_trunc;
            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("LSHL -> {a} << {b} = {result}");
        }
        ISHR => {
            // todo: recheck spec
            let b: i32 = stack_frame.pop();
            let a: i32 = stack_frame.pop();

            let b_trunc = b as u32 & 0b00011111u32;
            let result = a >> b_trunc;
            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("ISHR -> {a} >> {b} = {result}");
        }
        LSHR => {
            let b: i32 = stack_frame.pop();
            let a: i64 = stack_frame.pop();

            let b_trunc = b as u32 & 0b00111111u32;
            let result = a >> b_trunc;
            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("LSHR -> {a} >> {b} = {result}");
        }
        IUSHR => {
            let b: i32 = stack_frame.pop();
            let a: i32 = stack_frame.pop();

            let b_trunc = b as u32 & 0b00011111u32;
            let result = (a as u32 >> b_trunc) as i32;
            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("IUSHR -> {a} >> {b} = {result}");
        }
        LUSHR => {
            let b: i32 = stack_frame.pop();
            let a: i64 = stack_frame.pop();

            let b_trunc = b as u32 & 0b00111111u32;
            let result = (a as u64 >> b_trunc) as i64;
            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("LUSHR -> {a} >> {b} = {result}");
        }
        IAND => {
            let b: i32 = stack_frame.pop();
            let a: i32 = stack_frame.pop();

            let result = a & b;
            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("IAND -> {a} & {b} = {result}");
        }
        LAND => {
            let b: i64 = stack_frame.pop();
            let a: i64 = stack_frame.pop();

            let result = a & b;

            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("LAND -> {a} & {b} = {result}");
        }
        IOR => {
            let b: i32 = stack_frame.pop();
            let a: i32 = stack_frame.pop();

            let result = a | b;
            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("IOR -> {a} | {b} = {result}");
        }
        LOR => {
            let b: i64 = stack_frame.pop();
            let a: i64 = stack_frame.pop();

            let result = a | b;

            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("LOR -> {a} | {b} = {result}");
        }
        IXOR => {
            let b: i32 = stack_frame.pop();
            let a: i32 = stack_frame.pop();

            let result = a ^ b;
            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("IXOR -> {a} & {b} = {result}");
        }
        LXOR => {
            let b: i64 = stack_frame.pop();
            let a: i64 = stack_frame.pop();

            let result = a ^ b;

            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("LXOR -> {a} & {b} = {result}");
        }
        IINC => {
            stack_frame.incr_pc();
            let index = stack_frame.get_bytecode_byte() as usize;

            stack_frame.incr_pc();
            let const_val = stack_frame.get_bytecode_byte() as i8;

            let current_val: i32 = stack_frame.get_local(index);
            let new_val = current_val + const_val as i32;
            stack_frame.set_local(index, new_val);

            stack_frame.incr_pc();
            trace!("IINC -> {current_val} + {const_val} = {new_val}");
        }
        _ => {
            return Err(crate::error::Error::new_execution(&format!(
                "Unknown math opcode: {}",
                code
            )));
        }
    }

    Ok(())
}
