use crate::execution_engine::opcode::*;
use crate::stack::stack_frame::StackFrame;
use tracing::trace;

pub(crate) fn process(code: u8, stack_frames: &mut Vec<StackFrame>) -> crate::error::Result<()> {
    let stack_frame = stack_frames.last_mut().unwrap();
    match code {
        LCMP => {
            let b: i64 = stack_frame.pop();
            let a: i64 = stack_frame.pop();

            if a > b {
                stack_frame.push(1);
            } else if a < b {
                stack_frame.push(-1);
            } else {
                stack_frame.push(0);
            }

            stack_frame.incr_pc();
            trace!("LCMP -> {a} ? {b}");
        }
        FCMPL => {
            let b: f32 = stack_frame.pop();
            let a: f32 = stack_frame.pop();

            let result = if a.is_nan() || b.is_nan() {
                -1
            } else if a < b {
                -1
            } else if a > b {
                1
            } else {
                0
            };

            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("FCMPL -> {a} ? {b}");
        }
        FCMPG => {
            let b: f32 = stack_frame.pop();
            let a: f32 = stack_frame.pop();

            let result = if a.is_nan() || b.is_nan() {
                1
            } else if a < b {
                -1
            } else if a > b {
                1
            } else {
                0
            };

            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("FCMPG -> {a} ? {b}");
        }
        DCMPL => {
            let b: f64 = stack_frame.pop();
            let a: f64 = stack_frame.pop();

            let result = if a.is_nan() || b.is_nan() {
                -1
            } else if a < b {
                -1
            } else if a > b {
                1
            } else {
                0
            };

            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("DCMPL -> {a} ? {b}");
        }
        DCMPG => {
            let b: f64 = stack_frame.pop();
            let a: f64 = stack_frame.pop();

            let result = if a.is_nan() || b.is_nan() {
                1
            } else if a < b {
                -1
            } else if a > b {
                1
            } else {
                0
            };

            stack_frame.push(result);

            stack_frame.incr_pc();
            trace!("DCMPG -> {a} ? {b}");
        }
        IFEQ => {
            let value: i32 = stack_frame.pop();
            let offset = stack_frame.get_two_bytes_ahead();
            stack_frame.advance_pc(if value == 0 { offset } else { 3 });
            trace!("IFEQ -> value={value}, offset={offset}");
        }
        IFNE => {
            let value: i32 = stack_frame.pop();
            let offset = stack_frame.get_two_bytes_ahead();
            stack_frame.advance_pc(if value != 0 { offset } else { 3 });
            trace!("IFNE -> value={value}, offset={offset}");
        }
        IFLT => {
            let value: i32 = stack_frame.pop();
            let offset = stack_frame.get_two_bytes_ahead();
            stack_frame.advance_pc(if value < 0 { offset } else { 3 });
            trace!("IFLT -> value={value}, offset={offset}");
        }
        IFGE => {
            let value: i32 = stack_frame.pop();
            let offset = stack_frame.get_two_bytes_ahead();
            stack_frame.advance_pc(if value >= 0 { offset } else { 3 });
            trace!("IFGE -> value={value}, offset={offset}");
        }
        IFGT => {
            let value: i32 = stack_frame.pop();
            let offset = stack_frame.get_two_bytes_ahead();
            stack_frame.advance_pc(if value > 0 { offset } else { 3 });
            trace!("IFGT -> value={value}, offset={offset}");
        }
        IFLE => {
            let value: i32 = stack_frame.pop();
            let offset = stack_frame.get_two_bytes_ahead();
            stack_frame.advance_pc(if value <= 0 { offset } else { 3 });
            trace!("IFLE -> value={value}, offset={offset}");
        }
        IF_ICMPEQ => {
            branch(|a: i32, b| a == b, stack_frame, "IF_ICMPEQ");
        }
        IF_ICMPNE => {
            branch(|a: i32, b| a != b, stack_frame, "IF_ICMPNE");
        }
        IF_ICMPLT => {
            branch(|a: i32, b| a < b, stack_frame, "IF_ICMPLT");
        }
        IF_ICMPGE => {
            branch(|a: i32, b| a >= b, stack_frame, "IF_ICMPGE");
        }
        IF_ICMPGT => {
            branch(|a: i32, b| a > b, stack_frame, "IF_ICMPGT");
        }
        IF_ICMPLE => {
            branch(|a: i32, b| a <= b, stack_frame, "IF_ICMPLE");
        }
        IF_ACMPEQ => {
            branch(|a: i32, b| a == b, stack_frame, "IF_ACMPEQ");
        }
        IF_ACMPNE => {
            branch(|a: i32, b| a != b, stack_frame, "IF_ACMPNE");
        }
        _ => {
            return Err(crate::error::Error::new_execution(&format!(
                "Unknown comparison opcode: {}",
                code
            )));
        }
    }

    Ok(())
}

fn branch<T>(op: impl Fn(T, T) -> bool, stack_frame: &mut StackFrame, op_code: &str)
where
    T: PartialOrd + TryFrom<i32> + Copy + std::fmt::Display,
{
    let value2 = T::try_from(stack_frame.pop()).unwrap_or_else(|_| panic!("Invalid conversion"));
    let value1 = T::try_from(stack_frame.pop()).unwrap_or_else(|_| panic!("Invalid conversion"));

    let offset = stack_frame.get_two_bytes_ahead();

    stack_frame.advance_pc(if op(value1, value2) { offset } else { 3 });

    trace!("{op_code} -> value1={value1}, value2={value2}, offset={offset}");
}
