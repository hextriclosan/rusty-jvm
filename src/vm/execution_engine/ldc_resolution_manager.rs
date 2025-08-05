use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::executor::Executor;
use crate::vm::execution_engine::reflection_class_loader::ReflectionClassLoader;
use crate::vm::execution_engine::static_init::StaticInit;
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::helper::{clazz_ref, i64_to_vec, vec_to_i64};
use crate::vm::method_area::cpool_helper::CPoolHelperTrait;
use crate::vm::method_area::method_area::with_method_area;
use crate::vm::system_native::method_handle_natives::types::ReferenceKind;
use std::collections::HashMap;
use std::sync::RwLock;

type CPoolIndex = u16;
type Value = Vec<i32>;

#[derive(Debug, Default)]
pub struct LdcResolutionManager {
    reflection_class_loader: ReflectionClassLoader,
    cache: RwLock<HashMap<String, HashMap<CPoolIndex, Value>>>,
}

impl LdcResolutionManager {
    pub fn resolve_ldc(&self, current_class_name: &str, cpoolindex: u16) -> Result<i32> {
        if let Some(Some(value)) = self
            .cache
            .read()?
            .get(current_class_name)
            .map(|map| map.get(&cpoolindex))
        {
            return Ok(value[0]);
        }

        let java_class = with_method_area(|method_area| method_area.get(current_class_name))?;
        let cpool_helper = java_class.cpool_helper();

        let result = if let Some(value) = cpool_helper.get_integer(cpoolindex) {
            value
        } else if let Some(value) = cpool_helper.get_float(cpoolindex) {
            Self::float_to_int(value)
        } else if let Some(value) = cpool_helper.get_string(cpoolindex) {
            StringPoolHelper::get_string(&value)?
        } else if let Some(class_name) = cpool_helper.get_class_name(cpoolindex) {
            self.load_reflection_class(&class_name)?
        } else if let Some(method_type) = cpool_helper.get_method_type(cpoolindex) {
            build_methodtype_ref(&method_type)?
        } else if let Some((reference_kind, class_name, name, descriptor)) =
            cpool_helper.get_method_handle(cpoolindex)
        {
            resolve_method_handle(
                current_class_name,
                ReferenceKind::try_from(reference_kind)?,
                &class_name,
                &name,
                &descriptor,
            )?
        } else {
            return Err(Error::new_constant_pool(&format!(
                "Error resolving ldc: {}",
                cpoolindex
            )));
        };

        self.cache
            .write()?
            .entry(current_class_name.to_string())
            .or_insert_with(HashMap::new)
            .insert(cpoolindex, vec![result]);

        Ok(result)
    }

    pub fn load_reflection_class(&self, class_name: &str) -> Result<i32> {
        self.reflection_class_loader.load(&class_name)
    }

    pub fn resolve_ldc2_w(&self, current_class_name: &str, cpoolindex: u16) -> Result<i64> {
        if let Some(Some(value)) = self
            .cache
            .read()?
            .get(current_class_name)
            .map(|map| map.get(&cpoolindex))
        {
            return Ok(vec_to_i64(value));
        }

        let java_class = with_method_area(|method_area| method_area.get(current_class_name))?;
        let cpool_helper = java_class.cpool_helper();

        let result = if let Some(value) = cpool_helper.get_long(cpoolindex) {
            value
        } else if let Some(value) = cpool_helper.get_double(cpoolindex) {
            Self::double_to_int(value)
        } else {
            return Err(Error::new_constant_pool(&format!(
                "Error resolving ldc: {}",
                cpoolindex
            )));
        };

        self.cache
            .write()?
            .entry(current_class_name.to_string())
            .or_insert_with(HashMap::new)
            .insert(cpoolindex, i64_to_vec(result));

        Ok(result)
    }

    fn float_to_int(value: f32) -> i32 {
        value.to_bits() as i32
    }

    fn double_to_int(value: f64) -> i64 {
        value.to_bits() as i64
    }
}

