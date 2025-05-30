use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::common::last_frame_mut;
use crate::vm::execution_engine::opcode::*;
use crate::vm::method_area::method_area::{with_method_area, MethodArea};
use crate::vm::stack::stack_frame::{StackFrame, StackFrames};
use crate::vm::stack::stack_value::StackValue;
use std::fmt::Display;
use tracing::trace;

pub(crate) fn process(
    code: u8,
    current_class_name: &str,
    stack_frames: &mut StackFrames,
) -> Result<()> {
    let stack_frame = last_frame_mut(stack_frames)?;
    match code {
        NOP => {
            stack_frame.incr_pc();
            trace!("NOP");
            Ok(())
        }
        ACONST_NULL => push_constant(stack_frame, 0i32, "ACONST_NULL"),
        ICONST_M1 => push_constant(stack_frame, -1i32, "ICONST_M1"),
        ICONST_0 => push_constant(stack_frame, 0i32, "ICONST_0"),
        ICONST_1 => push_constant(stack_frame, 1i32, "ICONST_1"),
        ICONST_2 => push_constant(stack_frame, 2i32, "ICONST_2"),
        ICONST_3 => push_constant(stack_frame, 3i32, "ICONST_3"),
        ICONST_4 => push_constant(stack_frame, 4i32, "ICONST_4"),
        ICONST_5 => push_constant(stack_frame, 5i32, "ICONST_5"),
        LCONST_0 => push_constant(stack_frame, 0i64, "LCONST_0"),
        LCONST_1 => push_constant(stack_frame, 1i64, "LCONST_1"),
        FCONST_0 => push_constant(stack_frame, 0.0f32, "FCONST_0"),
        FCONST_1 => push_constant(stack_frame, 1.0f32, "FCONST_1"),
        FCONST_2 => push_constant(stack_frame, 2.0f32, "FCONST_2"),
        DCONST_0 => push_constant(stack_frame, 0.0f64, "DCONST_0"),
        DCONST_1 => push_constant(stack_frame, 1.0f64, "DCONST_1"),
        BIPUSH => handle_push(
            stack_frame,
            |stack_frame| stack_frame.extract_one_byte() as i8 as i32,
            "BIPUSH",
        ),
        SIPUSH => handle_push(
            stack_frame,
            |stack_frame| stack_frame.extract_two_bytes() as i32,
            "SIPUSH",
        ),
        LDC | LDC_W | LDC2_W => handle_ldc_cases(stack_frame, code, &current_class_name),
        _ => Err(Error::new_execution(&format!(
            "Unknown constant opcode: {}",
            code
        ))),
    }
}

fn push_constant<T: StackValue>(stack_frame: &mut StackFrame, value: T, name: &str) -> Result<()> {
    stack_frame.push(value)?;
    stack_frame.incr_pc();
    trace!("{name}");
    Ok(())
}

fn handle_push<T: StackValue + Display + Copy>(
    stack_frame: &mut StackFrame,
    extractor: impl FnOnce(&mut StackFrame) -> T,
    name: &str,
) -> Result<()> {
    let value = extractor(stack_frame);
    stack_frame.push(value)?;
    stack_frame.incr_pc();
    trace!("{name} -> value={value}");
    Ok(())
}

fn handle_ldc_cases(
    stack_frame: &mut StackFrame,
    code: u8,
    current_class_name: &str,
) -> Result<()> {
    match code {
        LDC | LDC_W => {
            let extract_func = if code == LDC {
                |sf: &mut StackFrame| sf.extract_one_byte() as u16
            } else {
                |sf: &mut StackFrame| sf.extract_two_bytes() as u16
            };

            handle_ldc_generic(
                stack_frame,
                extract_func,
                |method_area, class_name, index| method_area.resolve_ldc(class_name, index),
                current_class_name,
                if code == LDC { "LDC" } else { "LDC_W" },
            )
        }
        LDC2_W => handle_ldc_generic(
            stack_frame,
            |sf| sf.extract_two_bytes() as u16,
            |method_area, class_name, index| method_area.resolve_ldc2_w(class_name, index),
            current_class_name,
            "LDC2_W",
        ),
        _ => unreachable!(),
    }
}

fn handle_ldc_generic<T: StackValue + Display + Copy>(
    stack_frame: &mut StackFrame,
    extractor: impl FnOnce(&mut StackFrame) -> u16,
    resolver: impl FnOnce(&MethodArea, &str, u16) -> Result<T>,
    current_class_name: &str,
    name: &str,
) -> Result<()> {
    let cpoolindex = extractor(stack_frame);

    let value =
        with_method_area(|method_area| resolver(method_area, current_class_name, cpoolindex))?;

    stack_frame.push(value)?;

    stack_frame.incr_pc();
    trace!("{name} -> cpoolindex={cpoolindex}, value={value}");

    Ok(())
}
