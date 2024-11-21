use crate::error::Error;
use crate::execution_engine::opcode::*;
use crate::execution_engine::system_native_table::invoke_native_method;
use crate::heap::heap::{with_heap_read_lock, with_heap_write_lock};
use crate::method_area::instance_checker::InstanceChecker;
use crate::method_area::method_area::with_method_area;
use crate::stack::stack_frame::StackFrame;
use jdescriptor::get_length;
use tracing::trace;

pub(crate) fn process(
    code: u8,
    current_class_name: &str,
    stack_frames: &mut Vec<StackFrame>,
) -> crate::error::Result<()> {
    let stack_frame = stack_frames.last_mut().unwrap();
    match code {
        GETSTATIC => {
            let (class_name, field_name, _field_descriptor) = get_field_info(stack_frame, current_class_name)?;

            let field = with_method_area(|method_area| {
                method_area.lookup_for_static_field(&class_name, &field_name)
            })?;

            field
                .raw_value()
                .iter()
                .rev()
                .for_each(|x| stack_frame.push(*x));

            stack_frame.incr_pc();
            trace!("GETSTATIC -> {class_name}.{field_name} is {:?}", field.raw_value());
        }
        PUTSTATIC => {
            let (class_name, field_name, _field_descriptor) = get_field_info(stack_frame, current_class_name)?;

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
            let (class_name, field_name, field_descriptor) = get_field_info(stack_frame, current_class_name)?;
            let field_name_type = format!("{field_name}:{field_descriptor}");
            let objectref = stack_frame.pop();
            let value = with_heap_read_lock(|heap| {
                heap.get_object_field_value(
                    objectref,
                    class_name.as_str(),
                    field_name_type.as_str(),
                )
            })?;

            value.iter().rev().for_each(|x| stack_frame.push(*x));

            stack_frame.incr_pc();
            trace!("GETFIELD -> objectref={objectref}, class_name={class_name}, field_name_type={field_name_type}, value={value:?}");
        }
        PUTFIELD => {
            let (class_name, field_name, field_descriptor) = get_field_info(stack_frame, current_class_name)?;
            let field_name_type = format!("{field_name}:{field_descriptor}");
            let type_descriptor = with_method_area(|method_area| {
                method_area
                    .lookup_for_field_descriptor(&class_name, &field_name_type)
                    .ok_or_else(|| {
                        Error::new_constant_pool(&format!(
                            "Error getting type descriptor for {class_name}.{field_name_type}"
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
                    field_name_type.as_str(),
                    value.clone(),
                )
            })?;

            trace!("PUTFIELD -> objectref={objectref}, class_name={class_name}, field_name_type={field_name_type} value={value:?}");
            stack_frame.incr_pc();
        }
        INVOKEVIRTUAL => {
            let methodref_constpool_index = stack_frame.extract_two_bytes() as u16;
            stack_frame.incr_pc();

            let rc = with_method_area(|method_area| method_area.get(current_class_name))?;
            let cpool_helper = rc.cpool_helper();

            let (reference_type_class_name, method_name, method_descriptor) = cpool_helper
                .get_full_method_info(methodref_constpool_index)
                .ok_or_else(|| {
                    Error::new_constant_pool(&format!(
                        "Error getting full method info by index {methodref_constpool_index}"
                    ))
                })?;
            let full_signature = format!("{}:{}", method_name, method_descriptor);

            let java_method = with_method_area(|method_area| method_area.lookup_for_implementation(&reference_type_class_name, &full_signature))
                .ok_or_else(|| Error::new_constant_pool(&format!("Error getting instance type JavaMethod by class name {reference_type_class_name} and full signature {full_signature} getting java_method")))?;
            let method_arg_num = java_method.get_method_descriptor().arguments_length();
            let mut method_args = Vec::with_capacity(method_arg_num);
            for _ in 0..method_arg_num {
                let val = stack_frame.pop();
                method_args.push(val);
            }
            let reference = stack_frame.pop();
            method_args.push(reference);
            method_args.reverse();

            let instance_type_class_name =
                with_heap_read_lock(|heap| heap.get_instance_name(reference))?;

            let virtual_method = with_method_area(|method_area| {
                method_area.lookup_for_implementation(&instance_type_class_name, &full_signature)
                    .ok_or_else(|| Error::new_constant_pool(&format!("Error getting instance type JavaMethod by class name {instance_type_class_name} and full signature {full_signature} getting virtual_method")))
            })?;

            let found_impl_type_class_name = virtual_method.class_name();
            if virtual_method.is_native() {
                let full_native_signature =
                    format!("{found_impl_type_class_name}:{full_signature}");
                trace!(
                    "<Calling native virtual method> -> {full_native_signature} ({method_args:?})"
                );

                let result = invoke_native_method(&full_native_signature, &method_args)?;

                result.iter().rev().for_each(|x| stack_frame.push(*x));
            } else {
                let mut next_frame = virtual_method.new_stack_frame()?;

                method_args
                    .iter()
                    .enumerate()
                    .for_each(|(index, val)| next_frame.set_local(index, *val));

                stack_frames.push(next_frame);
            }
            trace!("INVOKEVIRTUAL -> {found_impl_type_class_name}.{method_name}({method_args:?})");
        }
        INVOKESPECIAL => {
            let methodref_constpool_index = stack_frame.extract_two_bytes() as u16;
            stack_frame.incr_pc();

            let rc = with_method_area(|method_area| method_area.get(current_class_name))?;
            let cpool_helper = rc.cpool_helper();

            let (class_name, method_name, method_descriptor) = cpool_helper
                .get_full_method_info(methodref_constpool_index)
                .ok_or_else(|| {
                    Error::new_constant_pool(&format!(
                        "Error getting full method info by index {methodref_constpool_index}"
                    ))
                })?;
            let full_signature = format!("{}:{}", method_name, method_descriptor);
            let rc = with_method_area(|method_area| method_area.get(class_name.as_str()))?;
            let special_method = rc.get_method(&full_signature)?;
            // ^^^ todo: implement lookup in parents

            let arg_num = special_method.get_method_descriptor().arguments_length();
            let mut method_args = Vec::with_capacity(arg_num);
            for _ in 0..arg_num {
                let val = stack_frame.pop();
                method_args.push(val);
            }
            let reference = stack_frame.pop();
            method_args.push(reference);
            method_args.reverse();

            if special_method.is_native() {
                let full_native_signature = format!("{class_name}:{full_signature}");
                trace!(
                    "<Calling native special method> -> {full_native_signature} ({method_args:?})"
                );

                let result = invoke_native_method(&full_native_signature, &method_args)?;

                result.iter().rev().for_each(|x| stack_frame.push(*x));
            } else {
                let mut next_frame = special_method.new_stack_frame()?;

                method_args
                    .iter()
                    .enumerate()
                    .for_each(|(index, val)| next_frame.set_local(index, *val));

                stack_frames.push(next_frame);
            }
            trace!("INVOKESPECIAL -> {class_name}.{method_name}({method_args:?})");
        }
        INVOKESTATIC => {
            let methodref_constpool_index = stack_frame.extract_two_bytes() as u16;
            stack_frame.incr_pc();

            let rc = with_method_area(|method_area| method_area.get(current_class_name))?;
            let cpool_helper = rc.cpool_helper();

            let (class_name, method_name, method_descriptor) = cpool_helper
                .get_full_method_info(methodref_constpool_index)
                .ok_or_else(|| {
                    Error::new_constant_pool(&format!(
                        "Error getting full method info by index {methodref_constpool_index}"
                    ))
                })?;
            let full_signature = format!("{}:{}", method_name, method_descriptor);
            let rc = with_method_area(|method_area| method_area.get(class_name.as_str()))?;
            let static_method = rc.get_method(&full_signature)?;
            // todo: according to requirements of JVMS Section 5.4
            // all static fields of the class should be initialized
            // at this point

            let arg_num = static_method.get_method_descriptor().arguments_length();
            let method_args: Vec<i32> = (0..arg_num)
                .map(|_| stack_frame.pop())
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect();

            if static_method.is_native() {
                let full_native_signature = format!("{class_name}:{full_signature}");
                trace!("<Calling native method> -> {full_native_signature} ({method_args:?})");

                let result = invoke_native_method(&full_native_signature, &method_args)?;

                result.iter().rev().for_each(|x| stack_frame.push(*x));
            } else {
                let mut next_frame = static_method.new_stack_frame()?;

                method_args
                    .iter()
                    .enumerate()
                    .for_each(|(index, val)| next_frame.set_local(index, *val));

                stack_frames.push(next_frame);
            }
            trace!("INVOKESTATIC -> {class_name}.{method_name}({method_args:?})");
        }
        INVOKEINTERFACE => {
            let interfacemethodref_constpool_index = stack_frame.extract_two_bytes() as u16;
            let arg_count = stack_frame.extract_one_byte() as usize;
            stack_frame.incr_pc();

            let mut method_args = Vec::with_capacity(arg_count);
            for _ in 0..(arg_count - 1) {
                let val = stack_frame.pop();
                method_args.push(val);
            }
            let reference = stack_frame.pop();
            method_args.push(reference);
            method_args.reverse();

            let zero = stack_frame.get_bytecode_byte();
            stack_frame.incr_pc();
            if zero != 0 {
                return Err(Error::new_execution(&format!(
                    "Error calling interface method by index {interfacemethodref_constpool_index}"
                )));
            }

            let rc = with_method_area(|method_area| method_area.get(current_class_name))?;
            let cpool_helper = rc.cpool_helper();

            let (interface_class_name, method_name, method_descriptor) = cpool_helper
                .get_full_interfacemethodref_info(interfacemethodref_constpool_index)
                .ok_or_else(|| Error::new_constant_pool(&format!("Error getting full interfacemethodref info by index {interfacemethodref_constpool_index}")))?;

            let instance_name = with_heap_read_lock(|heap| heap.get_instance_name(reference))?;

            let full_method_signature = format!("{method_name}:{method_descriptor}");
            let interface_implementation_method = with_method_area(|method_area| {
                method_area.lookup_for_implementation(&instance_name, &full_method_signature)
                    .ok_or_else(|| Error::new_constant_pool(&format!("Error getting implementaion of {interface_class_name}.{method_name}{method_descriptor} in {instance_name}")))
            })?;

            let mut next_frame = interface_implementation_method.new_stack_frame()?;

            method_args
                .iter()
                .enumerate()
                .for_each(|(index, val)| next_frame.set_local(index, *val));

            stack_frames.push(next_frame);

            trace!("INVOKEINTERFACE -> {interface_class_name}.{method_name}{method_descriptor}({method_args:?}) on instance {instance_name}");
        }
        NEW => {
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
            });

            let instanceref =
                with_heap_write_lock(|heap| heap.create_instance(instance_with_default_fields));
            stack_frame.push(instanceref);

            trace!("NEW -> class={class_to_invoke_new_for}, reference={instanceref}");
            stack_frame.incr_pc();
        }
        NEWARRAY => {
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
            let arrayref = stack_frame.pop();

            let len = with_heap_read_lock(|heap| heap.get_array_len(arrayref))?;
            stack_frame.push(len);

            stack_frame.incr_pc();
            trace!("ARRAYLENGTH -> arrayref={arrayref}, len={len}");
        }
        CHECKCAST => {
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
            let objectref: i32 = stack_frame.pop();
            // todo: implement me
            stack_frame.incr_pc();
            trace!("MONITORENTER -> objectref={objectref}");
        }
        MONITOREXIT => {
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

fn get_field_info(stack_frame: &mut StackFrame, current_class_name: &str) -> crate::error::Result<(String, String, String)> {
    let fieldref_constpool_index = stack_frame.extract_two_bytes() as u16;

    let rc = with_method_area(|method_area| method_area.get(current_class_name))?;
    let cpool_helper = rc.cpool_helper();

    let (class_name, field_name, field_descriptor) = cpool_helper
        .get_full_field_info(fieldref_constpool_index)
        .ok_or_else(|| {
            Error::new_constant_pool(&format!(
                "Error getting full field info by index {fieldref_constpool_index}"
            ))
        })?;
    Ok((class_name, field_name, field_descriptor))
}
