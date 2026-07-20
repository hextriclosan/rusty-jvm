use crate::vm::error::{Error, Result};
use crate::vm::jni::java_thread::JavaThread;
use crate::vm::method_area::loaded_classes::CLASSES;
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
    pub fn get_caller_class_name() -> Result<String> {
        JavaThread::with_frames(|frames| {
            // `frames` yields every live frame newest → oldest, across all segments. Skip synthetic
            // native frames (e.g. `Reflection.getCallerClass` itself) so caller resolution walks
            // only interpreted frames, matching the behavior before native frames were recorded.
            let mut class_names = frames
                .filter(|frame| !frame.is_native())
                .map(|frame| frame.current_class_name());

            let nearest_class_name = class_names
                .next()
                .ok_or_else(|| Error::new_execution("stack frames is empty"))?;

            // Skip frames belonging to the nearest class, returning the first differing caller;
            // fall back to the nearest class when the whole stack is a single class.
            let class_name = class_names
                .find(|current_class_name| *current_class_name != nearest_class_name)
                .unwrap_or(nearest_class_name);

            Ok(class_name.to_string())
        })?
    }

    pub fn collect_stack_trace(
        throwable_name: &str,
        max_stack_size: usize,
    ) -> Result<Vec<StackElement>> {
        let mut stack_trace = JavaThread::with_frames(|frames| {
            // `with_frames` yields frames newest → oldest, but the throwable-skip `break` below
            // (and the trailing `reverse()`) require oldest → newest order: we must collect user
            // frames from the bottom and stop as soon as we reach the throwable's own constructor,
            // which excludes it together with its superclass `<init>` frames sitting above it.
            let newest_to_oldest: Vec<_> = frames.collect();
            let mut stack_trace = Vec::new();
            for frame in newest_to_oldest.into_iter().rev() {
                if stack_trace.len() >= max_stack_size {
                    break;
                }
                let class_name = frame.current_class_name().to_string();
                // If we reached the throwable class, stop collecting because we don't want to include it and its superclasses in the stack trace
                if class_name == throwable_name {
                    break;
                }

                let klass = CLASSES.get(&class_name)?;
                let method_name = frame.method_name();
                let method = klass.get_method(method_name)?;
                let method_raw = Arc::as_ptr(&method) as i64;

                let pc = frame.ex_pc();
                let line_numbers = frame.line_numbers();
                let instruction_line_num = Self::extract_line_number(line_numbers, pc);

                let class_ref = klass.mirror_clazz_ref()?;
                let native = method.is_native();

                stack_trace.push(StackElement::new(
                    class_ref,
                    method_raw,
                    instruction_line_num,
                    native,
                ));
            }

            Ok::<_, Error>(stack_trace)
        })??;

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
