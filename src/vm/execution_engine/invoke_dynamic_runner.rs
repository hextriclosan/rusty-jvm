use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::executor::Executor;
use crate::vm::execution_engine::ldc_resolution_manager::{
    build_methodtype_ref, resolve_method_handle,
};
use crate::vm::execution_engine::ops_reference_processor::prepare_invoke_context;
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::clazz_ref;
use crate::vm::method_area::attributes_helper::BootstrapMethodInfo;
use crate::vm::method_area::loaded_classes::CLASSES;
use crate::vm::method_area::method_area::with_method_area;
use crate::vm::stack::stack_frame::StackFrames;
use crate::vm::stack::stack_value::StackValueKind;
use crate::vm::system_native::method_handle_natives::invocation::invoke_exact;
use crate::vm::system_native::method_handle_natives::types::ReferenceKind;
use dashmap::DashMap;
use derive_new::new;
use getset::{CopyGetters, Getters};
use jdescriptor::MethodDescriptor;

/// The `InvokeDynamicRunner` is responsible for handling the `invokedynamic` instruction in the JVM.
/// It resolves the dynamic call site, caches the resolved method, and invokes it with the appropriate
/// arguments.
#[derive(Debug, Default)]
pub(crate) struct InvokeDynamicRunner {
    resolved_methods: DashMap<u16, ResolvedMethod>,
}

#[derive(Debug, new, CopyGetters, Getters)]
struct ResolvedMethod {
    #[get_copy = "pub"]
    method_handle_dynamic_invoked_ref: i32,
    #[get = "pub"]
    invoke_dynamic_method_type_desc: MethodDescriptor,
}

/// Contains all relevant metadata required to resolve an `invokedynamic` call site.
/// This struct represents the constants and arguments extracted from the `BootstrapMethods` attribute
/// and associated constant pool entries.
#[derive(Debug, Clone, Getters, CopyGetters)]
struct BootstrapInfo {
    /// The kind of reference used to look up the bootstrap method handle.
    ///
    /// Determines the lookup mechanism (e.g., `REF_invokeStatic`, `REF_invokeVirtual`, etc.).
    #[get_copy = "pub"]
    ref_kind: ReferenceKind,

    /// Fully qualified internal class name (e.g., `java/lang/invoke/StringConcatFactory`)
    /// where the bootstrap method is declared.
    #[get = "pub"]
    bootstrap_method_class: String,

    /// Name of the bootstrap method (e.g., `"makeConcatWithConstants"`).
    #[get = "pub"]
    bootstrap_method_name: String,

    /// Descriptor of the bootstrap method (e.g.,
    /// `"(Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/CallSite;"`).
    #[get = "pub"]
    bootstrap_method_descriptor: String,

    /// Bootstrap method arguments provided via the constant pool.
    ///
    /// These may be class references, strings, or method handles — already resolved to heap references (`i32`).
    #[get = "pub"]
    bootstrap_args: Vec<i32>,

    /// The name of the method being dynamically resolved by the `invokedynamic` instruction.
    ///
    /// This is the symbolic name seen in the constant pool — not the name of the bootstrap method itself.
    #[get = "pub"]
    invoke_dynamic_name: String,

    /// The method descriptor for the call site to be created by the bootstrap method.
    ///
    /// For example: `"(Ljava/lang/String;)Ljava/lang/String;"`.
    #[get = "pub"]
    invoke_dynamic_descriptor: String,
}

impl TryFrom<(BootstrapMethodInfo, &str)> for BootstrapInfo {
    type Error = Error;

    fn try_from(value: (BootstrapMethodInfo, &str)) -> Result<Self> {
        let (value, current_class_name) = value;
        let bootstrap_args = with_method_area(|area| {
            value
                .bootstrap_arguments_cpool_indexes()
                .iter()
                .map(|&cpool_index| area.resolve_ldc(current_class_name, cpool_index)) //todo: extend with resolve_ldc2_w for long/double
                .collect::<Result<Vec<i32>>>()
        })?;

        Ok(BootstrapInfo {
            ref_kind: value.ref_kind().try_into()?,
            bootstrap_method_class: value.class_name().clone(),
            bootstrap_method_name: value.method_name().clone(),
            bootstrap_method_descriptor: value.method_descriptor().clone(),
            bootstrap_args,
            invoke_dynamic_name: value.invoke_dynamic_method_name().clone(),
            invoke_dynamic_descriptor: value.invoke_dynamic_method_descriptor().clone(),
        })
    }
}

