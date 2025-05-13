use crate::error::Error;
use crate::heap::heap::{with_heap_read_lock, with_heap_write_lock};
use crate::helper::i64_to_vec;
use crate::stack::stack_frame::StackFrames;
use crate::stack::stack_frames_util::StackFramesUtil;

const MAX_DEPTH: usize = 32;

pub(crate) fn fill_in_stack_trace_wrp(
    args: &[i32],
    stack_frames: &StackFrames,
) -> crate::error::Result<Vec<i32>> {
    let throwable_ref = args[0];
    let _dummy = args[1];

    let throwable_ref = fill_in_stack_trace(throwable_ref, stack_frames)?;

    Ok(vec![throwable_ref])
}

const INTERPRETED_METHOD: i32 = 1;
pub const NATIVE_METHOD: i32 = 2;

/// This function is called from the Java code to fill in the stack trace of a throwable object.
/// Fields are set as follows:
///   - Throwable.backtrace: is Object[] type, internal object that holds the stack trace information
///       - backtrace[0]: Class[] holds Class reference
///       - backtrace[1]: long[] holds method reference (raw pointer address)
///       - backtrace[2]: int[] holds line number
///       - backtrace[3]: int[] holds method tag code: 1 - interpreted method, 2 - native method
///   - Throwable.depth: size of the stack trace
fn fill_in_stack_trace(
    throwable_ref: i32,
    stack_frames: &StackFrames,
) -> crate::error::Result<i32> {
    let throwable_name = with_heap_read_lock(|heap| heap.get_instance_name(throwable_ref))?;
    let stack_elements =
        StackFramesUtil::collect_stack_trace(stack_frames, &throwable_name, MAX_DEPTH)?;
    let depth = stack_elements.len() as i32;

    with_heap_write_lock(|heap| {
        let backtrace_ref = heap.create_array("[Ljava/lang/Object;", 4)?;

        let class_array_ref = heap.create_array("[Ljava/lang/Class;", depth)?;
        let method_array_ref = heap.create_array("[J", depth)?;
        let line_array_ref = heap.create_array("[I", depth)?;
        let tag_array_ref = heap.create_array("[I", depth)?;

        heap.set_array_value(backtrace_ref, 0, vec![class_array_ref])?;
        heap.set_array_value(backtrace_ref, 1, vec![method_array_ref])?;
        heap.set_array_value(backtrace_ref, 2, vec![line_array_ref])?;
        heap.set_array_value(backtrace_ref, 3, vec![tag_array_ref])?;

        for (index, stack_element) in stack_elements.iter().enumerate() {
            heap.set_array_value(
                class_array_ref,
                index as i32,
                vec![stack_element.class_ref()],
            )?;
            heap.set_array_value(
                method_array_ref,
                index as i32,
                i64_to_vec(stack_element.method_ref()),
            )?;
            heap.set_array_value(
                line_array_ref,
                index as i32,
                vec![stack_element.line_number() as i32],
            )?;
            heap.set_array_value(
                tag_array_ref,
                index as i32,
                vec![if stack_element.is_native() {
                    NATIVE_METHOD
                } else {
                    INTERPRETED_METHOD
                }],
            )?;
        }

        heap.set_object_field_value(
            throwable_ref,
            &throwable_name,
            "backtrace",
            vec![backtrace_ref],
        )?;
        heap.set_object_field_value(throwable_ref, &throwable_name, "depth", vec![depth])?;
        Ok::<i32, Error>(throwable_ref)
    })
}
