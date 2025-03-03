use crate::error::Error;
use crate::execution_engine::common::last_frame_mut;
use crate::execution_engine::executor::Executor;
use crate::execution_engine::invoker::invoke;
use crate::heap::heap::{with_heap_read_lock, with_heap_write_lock};
use crate::helper::{i64_to_vec, vec_to_i64};
use crate::method_area::java_method::JavaMethod;
use crate::method_area::method_area::with_method_area;
use crate::method_area::primitives_helper::PRIMITIVE_TYPE_BY_CODE;
use crate::stack::stack_frame::StackFrames;
use crate::system_native::string::get_utf8_string_by_ref;
use num_enum::TryFromPrimitive;
use std::sync::Arc;

const RESOLVED_METHOD_NAME: &'static str = "java/lang/invoke/ResolvedMethodName";
const METHOD_TYPE: &'static str = "java/lang/invoke/MethodType";
const MEMBER_NAME: &'static str = "java/lang/invoke/MemberName";
const DIRECT_METHOD_HANDLE: &'static str = "java/lang/invoke/DirectMethodHandle";

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
enum ReferenceKind {
    REF_getField = 1,
    REF_getStatic = 2,
    REF_putField = 3,
    REF_putStatic = 4,
    REF_invokeVirtual = 5,
    REF_invokeStatic = 6,
    REF_invokeSpecial = 7,
    REF_newInvokeSpecial = 8,
    REF_invokeInterface = 9,
}

pub(crate) fn method_handle_natives_resolve_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let member_mame_ref = args[0];
    let caller_class_ref = args[1];
    let lookup_mode = args[2];
    let speculative_resolve = args[3] != 0;
    let member_mame_ref = resolve(
        member_mame_ref,
        caller_class_ref,
        lookup_mode,
        speculative_resolve,
    )?;
    Ok(vec![member_mame_ref])
}

/// Sets MemberName.method instance of ResolvedMethodName with injected `long vmtarget` (which is a pointer to the target method's vtable (Method* in HotSpot)).
/// Also sets ResolvedMethodName.vmholder - the class that declares the resolved method.
fn resolve(
    member_name_ref: i32,
    _caller_class_ref: i32,
    _lookup_mode: i32,
    _speculative_resolve: bool,
) -> crate::error::Result<i32> {
    let (current_flags, class_ref, name_ref, type_obj_ref) = with_heap_read_lock(|heap| {
        let current_flags = heap.get_object_field_value(member_name_ref, MEMBER_NAME, "flags")?[0];
        let class_ref = heap.get_object_field_value(member_name_ref, MEMBER_NAME, "clazz")?[0];
        let name_ref = heap.get_object_field_value(member_name_ref, MEMBER_NAME, "name")?[0];
        let type_obj_ref = heap.get_object_field_value(member_name_ref, MEMBER_NAME, "type")?[0];
        Ok::<(i32, i32, i32, i32), Error>((current_flags, class_ref, name_ref, type_obj_ref))
    })?;

    let (rtype_class_ref, ptype_class_refs) = with_heap_read_lock(|heap| {
        let rtype_class_ref = heap.get_object_field_value(type_obj_ref, METHOD_TYPE, "rtype")?[0];
        let ptype_class_refs =
            heap.get_object_field_value(type_obj_ref, METHOD_TYPE, "ptypes")?[0];
        Ok::<(i32, i32), Error>((rtype_class_ref, ptype_class_refs))
    })?;

    let len = with_heap_read_lock(|heap| heap.get_array_len(ptype_class_refs))?;
    let mut ptype_names = Vec::with_capacity(len as usize);
    for i in 0..len {
        let ptype_class_ref =
            with_heap_read_lock(|heap| heap.get_array_value(ptype_class_refs, i))?[0];
        let ptype_name = with_method_area(|area| area.get_from_reflection_table(ptype_class_ref))?;
        ptype_names.push(decorate(ptype_name));
    }
    let result = format!("({})", ptype_names.join(""));

    let rtype_name = with_method_area(|area| area.get_from_reflection_table(rtype_class_ref))?;

    let class_name = with_method_area(|area| area.get_from_reflection_table(class_ref))?;
    let arc = with_method_area(|area| area.get(class_name.as_str()))?;

    let rtype_name = decorate(rtype_name);

    let name = get_utf8_string_by_ref(name_ref)?;
    let full_method_signature = name + ":" + result.as_str() + &rtype_name;

    let (method_index, method) = match arc.get_method_full(&full_method_signature) {
        Some((method_index, method)) => (method_index, method),
        None => {
            return Ok(0); // return null ref if method not found
        }
    };

    let flags_to_enrich_with = method.access_flags();
    let enriched_flags = current_flags | flags_to_enrich_with;
    with_heap_write_lock(|heap| {
        heap.set_object_field_value(member_name_ref, MEMBER_NAME, "flags", vec![enriched_flags])
    })?;

    //0. ResolvedMethodName resolvedMethodName = new ResolvedMethodName()
    let resolved_method_name_ref = Executor::invoke_default_constructor(RESOLVED_METHOD_NAME)?;

    //1. resolvedMethodName.vmholder = methodName.clazz;
    let this_class_name = arc.this_class_name();
    let reflection_ref =
        with_method_area(|method_area| method_area.load_reflection_class(this_class_name))?;
    with_heap_write_lock(|heap| {
        heap.set_object_field_value(
            resolved_method_name_ref,
            RESOLVED_METHOD_NAME,
            "vmholder",
            vec![reflection_ref],
        )
    })?;

    //2. resolvedMethodName.vmtarget = ref to the target method's vtable (Method* in HotSpot), index in the IndexMap in our case
    with_heap_write_lock(|heap| {
        heap.set_object_field_value(
            resolved_method_name_ref,
            RESOLVED_METHOD_NAME,
            "vmtarget",
            i64_to_vec(method_index as i64),
        )
    })?;

    with_heap_write_lock(|heap| {
        heap.set_object_field_value(
            member_name_ref,
            MEMBER_NAME,
            "method",
            vec![resolved_method_name_ref],
        )
    })?;

    Ok(member_name_ref)
}

