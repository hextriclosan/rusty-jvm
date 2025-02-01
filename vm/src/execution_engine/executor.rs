use crate::execution_engine::engine::Engine;
use crate::heap::heap::with_heap_write_lock;
use crate::method_area::java_class::JavaClass;
use crate::method_area::method_area::with_method_area;
use crate::stack::sack_value::StackValueKind;
use crate::stack::stack_frame::StackFrame;

pub struct Executor {}

impl Executor {
    const STATIC_INIT_METHOD: &'static str = "<clinit>:()V";
    const INIT_METHOD: &'static str = "<init>:()V";

    pub fn do_static_fields_initialization(java_class_name: &str) -> crate::error::Result<()> {
        let java_class = with_method_area(|area| area.get(java_class_name))?;
        Self::do_java_class_static_fields_initialization(&java_class)
    }

    pub fn do_java_class_static_fields_initialization(
        java_class: &JavaClass,
    ) -> crate::error::Result<()> {
        //todo: protect me with recursive mutex
        if let Some(static_init_method) = java_class.try_get_method(Self::STATIC_INIT_METHOD) {
            Engine::execute(
                static_init_method.new_stack_frame()?,
                &format!("static initialization {}", java_class.this_class_name()),
            )?;
        }

        Ok(())
    }

    pub fn invoke_static_method(
        class_name: &str,
        method_name: &str,
        args: &[StackValueKind],
    ) -> crate::error::Result<()> {
        Self::exec(class_name, method_name, args, None)
    }

    pub fn invoke_default_constructor(class_name: &str) -> crate::error::Result<i32> {
        let instance_with_default_fields = with_method_area(|method_area| {
            method_area.create_instance_with_default_fields(class_name)
        })?;

        let instance_ref =
            with_heap_write_lock(|heap| heap.create_instance(instance_with_default_fields));
        Self::exec(class_name, Self::INIT_METHOD, &[instance_ref.into()], None)?;

        Ok(instance_ref)
    }

    pub fn invoke_args_constructor(
        class_name: &str,
        full_signature: &str,
        args: &[StackValueKind],
        detailed_reason: Option<&str>,
    ) -> crate::error::Result<i32> {
        let instance_with_default_fields = with_method_area(|method_area| {
            method_area.create_instance_with_default_fields(&class_name)
        })?;

        let instance_ref =
            with_heap_write_lock(|heap| heap.create_instance(instance_with_default_fields));

        let mut new_args = Vec::with_capacity(args.len() + 1);
        new_args.push(instance_ref.into());
        new_args.extend_from_slice(args);
        Self::exec(class_name, full_signature, &new_args, detailed_reason)?;

        Ok(instance_ref)
    }

    pub fn create_primordial_thread(args: &[StackValueKind]) -> crate::error::Result<i32> {
        let instance_with_default_fields = with_method_area(|method_area| {
            method_area.create_instance_with_default_fields("java/lang/Thread")
        })?;

        let thread_obj_ref =
            with_heap_write_lock(|heap| heap.create_instance(instance_with_default_fields));
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
    ) -> crate::error::Result<()> {
        let java_class = with_method_area(|area| area.get(class_name))?;
        let java_method = java_class.get_method(method_name)?;
        let mut stack_frame = java_method.new_stack_frame()?;
        Self::set_stack_arguments(&mut stack_frame, args)?;
        Engine::execute(
            stack_frame,
            &format!("invoke {class_name}.{method_name} {detailed_reason:?}"),
        )
    }

    fn set_stack_arguments(
        stack_frame: &mut StackFrame,
        args: &[StackValueKind],
    ) -> crate::error::Result<()> {
        for (i, arg) in args.iter().enumerate() {
            match arg {
                StackValueKind::I32(value) => stack_frame.set_local(i, *value),
                StackValueKind::I64(value) => stack_frame.set_local(i, *value),
                StackValueKind::F32(value) => stack_frame.set_local(i, *value),
                StackValueKind::F64(value) => stack_frame.set_local(i, *value),
            }
        }

        Ok(())
    }
}
