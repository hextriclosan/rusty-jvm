use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::executor::Executor;
use crate::vm::execution_engine::ldc_resolution_manager::{
    build_methodtype_ref, resolve_method_handle, LdcConstant,
};
use crate::vm::execution_engine::ops_reference_processor::prepare_invoke_context;
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::clazz_ref;
use crate::vm::jni::java_thread::{JavaThread, TempRootsGuard};
use crate::vm::method_area::loaded_classes::CLASSES;
use crate::vm::method_area::method_area::with_method_area;
use crate::vm::stack::slot::Slot;
use crate::vm::stack::stack_frame::StackFrames;
use crate::vm::stack::stack_value::StackValueKind;
use crate::vm::system_native::method_handle_natives::invocation::invoke_exact;
use crate::vm::system_native::method_handle_natives::types::ReferenceKind;
use dashmap::DashMap;
use derive_new::new;
use getset::{CopyGetters, Getters};
use jclassmodel::attributes::BootstrapMethodInfo;
use jdescriptor::MethodDescriptor;

/// The `InvokeDynamicRunner` is responsible for handling the `invokedynamic` instruction in the JVM.
/// It resolves the dynamic call site, caches the resolved method, and invokes it with the appropriate
/// arguments.
#[derive(Debug, Default)]
pub(crate) struct InvokeDynamicRunner {
    resolved_methods: DashMap<u16, ResolvedMethod>,
}

