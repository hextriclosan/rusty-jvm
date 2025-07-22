use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::common::last_frame_mut;
use crate::vm::execution_engine::invoker::invoke;
use crate::vm::execution_engine::static_init::StaticInit;
use crate::vm::heap::heap::{with_heap_read_lock, with_heap_write_lock};
use crate::vm::helper::vec_to_i64;
use crate::vm::method_area::field::FieldValue;
use crate::vm::method_area::java_method::JavaMethod;
use crate::vm::method_area::method_area::with_method_area;
use crate::vm::stack::stack_frame::StackFrames;
use crate::vm::system_native::method_handle_natives::member_name::MemberName;
use crate::vm::system_native::method_handle_natives::member_name::ReferenceKind::*;
use crate::vm::system_native::method_handle_natives::method_type::MethodType;
use crate::vm::system_native::method_handle_natives::offsets::{
    get_field_offset, get_static_field_offset,
};
use once_cell::sync::Lazy;
use std::env;
use std::sync::Arc;

const DIRECT_METHOD_HANDLE: &'static str = "java/lang/invoke/DirectMethodHandle";
const BOUND_METHOD_HANDLE: &'static str = "java/lang/invoke/BoundMethodHandle";
const MUTABLE_CALL_SITE: &'static str = "java/lang/invoke/MutableCallSite";

static PRINT_SPECIES: Lazy<bool> = Lazy::new(|| env::var("PRINT_SPECIES").is_ok());

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
    if *PRINT_SPECIES {
        print_species(handle_ref, 0)?;
    }

    let lambda_form_ref =
        with_heap_read_lock(|heap| heap.get_object_field_value(handle_ref, handle_name, "form"))?
            [0];

    let vmentry_ref = with_heap_read_lock(|heap| {
        heap.get_object_field_value(lambda_form_ref, "java/lang/invoke/LambdaForm", "vmentry")
    })?[0];

    let member_name = MemberName::new(vmentry_ref)?;
    let class_name_to_load = member_name.class_name();

    let jc = with_method_area(|method_area| method_area.get(class_name_to_load))?;

    // todo: hide this block into MemberName struct
    let type_obj_ref = member_name.type_obj_ref();
    let method_type = MethodType::new(type_obj_ref)?;
    let method_name = member_name.name();
    let ptype_names = method_type.ptype_names();
    let rtype_name = method_type.rtype_name();
    let full_method_signature = format!("{method_name}:{ptype_names}{rtype_name}");

    let method_to_invoke = Arc::clone(
        jc.get_methods()
            .iter()
            .find(|m| m.name_signature() == full_method_signature)
            .ok_or(Error::new_execution(&format!(
                "bound_method_handle_invocation: {full_method_signature} not found"
            )))?,
    );

    let mut new_args = Vec::new();
    new_args.push(handle_ref);
    new_args.extend_from_slice(method_args);

    invoke(
        stack_frames,
        method_to_invoke.name_signature(),
        &new_args,
        Arc::clone(&method_to_invoke),
        &class_name_to_load,
    )
}