/// Builds a reference to a `java.lang.invoke.MethodType` object from a method descriptor string.
///
/// # Arguments
///
/// * `descriptor` - A string slice representing the method descriptor (e.g., "(Ljava/lang/String;)V").
///
/// # Returns
///
/// Returns a `Result<i32>` containing the reference to the created `MethodType` object,
/// or an error if the operation fails.
// todo: consider separate cache for method type references
pub fn build_methodtype_ref(descriptor: &str) -> Result<i32> {
    let string_ref = StringPoolHelper::get_string(descriptor)?;
    let method_type_ref = Executor::invoke_static_method(
        "java/lang/invoke/MethodType",
        "fromMethodDescriptorString:(Ljava/lang/String;Ljava/lang/ClassLoader;)Ljava/lang/invoke/MethodType;",
        &[string_ref.into()],
    )?[0];
    Ok(method_type_ref)
}

/// Resolves a method handle for a given method or field in the JVM.
///
/// # Parameters
/// - `current_class_name`: The name of the class from which the resolution is initiated.
/// - `reference_kind`: The kind of reference (e.g., method invocation, field access) as defined by `ReferenceKind`.
/// - `class_name_to_lookup_in`: The name of the class in which to look up the method or field.
/// - `method_or_field_to_lookup_for`: The name of the method or field to resolve.
/// - `method_or_field_descriptor`: The descriptor string of the method or field (e.g., type signature).
///
/// # Returns
/// Returns `Result<i32>` containing a reference to the resolved method handle object on success,
/// or an error if the resolution fails.
pub fn resolve_method_handle(
    current_class_name: &str,
    reference_kind: ReferenceKind,
    class_name_to_lookup_in: &str,
    method_or_field_to_lookup_for: &str,
    method_or_field_descriptor: &str,
) -> Result<i32> {
    let (lookup_class_name, method_name_lookup_for) = reference_kind.to_findmethod_signature()?;
    let new_lookup = build_lookup_for_class(current_class_name)?;
    let refc = clazz_ref(class_name_to_lookup_in)?;
    let method_name_ref = StringPoolHelper::get_string(method_or_field_to_lookup_for)?;

    let method_type_ref = build_methodtype_ref(&method_or_field_descriptor)?;
    let args = match reference_kind {
        ReferenceKind::REF_invokeStatic
        | ReferenceKind::REF_invokeInterface
        | ReferenceKind::REF_invokeVirtual => {
            vec![refc.into(), method_name_ref.into(), method_type_ref.into()]
        }
        ReferenceKind::REF_newInvokeSpecial => vec![refc.into(), method_type_ref.into()],
        ReferenceKind::REF_getField
        | ReferenceKind::REF_getStatic
        | ReferenceKind::REF_putField
        | ReferenceKind::REF_putStatic
        | ReferenceKind::REF_invokeSpecial => {
            return Err(Error::new_execution(&format!(
                "resolve_method_handle: Unsupported yet reference kind: {reference_kind:?}"
            )))
        }
    };
    let method_handle_ref = Executor::invoke_non_static_method(
        lookup_class_name,
        method_name_lookup_for,
        new_lookup,
        &args,
    )?[0];
    Ok(method_handle_ref)
}

/// Constructs a new `java/lang/invoke/MethodHandles$Lookup` object for the specified class.
///
/// This function performs the following steps:
/// 1. Retrieves the `MethodHandles$Lookup` class from the method area.
/// 2. Ensures the class is initialized.
/// 3. Accesses the static `IMPL_LOOKUP` field of the class.
/// 4. Invokes the non-static `in` method on the `IMPL_LOOKUP` object, passing the target class.
///
/// # Parameters
/// - `current_class_name`: The name of the class for which the lookup object should be constructed.
///
/// # Returns
/// Returns `Result<i32>` containing a reference to the new `MethodHandles$Lookup` object on success,
/// or an error if any step fails.
fn build_lookup_for_class(current_class_name: &str) -> Result<i32> {
    let jc_lookup = with_method_area(|a| a.get("java/lang/invoke/MethodHandles$Lookup"))?;
    StaticInit::initialize_java_class(&jc_lookup)?;
    let impl_lookup = jc_lookup
        .static_field("IMPL_LOOKUP")
        .ok_or(Error::new_execution("Error getting IMPL_LOOKUP field"))?;

    let impl_lookup_ref = impl_lookup.raw_value()?[0];

    let current_clazz = clazz_ref(current_class_name)?;

    let new_lookup = Executor::invoke_non_static_method(
        "java/lang/invoke/MethodHandles$Lookup",
        "in:(Ljava/lang/Class;)Ljava/lang/invoke/MethodHandles$Lookup;",
        impl_lookup_ref,
        &[current_clazz.into()],
    )?[0];
    Ok(new_lookup)
}
