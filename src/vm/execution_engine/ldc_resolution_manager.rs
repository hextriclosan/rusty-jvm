use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::executor::Executor;
use crate::vm::execution_engine::static_init::StaticInit;
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::helper::{clazz_ref, i64_to_vec, vec_to_i64};
use crate::vm::method_area::loaded_classes::CLASSES;
use crate::vm::stack::slot::Slot;
use crate::vm::system_native::method_handle_natives::types::ReferenceKind;
use jclassmodel::constant_pool::ConstantPoolLookup;
use std::collections::HashMap;
use std::sync::RwLock;

type CPoolIndex = u16;
/// A resolved constant, cached as the chunks it occupies.
///
/// `ldc2_w` constants are `long`/`double`, whose two halves are stored as [`LdcConstant::Int`]
/// because only their raw bits are ever read back ([`LdcResolutionManager::constants_to_i64`]).
type Value = Vec<LdcConstant>;

/// A resolved constant-pool constant, with enough of its kind kept to serve every consumer.
///
/// `ldc` needs to know only whether the result is a reference, which a [`Slot`] would express. A
/// bootstrap argument needs more: passed to a bootstrap method it is an `Object`, so a numeric
/// constant has to be boxed first — and `Integer` and `Float` box differently while being
/// indistinguishable once both are 32 raw bits. Collapsing them early is what made primitive
/// bootstrap arguments reach `[Ljava/lang/Object;` as bare bits.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum LdcConstant {
    Int(i32),
    Float(f32),
    /// A `String`, `Class`, `MethodType` or `MethodHandle` entry, already resolved to a heap
    /// reference.
    Reference(i32),
}

impl LdcConstant {
    /// The operand-stack slot this constant occupies, tagged when it is a reference.
    pub(crate) fn slot(&self) -> Slot {
        match self {
            LdcConstant::Int(value) => Slot::Value(*value),
            LdcConstant::Float(value) => Slot::Value(value.to_bits() as i32),
            LdcConstant::Reference(reference) => Slot::Ref(*reference),
        }
    }

    /// The raw 32 bits, for callers that only move the value around.
    pub(crate) fn raw(&self) -> i32 {
        self.slot().value()
    }
}

#[derive(Debug, Default)]
pub struct LdcResolutionManager {
    cache: RwLock<HashMap<String, HashMap<CPoolIndex, Value>>>,
}

impl LdcResolutionManager {
    pub fn resolve_ldc(&self, current_class_name: &str, cpoolindex: u16) -> Result<LdcConstant> {
        if let Some(Some(value)) = self
            .cache
            .read()?
            .get(current_class_name)
            .map(|map| map.get(&cpoolindex))
        {
            return Ok(value[0]);
        }

        let java_class = CLASSES.get(current_class_name)?;
        let constant_pool = java_class.constant_pool();

        // The entry's kind is settled here, where the constant pool still says what it is, and
        // travels with the value from now on.
        let result = if let Some(value) = constant_pool.get_integer(cpoolindex) {
            LdcConstant::Int(value)
        } else if let Some(value) = constant_pool.get_float(cpoolindex) {
            LdcConstant::Float(value)
        } else if let Some(value) = constant_pool.get_string(cpoolindex) {
            LdcConstant::Reference(StringPoolHelper::get_string(&value)?)
        } else if let Some(class_name) = constant_pool.get_class_name(cpoolindex) {
            LdcConstant::Reference(clazz_ref(&class_name)?)
        } else if let Some(method_type) = constant_pool.get_method_type(cpoolindex) {
            LdcConstant::Reference(build_methodtype_ref(&method_type)?)
        } else if let Some((reference_kind, class_name, name, descriptor)) =
            constant_pool.get_method_handle(cpoolindex)
        {
            LdcConstant::Reference(resolve_method_handle(
                current_class_name,
                ReferenceKind::try_from(reference_kind)?,
                &class_name,
                &name,
                &descriptor,
            )?)
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

    pub fn resolve_ldc2_w(&self, current_class_name: &str, cpoolindex: u16) -> Result<i64> {
        if let Some(Some(value)) = self
            .cache
            .read()?
            .get(current_class_name)
            .map(|map| map.get(&cpoolindex))
        {
            return Ok(Self::constants_to_i64(value));
        }

        let java_class = CLASSES.get(current_class_name)?;
        let constant_pool = java_class.constant_pool();

        let result = if let Some(value) = constant_pool.get_long(cpoolindex) {
            value
        } else if let Some(value) = constant_pool.get_double(cpoolindex) {
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
            .insert(
                cpoolindex,
                i64_to_vec(result)
                    .into_iter()
                    .map(LdcConstant::Int)
                    .collect(),
            );

        Ok(result)
    }

    /// `ldc2_w` constants are always `long`/`double`, stored as their two raw halves.
    fn constants_to_i64(constants: &[LdcConstant]) -> i64 {
        let raw = constants.iter().map(LdcConstant::raw).collect::<Vec<_>>();
        vec_to_i64(&raw)
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

    let args = match reference_kind {
        ReferenceKind::REF_invokeStatic
        | ReferenceKind::REF_invokeInterface
        | ReferenceKind::REF_invokeVirtual => {
            let method_type_ref = build_methodtype_ref(method_or_field_descriptor)?;
            vec![refc.into(), method_name_ref.into(), method_type_ref.into()]
        }
        ReferenceKind::REF_newInvokeSpecial => {
            let method_type_ref = build_methodtype_ref(method_or_field_descriptor)?;
            vec![refc.into(), method_type_ref.into()]
        }
        ReferenceKind::REF_getField => {
            let field_type_ref = clazz_ref(method_or_field_descriptor)?;
            vec![refc.into(), method_name_ref.into(), field_type_ref.into()]
        }
        ReferenceKind::REF_getStatic
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
    let lookup_klass = CLASSES.get("java/lang/invoke/MethodHandles$Lookup")?;
    StaticInit::initialize_java_class(&lookup_klass)?;
    let impl_lookup = lookup_klass
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

#[cfg(test)]
mod tests {
    use super::*;

    /// The distinction the old `i32` representation lost. Both are 32 bits, but one boxes as
    /// `Integer` and the other as `Float`, so collapsing them makes correct boxing impossible.
    #[test]
    fn should_keep_int_and_float_constants_distinct() {
        let one_as_int = LdcConstant::Int(1);
        let one_as_float = LdcConstant::Float(1.0);

        assert_ne!(one_as_int, one_as_float);
        assert_ne!(one_as_int.raw(), one_as_float.raw());
    }

    #[test]
    fn should_place_numeric_constants_in_untagged_slots() {
        assert_eq!(LdcConstant::Int(7).slot(), Slot::Value(7));
        assert_eq!(
            LdcConstant::Float(1.5).slot(),
            Slot::Value(1.5f32.to_bits() as i32)
        );
    }

    #[test]
    fn should_place_resolved_references_in_tagged_slots() {
        assert_eq!(LdcConstant::Reference(9).slot(), Slot::Ref(9));
    }

    /// `ldc` pushes a float as its bit pattern, so the round trip has to be exact.
    #[test]
    fn should_round_trip_a_float_through_its_slot() {
        for value in [0.0f32, -1.5, f32::MIN, f32::MAX, f32::NAN] {
            let bits = LdcConstant::Float(value).slot().value();
            let restored = f32::from_bits(bits as u32);
            assert_eq!(restored.to_bits(), value.to_bits());
        }
    }
}