pub(crate) fn method_handle_invoke_exact_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> crate::error::Result<Vec<i32>> {
    let direct_method_handle_ref = args[0]; // todo: check if it is DirectMethodHandle
    let method_args = &args[1..];
    invoke_exact(direct_method_handle_ref, method_args, stack_frames)?;
    Ok(vec![])
}
fn invoke_exact(
    handle_ref: i32,
    method_args: &[i32],
    stack_frames: &mut StackFrames,
) -> crate::error::Result<()> {
    let handle_name = with_heap_read_lock(|heap| heap.get_instance_name(handle_ref))?;
    if !handle_name.starts_with(DIRECT_METHOD_HANDLE) {
        return Err(Error::new_execution(&format!(
            "error invoking exact: handle: {} is not DirectMethodHandle",
            handle_name
        )));
    }

    let (flags, member_name_ref) = with_heap_read_lock(|heap| {
        let member_name_ref = heap.get_object_field_value(handle_ref, &handle_name, "member")?[0];
        let flags = heap.get_object_field_value(member_name_ref, MEMBER_NAME, "flags")?[0];
        Ok::<(i32, i32), Error>((flags, member_name_ref))
    })?;
    let reference_kind = get_reference_kind(flags)?;

    let resolved_method_name_ref = with_heap_read_lock(|heap| {
        heap.get_object_field_value(member_name_ref, MEMBER_NAME, "method")
    })?[0];

    let (vmtarget_raw, vmholder_class_ref) = with_heap_read_lock(|heap| {
        let vmtarget_raw = heap.get_object_field_value(
            resolved_method_name_ref,
            RESOLVED_METHOD_NAME,
            "vmtarget",
        )?;
        let vmholder_class_ref = heap.get_object_field_value(
            resolved_method_name_ref,
            RESOLVED_METHOD_NAME,
            "vmholder",
        )?[0];
        Ok::<(Vec<i32>, i32), Error>((vmtarget_raw, vmholder_class_ref))
    })?;
    let method_index = vec_to_i64(&vmtarget_raw);

    let (class_name, method_to_invoke) = with_method_area(|method_area| {
        let class_name = method_area.get_from_reflection_table(vmholder_class_ref)?;
        let jc = method_area.get(&class_name)?;
        Ok::<(String, Arc<JavaMethod>), Error>((class_name, jc.get_method_by_index(method_index)?))
    })?;

    let method_args = if reference_kind == ReferenceKind::REF_newInvokeSpecial {
        // bellow we mimic the behavior of the NEW opcode
        let instance_with_default_fields = with_method_area(|method_area| {
            method_area.create_instance_with_default_fields(&class_name)
        })?;

        let instanceref =
            with_heap_write_lock(|heap| heap.create_instance(instance_with_default_fields));

        let stack_frame = last_frame_mut(stack_frames)?;
        stack_frame.push(instanceref);
        stack_frame.push(instanceref);

        let method_args = std::iter::once(instanceref)
            .chain(method_args.iter().cloned())
            .collect::<Vec<_>>();
        method_args
    } else {
        method_args.to_owned()
    };

    invoke(
        stack_frames,
        method_to_invoke.name_signature(),
        method_args.as_slice(),
        Arc::clone(&method_to_invoke),
        &class_name,
    )
}

/**
 * Mimics MemberName.getReferenceKind():
 *  public byte getReferenceKind() {
 *      return (byte) ((flags >>> MN_REFERENCE_KIND_SHIFT) & MN_REFERENCE_KIND_MASK);
 *  }
 */
fn get_reference_kind(flags: i32) -> crate::error::Result<ReferenceKind> {
    let kind_shift = 24u32;
    let kind_mask = 0x0F000000u32 >> kind_shift;
    let result = (flags as u32 >> kind_shift) & kind_mask;

    ReferenceKind::try_from(result as u8).map_err(|e| {
        Error::new_execution(&format!("error converting flags to ReferenceKind: {}", e))
    })
}

fn decorate(type_name: String) -> String {
    if PRIMITIVE_TYPE_BY_CODE.contains_key(type_name.as_str())
        || (type_name.starts_with('L') && type_name.ends_with(';'))
    {
        type_name
    } else {
        format!("L{};", type_name)
    }
}
