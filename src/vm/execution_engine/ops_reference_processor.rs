use crate::unwrap_result_or_return_default;
use crate::vm::error::{Error, Result};
use crate::vm::exception::common::throw_exception_with_ref;
use crate::vm::exception::helpers::throw_null_pointer_exception_with_message;
use crate::vm::exception::throwing_result::ThrowingResult;
use crate::vm::execution_engine::common::{last_frame_mut, store_ex_pc};
use crate::vm::execution_engine::invoker::invoke;
use crate::vm::execution_engine::opcode::*;
use crate::vm::execution_engine::static_init::StaticInit;
use crate::vm::heap::heap::{with_heap_read_lock, with_heap_write_lock};
use crate::vm::helper::{argument_length, get_length};
use crate::vm::method_area::cpool_helper::{CPoolHelper, CPoolHelperTrait};
use crate::vm::method_area::field::FieldValue;
use crate::vm::method_area::instance_checker::InstanceChecker;
use crate::vm::method_area::method_area::with_method_area;
use crate::vm::stack::stack_frame::{StackFrame, StackFrames};
use jdescriptor::MethodDescriptor;
use std::sync::Arc;
use tracing::trace;

pub(crate) fn process(
    code: u8,
    current_class_name: &str,
    stack_frames: &mut StackFrames,
) -> Result<()> {
    match code {
        GETSTATIC => {
            let stack_frame = last_frame_mut(stack_frames)?;
            let (class_name, field_name) = get_field_info(stack_frame, current_class_name)?;

            let (fields_class_name, field) = with_method_area(|method_area| {
                method_area.lookup_for_static_field(&class_name, &field_name)
            })?;

            StaticInit::initialize(&fields_class_name)?;

            field
                .raw_value()?
                .iter()
                .rev()
                .try_for_each(|x| stack_frame.push(*x))?;

            stack_frame.incr_pc();
            trace!(
                "GETSTATIC -> {class_name}.{field_name} is {:?}",
                field.raw_value()
            );
        }
        PUTSTATIC => {
            let stack_frame = last_frame_mut(stack_frames)?;
            let (class_name, field_name) = get_field_info(stack_frame, current_class_name)?;

            let (len, fields_class_name, field_value) = with_method_area(|method_area| {
                let (fields_class_name, field_value) =
                    method_area.lookup_for_static_field(&class_name, &field_name)?;

                let jc = method_area.get(&fields_class_name)?;
                let field_info = jc
                    .field_info(&field_name)
                    .ok_or(Error::new_execution("Error getting field info"))?;
                let len = get_length(field_info.type_descriptor())?;

                Ok::<(i32, String, Arc<FieldValue>), Error>((len, fields_class_name, field_value))
            })?;

            StaticInit::initialize(&fields_class_name)?;

            let mut value = Vec::with_capacity(len as usize);
            for _ in 0..len {
                value.push(stack_frame.pop());
            }

            field_value.set_raw_value(value.clone())?;

            stack_frame.incr_pc();
            trace!("PUTSTATIC -> {class_name}.{field_name} = {value:?}");
        }
        GETFIELD => {
            let (class_name, field_name, objectref) = {
                let stack_frame = last_frame_mut(stack_frames)?;
                let (class_name, field_name) = get_field_info(stack_frame, current_class_name)?;
                let objectref = stack_frame.pop();
                (class_name, field_name, objectref)
            };

            let value = with_heap_read_lock(|heap| {
                heap.get_object_field_value_throwing(
                    objectref,
                    class_name.as_str(),
                    field_name.as_str(),
                    stack_frames,
                )
            });

            let value = unwrap_result_or_return_default!(value, ());

            let stack_frame = last_frame_mut(stack_frames)?;
            value.iter().rev().try_for_each(|x| stack_frame.push(*x))?;

            stack_frame.incr_pc();
            trace!("GETFIELD -> objectref={objectref}, class_name={class_name}, field_name={field_name}, value={value:?}");
        }
        PUTFIELD => {
            let stack_frame = last_frame_mut(stack_frames)?;
            let (class_name, field_name) = get_field_info(stack_frame, current_class_name)?;
            let type_descriptor = with_method_area(|method_area| {
                method_area
                    .lookup_for_field_descriptor(&class_name, &field_name)
                    .ok_or_else(|| {
                        Error::new_constant_pool(&format!(
                            "Error getting type descriptor for {class_name}.{field_name}"
                        ))
                    })
            })?;
            let len = get_length(&type_descriptor)?;

            let mut value = Vec::with_capacity(len as usize);
            for _ in 0..len {
                value.push(stack_frame.pop());
            }

            let objectref = stack_frame.pop();

            with_heap_write_lock(|heap| {
                heap.set_object_field_value(
                    objectref,
                    class_name.as_str(),
                    field_name.as_str(),
                    value.clone(),
                )
            })?;

            trace!("PUTFIELD -> objectref={objectref}, class_name={class_name}, field_name={field_name} value={value:?}");
            stack_frame.incr_pc();
        }
        INVOKEVIRTUAL => {
            store_ex_pc(stack_frames)?;
            let (class_name_by_ref_type, full_signature, method_signature) =
                get_class_name_and_signature(
                    stack_frames,
                    current_class_name,
                    CPoolHelper::get_full_method_info,
                )?;
            let method_descriptor = method_signature.parse()?;
            let method_args = prepare_invoke_context(stack_frames, &method_descriptor, true)?;
            let reference = method_args
                .first()
                .ok_or_else(|| Error::new_execution("Error getting reference from method_args"))?;

            if *reference == 0 {
                throw_null_pointer_exception_with_message(&format!("Cannot invoke \"{class_name_by_ref_type}.{full_signature}\" because \"<VALUE>\" is null"), stack_frames)?;
                return Ok(());
            }

            let class_name_by_instance =
                with_heap_read_lock(|heap| heap.get_instance_name(*reference))?;

            let exact_implementation = with_method_area(|method_area| {
                method_area
                    .lookup_for_implementation(&class_name_by_instance, &full_signature) // first looking for method in parent and above classes
                    .or_else(|| { // if not found, looking for default method implementation in interfaces
                        method_area.lookup_for_implementation_interface(
                            &class_name_by_instance,
                            &full_signature,
                        )
                    })
            }).ok_or_else(|| Error::new_constant_pool(&format!(
                "Error getting instance type JavaMethod by class name {class_name_by_instance} and full signature {full_signature} getting virtual_method"
            )))?;

            let class_name = exact_implementation.class_name();
            invoke(
                stack_frames,
                &full_signature,
                &method_args,
                Arc::clone(&exact_implementation),
                &class_name,
            )?;
            trace!("INVOKEVIRTUAL -> invoked {class_name}.{full_signature}({method_args:?}) via {class_name_by_ref_type}.{full_signature}");
        }
        INVOKESPECIAL => {
            store_ex_pc(stack_frames)?;
            let (class_name_to_start_lookup_from, full_signature, _) =
                get_class_name_and_signature(
                    stack_frames,
                    current_class_name,
                    CPoolHelper::get_full_method_info,
                )?;
            let java_method = with_method_area(|method_area| {
                method_area.lookup_for_implementation(&class_name_to_start_lookup_from, &full_signature)
                    .or_else(|| { // if not found, looking for default method implementation in interfaces
                        method_area.lookup_for_implementation_interface(
                            &class_name_to_start_lookup_from,
                            &full_signature,
                        )
                    })
                                .ok_or_else(|| Error::new_constant_pool(&format!("Error getting instance type JavaMethod by class name {class_name_to_start_lookup_from} and full signature {full_signature} calling invokespecial")))
            })?;
            let method_args =
                prepare_invoke_context(stack_frames, java_method.get_method_descriptor(), true)?;
            let class_name = java_method.class_name();
            invoke(
                stack_frames,
                &full_signature,
                &method_args,
                Arc::clone(&java_method),
                &class_name,
            )?;
            trace!("INVOKESPECIAL -> {class_name}.{full_signature}({method_args:?})");
        }
        INVOKESTATIC => {
            store_ex_pc(stack_frames)?;
            let (class_name_to_start_lookup_from, full_signature, _) =
                get_class_name_and_signature(
                    stack_frames,
                    current_class_name,
                    CPoolHelper::get_full_method_info,
                )?;
            let rc =
                with_method_area(|method_area| method_area.get(&class_name_to_start_lookup_from))?;
            StaticInit::initialize_java_class(&rc)?;
            let java_method = with_method_area(|method_area| {
                method_area.lookup_for_implementation(&class_name_to_start_lookup_from, &full_signature)
                    .ok_or_else(|| Error::new_constant_pool(&format!("Error getting instance type JavaMethod by class name {class_name_to_start_lookup_from} and full signature {full_signature} calling invokestatic")))
            })?;
            let method_args =
                prepare_invoke_context(stack_frames, java_method.get_method_descriptor(), false)?;
            invoke(
                stack_frames,
                &full_signature,
                &method_args,
                Arc::clone(&java_method),
                &class_name_to_start_lookup_from,
            )?;
            trace!("INVOKESTATIC -> {class_name_to_start_lookup_from}.{full_signature}({method_args:?})");
        }
        INVOKEINTERFACE => {
            store_ex_pc(stack_frames)?;
            let index = last_frame_mut(stack_frames)?.extract_two_bytes() as u16;
            let arg_num = last_frame_mut(stack_frames)?.extract_one_byte() as usize;
            let method_args = get_args(stack_frames, arg_num)?;

            let zero = last_frame_mut(stack_frames)?.extract_one_byte();
            last_frame_mut(stack_frames)?.incr_pc();
            if zero != 0 {
                return Err(Error::new_execution(&format!(
                    "Error calling interface method by index {index}"
                )));
            }

            let (class_name, full_signature, _) = get_class_name_and_signature_by_index(
                current_class_name,
                CPoolHelper::get_full_interfacemethodref_info,
                index,
            )?;
            let reference = method_args
                .first()
                .ok_or_else(|| Error::new_execution("Error getting reference from method_args"))?;

            if *reference == 0 {
                throw_null_pointer_exception_with_message(&format!("Cannot invoke \"{class_name}.{full_signature}\" because \"<VALUE>\" is null"), stack_frames)?;
                return Ok(());
            }

            let instance_name = with_heap_read_lock(|heap| heap.get_instance_name(*reference))?;
            let java_method = with_method_area(|method_area| {
                method_area
                    .lookup_for_implementation(&instance_name, &full_signature) // first looking for method in parent and above classes
                    .or_else(|| { // if not found, looking for default method implementation in interfaces
                        method_area.lookup_for_implementation_interface(
                            &instance_name,
                            &full_signature,
                        )
                    })
            }).ok_or_else(|| Error::new_constant_pool(&format!(
                "Error getting instance type JavaMethod by class name {instance_name} and full signature {full_signature} getting interface implementation"
            )))?;

            let exact_class_name = java_method.class_name();
            invoke(
                stack_frames,
                &full_signature,
                &method_args,
                Arc::clone(&java_method),
                exact_class_name,
            )?;
            trace!("INVOKEINTERFACE -> {exact_class_name}.{full_signature}({method_args:?}) on instance {instance_name}");
        }
        INVOKEDYNAMIC => {
            store_ex_pc(stack_frames)?;
            let stack_frame = last_frame_mut(stack_frames)?;
            let invokedynamic_index = stack_frame.extract_two_bytes() as u16;
            stack_frame.incr_pc();

            let jc = with_method_area(|a| a.get(current_class_name))?;
            let invoke_dynamic_runner = jc.invoke_dynamic_runner();
            invoke_dynamic_runner
                .run(stack_frames, current_class_name, invokedynamic_index)
                .map_err(|e| {
                    Error::new_execution(&format!(
                        "Error running invokedynamic for class {current_class_name} on index {invokedynamic_index}: {e}"
                    ))
                })?;

            trace!("INVOKEDYNAMIC -> {current_class_name} on index {invokedynamic_index}");
        }
        NEW => {
            let stack_frame = last_frame_mut(stack_frames)?;
            let class_constpool_index = stack_frame.extract_two_bytes() as u16;

            let rc = with_method_area(|method_area| method_area.get(current_class_name))?;
            let cpool_helper = rc.cpool_helper();

            let class_to_invoke_new_for = cpool_helper
                .get_class_name(class_constpool_index)
                .ok_or_else(|| {
                    Error::new_constant_pool(&format!(
                        "Error getting class name by index {class_constpool_index}"
                    ))
                })?;

            StaticInit::initialize(&class_to_invoke_new_for)?;

            let instance_with_default_fields = with_method_area(|method_area| {
                method_area.create_instance_with_default_fields(&class_to_invoke_new_for)
            })?;

            let instanceref =
                with_heap_write_lock(|heap| heap.create_instance(instance_with_default_fields));
            stack_frame.push(instanceref)?;

            trace!("NEW -> class={class_to_invoke_new_for}, reference={instanceref}");
            stack_frame.incr_pc();
        }
        NEWARRAY => {
            let stack_frame = last_frame_mut(stack_frames)?;
            let atype = stack_frame.extract_one_byte();

            let type_name = match atype {
                4 => "[Z",
                5 => "[C",
                6 => "[F",
                7 => "[D",
                8 => "[B",
                9 => "[S",
                10 => "[I",
                11 => "[J",
                _ => {
                    return Err(Error::new_execution(&format!(
                        "Error creating array of type {atype}"
                    )))
                }
            };

            let length = stack_frame.pop();

            let arrayref = with_heap_write_lock(|heap| heap.create_array(type_name, length));
            stack_frame.push(arrayref)?;

            stack_frame.incr_pc();
            trace!("NEWARRAY -> atype={atype}, length={length}, arrayref={arrayref}");
        }
        ANEWARRAY => {
            let stack_frame = last_frame_mut(stack_frames)?;
            let length = stack_frame.pop();

            let class_constpool_index = stack_frame.extract_two_bytes() as u16;
            let rc = with_method_area(|method_area| method_area.get(current_class_name))?;
            let cpool_helper = rc.cpool_helper();

            let class_of_array = cpool_helper
                .get_class_name(class_constpool_index)
                .ok_or_else(|| {
                    Error::new_constant_pool(&format!(
                        "Error getting class name by index {class_constpool_index}"
                    ))
                })?;
            let class_of_array = format!("[L{class_of_array};");
            let arrayref = with_heap_write_lock(|heap| heap.create_array(&class_of_array, length));
            stack_frame.push(arrayref)?;

            stack_frame.incr_pc();
            trace!("ANEWARRAY -> class_of_array={class_of_array}, length={length}, arrayref={arrayref}");
        }
        ARRAYLENGTH => {
            let stack_frame = last_frame_mut(stack_frames)?;
            let arrayref = stack_frame.pop();

            let len = with_heap_read_lock(|heap| heap.get_array_len(arrayref))?;
            stack_frame.push(len)?;

            stack_frame.incr_pc();
            trace!("ARRAYLENGTH -> arrayref={arrayref}, len={len}");
        }
        ATHROW => {
            store_ex_pc(stack_frames)?;
            let throwable_ref = {
                let stack_frame = last_frame_mut(stack_frames)?;
                let throwable_ref: i32 = stack_frame.pop();
                throwable_ref
            };
            let (exception_name, found_exception_handler) =
                throw_exception_with_ref(throwable_ref, stack_frames)?;
            trace!("ATHROW -> throwable_ref={throwable_ref}, exception_name={exception_name}, found_exception_handler={found_exception_handler}");
        }
        CHECKCAST => {
            let stack_frame = last_frame_mut(stack_frames)?;
            let class_constpool_index = stack_frame.extract_two_bytes() as u16;
            stack_frame.incr_pc();
            let objectref = stack_frame.pop();

            if objectref != 0 {
                let rc = with_method_area(|method_area| method_area.get(current_class_name))?;
                let cpool_helper = rc.cpool_helper();
                let class_name = cpool_helper
                    .get_class_name(class_constpool_index)
                    .ok_or_else(|| {
                        Error::new_constant_pool(&format!(
                            "Error getting class name by index {class_constpool_index}"
                        ))
                    })?;

                let instance_class_name =
                    with_heap_read_lock(|heap| heap.get_instance_name(objectref))?;

                let possible_cast = InstanceChecker::checkcast(&instance_class_name, &class_name)?;
                if !possible_cast {
                    return Err(Error::new_execution(&format!(
                        "Error casting {instance_class_name} to {class_name}"
                    ))); //todo: throw ClassCastException here
                }
            }

            stack_frame.push(objectref)?;

            trace!("CHECKCAST -> class_constpool_index={class_constpool_index}, objectref={objectref}");
        }
        INSTANCEOF => {
            let stack_frame = last_frame_mut(stack_frames)?;
            // todo: merge me with CHECKCAST
            let class_constpool_index = stack_frame.extract_two_bytes() as u16;
            stack_frame.incr_pc();
            let mut objectref = stack_frame.pop();

            if objectref != 0 {
                let rc = with_method_area(|method_area| method_area.get(current_class_name))?;
                let cpool_helper = rc.cpool_helper();
                let class_name = cpool_helper
                    .get_class_name(class_constpool_index)
                    .ok_or_else(|| {
                        Error::new_constant_pool(&format!(
                            "Error getting class name by index {class_constpool_index}"
                        ))
                    })?;

                let instance_class_name =
                    with_heap_read_lock(|heap| heap.get_instance_name(objectref))?;

                let instanse_of = InstanceChecker::checkcast(&instance_class_name, &class_name)?;
                objectref = if instanse_of { 1 } else { 0 };
            }

            stack_frame.push(objectref)?;

            trace!("INSTANCEOF -> class_constpool_index={class_constpool_index}, objectref={objectref}");
        }
        MONITORENTER => {
            let stack_frame = last_frame_mut(stack_frames)?;
            let objectref: i32 = stack_frame.pop();
            // todo: implement me
            stack_frame.incr_pc();
            trace!("MONITORENTER -> objectref={objectref}");
        }
        MONITOREXIT => {
            let stack_frame = last_frame_mut(stack_frames)?;
            let objectref: i32 = stack_frame.pop();
            // todo: implement me
            stack_frame.incr_pc();
            trace!("MONITOREXIT -> objectref={objectref}");
        }
        _ => {
            return Err(Error::new_execution(&format!(
                "Unknown reference opcode: {}",
                code
            )));
        }
    }

    Ok(())
}

