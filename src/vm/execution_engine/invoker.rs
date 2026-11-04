use crate::vm::error::{Error, Result};
use crate::vm::exception::common::throw_exception_with_ref;
use crate::vm::execution_engine::common::{last_frame_mut, push_slots};
use crate::vm::execution_engine::executor::Executor;
use crate::vm::helper::{get_length, is_reference, ref_arg_chunks, validate_arg_slots};
use crate::vm::jni::java_thread::JavaThread;
use crate::vm::method_area::java_method::JavaMethod;
use crate::vm::method_area::loaded_classes::CLASSES;
use crate::vm::stack::rooted_slots::RootedSlots;
use crate::vm::stack::slot::Slot;
use crate::vm::stack::stack_frame::StackFrames;
use crate::vm::system_native::dispatcher::invoke::invoke as invoke_native;
use crate::vm::system_native::dispatcher::polymorphic::invoke_polymorphic;
use jdescriptor::{MethodDescriptor, TypeDescriptor};
use std::sync::Arc;
use tracing::trace;

/// Only a call site or an adapter's actual MethodType can type a polymorphic call.
/// A resolved method's erased declaration is deliberately not representable here.
#[derive(Clone, Copy, Debug)]
pub(crate) enum SignatureOrigin<'a> {
    CallSite,
    EffectiveMethodType(&'a MethodDescriptor),
}

pub(crate) fn invoke(
    stack_frames: &mut StackFrames,
    full_signature: &str,
    signature_origin: SignatureOrigin<'_>,
    method_args: &[Slot],
    java_method: Arc<JavaMethod>,
    class_name: &str,
) -> Result<()> {
    let _arguments = JavaThread::hold_temp_roots(method_args.to_vec());
    let (method_name, descriptor) = full_signature.split_once(':').ok_or_else(|| {
        Error::new_execution(&format!("full_signature {full_signature} must contain ':'"))
    })?;
    let call_site;
    let effective_type = match signature_origin {
        SignatureOrigin::CallSite => {
            call_site = descriptor.parse::<MethodDescriptor>()?;
            &call_site
        }
        SignatureOrigin::EffectiveMethodType(method_type) => method_type,
    };
    let is_static = java_method.is_static();
    let is_polymorphic = java_method.is_polymorphic_signature();
    let argument_type = if is_polymorphic {
        effective_type
    } else {
        java_method.get_method_descriptor()
    };
    validate_arg_slots(&ref_arg_chunks(argument_type, !is_static)?, method_args)?;

    let mut native_monitor = if java_method.is_native() && java_method.is_synchronized() {
        Some(SynchronizedNativeMonitor::enter(&java_method, method_args)?)
    } else {
        None
    };

    if java_method.is_native() {
        // The caller owns this slot before native resolution, Java callbacks or frame teardown.
        let mut destination = RootedSlots::new();
        let result = if is_polymorphic {
            // These intrinsics access the caller's segment directly; no synthetic frame.
            invoke_polymorphic(class_name, method_name, method_args, &mut destination)
        } else {
            let full_native_signature = format!("{class_name}:{full_signature}");
            trace!("<Calling native method> -> {full_native_signature} ({method_args:?})");
            stack_frames.new_frame(java_method.new_native_stack_frame(method_args.to_vec()));
            let result = invoke_native(
                &full_native_signature,
                method_args,
                is_static,
                &mut destination,
            );
            // Extract into an owned slot while the native frame is still published.
            let pending = pop_native_frame(stack_frames);
            if let Some(throwable) = pending {
                if let Some(monitor) = &mut native_monitor {
                    let _ = monitor.release();
                }
                throw_exception_with_ref(throwable.slot(), stack_frames)?;
                return Ok(());
            }
            result
        };
        if let Some(monitor) = &mut native_monitor {
            if !monitor.release() {
                throw_illegal_monitor_state_exception(stack_frames)?;
                return Ok(());
            }
        }
        result?;

        if let Some(throwable) = JavaThread::take_pending_exception_rooted() {
            throw_exception_with_ref(throwable.slot(), stack_frames)?;
            return Ok(());
        }
        // MethodHandle wrappers install a Java frame and have no immediate result.
        // Other producers return their original slots, even for VM-side polymorphic calls.
        if !is_polymorphic || !destination.is_empty() {
            publish_result(stack_frames, &destination, argument_type.return_type())?;
        }
    } else {
        let mut next_frame = java_method.new_stack_frame()?;
        for (index, &slot) in method_args.iter().enumerate() {
            next_frame.set_local(index, slot);
        }
        stack_frames.new_frame(next_frame);
    }
    Ok(())
}

