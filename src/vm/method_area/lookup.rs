use crate::vm::error::{Error, Result};
use crate::vm::heap::java_instance::{ClassName, FieldNameType};
use crate::vm::method_area::field::FieldValue;
use crate::vm::method_area::java_class::JavaClass;
use crate::vm::method_area::java_method::JavaMethod;
use crate::vm::method_area::loaded_classes::CLASSES;
use indexmap::{IndexMap, IndexSet};
use jdescriptor::TypeDescriptor;
use std::sync::Arc;

/// Resolves a method for the given class by first consulting the per-class vtable
/// cached in [`JavaClass`] and then falling back to a hierarchy/interface walk when
/// needed. This helper is used for general method resolution, not only virtual
/// dispatch; the fallback is also required for polymorphic-signature methods whose
/// descriptor varies per call site.
pub(crate) fn lookup_method(
    class_name: &str,
    full_method_signature: &str,
) -> Result<Option<Arc<JavaMethod>>> {
    let klass = CLASSES.get(class_name)?;
    let vtable = klass.vtable()?;

    if let Some(method) = vtable.get(full_method_signature) {
        return Ok(Some(Arc::clone(method)));
    }

    // Fall back for polymorphic signature methods whose descriptor varies per call site
    Ok(lookup_for_implementation(class_name, full_method_signature)
        .or_else(|| lookup_for_implementation_interface(class_name, full_method_signature)))
}

/// Builds the vtable for `class_name` by collecting all method signatures reachable
/// through the class and interface hierarchy and resolving each to the method
/// selected by normal lookup. Some entries may still be abstract (for example,
/// unresolved interface methods or abstract superclass declarations) and will
/// error at invoke-time if no concrete implementation exists. The result is
/// cached by the caller ([`JavaClass::vtable`]).
pub(crate) fn build_vtable(class_name: &str) -> Result<IndexMap<String, Arc<JavaMethod>>> {
    let mut sigs: IndexSet<String> = IndexSet::new();
    collect_class_and_interface_method_signatures(class_name, &mut sigs)?;

    let mut vtable = IndexMap::new();
    for sig in sigs {
        if let Some(method) = lookup_for_implementation(class_name, &sig)
            .or_else(|| lookup_for_implementation_interface(class_name, &sig))
        {
            vtable.insert(sig, method);
        }
    }
    Ok(vtable)
}

/// Resolves a static field by walking the class/interface hierarchy.
/// Returns the declaring class name together with the field value.
pub(crate) fn lookup_for_static_field(
    class_name: &str,
    field_name: &str,
) -> Result<(String, Arc<FieldValue>)> {
    let klass = CLASSES.get(class_name)?;

    if klass.is_interface() {
        lookup_for_static_field_in_interface(&klass, class_name, field_name)
    } else {
        lookup_for_static_field_in_class(&klass, class_name, field_name)
    }
}

/// Returns the [`TypeDescriptor`] for an instance field by walking up the class hierarchy.
pub(crate) fn lookup_for_field_descriptor(
    class_name: &str,
    field_name: &str,
) -> Option<TypeDescriptor> {
    let klass = CLASSES.get(class_name).ok()?;

    if let Some(type_descriptor) = klass.instance_field_descriptor(field_name) {
        Some(type_descriptor.clone())
    } else {
        let parent_class_name = klass.parent().clone()?;
        lookup_for_field_descriptor(&parent_class_name, field_name)
    }
}

/// Populates `instance_fields_hierarchy` with the ordered per-class field maps by
/// walking up the class hierarchy from the root down to `class_name`.
pub(crate) fn lookup_and_fill_instance_fields_hierarchy(
    class_name: &str,
    instance_fields_hierarchy: &mut IndexMap<ClassName, IndexMap<FieldNameType, FieldValue>>,
) -> Result<()> {
    let klass = CLASSES.get(class_name)?;
    if let Some(parent_class_name) = klass.parent().as_ref() {
        lookup_and_fill_instance_fields_hierarchy(parent_class_name, instance_fields_hierarchy)?;
    }

    let instance_fields = klass.default_value_instance_fields();
    instance_fields_hierarchy.insert(class_name.to_string(), instance_fields);

    Ok(())
}

// ── Private helpers ──────────────────────────────────────────────────────────

