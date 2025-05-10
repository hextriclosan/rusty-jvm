use crate::error::Error;
use crate::execution_engine::common::last_frame_mut;
use crate::execution_engine::executor::Executor;
use crate::heap::heap::with_heap_read_lock;
use crate::stack::stack_frame::StackFrames;
use crate::stack::stack_value::StackValueKind;
use tracing::trace;

pub fn construct_exception_and_throw(
    exception_class_name: &str,
    constructor_signature: &str,
    args: &[StackValueKind],
    stack_frames: &mut StackFrames,
) -> crate::error::Result<()> {
    let exception_instance_ref = Executor::invoke_args_constructor(
        exception_class_name,
        constructor_signature,
        &args,
        Some(&format!(
            "construction of {exception_class_name}:{constructor_signature}({args:?}) instance"
        )),
    )?;

    let (exception_name, found_exception_handler) =
        throw_exception_with_ref(exception_instance_ref, stack_frames)?;

    trace!("<THROWING> -> exception_name={exception_name}, found_exception_handler={found_exception_handler}");
    Ok(())
}

pub fn throw_exception_with_ref(
    throwable_ref: i32,
    stack_frames: &mut StackFrames,
) -> crate::error::Result<(String, i16)> {
    let exception_name = with_heap_read_lock(|heap| heap.get_instance_name(throwable_ref))?;
    trace!("<THROWING> -> about to throw: throwable_ref={throwable_ref}, exception_name={exception_name}");
    let found_exception_handler = unwind_stack(stack_frames, &exception_name)?;

    let stack_frame = last_frame_mut(stack_frames)?;
    stack_frame.set_pc(found_exception_handler);
    stack_frame.clear_stack(); // according to JVM spec
    stack_frame.push(throwable_ref)?;

    Ok((exception_name, found_exception_handler))
}

fn unwind_stack(
    stack_frames: &mut StackFrames,
    exception_name: &str,
) -> crate::error::Result<i16> {
    while !stack_frames.is_empty() {
        let stack_frame = last_frame_mut(stack_frames)?;
        let exception_table = stack_frame.exception_table();
        let pc = stack_frame.ex_pc() as u16;
        match exception_table.find_exception_handler(
            exception_name,
            pc,
            stack_frame.method_name(),
        )? {
            Some(exception_handler) => {
                return Ok(exception_handler as i16);
            }
            None => {
                stack_frames.propagate_exception();
            }
        }
    }

    Err(Error::new_execution(&format!(
        "Exception {exception_name} not handled"
    )))
}