pub fn prepare_invoke_context(
    stack_frames: &mut StackFrames,
    method_descriptor: &MethodDescriptor,
    use_self_ref: bool,
) -> Result<Vec<i32>> {
    let arg_num = argument_length(method_descriptor.parameter_types())?;
    let arg_num = arg_num + if use_self_ref { 1 } else { 0 };

    get_args(stack_frames, arg_num as usize)
}

fn get_args(stack_frames: &mut StackFrames, arg_num: usize) -> Result<Vec<i32>> {
    let mut method_args = Vec::with_capacity(arg_num);
    for _ in 0..arg_num {
        let val = last_frame_mut(stack_frames)?.pop();
        method_args.push(val);
    }
    method_args.reverse();

    Ok(method_args)
}

fn get_class_name_and_signature<F>(
    stack_frames: &mut StackFrames,
    current_class_name: &str,
    cpool_getter: F,
) -> Result<(String, String, String)>
where
    F: Fn(&CPoolHelper, u16) -> Option<(String, String, String)>,
{
    let stack_frame = last_frame_mut(stack_frames)?;
    let index = stack_frame.extract_two_bytes() as u16;
    stack_frame.incr_pc();

    get_class_name_and_signature_by_index(current_class_name, cpool_getter, index)
}

fn get_class_name_and_signature_by_index<F>(
    current_class_name: &str,
    cpool_getter: F,
    index: u16,
) -> Result<(String, String, String)>
where
    F: Fn(&CPoolHelper, u16) -> Option<(String, String, String)>,
{
    let rc = with_method_area(|method_area| method_area.get(current_class_name))?;
    let cpool_helper = rc.cpool_helper();
    let (class_name, method_name, method_descriptor) = cpool_getter(cpool_helper, index)
        .ok_or_else(|| {
            Error::new_constant_pool(&format!(
                "Error getting full method info by index {index} in {current_class_name}"
            ))
        })?;
    let full_signature = format!("{}:{}", method_name, method_descriptor);
    Ok((class_name, full_signature, method_descriptor))
}

fn get_field_info(
    stack_frame: &mut StackFrame,
    current_class_name: &str,
) -> Result<(String, String)> {
    let fieldref_constpool_index = stack_frame.extract_two_bytes() as u16;

    let rc = with_method_area(|method_area| method_area.get(current_class_name))?;
    let cpool_helper = rc.cpool_helper();

    let (class_name, field_name, _) = cpool_helper
        .get_full_field_info(fieldref_constpool_index)
        .ok_or_else(|| {
        Error::new_constant_pool(&format!(
            "Error getting full field info by index {fieldref_constpool_index}"
        ))
    })?;
    Ok((class_name, field_name))
}
