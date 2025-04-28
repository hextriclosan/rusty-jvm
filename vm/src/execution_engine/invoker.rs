use crate::execution_engine::common::last_frame_mut;
use crate::execution_engine::system_native_table::invoke_native_method;
use crate::method_area::java_method::JavaMethod;
use crate::stack::stack_frame::StackFrames;
use std::sync::Arc;
use tracing::trace;

pub(crate) fn invoke(
    stack_frames: &mut StackFrames,
    full_signature: &str,
    method_args: &[i32],
    java_method: Arc<JavaMethod>,
    class_name: &str,
) -> crate::error::Result<()> {
    if java_method.is_native() {
        let full_signature = if java_method
            .runtime_visible_annotations()
            .contains("Ljava/lang/invoke/MethodHandle$PolymorphicSignature;")
        {
            // we heed normalize the signature for PolymorphicSignature annotated methods
            full_signature.split(':').next().ok_or_else(|| {
                crate::error::Error::new_execution(&format!(
                    "full_signature {full_signature} must contain ':'"
                ))
            })?
        } else {
            full_signature
        };

        let full_native_signature = format!("{class_name}:{full_signature}");
        trace!("<Calling native method> -> {full_native_signature} ({method_args:?})");

        let result = invoke_native_method(&full_native_signature, &method_args, stack_frames)?;
        for result_chunk in result.iter().rev() {
            last_frame_mut(stack_frames)?.push(*result_chunk)?;
        }
    } else {
        let mut next_frame = java_method.new_stack_frame()?;

        method_args
            .iter()
            .enumerate()
            .for_each(|(index, val)| next_frame.set_local(index, *val));

        stack_frames.push(next_frame);
    }
    Ok(())
}
