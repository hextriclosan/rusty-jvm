use crate::vm::error::{Error, Result};
use crate::vm::method_area::method_area::with_method_area;
use crate::vm::system_native::method_handle_natives::member_name::MemberName;
use crate::vm::system_native::method_handle_natives::member_name::ReferenceKind::*;
use crate::vm::system_native::method_handle_natives::method_type::MethodType;
use crate::vm::system_native::method_handle_natives::resolved_method_name::ResolvedMethodName;

pub fn resolve(
    member_name_ref: i32,
    _caller_class_ref: i32,
    _lookup_mode: i32,
    _speculative_resolve: bool,
) -> Result<i32> {
    let mut member_name = MemberName::new(member_name_ref)?;
    let reference_kind = member_name.reference_kind();
    match reference_kind {
        REF_invokeVirtual | REF_invokeStatic | REF_invokeSpecial | REF_newInvokeSpecial => {
            resolve_method(&mut member_name)
        }
        REF_getField | REF_putField => Ok(member_name_ref),
        REF_getStatic | REF_putStatic => resolve_static_field(&mut member_name),
        _ => unimplemented!("reference_kind: {:?}", reference_kind),
    }
}

/// Sets MemberName.method instance of ResolvedMethodName with injected `long vmtarget` (which is a pointer to the target method's vtable (Method* in HotSpot)).
/// Also sets ResolvedMethodName.vmholder - the class that declares the resolved method.
fn resolve_method(member_name: &mut MemberName) -> Result<i32> {
    let type_obj_ref = member_name.type_obj_ref();
    let method_type = MethodType::new(type_obj_ref)?;
    let method_name = member_name.name();
    let ptype_names = method_type.ptype_names();
    let rtype_name = method_type.rtype_name();
    let full_method_signature = format!("{method_name}:{ptype_names}{rtype_name}");

    let class_name = member_name.class_name();
    let arc = with_method_area(|area| area.get(class_name))?;
    let (method_index, method) = match arc.get_method_full(&full_method_signature) {
        Some((method_index, method)) => (method_index, method),
        None => {
            return Ok(0); // return null-ref if method not found
        }
    };

    let current_flags = member_name.flags();
    let flags_to_enrich_with = method.access_flags();
    let enriched_flags = current_flags | flags_to_enrich_with;
    member_name.propagate_flags(enriched_flags)?;

    let this_class_name = arc.this_class_name();
    let reflection_ref =
        with_method_area(|method_area| method_area.load_reflection_class(this_class_name))?;

    let resolved_method_name =
        ResolvedMethodName::new_create_instance(reflection_ref, method_index as i64)?;
    member_name.propagate_method(resolved_method_name)?;

    Ok(member_name.member_name_ref())
}

fn resolve_static_field(member_name: &mut MemberName) -> Result<i32> {
    let class_name = member_name.class_name();
    let static_field_name = member_name.name();

    let jc = with_method_area(|area| area.get(class_name))?;
    let field_info = jc.field_info(static_field_name).ok_or_else(|| {
        Error::new_execution(&format!(
            "Static field not found: {class_name}:{static_field_name}"
        ))
    })?; // todo throw NoSuchFieldError if field not found

    let current_flags = member_name.flags();
    let flags_to_enrich_with = field_info.flags() as i32;
    let enriched_flags = current_flags | flags_to_enrich_with;
    member_name.propagate_flags(enriched_flags)?;

    Ok(member_name.member_name_ref())
}
