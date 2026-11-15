use crate::vm::error::{Error, Result};
use crate::vm::exception::common::throw_exception_with_ref;
use crate::vm::exception::helpers::throw_stack_overflow_error;
use crate::vm::execution_engine::common::{last_frame_mut, push_typed};
use crate::vm::execution_engine::thread_stack;
use crate::vm::helper::{arg_slots, polymorphic_arg_slots};
use crate::vm::jni::java_thread::JavaThread;
use crate::vm::method_area::java_method::JavaMethod;
use crate::vm::method_area::loaded_classes::CLASSES;
use crate::vm::monitor;
use crate::vm::stack::slot::Slot;
use crate::vm::stack::stack_frame::{StackFrame, StackFrames};
use crate::vm::system_native::dispatcher::invoke::invoke as invoke_native;
use crate::vm::system_native::dispatcher::polymorphic::invoke_polymorphic;
use jdescriptor::MethodDescriptor;
use std::sync::Arc;
use tracing::trace;

/// Where a call's `full_signature` came from, and so whether its descriptor describes the types
/// this particular call actually uses.
///
/// It always does for an ordinary method, whose declared descriptor is its real one. It is only in
/// question for a `@PolymorphicSignature` intrinsic, where the declared descriptor is the
/// placeholder `(Object[])Object` and the real types live at the call site instead.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum SignatureOrigin {
    /// Read from the constant pool entry that made the call. For a polymorphic method this is the
    /// descriptor the call was written with, so it names the real argument and return types
    /// (JVMS §5.4.3.3).
    CallSite,
    /// A resolved method's own declared signature, handed over by VM-side MethodHandle dispatch.
    /// For a polymorphic target that is the placeholder, which describes neither the real types nor
    /// the real widths — and cannot be told apart from a fitting descriptor by inspection, since a
    /// placeholder with a matching chunk count parses and counts just fine.
    ResolvedMethod,
}

