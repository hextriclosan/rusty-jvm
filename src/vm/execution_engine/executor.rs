use crate::vm::error::Result;
use crate::vm::execution_engine::engine::Engine;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::validate_arg_slots;
use crate::vm::jni::java_thread::JavaThread;
use crate::vm::method_area::java_class::JavaClass;
use crate::vm::method_area::java_method::JavaMethod;
use crate::vm::method_area::loaded_classes::CLASSES;
use crate::vm::method_area::method_area::with_method_area;
use crate::vm::stack::rooted_slots::RootedSlots;
#[cfg(test)]
use crate::vm::stack::slot::Slot;
use crate::vm::stack::stack_frame::StackFrame;
use crate::vm::stack::stack_value::StackValueKind;
use crate::vm::threads;
use std::sync::Arc;

pub struct Executor {}

impl Executor {
    const INIT_METHOD: &'static str = "<init>:()V";

    pub fn invoke_static_method(
        class_name: &str,
        method_name: &str,
        args: &[StackValueKind],
    ) -> Result<RootedSlots> {
        Self::exec(class_name, method_name, args, None)
    }

    pub fn invoke_static_method_jc(
        java_class: &Arc<JavaClass>,
        method_name: &str,
        args: &[StackValueKind],
    ) -> Result<RootedSlots> {
        Self::exec_jc(java_class, method_name, args, None)
    }

    /// Invokes a non-static method on an instance of a class.
    /// This method assumes arguments are already resolved; it doesn't perform lookups.
    /// Calls like invokevirtual and invokeinterface should be pre-resolved before calling this method.
    pub fn invoke_non_static_method(
        class_name: &str,
        method_name: &str,
        instance_ref: i32,
        args: &[StackValueKind],
    ) -> Result<RootedSlots> {
        let mut new_args = vec![StackValueKind::Ref(instance_ref)];
        new_args.extend_from_slice(args);
        Self::exec(class_name, method_name, &new_args, None)
    }

    /// Invokes a non-static method on an instance of a class.
    /// This method assumes arguments are already resolved; it doesn't perform lookups.
    /// Calls like invokevirtual and invokeinterface should be pre-resolved before calling this method.
    pub fn invoke_non_static_method_jc(
        java_class: &Arc<JavaClass>,
        method_name: &str,
        instance_ref: i32,
        args: &[StackValueKind],
    ) -> Result<RootedSlots> {
        let new_args = {
            let mut new_args = Vec::with_capacity(args.len() + 1);
            new_args.push(StackValueKind::Ref(instance_ref));
            new_args.extend_from_slice(args);
            new_args
        };
        Self::exec_jc(java_class, method_name, &new_args, None)
    }

    pub fn invoke_default_constructor(class_name: &str) -> Result<RootedSlots> {
        let instance_with_default_fields = with_method_area(|method_area| {
            method_area.create_instance_with_default_fields(class_name)
        })?;

        let mut destination = RootedSlots::new();
        let instance_ref = HEAP.create_instance(instance_with_default_fields, &mut destination)?;
        Self::exec(
            class_name,
            Self::INIT_METHOD,
            &[StackValueKind::Ref(instance_ref)],
            None,
        )?;

        Ok(destination)
    }

    pub fn invoke_default_constructor_jc(java_class: &Arc<JavaClass>) -> Result<RootedSlots> {
        let instance_with_default_fields = with_method_area(|method_area| {
            method_area.create_instance_with_default_fields(java_class.this_class_name())
        })?;

        let mut destination = RootedSlots::new();
        let instance_ref = HEAP.create_instance(instance_with_default_fields, &mut destination)?;
        Self::exec_jc(
            java_class,
            Self::INIT_METHOD,
            &[StackValueKind::Ref(instance_ref)],
            None,
        )?;

        Ok(destination)
    }

    pub fn invoke_args_constructor(
        class_name: &str,
        full_signature: &str,
        args: &[StackValueKind],
        detailed_reason: Option<&str>,
    ) -> Result<RootedSlots> {
        let _args =
            JavaThread::hold_temp_roots(args.iter().flat_map(StackValueKind::slots).collect());
        let instance_with_default_fields = with_method_area(|method_area| {
            method_area.create_instance_with_default_fields(class_name)
        })?;

        let mut destination = RootedSlots::new();
        let instance_ref = HEAP.create_instance(instance_with_default_fields, &mut destination)?;

        let mut new_args = Vec::with_capacity(args.len() + 1);
        new_args.push(StackValueKind::Ref(instance_ref));
        new_args.extend_from_slice(args);
        Self::exec(class_name, full_signature, &new_args, detailed_reason)?;

        Ok(destination)
    }