#[derive(Debug, Clone, new, CopyGetters, Getters)]
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
    /// Class references, strings, method handles or numeric constants, resolved but not yet boxed;
    /// see [`LdcConstant`].
    #[get = "pub"]
    bootstrap_args: Vec<LdcConstant>,

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
                //todo: extend with resolve_ldc2_w for long/double
                //
                // The constant's kind is kept rather than flattened to raw bits: these are handed
                // to the bootstrap method as `Object`s, so a numeric constant has to be boxed, and
                // only the kind says whether that means `Integer` or `Float`.
                .map(|&cpool_index| area.resolve_ldc_constant(current_class_name, cpool_index))
                .collect::<Result<Vec<LdcConstant>>>()
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
    /// Quiescent read only. Resolution runs outside the map's entry guard.
    pub(crate) fn resolved_refs(&self) -> Vec<i32> {
        self.resolved_methods
            .iter()
            .map(|entry| entry.method_handle_dynamic_invoked_ref())
            .collect()
    }

    /// Runs the invokedynamic call site logic, resolving it if not cached,
    /// and invoking the dynamic method with the correct arguments.
    pub fn run(
        &self,
        stack_frames: &mut StackFrames,
        current_class_name: &str,
        invokedynamic_index: u16,
    ) -> Result<()> {
        let cached = self
            .resolved_methods
            .get(&invokedynamic_index)
            .map(|entry| entry.clone());
        let resolved_method = match cached {
            Some(resolved) => resolved,
            None => {
                let resolved = Self::resolve(current_class_name, invokedynamic_index)?;
                self.resolved_methods
                    .entry(invokedynamic_index)
                    .or_insert(resolved)
                    .clone()
            }
        };

        let method_handle_dynamic_invoked_ref =
            resolved_method.method_handle_dynamic_invoked_ref();
        let method_descriptor = resolved_method.invoke_dynamic_method_type_desc();
        let args_to_invoke_with = prepare_invoke_context(stack_frames, method_descriptor, false)?;

        invoke_exact(method_handle_dynamic_invoked_ref, &args_to_invoke_with)
    }

    fn resolve(current_class_name: &str, invokedynamic_index: u16) -> Result<ResolvedMethod> {
        let bootstrap_info =
            Self::extract_bootstrap_info(current_class_name, invokedynamic_index)?;
        // The caller owns the scope; prepare_args never returns an inner LIFO guard.
        let mut roots = JavaThread::hold_temp_roots(vec![]);
        let args = Self::prepare_args(current_class_name, &bootstrap_info, &mut roots)?;
        let method_handle_dynamic_invoked_ref = Self::build_method_handle_dynamic_invoked(&args)?;

        Ok(ResolvedMethod::new(
            method_handle_dynamic_invoked_ref,
            bootstrap_info.invoke_dynamic_descriptor().parse()?,
        ))
    }

    fn build_method_handle_dynamic_invoked(args: &[StackValueKind; 6]) -> Result<i32> {
        let call_site = Executor::invoke_static_method(
            "java/lang/invoke/BootstrapMethodInvoker",
            "invoke:(Ljava/lang/Class;Ljava/lang/invoke/MethodHandle;Ljava/lang/String;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Class;)Ljava/lang/Object;",
            args,
        )?;
        let call_site_ref = call_site[0].value();

        let call_site_name = HEAP.get_instance_name(call_site_ref)?;

        let method_handle_dynamic_invoked_ref = Executor::invoke_non_static_method(
            &call_site_name,
            "dynamicInvoker:()Ljava/lang/invoke/MethodHandle;",
            call_site_ref,
            &[],
        )?;
        Ok(method_handle_dynamic_invoked_ref[0].value())
    }

    /// Boxes a numeric bootstrap argument so it can be stored in the `Object[]` handed to the
    /// bootstrap method.
    ///
    /// An `Integer` or `Float` constant is 32 bits of value, not a reference. Storing those bits
    /// straight into a `[Ljava/lang/Object;` leaves the bootstrap method reading them as an object
    /// reference; JVMS §5.4.3.6 calls for the boxed form.
    fn box_bootstrap_arg(argument: &LdcConstant) -> Result<i32> {
        Ok(match argument {
            LdcConstant::Int(value) => Executor::invoke_static_method(
                "java/lang/Integer",
                "valueOf:(I)Ljava/lang/Integer;",
                &[(*value).into()],
            )?[0]
                .value(),
            LdcConstant::Float(value) => Executor::invoke_static_method(
                "java/lang/Float",
                "valueOf:(F)Ljava/lang/Float;",
                &[(*value).into()],
            )?[0]
                .value(),
            LdcConstant::Reference(reference) => *reference,
        })
    }

    fn prepare_args(
        current_class_name: &str,
        bootstrap_info: &BootstrapInfo,
        roots: &mut TempRootsGuard,
    ) -> Result<[StackValueKind; 6]> {
        let bootstrap_args = bootstrap_info.bootstrap_args();
        let arguments_ref = HEAP.create_array("[Ljava/lang/Object;", bootstrap_args.len() as i32);

        // Each argument below lives only in a Rust local until the bootstrap call takes it, and
        // every step in between (boxing, handle resolution, interning, method type construction)
        // runs Java and so can collect — hence a root per argument, added as soon as it exists.
        // The caller's scope covers both preparation and the bootstrap call.
        roots.add_root(Slot::Ref(arguments_ref));

        bootstrap_args
            .iter()
            .enumerate()
            .try_for_each(|(index, argument)| {
                let value = Self::box_bootstrap_arg(argument)?;
                HEAP.set_array_value(arguments_ref, index as i32, vec![value])
            })?;

        let call_site_clazz = clazz_ref("java/lang/invoke/CallSite")?;
        roots.add_root(Slot::Ref(call_site_clazz));

        let method_handle_ref = resolve_method_handle(
            current_class_name,
            bootstrap_info.ref_kind(),
            bootstrap_info.bootstrap_method_class(),
            bootstrap_info.bootstrap_method_name(),
            bootstrap_info.bootstrap_method_descriptor(),
        )?;
        roots.add_root(Slot::Ref(method_handle_ref));

        let method_name_ref = StringPoolHelper::get_string(bootstrap_info.invoke_dynamic_name())?;
        roots.add_root(Slot::Ref(method_name_ref));
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
        roots.add_root(Slot::Ref(invoke_dynamic_methodtype_or_type_ref));

        let current_clazz = clazz_ref(current_class_name)?;
        roots.add_root(Slot::Ref(current_clazz));

        let args = [
            StackValueKind::Ref(call_site_clazz),
            StackValueKind::Ref(method_handle_ref),
            StackValueKind::Ref(method_name_ref),
            StackValueKind::Ref(invoke_dynamic_methodtype_or_type_ref),
            StackValueKind::Ref(arguments_ref),
            StackValueKind::Ref(current_clazz),
        ];
        Ok(args)
    }

    fn extract_bootstrap_info(
        current_class_name: &str,
        invokedynamic_index: u16,
    ) -> Result<BootstrapInfo> {
        let klass = CLASSES.get(current_class_name)?;
        let attributes = klass.attributes();
        let bootstrap_method_info = attributes.get_bootstrap_method(
            klass.constant_pool(),
            invokedynamic_index,
        ).ok_or_else(|| {
            Error::new_constant_pool(&format!(
                "Error getting bootstrap method for invokedynamic index {invokedynamic_index} in class {current_class_name}"
            ))
        })?;

        BootstrapInfo::try_from((bootstrap_method_info, current_class_name))
    }
}

#[cfg(test)]
mod gc_tests {
    use super::*;

    #[test]
    fn gc_roots_indy_cache_exposes_only_already_resolved_handles() {
        let runner = InvokeDynamicRunner::default();
        assert!(runner.resolved_refs().is_empty());
        runner
            .resolved_methods
            .insert(7, ResolvedMethod::new(53, "()V".parse().unwrap()));

        assert_eq!(runner.resolved_refs(), vec![53]);
    }
}
