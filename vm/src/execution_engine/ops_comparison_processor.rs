use crate::execution_engine::common::last_frame_mut;
use crate::execution_engine::opcode::*;
use crate::stack::stack_frame::{StackFrame, StackFrames};
use crate::stack::stack_value::StackValue;
use tracing::trace;

pub(crate) fn process(code: u8, stack_frames: &mut StackFrames) -> crate::error::Result<()> {
    let stack_frame = last_frame_mut(stack_frames)?;
    match code {
        LCMP => {
            perform_comparison(
                stack_frame,
                |_: i64, _| None, // No special handling for LCMP
                |a, b| Some(a.cmp(&b)),
                0, // No NaN handling for integers
                "LCMP",
            )?;
        }
        FCMPL | FCMPG => {
            let nan_result = if code == FCMPL { -1 } else { 1 };
            perform_comparison(
                stack_frame,
                |a: f32, b| handle_nan(a, b, nan_result),
                |a, b| a.partial_cmp(&b),
                nan_result,
                if code == FCMPL { "FCMPL" } else { "FCMPG" },
            )?;
        }
        DCMPL | DCMPG => {
            let nan_result = if code == DCMPL { -1 } else { 1 };
            perform_comparison(
                stack_frame,
                |a: f64, b| handle_nan(a, b, nan_result),
                |a, b| a.partial_cmp(&b),
                nan_result,
                if code == DCMPL { "DCMPL" } else { "DCMPG" },
            )?;
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

pub(crate) fn branch1arg(op: impl Fn(i32) -> bool, stack_frame: &mut StackFrame, op_code: &str) {
    let value = stack_frame.pop();
    let offset = stack_frame.get_two_bytes_ahead();

    stack_frame.advance_pc(if op(value) { offset } else { 3 });
    trace!("{op_code} -> value={value}, offset={offset}");
}

fn perform_comparison<T: StackValue + std::fmt::Display + Copy, F, G>(
    stack_frame: &mut StackFrame,
    handle_special: F,
    compare: G,
    nan_result: i32,
    name: &str,
) -> crate::error::Result<()>
where
    F: Fn(T, T) -> Option<i32>,
    G: Fn(T, T) -> Option<std::cmp::Ordering>,
{
    let b: T = stack_frame.pop();
    let a: T = stack_frame.pop();

    let result = handle_special(a, b)
        .or_else(|| compare(a, b).map(|ord| ordering_to_i32(Some(ord))))
        .unwrap_or(nan_result);

    stack_frame.push(result)?;
    stack_frame.incr_pc();
    trace!("{name} -> {a} ? {b}");

    Ok(())
}

fn ordering_to_i32(ordering: Option<std::cmp::Ordering>) -> i32 {
    match ordering {
        Some(std::cmp::Ordering::Less) => -1,
        Some(std::cmp::Ordering::Equal) => 0,
        Some(std::cmp::Ordering::Greater) => 1,
        None => unreachable!(),
    }
}

fn handle_nan<T: num_traits::float::Float>(a: T, b: T, nan_result: i32) -> Option<i32> {
    if a.is_nan() || b.is_nan() {
        Some(nan_result)
    } else {
        None
    }
}
