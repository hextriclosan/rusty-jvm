use crate::error::Error;
use crate::execution_engine::opcode::*;
use crate::execution_engine::system_native_table::invoke_native_method;
use crate::heap::heap::{with_heap_read_lock, with_heap_write_lock};
use crate::method_area::cpool_helper::CPoolHelper;
use crate::method_area::instance_checker::InstanceChecker;
use crate::method_area::java_method::JavaMethod;
use crate::method_area::method_area::with_method_area;
use crate::stack::stack_frame::{StackFrame, StackFrames};
use jdescriptor::{get_length, MethodDescriptor};
use std::sync::Arc;
use tracing::trace;

pub(crate) fn process(
    code: u8,
    current_class_name: &str,
    stack_frames: &mut StackFrames,
) -> crate::error::Result<()> {
    match code {
        GETSTATIC => {
            let stack_frame = stack_frames.last_mut().unwrap();
            let (class_name, field_name) = get_field_info(stack_frame, current_class_name)?;

            let field = with_method_area(|method_area| {
                method_area.lookup_for_static_field(&class_name, &field_name)
            })?;

            field
                .raw_value()
                .iter()
                .rev()
                .for_each(|x| stack_frame.push(*x));

            stack_frame.incr_pc();
            trace!(
                "GETSTATIC -> {class_name}.{field_name} is {:?}",
                field.raw_value()
            );
        }
        PUTSTATIC => {
            let stack_frame = stack_frames.last_mut().unwrap();
            let (class_name, field_name) = get_field_info(stack_frame, current_class_name)?;

            let (len, field_ref) = {
                let field_ref = with_method_area(|method_area| {
                    method_area.lookup_for_static_field(&class_name, &field_name)
                })?;
                let len = get_length(field_ref.type_descriptor());
                (len, field_ref)
            };

            let mut value = Vec::with_capacity(len);
            for _ in 0..len {
                value.push(stack_frame.pop());
            }

            field_ref.set_raw_value(value.clone());

            stack_frame.incr_pc();
            trace!("PUTSTATIC -> {class_name}.{field_name} = {value:?}");
        }
        GETFIELD => {
            let stack_frame = stack_frames.last_mut().unwrap();
            let (class_name, field_name) = get_field_info(stack_frame, current_class_name)?;
            let objectref = stack_frame.pop();
            let value = with_heap_read_lock(|heap| {
                heap.get_object_field_value(objectref, class_name.as_str(), field_name.as_str())
            })?;

            value.iter().rev().for_each(|x| stack_frame.push(*x));

            stack_frame.incr_pc();
            trace!("GETFIELD -> objectref={objectref}, class_name={class_name}, field_name={field_name}, value={value:?}");
        }
        PUTFIELD => {
            let stack_frame = stack_frames.last_mut().unwrap();
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
            let len = get_length(&type_descriptor);

            let mut value = Vec::with_capacity(len);
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
            let reference = *method_args.first().unwrap();
            let instance_type_class_name =
                with_heap_read_lock(|heap| heap.get_instance_name(reference))?;

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
            let (class_name, full_signature, _) = get_class_name_and_signature(
                stack_frames,
                current_class_name,
                CPoolHelper::get_full_method_info,
            )?;
            let rc = with_method_area(|method_area| method_area.get(&class_name))?;
            let java_method = rc.get_method(&full_signature)?;
            let method_args =
                prepare_invoke_context(stack_frames, java_method.get_method_descriptor(), false)?;
            invoke(
                stack_frames,
                &full_signature,
                &method_args,
                Arc::clone(&java_method),
                &class_name,
            )?;
            trace!("INVOKESTATIC -> {class_name}.{full_signature}({method_args:?})");
        }
        INVOKEINTERFACE => {
            let index = frame(stack_frames).extract_two_bytes() as u16;
            let arg_num = frame(stack_frames).extract_one_byte() as usize;
            let method_args = get_args(stack_frames, arg_num)?;

            let zero = frame(stack_frames).extract_one_byte();
            frame(stack_frames).incr_pc();
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
            let reference = *method_args.first().unwrap();
            let instance_name = with_heap_read_lock(|heap| heap.get_instance_name(reference))?;
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

            invoke(
                stack_frames,
                &full_signature,
                &method_args,
                Arc::clone(&java_method),
                &class_name,
            )?;
            trace!("INVOKEINTERFACE -> {class_name}.{full_signature}({method_args:?}) on instance {instance_name}");
        }
        NEW => {
            let stack_frame = stack_frames.last_mut().unwrap();
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
            let instance_with_default_fields = with_method_area(|method_area| {
                method_area.create_instance_with_default_fields(&class_to_invoke_new_for)
            })?;

            let instanceref =
                with_heap_write_lock(|heap| heap.create_instance(instance_with_default_fields));
            stack_frame.push(instanceref);

            trace!("NEW -> class={class_to_invoke_new_for}, reference={instanceref}");
            stack_frame.incr_pc();
        }
        NEWARRAY => {
            let stack_frame = stack_frames.last_mut().unwrap();
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
            stack_frame.push(arrayref);

            stack_frame.incr_pc();
            trace!("NEWARRAY -> atype={atype}, length={length}, arrayref={arrayref}");
        }
        ANEWARRAY => {
            let stack_frame = stack_frames.last_mut().unwrap();
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
            stack_frame.push(arrayref);

            stack_frame.incr_pc();
            trace!("ANEWARRAY -> class_of_array={class_of_array}, length={length}, arrayref={arrayref}");
        }
        ARRAYLENGTH => {
            let stack_frame = stack_frames.last_mut().unwrap();
            let arrayref = stack_frame.pop();

            let len = with_heap_read_lock(|heap| heap.get_array_len(arrayref))?;
            stack_frame.push(len);

            stack_frame.incr_pc();
            trace!("ARRAYLENGTH -> arrayref={arrayref}, len={len}");
        }
        CHECKCAST => {
            let stack_frame = stack_frames.last_mut().unwrap();
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

            stack_frame.push(objectref);

            trace!("CHECKCAST -> class_constpool_index={class_constpool_index}, objectref={objectref}");
        }
        INSTANCEOF => {
            let stack_frame = stack_frames.last_mut().unwrap();
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

            stack_frame.push(objectref);

            trace!("INSTANCEOF -> class_constpool_index={class_constpool_index}, objectref={objectref}");
        }
        MONITORENTER => {
            let stack_frame = stack_frames.last_mut().unwrap();
            let objectref: i32 = stack_frame.pop();
            // todo: implement me
            stack_frame.incr_pc();
            trace!("MONITORENTER -> objectref={objectref}");
        }
        MONITOREXIT => {
            let stack_frame = stack_frames.last_mut().unwrap();
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

fn prepare_invoke_context(
    stack_frames: &mut [StackFrame],
    method_descriptor: &MethodDescriptor,
    use_self_ref: bool,
) -> crate::error::Result<Vec<i32>> {
    let arg_num = method_descriptor.arguments_length();
    let arg_num = arg_num + if use_self_ref { 1 } else { 0 };

    get_args(stack_frames, arg_num)
}

fn get_args(stack_frames: &mut [StackFrame], arg_num: usize) -> crate::error::Result<Vec<i32>> {
    let mut method_args = Vec::with_capacity(arg_num);
    for _ in 0..arg_num {
        let val = frame(stack_frames).pop();
        method_args.push(val);
    }
    method_args.reverse();

    Ok(method_args)
}

fn invoke(
    stack_frames: &mut StackFrames,
    full_signature: &str,
    method_args: &[i32],
    java_method: Arc<JavaMethod>,
    class_name: &str,
) -> crate::error::Result<()> {
    if java_method.is_native() {
        let full_native_signature = format!("{class_name}:{full_signature}");
        trace!("<Calling native method> -> {full_native_signature} ({method_args:?})");

        let result = invoke_native_method(&full_native_signature, &method_args, stack_frames)?;

        result
            .iter()
            .rev()
            .for_each(|x| frame(stack_frames).push(*x));
    } else {
        let mut next_frame = java_method.new_stack_frame()?;

        method_args
            .iter()
            .enumerate()
            .for_each(|(index, val)| next_frame.set_local(index, *val));

        stack_frames.push(next_frame);
    }
    Ok(())
}

fn get_class_name_and_signature<F>(
    stack_frames: &mut [StackFrame],
    current_class_name: &str,
    cpool_getter: F,
) -> crate::error::Result<(String, String, String)>
where
    F: Fn(&CPoolHelper, u16) -> Option<(String, String, String)>,
{
    let stack_frame = frame(stack_frames);
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
            Error::new_constant_pool(&format!("Error getting full method info by index {index}"))
        })?;
    let full_signature = format!("{}:{}", method_name, method_descriptor);
    Ok((class_name, full_signature, method_descriptor))
}

fn frame(stack_frames: &mut [StackFrame]) -> &mut StackFrame {
    stack_frames.last_mut().unwrap()
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