struct SynchronizedNativeMonitor {
    reference: Option<i32>,
}

impl SynchronizedNativeMonitor {
    fn enter(method: &JavaMethod, args: &[Slot]) -> Result<Self> {
        let reference = if method.is_static() {
            CLASSES.class_mirror_ref(method.class_name())?
        } else {
            let receiver = args.first().ok_or_else(|| {
                Error::new_execution("Synchronized native method has no receiver")
            })?;
            if !receiver.is_ref() {
                return Err(Error::new_execution(
                    "Synchronized native method receiver is not a reference",
                ));
            }
            receiver.value()
        };
        if reference == 0 {
            return Err(Error::new_execution(
                "Synchronized native method has a null monitor reference",
            ));
        }
        crate::vm::monitor::enter(reference);
        Ok(Self {
            reference: Some(reference),
        })
    }

    fn release(&mut self) -> bool {
        if let Some(reference) = self.reference.take() {
            crate::vm::monitor::exit_synchronized_method(reference)
        } else {
            true
        }
    }
}

impl Drop for SynchronizedNativeMonitor {
    fn drop(&mut self) {
        if let Some(reference) = self.reference.take() {
            crate::vm::monitor::exit_synchronized_method(reference);
        }
    }
}

fn throw_illegal_monitor_state_exception(stack_frames: &mut StackFrames) -> Result<()> {
    let throwable =
        Executor::invoke_default_constructor("java/lang/IllegalMonitorStateException")?;
    throw_exception_with_ref(Slot::Ref(throwable[0].value()), stack_frames)?;
    Ok(())
}

/// This ordering is shared by every ordinary native call, on success and failure.
fn pop_native_frame(stack_frames: &mut StackFrames) -> Option<crate::vm::error::RootedThrowable> {
    let pending = JavaThread::take_pending_exception_rooted();
    crate::vm::safepoint::event(crate::vm::safepoint::Event::NativeFramePop);
    stack_frames
        .propagate_exception()
        .expect("native frame monitor release failed");
    pending
}

fn publish_result(
    stack_frames: &mut StackFrames,
    result: &RootedSlots,
    result_type: &TypeDescriptor,
) -> Result<()> {
    let width = if matches!(result_type, TypeDescriptor::Void) {
        0
    } else {
        get_length(result_type)? as usize
    };
    validate_arg_slots(&vec![is_reference(result_type); width], result)?;
    push_slots(last_frame_mut(stack_frames)?, result)
}

