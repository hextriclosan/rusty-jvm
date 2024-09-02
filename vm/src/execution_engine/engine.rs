use crate::error::Error;
use crate::execution_engine::opcode::*;
use crate::heap::heap::Heap;
use crate::heap::java_instance::JavaInstance;
use crate::method_area::java_method::JavaMethod;
use crate::method_area::method_area::MethodArea;
use crate::util::get_class_name_by_cpool_class_index;

pub(crate) struct Engine<'a> {
    method_area: &'a MethodArea,
    heap: Heap<'a>,
}

impl<'a> Engine<'a> {
    pub(crate) fn execute(
        &mut self,
        main_class_name: &str,
        method: &JavaMethod,
    ) -> crate::error::Result<Option<i32>> {
        let mut stack_frames = vec![method.new_stack_frame()];
        let mut last_value: Option<i32> = None;

        while !stack_frames.is_empty() {
            let stack_frame = stack_frames
                .last_mut()
                .ok_or(Error::new_execution("Error getting stack frame"))?;

            match stack_frame.get_bytecode_byte() {
                ICONST_0 => {
                    println!("ICONST_0");
                    stack_frame.push(0);

                    stack_frame.incr_pc();
                }
                ICONST_1 => {
                    println!("ICONST_1");
                    stack_frame.push(1);

                    stack_frame.incr_pc();
                }
                ICONST_2 => {
                    println!("ICONST_2");
                    stack_frame.push(2);

                    stack_frame.incr_pc();
                }
                ICONST_3 => {
                    println!("ICONST_3");
                    stack_frame.push(3);

                    stack_frame.incr_pc();
                }
                ICONST_4 => {
                    println!("ICONST_4");
                    stack_frame.push(4);

                    stack_frame.incr_pc();
                }
                ICONST_5 => {
                    println!("ICONST_5");
                    stack_frame.push(5);

                    stack_frame.incr_pc();
                }
                BIPUSH => {
                    println!("BIPUSH");
                    stack_frame.incr_pc();
                    stack_frame.push(stack_frame.get_bytecode_byte() as i32);

                    stack_frame.incr_pc();
                }
                SIPUSH => {
                    println!("SIPUSH");
                    stack_frame.incr_pc();
                    let high = stack_frame.get_bytecode_byte() as u16;
                    stack_frame.incr_pc();
                    let low = stack_frame.get_bytecode_byte() as u16;
                    let val = (high << 8) | low;

                    stack_frame.push(val as i32);

                    stack_frame.incr_pc();
                }
                ILOAD => {
                    println!("ILOAD");
                    stack_frame.incr_pc();
                    let pos = stack_frame.get_bytecode_byte() as usize;

                    let val = stack_frame.get_local(pos);
                    stack_frame.push(val);

                    stack_frame.incr_pc();
                }
                ILOAD_0 => {
                    println!("ILOAD_0");
                    let val = stack_frame.get_local(0);
                    stack_frame.push(val);

                    stack_frame.incr_pc();
                }
                ILOAD_1 => {
                    println!("ILOAD_1");
                    let val = stack_frame.get_local(1);
                    stack_frame.push(val);

                    stack_frame.incr_pc();
                }
                ILOAD_2 => {
                    println!("ILOAD_2");
                    let val = stack_frame.get_local(2);
                    stack_frame.push(val);

                    stack_frame.incr_pc();
                }
                ILOAD_3 => {
                    println!("ILOAD_3");
                    let val = stack_frame.get_local(3);
                    stack_frame.push(val);

                    stack_frame.incr_pc();
                }
                ALOAD_0 => {
                    let reference = stack_frame.get_local(0);
                    stack_frame.push(reference);

                    stack_frame.incr_pc();
                    println!("ALOAD_0 -> reference={reference}");
                }
                ALOAD_3 => {
                    let reference = stack_frame.get_local(3);
                    stack_frame.push(reference);

                    stack_frame.incr_pc();
                    println!("ALOAD_3 -> reference={reference}");
                }
                ISTORE => {
                    println!("ISTORE");
                    stack_frame.incr_pc();
                    let pos = stack_frame.get_bytecode_byte() as usize;

                    let val = stack_frame.pop();

                    stack_frame.set_local(pos, val);

                    stack_frame.incr_pc();
                }
                ASTORE => {
                    stack_frame.incr_pc();
                    let index = stack_frame.get_bytecode_byte() as usize;

                    let objectref = stack_frame.pop();

                    stack_frame.set_local(index, objectref);

                    stack_frame.incr_pc();
                    println!("ASTORE -> index={index}, objectref={objectref}");
                }
                ISTORE_0 => {
                    println!("ISTORE_0");
                    let val = stack_frame.pop();
                    stack_frame.set_local(0, val);

                    stack_frame.incr_pc();
                }
                ISTORE_1 => {
                    println!("ISTORE_1");
                    let val = stack_frame.pop();
                    stack_frame.set_local(1, val);

                    stack_frame.incr_pc();
                }
                ISTORE_2 => {
                    println!("ISTORE_2");
                    let val = stack_frame.pop();
                    stack_frame.set_local(2, val);

                    stack_frame.incr_pc();
                }
                ISTORE_3 => {
                    println!("ISTORE_3");
                    let val = stack_frame.pop();
                    stack_frame.set_local(3, val);

                    stack_frame.incr_pc();
                }
                ASTORE_3 => {
                    println!("ASTORE_3");
                    let objectref = stack_frame.pop();
                    stack_frame.set_local(3, objectref);

                    stack_frame.incr_pc();
                }
                POP => {
                    stack_frame.pop();

                    stack_frame.incr_pc();
                    println!("POP");
                }
                DUP => {
                    let val = stack_frame.pop();
                    stack_frame.push(val);
                    stack_frame.push(val);

                    stack_frame.incr_pc();
                    println!("DUP -> {val}");
                }
                IADD => {
                    let b = stack_frame.pop();
                    let a = stack_frame.pop();
                    let result = a + b;
                    stack_frame.push(result);

                    stack_frame.incr_pc();
                    println!("IADD -> {a} + {b} = {result}");
                }
                ISUB => {
                    let b = stack_frame.pop();
                    let a = stack_frame.pop();
                    let result = a - b;
                    stack_frame.push(result);

                    stack_frame.incr_pc();
                    println!("ISUB -> {a} - {b} = {result}");
                }
                IMUL => {
                    let b = stack_frame.pop();
                    let a = stack_frame.pop();
                    let result = a * b;
                    stack_frame.push(result);

                    stack_frame.incr_pc();
                    println!("IMUL -> {a} * {b} = {result}");
                }
                IREM => {
                    println!("IREM");
                    let b = stack_frame.pop();
                    let a = stack_frame.pop();
                    stack_frame.push(a % b);

                    stack_frame.incr_pc();
                }
                IINC => {
                    stack_frame.incr_pc();
                    let index = stack_frame.get_bytecode_byte() as usize;

                    stack_frame.incr_pc();
                    let const_val = stack_frame.get_bytecode_byte() as i8;

                    let current_val = stack_frame.get_local(index);
                    let new_val = current_val + const_val as i32;
                    stack_frame.set_local(index, new_val);

                    stack_frame.incr_pc();
                    println!("IINC -> {current_val} + {const_val} = {new_val}");
                }
                IFEQ => {
                    println!("IFEQ");

                    let value1 = stack_frame.pop();

                    let branchbyte1 = stack_frame.get_bytecode_byte_1() as u16;
                    let branchbyte2 = stack_frame.get_bytecode_byte_2() as u16;
                    let offset = ((branchbyte1 << 8) | branchbyte2) as i16;

                    if value1 == 0 {
                        stack_frame.advance_pc(offset);
                    } else {
                        stack_frame.advance_pc(3);
                    }
                }
                IFNE => {
                    println!("IFNE");

                    let value1 = stack_frame.pop();

                    let branchbyte1 = stack_frame.get_bytecode_byte_1() as u16;
                    let branchbyte2 = stack_frame.get_bytecode_byte_2() as u16;
                    let offset = ((branchbyte1 << 8) | branchbyte2) as i16;

                    if value1 != 0 {
                        stack_frame.advance_pc(offset);
                    } else {
                        stack_frame.advance_pc(3);
                    }
                }
                IF_ICMPNE => {
                    println!("IF_ICMPNE");

                    let value2 = stack_frame.pop();
                    let value1 = stack_frame.pop();

                    let branchbyte1 = stack_frame.get_bytecode_byte_1() as u16;
                    let branchbyte2 = stack_frame.get_bytecode_byte_2() as u16;
                    let offset = ((branchbyte1 << 8) | branchbyte2) as i16;

                    if value1 != value2 {
                        stack_frame.advance_pc(offset);
                    } else {
                        stack_frame.advance_pc(3);
                    }
                }
                IF_ICMPLT => {
                    println!("IF_ICMPLT");

                    let value2 = stack_frame.pop();
                    let value1 = stack_frame.pop();

                    let branchbyte1 = stack_frame.get_bytecode_byte_1() as u16;
                    let branchbyte2 = stack_frame.get_bytecode_byte_2() as u16;
                    let offset = ((branchbyte1 << 8) | branchbyte2) as i16;

                    if value1 < value2 {
                        stack_frame.advance_pc(offset);
                    } else {
                        stack_frame.advance_pc(3);
                    }
                }
                IF_ICMPGE => {
                    println!("IF_ICMPGE");

                    let value2 = stack_frame.pop();
                    let value1 = stack_frame.pop();

                    let branchbyte1 = stack_frame.get_bytecode_byte_1() as u16;
                    let branchbyte2 = stack_frame.get_bytecode_byte_2() as u16;
                    let offset = ((branchbyte1 << 8) | branchbyte2) as i16;

                    if value1 >= value2 {
                        stack_frame.advance_pc(offset);
                    } else {
                        stack_frame.advance_pc(3);
                    }
                }
                IF_ICMPGT => {
                    println!("IF_ICMPGT");

                    let value2 = stack_frame.pop();
                    let value1 = stack_frame.pop();

                    let branchbyte1 = stack_frame.get_bytecode_byte_1() as u16;
                    let branchbyte2 = stack_frame.get_bytecode_byte_2() as u16;
                    let offset = ((branchbyte1 << 8) | branchbyte2) as i16;

                    if value1 > value2 {
                        stack_frame.advance_pc(offset);
                    } else {
                        stack_frame.advance_pc(3);
                    }
                }
                GOTO => {
                    println!("GOTO");

                    let branchbyte1 = stack_frame.get_bytecode_byte_1() as u16;
                    let branchbyte2 = stack_frame.get_bytecode_byte_2() as u16;
                    let offset = ((branchbyte1 << 8) | branchbyte2) as i16;

                    stack_frame.advance_pc(offset);
                }
                IRETURN => {
                    println!(
                        "IRETURN -> locals={:?}, operand_stack={:?}",
                        stack_frame.locals, stack_frame.operand_stack
                    );
                    let ret = stack_frame.pop();
                    stack_frames.pop();
                    stack_frames
                        .last_mut()
                        .ok_or(Error::new_execution("Error getting stack last value"))?
                        .push(ret);
                }
                RETURN => {
                    println!(
                        "RETURN -> locals={:?}, operand_stack={:?}",
                        stack_frame.locals, stack_frame.operand_stack
                    );
                    last_value = stack_frames
                        .last()
                        .ok_or(Error::new_execution("Error getting stack last value"))?
                        .locals
                        .last()
                        .copied();
                    stack_frames.pop(); // Return from method, pop the current frame
                                        // add more logic here
                }
                GETFIELD => {
                    stack_frame.incr_pc();
                    let high = stack_frame.get_bytecode_byte() as u16;

                    stack_frame.incr_pc();
                    let low = stack_frame.get_bytecode_byte() as u16;
                    let fieldref_constpool_index = (high << 8) | low;

                    let java_class = self
                        .method_area
                        .loaded_classes
                        .get(main_class_name)
                        .unwrap();

                    let objectref = stack_frame.pop();
                    let field_name = self.method_area.get_fieldname_by_fieldref_cpool_index(
                        java_class,
                        fieldref_constpool_index,
                    )?;

                    let value = self
                        .heap
                        .get_object_field_value(objectref, field_name.as_str())?;

                    stack_frame.push(value);

                    stack_frame.incr_pc();
                    println!("GETFIELD -> fieldref_constpool_index={fieldref_constpool_index}, objectref={objectref}, value={value}");
                }
                PUTFIELD => {
                    stack_frame.incr_pc();
                    let high = stack_frame.get_bytecode_byte() as u16;

                    stack_frame.incr_pc();
                    let low = stack_frame.get_bytecode_byte() as u16;
                    let fieldref_constpool_index = (high << 8) | low;

                    let java_class = self
                        .method_area
                        .loaded_classes
                        .get(main_class_name)
                        .unwrap();

                    let field_name = self.method_area.get_fieldname_by_fieldref_cpool_index(
                        java_class,
                        fieldref_constpool_index,
                    )?;
                    let value = stack_frame.pop();
                    let objectref = stack_frame.pop();

                    self.heap
                        .set_object_field_value(objectref, field_name.as_str(), value)?;

                    stack_frame.incr_pc();
                    println!("PUTFIELD -> fieldref_constpool_index={fieldref_constpool_index}, objectref={objectref}, value={value}");
                }
                INVOKEVIRTUAL => {
                    println!(
                        "INVOKEVIRTUAL -> locals={:?}, operand_stack={:?}",
                        stack_frame.locals, stack_frame.operand_stack
                    );

                    stack_frame.incr_pc();
                    let high = stack_frame.get_bytecode_byte() as u16;

                    stack_frame.incr_pc();
                    let low = stack_frame.get_bytecode_byte() as u16;
                    let methodref_constpool_index = (high << 8) | low;

                    let java_class = self
                        .method_area
                        .loaded_classes
                        .get(main_class_name)
                        .unwrap();

                    let virtual_method = self.method_area.get_method_by_methodref_cpool_index(
                        java_class,
                        methodref_constpool_index,
                    )?;

                    let mut next_frame = virtual_method.new_stack_frame();
                    let arg_num = virtual_method.get_signature().get_arg_num();

                    for i in (0..arg_num).rev() {
                        let val = stack_frame.pop();
                        next_frame.set_local(i + 1, val);
                    }
                    let reference = stack_frame.pop();
                    next_frame.set_local(0, reference);

                    stack_frame.incr_pc(); //incr here because of borrowing problem

                    stack_frames.push(next_frame);
                }
                INVOKESPECIAL => {
                    println!(
                        "INVOKESPECIAL -> locals={:?}, operand_stack={:?}",
                        stack_frame.locals, stack_frame.operand_stack
                    );

                    stack_frame.incr_pc();
                    let high = stack_frame.get_bytecode_byte() as u16;

                    stack_frame.incr_pc();
                    let low = stack_frame.get_bytecode_byte() as u16;
                    let methodref_constpool_index = (high << 8) | low;

                    let java_class = self
                        .method_area
                        .loaded_classes
                        .get(main_class_name)
                        .unwrap();

                    let special_method = self.method_area.get_method_by_methodref_cpool_index(
                        java_class,
                        methodref_constpool_index,
                    )?;

                    let mut next_frame = special_method.new_stack_frame();
                    let arg_num = special_method.get_signature().get_arg_num();

                    for i in (0..arg_num).rev() {
                        let val = stack_frame.pop();
                        next_frame.set_local(i + 1, val);
                    }
                    let reference = stack_frame.pop();
                    next_frame.set_local(0, reference);

                    stack_frame.incr_pc(); //incr here because of borrowing problem

                    stack_frames.push(next_frame);
                }
                INVOKESTATIC => {
                    println!(
                        "INVOKESTATIC -> locals={:?}, operand_stack={:?}",
                        stack_frame.locals, stack_frame.operand_stack
                    );

                    stack_frame.incr_pc();
                    let high = stack_frame.get_bytecode_byte() as u16;

                    stack_frame.incr_pc();
                    let low = stack_frame.get_bytecode_byte() as u16;
                    let methodref_constpool_index = (high << 8) | low;
                    let java_class = self
                        .method_area
                        .loaded_classes
                        .get(main_class_name)
                        .unwrap();

                    let static_method = self.method_area.get_method_by_methodref_cpool_index(
                        java_class,
                        methodref_constpool_index,
                    )?;

                    let mut next_frame = static_method.new_stack_frame();
                    let arg_num = static_method.get_signature().get_arg_num();

                    for i in (0..arg_num).rev() {
                        let val = stack_frame.pop();
                        next_frame.set_local(i, val);
                    }

                    stack_frame.incr_pc(); //incr here because of borrowing problem

                    stack_frames.push(next_frame);
                }
                NEW => {
                    stack_frame.incr_pc();
                    let high = stack_frame.get_bytecode_byte() as u16;

                    stack_frame.incr_pc();
                    let low = stack_frame.get_bytecode_byte() as u16;
                    let class_constpool_index = ((high << 8) | low) as usize;
                    let java_class = self
                        .method_area
                        .loaded_classes
                        .get(main_class_name)
                        .unwrap();
                    let class_to_invoke_new_for = get_class_name_by_cpool_class_index(
                        class_constpool_index,
                        &java_class.class_file,
                    )
                    .unwrap();
                    let default_field_values_instance = JavaInstance::new(
                        self.method_area
                            .loaded_classes
                            .get(class_to_invoke_new_for.as_str())
                            .unwrap(),
                    )?;

                    let instanceref = self.heap.create_instance(default_field_values_instance);
                    stack_frame.push(instanceref);

                    println!("NEW -> class={class_constpool_index}, reference={instanceref}");
                    stack_frame.incr_pc();
                }
                _ => unreachable!("{}", format! {"xxx = {}", stack_frame.get_bytecode_byte()}),
            }
        }

        Ok(last_value)
    }

    pub fn new(method_area: &'a MethodArea) -> Self {
        Self {
            method_area,
            heap: Heap::new(),
        }
    }
}