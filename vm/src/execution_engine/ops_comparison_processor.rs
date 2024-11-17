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
            branch1arg(|a| a == 0, stack_frame, "IFEQ");
        }
        IFNE => {
            branch1arg(|a| a != 0, stack_frame, "IFNE");
        }
        IFLT => {
            branch1arg(|a| a < 0, stack_frame, "IFLT");
        }
        IFGE => {
            branch1arg(|a| a >= 0, stack_frame, "IFGE");
        }
        IFGT => {
            branch1arg(|a| a > 0, stack_frame, "IFGT");
        }
        IFLE => {
            branch1arg(|a| a <= 0, stack_frame, "IFLE");
        }
        IF_ICMPEQ => {
            branch2args(|a, b| a == b, stack_frame, "IF_ICMPEQ");
        }
        IF_ICMPNE => {
            branch2args(|a, b| a != b, stack_frame, "IF_ICMPNE");
        }
        IF_ICMPLT => {
            branch2args(|a, b| a < b, stack_frame, "IF_ICMPLT");
        }
        IF_ICMPGE => {
            branch2args(|a, b| a >= b, stack_frame, "IF_ICMPGE");
        }
        IF_ICMPGT => {
            branch2args(|a, b| a > b, stack_frame, "IF_ICMPGT");
        }
        IF_ICMPLE => {
            branch2args(|a, b| a <= b, stack_frame, "IF_ICMPLE");
        }
        IF_ACMPEQ => {
            branch2args(|a, b| a == b, stack_frame, "IF_ACMPEQ");
        }
        IF_ACMPNE => {
            branch2args(|a, b| a != b, stack_frame, "IF_ACMPNE");
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

fn branch2args(op: impl Fn(i32, i32) -> bool, stack_frame: &mut StackFrame, op_code: &str) {
    let value2 = stack_frame.pop();
    let value1 = stack_frame.pop();
    let offset = stack_frame.get_two_bytes_ahead();

    stack_frame.advance_pc(if op(value1, value2) { offset } else { 3 });
    trace!("{op_code} -> value1={value1}, value2={value2}, offset={offset}");
}

fn branch1arg(op: impl Fn(i32) -> bool, stack_frame: &mut StackFrame, op_code: &str) {
    let value = stack_frame.pop();
    let offset = stack_frame.get_two_bytes_ahead();

    stack_frame.advance_pc(if op(value) { offset } else { 3 });
    trace!("{op_code} -> value={value}, offset={offset}");
}
