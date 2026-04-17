//! Purpose: Handles the invocation setup for both standard and native Java methods.
//!
//! Implementation Details:
//! If the target method is standard bytecode, a new stack frame is prepared and pushed
//! onto the thread's execution stack. If the target method is native, it delegates
//! the call to the `system_native_table` asynchronously, awaiting the native implementation's
//! result and pushing it onto the caller's operand stack.

use crate::vm::error::Result;
use crate::vm::execution_engine::common::last_frame_mut;
use crate::vm::execution_engine::system_native_table::invoke_native_method;
use crate::vm::method_area::java_method::JavaMethod;
use crate::vm::stack::stack_frame::StackFrames;
use jclassfile::methods::MethodFlags;
use std::sync::Arc;
use tracing::trace;

/// Invokes a resolved Java method.
pub(crate) async fn invoke(
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
            // Normalize the signature for PolymorphicSignature annotated methods.
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
        
        // Native methods are awaited so Tokio tasks can yield during operations like Thread.sleep.
        let result =
            invoke_native_method(&full_native_signature, method_args, is_static, stack_frames).await?;
            
        for result_chunk in result.iter().rev() {
            last_frame_mut(stack_frames)?.push(*result_chunk)?;
        }
    } else {
        // Standard bytecode method: set up a new stack frame.
        let mut next_frame = java_method.new_stack_frame()?;

        method_args
            .iter()
            .enumerate()
            .for_each(|(index, val)| next_frame.set_local(index, *val));

        stack_frames.new_frame(next_frame);
    }
    
    Ok(())
}