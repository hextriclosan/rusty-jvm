use crate::execution_engine::common::last_frame_mut;
use crate::execution_engine::opcode::*;
use crate::stack::sack_value::StackValue;
use crate::stack::stack_frame::{StackFrame, StackFrames};
use std::fmt::Display;
use tracing::trace;

pub(crate) fn process(code: u8, stack_frames: &mut StackFrames) -> crate::error::Result<()> {
    let stack_frame = last_frame_mut(stack_frames)?;
    match code {
        IADD => binary_operation(stack_frame, |a: i32, b| a.wrapping_add(b), "IADD"),
        LADD => binary_operation(stack_frame, |a: i64, b| a.wrapping_add(b), "LADD"),
        FADD => binary_operation(stack_frame, |a: f32, b: f32| a + b, "FADD"),
        DADD => binary_operation(stack_frame, |a: f64, b: f64| a + b, "DADD"),
        ISUB => binary_operation(stack_frame, |a: i32, b| a.wrapping_sub(b), "ISUB"),
        LSUB => binary_operation(stack_frame, |a: i64, b| a.wrapping_sub(b), "LSUB"),
        FSUB => binary_operation(stack_frame, |a: f32, b: f32| a - b, "FSUB"),
        DSUB => binary_operation(stack_frame, |a: f64, b: f64| a - b, "DSUB"),
        IMUL => binary_operation(stack_frame, |a: i32, b| a.wrapping_mul(b), "IMUL"),
        LMUL => binary_operation(stack_frame, |a: i64, b| a.wrapping_mul(b), "LMUL"),
        FMUL => binary_operation(stack_frame, |a: f32, b: f32| a * b, "FMUL"),
        DMUL => binary_operation(stack_frame, |a: f64, b: f64| a * b, "DMUL"),
        IDIV => binary_operation(stack_frame, |a: i32, b| a.wrapping_div(b), "IDIV"), // todo add check for ArithmeticException here
        LDIV => binary_operation(stack_frame, |a: i64, b| a.wrapping_div(b), "LDIV"), // todo add check for ArithmeticException here
        FDIV => binary_operation(stack_frame, |a: f32, b: f32| a / b, "FDIV"),
        DDIV => binary_operation(stack_frame, |a: f64, b: f64| a / b, "DDIV"),
        IREM => binary_operation(stack_frame, |a: i32, b| a.wrapping_rem(b), "IREM"), // todo add check for ArithmeticException here
        LREM => binary_operation(stack_frame, |a: i64, b| a.wrapping_rem(b), "LREM"), // todo add check for ArithmeticException here
        DREM => binary_operation(stack_frame, |a: f64, b: f64| a % b, "DREM"),
        INEG => unary_operation(stack_frame, |a: i32| a.wrapping_neg(), "INEG"),
        LNEG => unary_operation(stack_frame, |a: i64| a.wrapping_neg(), "LNEG"),
        ISHL => binary_operation(stack_frame, |a: i32, b: i32| a << (b & 0b00011111), "ISHL"),
        LSHL => binary_operation(
            stack_frame,
            |a: i64, b: i32| a << (b as u32 & 0b00111111u32),
            "LSHL",
        ),
        ISHR => binary_operation(
            stack_frame,
            |a: i32, b: i32| a >> (b as u32 & 0b00011111u32),
            "ISHR",
        ),
        LSHR => binary_operation(
            stack_frame,
            |a: i64, b: i32| a >> (b as u32 & 0b00111111u32),
            "LSHR",
        ),
        IUSHR => binary_operation(
            stack_frame,
            |a: i32, b: i32| (a as u32 >> (b as u32 & 0b00011111u32)) as i32,
            "IUSHR",
        ),
        LUSHR => binary_operation(
            stack_frame,
            |a: i64, b: i32| (a as u64 >> (b as u32 & 0b00111111u32)) as i64,
            "LUSHR",
        ),
        IAND => binary_operation(stack_frame, |a: i32, b: i32| a & b, "IAND"),
        LAND => binary_operation(stack_frame, |a: i64, b: i64| a & b, "LAND"),
        IOR => binary_operation(stack_frame, |a: i32, b: i32| a | b, "IOR"),
        LOR => binary_operation(stack_frame, |a: i64, b: i64| a | b, "LOR"),
        IXOR => binary_operation(stack_frame, |a: i32, b: i32| a ^ b, "IXOR"),
        LXOR => binary_operation(stack_frame, |a: i64, b: i64| a ^ b, "LXOR"),
        IINC => increment(
            stack_frame,
            |sf| sf.extract_one_byte() as usize,
            |sf| sf.extract_one_byte() as i8 as i32,
            "IINC",
        ),
        _ => {
            return Err(crate::error::Error::new_execution(&format!(
                "Unknown math opcode: {}",
                code
            )));
        }
    }

    Ok(())
}

fn unary_operation<T: StackValue + Copy + Display>(
    stack_frame: &mut StackFrame,
    op: impl Fn(T) -> T,
    name: &str,
) {
    let value: T = stack_frame.pop();
    let result = op(value);
    stack_frame.push(result);

    stack_frame.incr_pc();
    trace!("{name} -> ({value})->{result}");
}

fn binary_operation<T1: StackValue + Copy + Display, T2: StackValue + Copy + Display>(
    stack_frame: &mut StackFrame,
    op: impl Fn(T1, T2) -> T1,
    name: &str,
) {
    let b: T2 = stack_frame.pop();
    let a: T1 = stack_frame.pop();
    let result = op(a, b);
    stack_frame.push(result);

    stack_frame.incr_pc();
    trace!("{name} -> ({a}, {b})->{result}");
}

pub(crate) fn increment<I, V>(
    stack_frame: &mut StackFrame,
    index_extractor: impl Fn(&mut StackFrame) -> I,
    val_extractor: impl Fn(&mut StackFrame) -> V,
    name: &str,
) where
    usize: From<I>,
    i32: From<V>,
{
    let index = index_extractor(stack_frame).into();
    let const_val = val_extractor(stack_frame).into();

    let current_val: i32 = stack_frame.get_local(index);
    let new_val = current_val.wrapping_add(const_val);
    stack_frame.set_local(index, new_val);

    stack_frame.incr_pc();
    trace!("{name} -> {current_val} + {const_val} = {new_val}");
}