pub(crate) fn invoke(
    stack_frames: &mut StackFrames,
    full_signature: &str,
    signature_origin: SignatureOrigin,
    method_args: &[i32],
    java_method: Arc<JavaMethod>,
    class_name: &str,
) -> Result<()> {
    if java_method.is_native() {
        let is_polymorphic = java_method.is_polymorphic_signature();
        let is_static = java_method.is_static();

        let (method_name, descriptor) = full_signature.split_once(':').ok_or_else(|| {
            Error::new_execution(&format!("full_signature {full_signature} must contain ':'"))
        })?;

        // Only a call-site descriptor describes a polymorphic call's real types; a resolved
        // method's own is the placeholder. Nothing distinguishes them by inspection, so provenance
        // has to be carried here rather than guessed at.
        let call_site_descriptor = match signature_origin {
            SignatureOrigin::CallSite => Some(descriptor),
            SignatureOrigin::ResolvedMethod => None,
        };

        let result = if is_polymorphic {
            // Polymorphic intrinsics (MethodHandle/VarHandle) manipulate the caller's top frame
            // directly, so they must remain on top of the current segment (no synthetic native
            // frame is pushed) and are dispatched by their normalized `class:method` key rather
            // than through the libffi-backed built-in native table.
            //
            // That rules out the synthetic frame an ordinary native gets, and leaves the arguments
            // — the receiving `MethodHandle`/`VarHandle` among them — in no frame at all: the
            // caller popped them, and the target's frame does not exist yet. The gap is not
            // instantaneous, since a direct handle runs class initialization and a bound handle
            // re-enters Java before installing that frame. A temporary root scope covers it without
            // disturbing which frame the wrappers see as the top one.
            let args = polymorphic_arg_slots(
                call_site_descriptor,
                !is_static,
                java_method.arg_ref_chunks()?,
                method_args,
            );
            let _temp_roots = JavaThread::hold_temp_roots(args);

            trace!("<Calling polymorphic native method> -> {class_name}:{method_name} ({method_args:?})");
            invoke_polymorphic(class_name, method_name, method_args)
        } else {
            let full_native_signature = format!("{class_name}:{full_signature}");
            trace!("<Calling native method> -> {full_native_signature} ({method_args:?})");

            // Push a synthetic frame so the native method shows up on the thread's stack chain while
            // it runs (e.g. as `(Native Method)` in a stack trace captured from a Java callback it
            // makes), carrying the arguments so its references stay reachable.
            //
            // Nothing else holds them: they were popped off the caller's operand stack before
            // dispatch and exist otherwise only in a Rust slice. Resolving the native's symbol
            // re-enters Java on its own, so there is a real window in which a receiver or reference
            // argument would sit in no frame at all.
            let args = arg_slots(java_method.arg_ref_chunks()?, method_args).map_err(|e| {
                Error::new_execution(&format!(
                    "invoking native {class_name}.{full_signature}: {e}"
                ))
            })?;
            let native_frame = java_method.new_native_stack_frame(args);
            if let Some(_raising) = thread_stack::exhausted(native_frame.stack_bytes()) {
                // No stack left for the native's frame (JVMS §2.5.2): the error goes where that
                // frame would have gone, the call site.
                throw_stack_overflow_error(stack_frames)?;
                return Ok(());
            }
            stack_frames.new_frame(native_frame);
            enter_method_monitor(last_frame_mut(stack_frames)?, &java_method)?;
            let result = invoke_native(&full_native_signature, method_args, is_static);
            // Plain pop (no ex_pc reset) leaves the caller frame exactly as the native call found
            // it, so pending-exception dispatch below is unaffected by the synthetic frame.
            stack_frames.propagate_exception();
            result
        };
        let result = result?;

        // JNI spec: if the native method set a pending exception, immediately throw it in Java.
        if let Some(throwable_ref) = JavaThread::take_pending_exception() {
            let (exception_name, handler_pc) =
                throw_exception_with_ref(throwable_ref, stack_frames)?;
            trace!("<JNI pending exception thrown> -> exception_name={exception_name}, handler_pc={handler_pc}");
            return Ok(());
        }

        if is_polymorphic {
            // Intrinsics that push onto the caller's frame themselves (the `MethodHandle.invoke*`
            // wrappers) hand back no chunks, and nothing is pushed for them here. The rest return
            // their result — `VarHandle.compareAndExchange` on a reference field returns a live
            // object — so it is tagged from the call site's return type. Using the declared
            // descriptor instead would be wrong in both directions: it calls every result a
            // reference, and would truncate a `long` result to a single chunk.
            match call_site_descriptor {
                Some(descriptor) if !result.is_empty() => {
                    let call_site: MethodDescriptor = descriptor.parse()?;
                    push_typed(
                        last_frame_mut(stack_frames)?,
                        call_site.return_type(),
                        &result,
                    )?;
                }
                // No descriptor that describes this call, so the chunks go back exactly as the
                // intrinsic produced them. The placeholder would claim `Object` — one chunk, a
                // reference — and silently truncate a `long` result to half of itself.
                _ => {
                    for result_chunk in result.iter().rev() {
                        last_frame_mut(stack_frames)?.push(*result_chunk)?;
                    }
                }
            }
        } else {
            // An ordinary native's declared return type is accurate, and is what says whether the
            // untagged chunks it handed back are a reference.
            push_typed(
                last_frame_mut(stack_frames)?,
                java_method.get_method_descriptor().return_type(),
                &result,
            )?;
        }
    } else {
        let mut next_frame = java_method.new_stack_frame()?;
        if let Some(_raising) = thread_stack::exhausted(next_frame.stack_bytes()) {
            // No stack left for the callee's frame (JVMS §2.5.2): the error goes where that frame
            // would have gone, the call site.
            throw_stack_overflow_error(stack_frames)?;
            return Ok(());
        }

        // Arguments were popped off the caller's operand stack as raw chunks, losing their tags on
        // the way. The method's cached mask restores them, exactly as it does for the native frame
        // above — written straight into the locals, since unlike a native frame there is no need to
        // materialise them first.
        let ref_chunks = java_method.arg_ref_chunks()?;
        if ref_chunks.len() != method_args.len() {
            return Err(Error::new_execution(&format!(
                "invoking {class_name}.{full_signature}: descriptor describes {} chunks but {} \
                 were supplied",
                ref_chunks.len(),
                method_args.len()
            )));
        }
        for (index, (&value, &is_ref)) in method_args.iter().zip(ref_chunks).enumerate() {
            if is_ref {
                next_frame.set_local(index, Slot::Ref(value));
            } else {
                next_frame.set_local(index, value);
            }
        }

        stack_frames.new_frame(next_frame);
        enter_method_monitor(last_frame_mut(stack_frames)?, &java_method)?;
    }
    Ok(())
}

/// Enters the monitor a `synchronized` method holds for as long as its frame, `frame`, is on the
/// stack (JVMS §2.11.10): the receiver's for an instance method, the declaring class's mirror for
/// a static one. The frame records it and releases it when it is popped, on return or on an
/// exception unwinding through the method. Any other method needs nothing.
///
/// Entering may block this OS thread until the monitor is free, so it comes last, once the
/// arguments are in the frame's locals and the frame is, where there is one, on the stack: a
/// stack trace taken meanwhile then shows the blocked method, as in HotSpot.
pub(crate) fn enter_method_monitor(
    frame: &mut StackFrame,
    java_method: &JavaMethod,
) -> Result<()> {
    if !java_method.is_synchronized() {
        return Ok(());
    }
    let obj_ref = if java_method.is_static() {
        CLASSES.get(java_method.class_name())?.mirror_clazz_ref()?
    } else {
        frame.get_local_raw(0)
    };
    frame.hold_monitor(obj_ref);
    monitor::enter(obj_ref);
    Ok(())
}
