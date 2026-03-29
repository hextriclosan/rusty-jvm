use crate::vm::error::Result;
use crate::vm::execution_engine::engine::Engine;
use crate::vm::heap::heap::HEAP;
use crate::vm::method_area::java_class::JavaClass;
use crate::vm::method_area::loaded_classes::CLASSES;
use crate::vm::method_area::method_area::with_method_area;
use crate::vm::stack::stack_frame::StackFrame;
use crate::vm::stack::stack_value::StackValueKind;
use std::sync::Arc;

pub struct Executor {}

impl Executor {
    const INIT_METHOD: &'static str = "<init>:()V";

    pub fn invoke_static_method(
        class_name: &str,
        method_name: &str,
        args: &[StackValueKind],
    ) -> Result<Vec<i32>> {
        let java_class = CLASSES.get(class_name)?;
        Self::invoke_static_method_jc(&java_class, method_name, args)
    }

    pub fn invoke_static_method_jc(
        java_class: &Arc<JavaClass>,
        method_name: &str,
        args: &[StackValueKind],
    ) -> Result<Vec<i32>> {
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
    ) -> Result<Vec<i32>> {
        let java_class = CLASSES.get(class_name)?;
        Self::invoke_non_static_method_jc(&java_class, method_name, instance_ref, args)
    }

    /// Invokes a non-static method on an instance of a class.
    /// This method assumes arguments are already resolved; it doesn't perform lookups.
    /// Calls like invokevirtual and invokeinterface should be pre-resolved before calling this method.
    pub fn invoke_non_static_method_jc(
        java_class: &Arc<JavaClass>,
        method_name: &str,
        instance_ref: i32,
        args: &[StackValueKind],
    ) -> Result<Vec<i32>> {
        let new_args = {
            let mut new_args = Vec::with_capacity(args.len() + 1);
            new_args.push(instance_ref.into());
            new_args.extend_from_slice(args);
            new_args
        };
        Self::exec_jc(java_class, method_name, &new_args, None)
    }

    pub fn invoke_default_constructor(class_name: &str) -> Result<i32> {
        let instance_with_default_fields = with_method_area(|method_area| {
            method_area.create_instance_with_default_fields(class_name)
        })?;

        let instance_ref = HEAP.create_instance(instance_with_default_fields);
        Self::exec(class_name, Self::INIT_METHOD, &[instance_ref.into()], None)?;

        Ok(instance_ref)
    }

    pub fn invoke_default_constructor_jc(java_class: &Arc<JavaClass>) -> Result<i32> {
        let instance_with_default_fields = with_method_area(|method_area| {
            method_area.create_instance_with_default_fields(java_class.this_class_name())
        })?;

        let instance_ref = HEAP.create_instance(instance_with_default_fields);
        Self::exec_jc(java_class, Self::INIT_METHOD, &[instance_ref.into()], None)?;

        Ok(instance_ref)
    }

    pub fn invoke_args_constructor(
        class_name: &str,
        full_signature: &str,
        args: &[StackValueKind],
        detailed_reason: Option<&str>,
    ) -> Result<i32> {
        let instance_with_default_fields = with_method_area(|method_area| {
            method_area.create_instance_with_default_fields(&class_name)
        })?;

        let instance_ref = HEAP.create_instance(instance_with_default_fields);

        let mut new_args = Vec::with_capacity(args.len() + 1);
        new_args.push(instance_ref.into());
        new_args.extend_from_slice(args);
        Self::exec(class_name, full_signature, &new_args, detailed_reason)?;

        Ok(instance_ref)
    }

    pub fn create_primordial_thread(args: &[StackValueKind]) -> Result<i32> {
        let instance_with_default_fields = with_method_area(|method_area| {
            method_area.create_instance_with_default_fields("java/lang/Thread")
        })?;

        let thread_obj_ref = HEAP.create_instance(instance_with_default_fields);
        with_method_area(|area| area.set_system_thread_id(thread_obj_ref))?; //save primordial thread id

        let mut new_args = Vec::with_capacity(args.len() + 1);
        new_args.push(thread_obj_ref.into());
        new_args.extend_from_slice(args);
        Self::exec(
            "java/lang/Thread",
            "<init>:(Ljava/lang/ThreadGroup;Ljava/lang/String;)V",
            &new_args,
            Some("primordial thread creation"),
        )?;

        Ok(thread_obj_ref)
    }

    fn exec(
        class_name: &str,
        method_name: &str,
        args: &[StackValueKind],
        detailed_reason: Option<&str>,
    ) -> Result<Vec<i32>> {
        let java_class = CLASSES.get(class_name)?;
        Self::exec_jc(&java_class, method_name, args, detailed_reason)
    }

    fn exec_jc(
        java_class: &Arc<JavaClass>,
        method_name: &str,
        args: &[StackValueKind],
        detailed_reason: Option<&str>,
    ) -> Result<Vec<i32>> {
        let java_method = java_class.get_method(method_name)?;
        let mut stack_frame = java_method.new_stack_frame()?;
        Self::set_stack_arguments(&mut stack_frame, args)?;
        Engine::execute(
            stack_frame,
            &format!(
                "invoke {}.{method_name} {detailed_reason:?}",
                java_class.this_class_name()
            ),
        )
    }

    fn set_stack_arguments(stack_frame: &mut StackFrame, args: &[StackValueKind]) -> Result<()> {
        let mut chunk_index = 0;
        for arg in args.iter() {
            match arg {
                StackValueKind::I32(value) => stack_frame.set_local(chunk_index, *value),
                StackValueKind::I64(value) => stack_frame.set_local(chunk_index, *value),
                StackValueKind::F32(value) => stack_frame.set_local(chunk_index, *value),
                StackValueKind::F64(value) => stack_frame.set_local(chunk_index, *value),
            }
            chunk_index += arg.chunks();
        }

        Ok(())
    }
}
