use crate::vm::error::Result;
use crate::vm::exception::common::throw_exception_with_ref;
use crate::vm::execution_engine::common::last_frame_mut;
use crate::vm::execution_engine::system_native_table::invoke_native_method;
use crate::vm::jni::java_thread::JavaThread;
use crate::vm::method_area::java_method::JavaMethod;
use crate::vm::stack::stack_frame::StackFrames;
use jclassfile::methods::MethodFlags;
use std::sync::Arc;
use tracing::trace;

pub(crate) fn invoke(
    stack_frames: &mut StackFrames,
    full_signature: &str,
    method_args: &[i32],
    java_method: Arc<JavaMethod>,
    class_name: &str,
) -> Result<()> {
    if java_method.is_native() {
        let full_signature = if java_method
            .runtime_visible_annotations()
            .contains("Ljava/lang/invoke/MethodHandle$PolymorphicSignature;")
        {
            // we heed normalize the signature for PolymorphicSignature annotated methods
            full_signature.split(':').next().ok_or_else(|| {
                crate::vm::error::Error::new_execution(&format!(
                    "full_signature {full_signature} must contain ':'"
                ))
            })?
        } else {
            full_signature
        };

        let full_native_signature = format!("{class_name}:{full_signature}");
        trace!("<Calling native method> -> {full_native_signature} ({method_args:?})");

        let method_flags = MethodFlags::from_bits_truncate(java_method.access_flags() as u16);
        let is_static = method_flags.contains(MethodFlags::ACC_STATIC);
        let result =
            invoke_native_method(&full_native_signature, method_args, is_static, stack_frames)?;

        // JNI spec: if the native method set a pending exception, immediately throw it in Java.
        if let Some(throwable_ref) = JavaThread::take_pending_exception() {
            let (exception_name, handler_pc) =
                throw_exception_with_ref(throwable_ref, stack_frames)?;
            trace!("<JNI pending exception thrown> -> exception_name={exception_name}, handler_pc={handler_pc}");
            return Ok(());
        }

        for result_chunk in result.iter().rev() {
            last_frame_mut(stack_frames)?.push(*result_chunk)?;
        }
    } else {
        let mut next_frame = java_method.new_stack_frame()?;

        method_args
            .iter()
            .enumerate()
            .for_each(|(index, val)| next_frame.set_local(index, *val));

        stack_frames.new_frame(next_frame);
    }
    Ok(())
}