#[cfg(test)]
mod ref_transfer_tests {
    use super::*;
    use crate::vm::execution_engine::common::typed_slots;
    use crate::vm::execution_engine::opcode::{ARETURN, LRETURN};
    use crate::vm::execution_engine::ops_control_processor;
    use crate::vm::heap::heap::HEAP;
    use crate::vm::stack::stack_frame::{observe_frame_pops, StackFrame};
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn ref_transfer_polymorphic_reference_survives_source_frame_pop() {
        let reference = HEAP
            .create_array(
                "[I",
                0,
                &mut crate::vm::stack::rooted_slots::RootedSlots::new(),
            )
            .unwrap();
        let mut source = StackFrame::for_test(0, 1);
        source.push(Slot::Ref(reference)).unwrap();
        let mut sources = StackFrames::new(vec![source]);
        let mut result = RootedSlots::new();
        let observed = Rc::new(Cell::new(false));
        let seen = Rc::clone(&observed);

        observe_frame_pops(
            move |frames| {
                assert!(frames.is_empty());
                assert!(JavaThread::temp_root_refs().contains(&reference));
                seen.set(true);
            },
            || ops_control_processor::process(ARETURN, &mut sources, &mut result).unwrap(),
        );
        let mut caller = StackFrames::new(vec![StackFrame::for_test(0, 1)]);
        publish_result(
            &mut caller,
            &result,
            &TypeDescriptor::Object("java/lang/Object".into()),
        )
        .unwrap();
        drop(result);

        assert!(observed.get());
        assert_eq!(
            caller.last().unwrap().ref_slots().collect::<Vec<_>>(),
            vec![reference]
        );
        assert!(JavaThread::temp_root_refs().is_empty());
    }

    #[test]
    fn ref_transfer_native_producer_tags_reference_before_publication() {
        let reference = HEAP
            .create_array(
                "[I",
                0,
                &mut crate::vm::stack::rooted_slots::RootedSlots::new(),
            )
            .unwrap();
        let descriptor = TypeDescriptor::Array(Box::new(TypeDescriptor::Integer), 1);
        let mut destination = RootedSlots::new();
        let mut caller = StackFrames::new(vec![StackFrame::for_test(0, 1)]);

        destination.replace(typed_slots(&descriptor, &[reference]).unwrap());
        assert_eq!(JavaThread::temp_root_refs(), vec![reference]);
        publish_result(&mut caller, &destination, &descriptor).unwrap();

        assert_eq!(
            caller.last().unwrap().ref_slots().collect::<Vec<_>>(),
            vec![reference]
        );
    }

    #[test]
    fn ref_transfer_polymorphic_wide_result_preserves_both_nonroot_halves() {
        let value = 0x1234_5678_9abc_def0i64;
        let mut source = StackFrame::for_test(0, 2);
        source.push(value).unwrap();
        let mut sources = StackFrames::new(vec![source]);
        let mut result = RootedSlots::new();
        let mut caller = StackFrames::new(vec![StackFrame::for_test(0, 2)]);

        ops_control_processor::process(LRETURN, &mut sources, &mut result).unwrap();
        publish_result(&mut caller, &result, &TypeDescriptor::Long).unwrap();

        assert!(JavaThread::temp_root_refs().is_empty());
        assert_eq!(caller.last_mut().unwrap().pop::<i64>(), value);
        assert!(publish_result(
            &mut caller,
            &result,
            &TypeDescriptor::Object("java/lang/Object".into())
        )
        .is_err());
    }

    #[test]
    fn ref_transfer_pending_exception_is_rooted_before_native_frame_pop() {
        let throwable = HEAP
            .create_array(
                "[I",
                0,
                &mut crate::vm::stack::rooted_slots::RootedSlots::new(),
            )
            .unwrap();
        JavaThread::try_set_pending_exception(throwable).unwrap();
        let mut frames = StackFrames::new(vec![StackFrame::new_native(
            Arc::new("native:()V".into()),
            Arc::new("Test".into()),
            vec![],
        )]);
        let observed = Rc::new(Cell::new(false));
        let seen = Rc::clone(&observed);

        let pending = observe_frame_pops(
            move |frames| {
                assert!(frames.is_empty());
                assert_eq!(JavaThread::get_pending_exception(), None);
                assert!(JavaThread::temp_root_refs().contains(&throwable));
                seen.set(true);
            },
            || pop_native_frame(&mut frames).unwrap(),
        );

        assert!(observed.get());
        assert_eq!(pending.slot(), Slot::Ref(throwable));
        drop(pending);
        assert!(JavaThread::temp_root_refs().is_empty());
    }
}
