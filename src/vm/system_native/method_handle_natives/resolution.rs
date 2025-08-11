use crate::vm::error::{Error, Result};
use crate::vm::heap::heap::{with_heap_read_lock, with_heap_write_lock};
use crate::vm::method_area::method_area::with_method_area;
use crate::vm::system_native::method_handle_natives::member_name::{
    set_reference_kind, MemberName,
};
use crate::vm::system_native::method_handle_natives::method_type::MethodType;
use crate::vm::system_native::method_handle_natives::resolved_method_name::ResolvedMethodName;
use crate::vm::system_native::method_handle_natives::types::ReferenceKind::*;

pub fn resolve(
    member_name_ref: i32,
    _caller_class_ref: i32,
    _lookup_mode: i32,
    _speculative_resolve: bool,
) -> Result<i32> {
    let mut member_name = MemberName::new(member_name_ref)?;
    let reference_kind = member_name.reference_kind();
    match reference_kind {
        REF_invokeVirtual | REF_invokeStatic | REF_invokeSpecial | REF_newInvokeSpecial
        | REF_invokeInterface => resolve_method(&mut member_name),
        REF_getField | REF_putField => Ok(member_name_ref),
        REF_getStatic | REF_putStatic => resolve_static_field(&mut member_name),
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

/// Resolves MemberName instance based on `obj_ref` which is expected to be a `java/lang/reflect/Method`, `java/lang/reflect/Constructor`, or `java/lang/reflect/Field`.
/// It initializes the `vmtarget` and `vmindex` fields of the MemberName
pub(crate) fn member_name_init(member_name_ref: i32, obj_ref: i32) -> Result<()> {
    // MethodHandleNatives.Constants
    const _MN_IS_METHOD: i32 = 0x00010000; // method (not constructor)
    const MN_IS_CONSTRUCTOR: i32 = 0x00020000; // constructor

    let obj_name = with_heap_read_lock(|heap| heap.get_instance_name(obj_ref))?;

    match obj_name.as_str() {
        "java/lang/reflect/Method" => {
            // fill in vmtarget, vmindex while we have Method ref in hand:
            todo!("Implement from Method initialization logic");
        }
        "java/lang/reflect/Constructor" => {
            // fill in vmtarget, vmindex while we have Constructor ref in hand:
            let (clazz_ref, modifiers, slot) = with_heap_read_lock(|h| {
                let clazz_ref = h.get_object_field_value(obj_ref, obj_name.as_str(), "clazz")?[0];
                let modifiers =
                    h.get_object_field_value(obj_ref, obj_name.as_str(), "modifiers")?[0];
                let slot = h.get_object_field_value(obj_ref, obj_name.as_str(), "slot")?[0];

                Ok::<(i32, i32, i32), Error>((clazz_ref, modifiers, slot))
            })?;

            let resolved_method_name =
                ResolvedMethodName::new_create_instance(clazz_ref, slot as i64)?;
            resolved_method_name.propagate_all()?;
            let method_ref = resolved_method_name.resolved_method_name_ref();

            let enriched_with_kind = set_reference_kind(modifiers, REF_newInvokeSpecial);
            let enriched_with_internal_flags = enriched_with_kind | MN_IS_CONSTRUCTOR;

            with_heap_write_lock(|h| {
                h.set_object_field_value(
                    member_name_ref,
                    "java/lang/invoke/MemberName",
                    "clazz",
                    vec![clazz_ref],
                )?;
                h.set_object_field_value(
                    member_name_ref,
                    "java/lang/invoke/MemberName",
                    "flags",
                    vec![enriched_with_internal_flags],
                )?;
                h.set_object_field_value(
                    member_name_ref,
                    "java/lang/invoke/MemberName",
                    "method",
                    vec![method_ref],
                )?;

                Ok::<(), Error>(())
            })?;
        }
        "java/lang/reflect/Field" => {
            // fill in vmtarget, vmindex while we have Field ref in hand:
            todo!("Implement from Field initialization logic")
        }
        _ => {
            return Err(Error::new_execution(&format!(
                "method_handle_natives_init_wrp: Unsupported object type: {obj_name}"
            )));
        }
    }

    Ok(())
}
