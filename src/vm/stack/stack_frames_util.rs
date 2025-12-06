use crate::vm::error::{Error, Result};
use crate::vm::method_area::loaded_classes::CLASSES;
use crate::vm::method_area::method_area::with_method_area;
use crate::vm::stack::stack_frame::StackFrames;
use derive_new::new;
use getset::CopyGetters;
use std::collections::BTreeMap;
use std::sync::Arc;

pub struct StackFramesUtil {}

#[derive(Debug, new, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct StackElement {
    class_ref: i32,
    method_ref: i64,
    line_number: u16,
    is_native: bool,
}

impl StackFramesUtil {
    pub fn get_caller_class_name(stack_frames: &StackFrames) -> Result<String> {
        let nearest_class_name = stack_frames
            .last()
            .ok_or_else(|| Error::new_execution("stack frames is empty"))?
            .current_class_name();

        let class_name = stack_frames
            .iter()
            .rev()
            .skip(1)
            .map(|frame| frame.current_class_name())
            .find_map(|current_class_name| {
                if current_class_name != nearest_class_name {
                    Some(current_class_name)
                } else {
                    None // Continue searching
                }
            })
            .unwrap_or(nearest_class_name);

        Ok(class_name.to_string())
    }

    pub fn collect_stack_trace(
        stack_frames: &StackFrames,
        throwable_name: &str,
        max_stack_size: usize,
    ) -> Result<Vec<StackElement>> {
        let mut stack_trace = Vec::new();
        for frame in stack_frames.iter() {
            if stack_trace.len() >= max_stack_size {
                break;
            }
            let class_name = frame.current_class_name().to_string();
            // If we reached the throwable class, stop collecting because we don't want to include it and its superclasses in the stack trace
            if class_name == throwable_name {
                break;
            }
            let class_ref = with_method_area(|area| area.load_reflection_class(&class_name))?;

            let jc = CLASSES.get(&class_name)?; // fixme!!! get Klass by class_ref?
            let method_name = frame.method_name();
            let method = jc.get_method(method_name)?;
            let method_raw = Arc::as_ptr(&method) as i64;

            let pc = frame.ex_pc();
            let line_numbers = frame.line_numbers();
            let instruction_line_num = Self::extract_line_number(line_numbers, pc);

            let native = method.is_native();

            stack_trace.push(StackElement::new(
                class_ref,
                method_raw,
                instruction_line_num,
                native,
            ));
        }

        stack_trace.reverse();
        Ok(stack_trace)
    }

    pub fn extract_line_number(line_numbers: &BTreeMap<u16, u16>, pc: usize) -> u16 {
        let instruction_line_num = line_numbers
            .range(..=&(pc as u16))
            .next_back()
            .map(|(_pc, line)| *line)
            .unwrap_or_default();
        instruction_line_num
    }
}