impl InvokeDynamicRunner {
    /// Runs the invokedynamic call site logic, resolving it if not cached,
    /// and invoking the dynamic method with the correct arguments.
    pub fn run(
        &self,
        stack_frames: &mut StackFrames,
        current_class_name: &str,
        invokedynamic_index: u16,
    ) -> Result<()> {
        let entry = &self
            .resolved_methods
            .entry(invokedynamic_index)
            .or_try_insert_with(|| Self::resolve(&current_class_name, invokedynamic_index))?;
        let resolved_method = entry.value();

        let method_handle_dynamic_invoked_ref =
            resolved_method.method_handle_dynamic_invoked_ref();
        let method_descriptor = resolved_method.invoke_dynamic_method_type_desc();
        let args_to_invoke_with = prepare_invoke_context(stack_frames, method_descriptor, false)?;

        invoke_exact(
            method_handle_dynamic_invoked_ref,
            &args_to_invoke_with,
            stack_frames,
        )
    }

    fn resolve(current_class_name: &str, invokedynamic_index: u16) -> Result<ResolvedMethod> {
        let bootstrap_info =
            Self::extract_bootstrap_info(&current_class_name, invokedynamic_index)?;
        let args = Self::prepare_args(current_class_name, &bootstrap_info)?;
        let method_handle_dynamic_invoked_ref = Self::build_method_handle_dynamic_invoked(&args)?;

        Ok(ResolvedMethod::new(
            method_handle_dynamic_invoked_ref,
            bootstrap_info.invoke_dynamic_descriptor().parse()?,
        ))
    }

    fn build_method_handle_dynamic_invoked(args: &[StackValueKind; 6]) -> Result<i32> {
        let call_site_ref = Executor::invoke_static_method(
            "java/lang/invoke/BootstrapMethodInvoker",
            "invoke:(Ljava/lang/Class;Ljava/lang/invoke/MethodHandle;Ljava/lang/String;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Class;)Ljava/lang/Object;",
            args,
        )?[0];

        let call_site_name = HEAP.get_instance_name(call_site_ref)?;

        let method_handle_dynamic_invoked_ref = Executor::invoke_non_static_method(
            &call_site_name,
            "dynamicInvoker:()Ljava/lang/invoke/MethodHandle;",
            call_site_ref,
            &[],
        )?[0];
        Ok(method_handle_dynamic_invoked_ref)
    }

    fn prepare_args(
        current_class_name: &str,
        bootstrap_info: &BootstrapInfo,
    ) -> Result<[StackValueKind; 6]> {
        let bootstrap_args = bootstrap_info.bootstrap_args();
        let arguments_ref = HEAP.create_array("[Ljava/lang/Object;", bootstrap_args.len() as i32);

        bootstrap_args
            .iter()
            .enumerate()
            .try_for_each(|(index, some_ref)| {
                HEAP.set_array_value(arguments_ref, index as i32, vec![*some_ref])
            })?;

        let call_site_clazz = clazz_ref("java/lang/invoke/CallSite")?;

        let method_handle_ref = resolve_method_handle(
            current_class_name,
            bootstrap_info.ref_kind(),
            bootstrap_info.bootstrap_method_class(),
            bootstrap_info.bootstrap_method_name(),
            bootstrap_info.bootstrap_method_descriptor(),
        )?;

        let method_name_ref = StringPoolHelper::get_string(bootstrap_info.invoke_dynamic_name())?;
        let invoke_dynamic_methodtype_or_type_ref = match bootstrap_info.ref_kind() {
            ReferenceKind::REF_invokeStatic
            | ReferenceKind::REF_invokeInterface
            | ReferenceKind::REF_invokeVirtual => {
                build_methodtype_ref(bootstrap_info.invoke_dynamic_descriptor())
            }
            ReferenceKind::REF_getField => clazz_ref(bootstrap_info.invoke_dynamic_descriptor()),
            ReferenceKind::REF_getStatic
            | ReferenceKind::REF_putField
            | ReferenceKind::REF_putStatic
            | ReferenceKind::REF_invokeSpecial
            | ReferenceKind::REF_newInvokeSpecial => {
                return Err(Error::new_execution(&format!(
                    "Unsupported yet reference kind for getting method/type ref: {:?}",
                    bootstrap_info.ref_kind()
                )))
            }
        }?;

        let current_clazz = clazz_ref(current_class_name)?;

        let args = [
            call_site_clazz.into(),
            method_handle_ref.into(),
            method_name_ref.into(),
            invoke_dynamic_methodtype_or_type_ref.into(),
            arguments_ref.into(),
            current_clazz.into(),
        ];
        Ok(args)
    }

    fn extract_bootstrap_info(
        current_class_name: &str,
        invokedynamic_index: u16,
    ) -> Result<BootstrapInfo> {
        let jc = CLASSES.get(current_class_name)?;
        let attributes_helper = jc.attributes_helper();
        let bootstrap_method_info = attributes_helper.get_bootstrap_method(
            jc.cpool_helper(),
            invokedynamic_index,
        ).ok_or_else(|| {
            Error::new_constant_pool(&format!(
                "Error getting bootstrap method for invokedynamic index {invokedynamic_index} in class {current_class_name}"
            ))
        })?;

        BootstrapInfo::try_from((bootstrap_method_info, current_class_name))
    }
}
