use crate::error::Error;
use crate::execution_engine::common::last_frame_mut;
use crate::execution_engine::invoker::invoke;
use crate::execution_engine::opcode::*;
use crate::execution_engine::static_init::StaticInit;
use crate::heap::heap::{with_heap_read_lock, with_heap_write_lock};
use crate::helper::{argument_length, get_length};
use crate::method_area::cpool_helper::CPoolHelper;
use crate::method_area::cpool_helper::CPoolHelperTrait;
use crate::method_area::instance_checker::InstanceChecker;
use crate::method_area::method_area::with_method_area;
use crate::stack::stack_frame::{StackFrame, StackFrames};
use jdescriptor::MethodDescriptor;
use std::sync::Arc;
use tracing::trace;

pub(crate) fn process(
    code: u8,
    current_class_name: &str,
    stack_frames: &mut StackFrames,
) -> crate::error::Result<()> {
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

            let (len, fields_class_name, field_ref) = {
                let (fields_class_name, field_ref) = with_method_area(|method_area| {
                    method_area.lookup_for_static_field(&class_name, &field_name)
                })?;
                let len = get_length(field_ref.type_descriptor())?;
                (len, fields_class_name, field_ref)
            };

            StaticInit::initialize(&fields_class_name)?;

            let mut value = Vec::with_capacity(len as usize);
            for _ in 0..len {
                value.push(stack_frame.pop());
            }

            field_ref.set_raw_value(value.clone())?;

            stack_frame.incr_pc();
            trace!("PUTSTATIC -> {class_name}.{field_name} = {value:?}");
        }
        GETFIELD => {
            let stack_frame = last_frame_mut(stack_frames)?;
            let (class_name, field_name) = get_field_info(stack_frame, current_class_name)?;
            let objectref = stack_frame.pop();
            let value = with_heap_read_lock(|heap| {
                heap.get_object_field_value(objectref, class_name.as_str(), field_name.as_str())
            })?;

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
            let (_, full_signature, method_signature) = get_class_name_and_signature(
                stack_frames,
                current_class_name,
                CPoolHelper::get_full_method_info,
            )?;
            let method_descriptor = method_signature.parse()?;
            let method_args = prepare_invoke_context(stack_frames, &method_descriptor, true)?;
            let reference = method_args
                .first()
                .ok_or_else(|| Error::new_execution("Error getting reference from method_args"))?;
            let instance_type_class_name =
                with_heap_read_lock(|heap| heap.get_instance_name(*reference))?;

            let java_method = with_method_area(|method_area| {
                method_area
                    .lookup_for_implementation(&instance_type_class_name, &full_signature) // first looking for method in parent and above classes
                    .or_else(|| { // if not found, looking for default method implementation in interfaces
                        method_area.lookup_for_implementation_interface(
                            &instance_type_class_name,
                            &full_signature,
                        )
                    })
            }).ok_or_else(|| Error::new_constant_pool(&format!(
                "Error getting instance type JavaMethod by class name {instance_type_class_name} and full signature {full_signature} getting virtual_method"
            )))?;

            let class_name = java_method.class_name();
            invoke(
                stack_frames,
                &full_signature,
                &method_args,
                Arc::clone(&java_method),
                class_name,
            )?;
            trace!("INVOKEVIRTUAL -> {class_name}.{full_signature}({method_args:?})");
        }
        INVOKESPECIAL => {
            let (class_name_to_start_lookup_from, full_signature, _) =
                get_class_name_and_signature(
                    stack_frames,
                    current_class_name,
                    CPoolHelper::get_full_method_info,
                )?;
            let java_method = with_method_area(|method_area| {
                method_area.lookup_for_implementation(&class_name_to_start_lookup_from, &full_signature)
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

            let (_, full_signature, _) = get_class_name_and_signature_by_index(
                current_class_name,
                CPoolHelper::get_full_interfacemethodref_info,
                index,
            )?;
            let reference = method_args
                .first()
                .ok_or_else(|| Error::new_execution("Error getting reference from method_args"))?;
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

            let arrayref = with_heap_write_lock(|heap| heap.create_array(type_name, length))?;
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
            let arrayref =
                with_heap_write_lock(|heap| heap.create_array(&class_of_array, length))?;
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
            let throwable_ref = {
                let stack_frame = last_frame_mut(stack_frames)?;
                let throwable_ref: i32 = stack_frame.pop();
                throwable_ref
            };
            let exception_name =
                with_heap_read_lock(|heap| heap.get_instance_name(throwable_ref))?;
            trace!("ATHROW -> about to throw: throwable_ref={throwable_ref}, exception_name={exception_name}");
            let found_exception_handler = throw_exception(stack_frames, &exception_name)?;

            let stack_frame = last_frame_mut(stack_frames)?;
            stack_frame.set_pc(found_exception_handler);
            stack_frame.clear_stack(); // according to JVM spec
            stack_frame.push(throwable_ref)?;

            trace!("ATHROW -> throwable_ref={throwable_ref}, exception_name={exception_name}");
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

fn throw_exception(
    stack_frames: &mut StackFrames,
    exception_name: &str,
) -> crate::error::Result<i16> {
    while !stack_frames.is_empty() {
        let stack_frame = last_frame_mut(stack_frames)?;
        let exception_table = stack_frame.exception_table();
        let pc = stack_frame.pc() as u16;
        match exception_table.find_exception_handler(exception_name, pc, stack_frame.method_name())
        {
            Some(exception_handler) => {
                return Ok(exception_handler as i16);
            }
            None => {
                stack_frames.pop();
            }
        }
    }

    Err(Error::new_execution(&format!(
        "Exception {exception_name} not handled"
    )))
}

fn prepare_invoke_context(
    stack_frames: &mut StackFrames,
    method_descriptor: &MethodDescriptor,
    use_self_ref: bool,
) -> crate::error::Result<Vec<i32>> {
    let arg_num = argument_length(method_descriptor.parameter_types())?;
    let arg_num = arg_num + if use_self_ref { 1 } else { 0 };

    get_args(stack_frames, arg_num as usize)
}

fn get_args(stack_frames: &mut StackFrames, arg_num: usize) -> crate::error::Result<Vec<i32>> {
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
) -> crate::error::Result<(String, String, String)>
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
) -> crate::error::Result<(String, String, String)>
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
) -> crate::error::Result<(String, String)> {
    let fieldref_constpool_index = stack_frame.extract_two_bytes() as u16;

    let rc = with_method_area(|method_area| method_area.get(current_class_name))?;
    let cpool_helper = rc.cpool_helper();

    let (class_name, field_name) = cpool_helper
        .get_full_field_info(fieldref_constpool_index)
        .ok_or_else(|| {
            Error::new_constant_pool(&format!(
                "Error getting full field info by index {fieldref_constpool_index}"
            ))
        })?;
    Ok((class_name, field_name))
}
