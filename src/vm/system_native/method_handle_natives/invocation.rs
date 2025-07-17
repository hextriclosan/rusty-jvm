use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::common::last_frame_mut;
use crate::vm::execution_engine::invoker::invoke;
use crate::vm::execution_engine::static_init::StaticInit;
use crate::vm::heap::heap::{with_heap_read_lock, with_heap_write_lock};
use crate::vm::method_area::field::FieldValue;
use crate::vm::method_area::java_method::JavaMethod;
use crate::vm::method_area::method_area::with_method_area;
use crate::vm::stack::stack_frame::StackFrames;
use crate::vm::system_native::method_handle_natives::member_name::MemberName;
use crate::vm::system_native::method_handle_natives::member_name::ReferenceKind::*;
use crate::vm::system_native::method_handle_natives::offsets::{
    get_field_offset, get_static_field_offset,
};
use std::sync::Arc;

const DIRECT_METHOD_HANDLE: &'static str = "java/lang/invoke/DirectMethodHandle";
const BOUND_METHOD_HANDLE: &'static str = "java/lang/invoke/BoundMethodHandle";
const MUTABLE_CALL_SITE: &'static str = "java/lang/invoke/MutableCallSite";

pub fn invoke_exact(
    handle_ref: i32,
    method_args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<()> {
    let handle_name = with_heap_read_lock(|heap| heap.get_instance_name(handle_ref))?;
    if handle_name.starts_with(DIRECT_METHOD_HANDLE) {
        return direct_method_handle_invocation(
            handle_ref,
            method_args,
            stack_frames,
            &handle_name,
        );
    } else if handle_name.starts_with(BOUND_METHOD_HANDLE) {
        return bound_method_handle_invocation(
            handle_ref,
            method_args,
            stack_frames,
            &handle_name,
        );
    } else if handle_name == MUTABLE_CALL_SITE {
        // todo: ends with CallSite?
        return mutable_call_site_invocation(handle_ref, method_args, stack_frames, &handle_name);
    }

    unimplemented!("invoke_exact: handle: {} is not supported", handle_name)
}

fn direct_method_handle_invocation(
    handle_ref: i32,
    method_args: &[i32],
    stack_frames: &mut StackFrames,
    handle_name: &String,
) -> Result<()> {
    let member_name_ref = with_heap_read_lock(|heap| {
        heap.get_object_field_value(handle_ref, &handle_name, "member")
    })?[0];
    let member_name = MemberName::new(member_name_ref)?;
    let reference_kind = member_name.reference_kind();

    match reference_kind {
        REF_invokeStatic | REF_newInvokeSpecial | REF_getStatic | REF_putStatic => {
            StaticInit::initialize(member_name.class_name())?
        }
        _ => {}
    }

    match reference_kind {
        REF_invokeVirtual | REF_invokeStatic | REF_invokeSpecial | REF_newInvokeSpecial => {
            invoke_exact_method(&member_name, method_args, stack_frames)
        }
        REF_getField => invoke_exact_get_field(&member_name, method_args, stack_frames),
        REF_putField => invoke_exact_put_field(&member_name, method_args),
        REF_getStatic => invoke_exact_get_static_field(&member_name, method_args, stack_frames),
        REF_putStatic => invoke_exact_put_static_field(&member_name, method_args),
        _ => unimplemented!("reference_kind: {:?}", reference_kind),
    }
}

fn bound_method_handle_invocation(
    handle_ref: i32,
    method_args: &[i32],
    stack_frames: &mut StackFrames,
    handle_name: &String,
) -> Result<()> {
    if handle_name == "java/lang/invoke/BoundMethodHandle$Species_LL" {
        let _arg_l0 = with_heap_read_lock(|heap| {
            heap.get_object_field_value(handle_ref, handle_name, "argL0")
        })?[0];
        let arg_l1 = with_heap_read_lock(|heap| {
            heap.get_object_field_value(handle_ref, handle_name, "argL1")
        })?[0];

        return invoke_exact(arg_l1, method_args, stack_frames);
    }

    unimplemented!("bound_method_handle_invocation: {}", handle_name)
}

fn mutable_call_site_invocation(
    mutable_call_site_ref: i32,
    method_args: &[i32],
    stack_frames: &mut StackFrames,
    handle_name: &String,
) -> Result<()> {
    let target_ref = with_heap_read_lock(|heap| {
        // todo: get with getTarget() getter
        heap.get_object_field_value(mutable_call_site_ref, handle_name, "target")
    })?[0];

    invoke_exact(target_ref, method_args, stack_frames)
}

fn invoke_exact_method(
    member_name: &MemberName,
    method_args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<()> {
    let resolved_method_name = member_name.method().ok_or_else(|| {
        Error::new_execution(&format!("field `method` not found in {member_name:?}"))
    })?;

    let vmholder_class_ref = resolved_method_name.vmholder();
    let method_index = resolved_method_name.vmtarget();
    let (class_name, method_to_invoke) = with_method_area(|method_area| {
        let class_name = method_area.get_from_reflection_table(vmholder_class_ref)?;
        let jc = method_area.get(&class_name)?;
        Ok::<(String, Arc<JavaMethod>), Error>((class_name, jc.get_method_by_index(method_index)?))
    })?;

    let reference_kind = member_name.reference_kind();
    let method_args = if reference_kind == REF_newInvokeSpecial {
        // bellow we mimic the behavior of the NEW opcode
        let instance_with_default_fields = with_method_area(|method_area| {
            method_area.create_instance_with_default_fields(&class_name)
        })?;

        let instanceref =
            with_heap_write_lock(|heap| heap.create_instance(instance_with_default_fields));

        let stack_frame = last_frame_mut(stack_frames)?;
        stack_frame.push(instanceref)?; // NEW opcode

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

fn invoke_exact_get_field(
    member_name: &MemberName,
    method_args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<()> {
    let (instance_ref, class_name, field_name, _) = prepare_field(member_name, method_args)?;

    let value = with_heap_read_lock(|heap| {
        heap.get_object_field_value(instance_ref, &class_name, &field_name)
    })?;
    let last_frame = last_frame_mut(stack_frames)?;
    value.iter().try_for_each(|val| last_frame.push(*val))?;

    Ok(())
}

fn invoke_exact_put_field(member_name: &MemberName, method_args: &[i32]) -> Result<()> {
    let (instance_ref, class_name, field_name, args) = prepare_field(member_name, method_args)?;

    with_heap_write_lock(|heap| {
        heap.set_object_field_value(instance_ref, &class_name, &field_name, args)
    })
}

fn invoke_exact_get_static_field(
    member_name: &MemberName,
    method_args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<()> {
    let (field, _args) = prepare_static_field(member_name, method_args)?;
    let value = field.raw_value()?;
    let last_frame = last_frame_mut(stack_frames)?;
    value.iter().try_for_each(|val| last_frame.push(*val))?;

    Ok(())
}

fn invoke_exact_put_static_field(member_name: &MemberName, method_args: &[i32]) -> Result<()> {
    let (field, args) = prepare_static_field(member_name, method_args)?;
    field.set_raw_value(args)
}

fn prepare_field(
    member_name: &MemberName,
    method_args: &[i32],
) -> Result<(i32, String, String, Vec<i32>)> {
    let instance_ref = method_args[0];
    let args = method_args[1..].to_vec();
    let class_name = with_heap_read_lock(|heap| heap.get_instance_name(instance_ref))?;

    let jc = with_method_area(|area| area.get(class_name.as_str()))?;
    let member_name_ref = member_name.member_name_ref();
    let offset = get_field_offset(member_name_ref)?;
    let (class_name, field_name) = jc.get_field_name_by_offset(offset)?;
    Ok((instance_ref, class_name, field_name, args))
}

fn prepare_static_field(
    member_name: &MemberName,
    method_args: &[i32],
) -> Result<(Arc<FieldValue>, Vec<i32>)> {
    let args = method_args.to_vec();
    let class_name = member_name.class_name();

    let jc = with_method_area(|area| area.get(class_name.as_str()))?;
    let member_name_ref = member_name.member_name_ref();
    let offset = get_static_field_offset(member_name_ref)?;
    let field = jc.get_static_field_by_offset(offset)?;
    Ok((field, args))
}
