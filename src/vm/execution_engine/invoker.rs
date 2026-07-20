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
        let is_polymorphic = java_method
            .runtime_visible_annotations()
            .contains("Ljava/lang/invoke/MethodHandle$PolymorphicSignature;");
        let full_signature = if is_polymorphic {
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

        // Push a synthetic frame so the native method shows up on the thread's stack chain while it
        // runs (e.g. as `(Native Method)` in a stack trace captured from a Java callback it makes).
        // Polymorphic natives (MethodHandle/VarHandle intrinsics) are excluded: they manipulate the
        // caller's top frame directly and must remain on top of the current segment.
        let push_native_frame = !is_polymorphic;
        if push_native_frame {
            stack_frames.new_frame(java_method.new_native_stack_frame());
        }
        let result =
            invoke_native_method(&full_native_signature, method_args, is_static, stack_frames);
        if push_native_frame {
            // Plain pop (no ex_pc reset) leaves the caller frame exactly as the native call found
            // it, so pending-exception dispatch below is unaffected by the synthetic frame.
            stack_frames.propagate_exception();
        }
        let result = result?;

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