fn print_species(handle_ref: i32, indent: usize) -> Result<()> {
    let ind = " ".repeat(indent * 2);
    let handle_name = with_heap_read_lock(|heap| heap.get_instance_name(handle_ref))?;
    eprint!("{ind}{handle_name} ");
    if handle_name.starts_with(DIRECT_METHOD_HANDLE) {
        let member_name_ref = with_heap_read_lock(|heap| {
            heap.get_object_field_value(handle_ref, &handle_name, "member")
        })?[0];
        let member_name = MemberName::new(member_name_ref)?;
        eprintln!("{}.{}", member_name.class_name(), member_name.name());
    } else if handle_name.starts_with(BOUND_METHOD_HANDLE) {
        let string = handle_name.replace("java/lang/invoke/BoundMethodHandle$Species_", "");
        let args_name = string
            .chars()
            .enumerate()
            .map(|(i, c)| (format!("arg{c}{i}"), c))
            .collect::<Vec<_>>();

        eprintln!();
        for (c, type_) in args_name {
            match type_ {
                'L' => {
                    let arg_ref = with_heap_read_lock(|heap| {
                        heap.get_object_field_value(handle_ref, &handle_name, &c)
                    })?[0];
                    print_species(arg_ref, indent + 1)?;
                }
                'J' => {
                    let raw = with_heap_read_lock(|heap| {
                        heap.get_object_field_value(handle_ref, &handle_name, &c)
                    })?;
                    let long = vec_to_i64(&raw);
                    eprintln!("{ind}  long={long}")
                }
                _ => unimplemented!(),
            }
        }
    } else if handle_name == "java/lang/invoke/MethodType" {
        let type_obj_ref = handle_ref;
        let method_type = MethodType::new(type_obj_ref)?;

        eprintln!(
            "{ind}  MethodType: ptype_names={}, rtype_name={}",
            method_type.ptype_names(),
            method_type.rtype_name()
        );
    } else if handle_name == MUTABLE_CALL_SITE {
        let mutable_call_site_ref = handle_ref;
        let target_ref = with_heap_read_lock(|heap| {
            // todo: get with getTarget() getter
            heap.get_object_field_value(mutable_call_site_ref, handle_name.as_str(), "target")
        })?[0];

        print_species(target_ref, indent + 1)?;
    } else {
        unimplemented!(
            "print_species: handle_name: {} is not supported",
            handle_name
        );
    }

    Ok(())
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
    let (resolved_class_name, resolved_method_to_invoke) = extract_resolved(member_name)?;

    let reference_kind = member_name.reference_kind();
    // todo: merge me with opcodes logic including proper exception handling
    // todo: cover functionality below with test within MethodHandleExample
    let (method_to_invoke, method_args) = match reference_kind {
        REF_invokeVirtual => {
            let reference = method_args
                .first()
                .ok_or_else(|| Error::new_execution("Error getting reference from method_args"))?;
            let instance_type_class_name =
                with_heap_read_lock(|heap| heap.get_instance_name(*reference))?;

            let type_obj_ref = member_name.type_obj_ref();
            let method_type = MethodType::new(type_obj_ref)?;
            let method_name = member_name.name();
            let ptype_names = method_type.ptype_names();
            let rtype_name = method_type.rtype_name();
            let full_method_signature = format!("{method_name}:{ptype_names}{rtype_name}");

            let java_method = with_method_area(|method_area| {
                method_area
                    .lookup_for_implementation(&instance_type_class_name, &full_method_signature) // first looking for method in parent and above classes
                    .or_else(|| { // if not found, looking for default method implementation in interfaces
                        method_area.lookup_for_implementation_interface(
                            &instance_type_class_name,
                            &full_method_signature,
                        )
                    })
            }).ok_or_else(|| Error::new_constant_pool(&format!(
                "Error getting instance type JavaMethod by class name {instance_type_class_name} and full signature {full_method_signature} getting virtual_method"
            )))?;

            (Arc::clone(&java_method), method_args.to_owned())
        }
        REF_invokeSpecial => (
            Arc::clone(&resolved_method_to_invoke),
            method_args.to_owned(),
        ),
        REF_invokeStatic => (
            Arc::clone(&resolved_method_to_invoke),
            method_args.to_owned(),
        ),
        REF_invokeInterface => {
            todo!()
        }
        REF_newInvokeSpecial => (
            Arc::clone(&resolved_method_to_invoke),
            mimic_new(&resolved_class_name, method_args, stack_frames)?,
        ),
        REF_getField | REF_putField | REF_getStatic | REF_putStatic => {
            unreachable!(
                "Reference kind `{:?}` is not supposed to be used with invoke_exact_method",
                reference_kind
            );
        }
    };

    invoke(
        stack_frames,
        method_to_invoke.name_signature(),
        method_args.as_slice(),
        Arc::clone(&method_to_invoke),
        method_to_invoke.class_name(),
    )
}

/// Extracts the resolved method name and its class from the `MemberName`.
/// Returns a tuple containing the class name and an `Arc` to the `JavaMethod`.
///
/// Depending on reference_kind returned values will be used in different ways:
/// - REF_invokeVirtual: exact implementation of the method will be invoked by the instance type (which is got from the first argument of the method)
/// - REF_invokeSpecial: exact implementation of the method will be invoked by the class that declares the method (or its superclass(es))
/// - REF_invokeStatic: exact implementation of the method will be invoked by the class that declares the method
/// - REF_invokeInterface: exact implementation of the method will be invoked by the class that implements the interface
/// - REF_newInvokeSpecial: the method will be invoked by the class that declares the method, but the first argument will be an instance of the class that declares the method
fn extract_resolved(member_name: &MemberName) -> Result<(String, Arc<JavaMethod>)> {
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

    Ok((class_name, method_to_invoke))
}

fn mimic_new(
    class_name: &str,
    method_args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
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

    Ok(method_args)
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