    pub fn create_primordial_thread(args: &[StackValueKind]) -> Result<RootedSlots> {
        let _args =
            JavaThread::hold_temp_roots(args.iter().flat_map(StackValueKind::slots).collect());
        let instance_with_default_fields = with_method_area(|method_area| {
            method_area.create_instance_with_default_fields("java/lang/Thread")
        })?;

        let mut destination = RootedSlots::new();
        let thread_obj_ref =
            HEAP.create_instance(instance_with_default_fields, &mut destination)?;
        with_method_area(|area| area.set_system_thread_id(thread_obj_ref))?; //save primordial thread id
                                                                             // Bind this OS thread's identity before `<init>` runs, since `Thread.<init>` calls
                                                                             // `currentThread()` (and relies on `parent == this` to recognize the primordial thread).
        JavaThread::set_current_thread(thread_obj_ref);
        // Register the main thread so it can be unparked (e.g. LockSupport) by other threads.
        threads::register_parker(thread_obj_ref, std::thread::current());

        let mut new_args = Vec::with_capacity(args.len() + 1);
        new_args.push(StackValueKind::Ref(thread_obj_ref));
        new_args.extend_from_slice(args);
        Self::exec(
            "java/lang/Thread",
            "<init>:(Ljava/lang/ThreadGroup;Ljava/lang/String;)V",
            &new_args,
            Some("primordial thread creation"),
        )?;
        // The main thread is running, not NEW; reflect that in getState().
        threads::set_thread_status(thread_obj_ref, threads::thread_status::RUNNABLE);

        Ok(destination)
    }

    fn exec(
        class_name: &str,
        method_name: &str,
        args: &[StackValueKind],
        detailed_reason: Option<&str>,
    ) -> Result<RootedSlots> {
        let _args =
            JavaThread::hold_temp_roots(args.iter().flat_map(StackValueKind::slots).collect());
        let java_class = CLASSES.get(class_name)?;
        Self::exec_jc(&java_class, method_name, args, detailed_reason)
    }

    fn exec_jc(
        java_class: &Arc<JavaClass>,
        method_name: &str,
        args: &[StackValueKind],
        detailed_reason: Option<&str>,
    ) -> Result<RootedSlots> {
        let _args =
            JavaThread::hold_temp_roots(args.iter().flat_map(StackValueKind::slots).collect());
        let java_method = java_class.get_method(method_name)?;
        let mut stack_frame = java_method.new_stack_frame()?;
        Self::set_stack_arguments(&mut stack_frame, &java_method, args)?;
        Engine::execute(
            stack_frame,
            &format!(
                "invoke {}.{method_name} {detailed_reason:?}",
                java_class.this_class_name()
            ),
        )
    }

    /// Preserve producer tags in callee locals, validating rather than reconstructing them.
    fn set_stack_arguments(
        stack_frame: &mut StackFrame,
        java_method: &JavaMethod,
        args: &[StackValueKind],
    ) -> Result<()> {
        let ref_chunks = java_method.arg_ref_chunks()?;
        let slots: Vec<_> = args.iter().flat_map(StackValueKind::slots).collect();
        validate_arg_slots(ref_chunks, &slots)?;
        for (index, slot) in slots.into_iter().enumerate() {
            stack_frame.set_local(index, slot);
        }

        Ok(())
    }
}

#[cfg(test)]
mod ref_transfer_tests {
    use super::*;
    use std::collections::HashSet;

    fn method() -> JavaMethod {
        JavaMethod::new(
            "(Ljava/lang/Object;IJ)V".parse().unwrap(),
            "Test",
            "mixed:(Ljava/lang/Object;IJ)V",
            None,
            false,
            vec![],
            8,
            "mixed",
            None,
            None,
            HashSet::new(),
            0,
        )
    }

    #[test]
    fn ref_transfer_vm_arguments_keep_explicit_tags_and_wide_order() {
        let reference = HEAP
            .create_array(
                "[I",
                0,
                &mut crate::vm::stack::rooted_slots::RootedSlots::new(),
            )
            .unwrap();
        let wide = 0x1234_5678_9abc_def0i64;
        let args = [
            StackValueKind::Ref(reference),
            reference.into(),
            wide.into(),
        ];
        let mut frame = StackFrame::for_test(4, 0);

        Executor::set_stack_arguments(&mut frame, &method(), &args).unwrap();

        assert_eq!(frame.ref_slots().collect::<Vec<_>>(), vec![reference]);
        assert_eq!(frame.get_local::<Slot>(1), Slot::Value(reference));
        assert_eq!(frame.get_local::<i64>(2), wide);
    }

    #[test]
    fn ref_transfer_vm_argument_without_producer_tag_is_rejected() {
        let reference = HEAP
            .create_array(
                "[I",
                0,
                &mut crate::vm::stack::rooted_slots::RootedSlots::new(),
            )
            .unwrap();
        let args = [reference.into(), 7.into(), 0i64.into()];
        let mut frame = StackFrame::for_test(4, 0);

        let result = Executor::set_stack_arguments(&mut frame, &method(), &args);

        assert!(result.is_err());
        assert_eq!(frame.ref_slots().count(), 0);
    }
}