fn lookup_for_implementation(
    class_name: &str,
    full_method_signature: &str,
) -> Option<Arc<JavaMethod>> {
    let klass = CLASSES.get(class_name).ok()?;

    if let Some(java_method) = klass.try_get_method(full_method_signature) {
        Some(Arc::clone(&java_method))
    } else {
        let parent_class_name = klass.parent().as_ref()?;
        lookup_for_implementation(parent_class_name, full_method_signature)
    }
}

fn lookup_for_implementation_interface(
    class_name: &str,
    full_method_signature: &str,
) -> Option<Arc<JavaMethod>> {
    let klass = CLASSES.get(class_name).ok()?;
    if let Some(java_method) =
        // lookup in interfaces for default methods
        lookup_in_interface_hierarchy(klass.interfaces(), full_method_signature)
    {
        return Some(java_method);
    }

    // if not found in interfaces of current class, lookup in parent class
    let parent_class_name = klass.parent().as_ref()?;
    lookup_for_implementation_interface(parent_class_name, full_method_signature)
}

fn lookup_in_interface_hierarchy(
    interfaces: &IndexSet<String>,
    full_method_signature: &str,
) -> Option<Arc<JavaMethod>> {
    for interface_name in interfaces.iter() {
        if let Some(interface_class) = CLASSES.get(interface_name).ok() {
            if let Some(java_method) = interface_class.try_get_method(full_method_signature) {
                return Some(java_method);
            }

            if let Some(java_method) =
                lookup_in_interface_hierarchy(interface_class.interfaces(), full_method_signature)
            {
                return Some(java_method);
            }
        }
    }

    None
}

fn collect_class_and_interface_method_signatures(
    class_name: &str,
    sigs: &mut IndexSet<String>,
) -> Result<()> {
    let mut visited = IndexSet::new();
    collect_class_and_interface_method_signatures_inner(class_name, sigs, &mut visited)
}

fn collect_class_and_interface_method_signatures_inner(
    class_name: &str,
    sigs: &mut IndexSet<String>,
    visited: &mut IndexSet<String>,
) -> Result<()> {
    if !visited.insert(class_name.to_string()) {
        // Class already visited, so we've already collected all of its methods.
        return Ok(());
    }

    let klass = CLASSES.get(class_name)?;
    for method in klass.get_methods() {
        sigs.insert(method.name_signature().to_string());
    }
    if let Some(parent) = klass.parent() {
        collect_class_and_interface_method_signatures_inner(parent, sigs, visited)?;
    }
    for iface in klass.interfaces() {
        collect_interface_method_signatures_inner(iface, sigs, visited)?;
    }
    Ok(())
}

fn collect_interface_method_signatures_inner(
    interface_name: &str,
    sigs: &mut IndexSet<String>,
    visited: &mut IndexSet<String>,
) -> Result<()> {
    if !visited.insert(interface_name.to_string()) {
        // Interface already visited, so we've already collected all of its methods.
        return Ok(());
    }

    let klass = CLASSES.get(interface_name)?;
    for method in klass.get_methods() {
        sigs.insert(method.name_signature().to_string());
    }
    for super_iface in klass.interfaces() {
        collect_interface_method_signatures_inner(super_iface, sigs, visited)?;
    }
    Ok(())
}

fn lookup_for_static_field_in_class(
    klass: &Arc<JavaClass>,
    class_name: &str,
    field_name: &str,
) -> Result<(String, Arc<FieldValue>)> {
    match klass.static_field(field_name) {
        Some(field) => Ok((class_name.to_string(), Arc::clone(&field))),
        None => match klass.parent() {
            Some(parent_class_name) => lookup_for_static_field(&parent_class_name, field_name),
            None => Err(Error::new_execution(&format!(
                "No field {class_name}.{field_name} found in class hierarchy"
            ))),
        },
    }
}

fn lookup_for_static_field_in_interface(
    klass: &Arc<JavaClass>,
    class_name: &str,
    field_name: &str,
) -> Result<(String, Arc<FieldValue>)> {
    match klass.static_field(field_name) {
        Some(field) => Ok((class_name.to_string(), Arc::clone(&field))),
        None => {
            let interfaces = klass.interfaces();
            for interface_name in interfaces.iter() {
                match lookup_for_static_field(interface_name, field_name) {
                    Ok((interface_class_name, field)) => {
                        return Ok((interface_class_name, field));
                    }
                    Err(_) => continue, // todo: 1. add test for this case; 2. refactor this approach
                }
            }

            Err(Error::new_execution(&format!(
                "No field {class_name}.{field_name} found in class hierarchy"
            )))
        }
    }
}
