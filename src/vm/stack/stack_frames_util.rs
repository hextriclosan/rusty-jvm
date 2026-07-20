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
            // `with_frames` yields frames newest → oldest.
            let newest_to_oldest: Vec<_> = frames.collect();

            // Drop the `fillInStackTrace` machinery sitting at the top of the stack: the synthetic
            // native `fillInStackTrace(int)` frame plus the `Throwable.fillInStackTrace()` wrapper
            // that called it. They are always the newest frames while this native runs, and the JVM
            // never records them — the trace must start at the throw / `fillInStackTrace()` call
            // site. This matters when `fillInStackTrace()` is invoked directly on an already
            // constructed throwable: there is then no `<init>` frame for the `throwable_name` skip
            // below to stop at, so without this the synthetic frames would leak in as the top
            // elements. In the normal construction path these same frames sit above the `<init>`
            // frames and are already excluded by the `throwable_name` skip, so this is a no-op there.
            // `method_name()` carries the `name:descriptor` form, so match on the bare name.
            let after_fill = newest_to_oldest
                .iter()
                .position(|frame| {
                    let name = frame.method_name().split(':').next();
                    !(frame.current_class_name() == "java/lang/Throwable"
                        && name == Some("fillInStackTrace"))
                })
                .unwrap_or(newest_to_oldest.len());

            // Reverse to oldest → newest so the throwable-skip stops at the throwable's own
            // constructor, excluding it together with its superclass `<init>` frames sitting above.
            let eligible: Vec<_> = newest_to_oldest[after_fill..]
                .iter()
                .rev()
                .take_while(|frame| frame.current_class_name() != throwable_name)
                .collect();

            // When the stack is deeper than `max_stack_size`, the JVM keeps the *newest* frames
            // (the throw site and its nearest callers) and truncates the oldest, so take the tail
            // of this oldest → newest slice. Resolving elements only for the retained frames avoids
            // wasted lookups — and error propagation — for frames that would be dropped.
            let start = eligible.len().saturating_sub(max_stack_size);
            let mut stack_trace = Vec::with_capacity(eligible.len() - start);
            for frame in &eligible[start..] {
                let klass = CLASSES.get(frame.current_class_name())?;
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
