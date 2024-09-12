use crate::error::Error;
use crate::execution_engine::opcode::*;
use crate::heap::heap::Heap;
use crate::heap::java_instance::JavaInstance;
use crate::method_area::java_method::JavaMethod;
use crate::method_area::method_area::MethodArea;
use crate::stack::stack_frame::StackFrame;
use crate::util::{get_class_name_by_cpool_class_index, get_cpool_integer};

pub(crate) struct Engine<'a> {
    method_area: &'a MethodArea,
    heap: Heap<'a>,
}

impl<'a> Engine<'a> {
    pub(crate) fn execute(&mut self, method: &JavaMethod) -> crate::error::Result<Option<i32>> {
        let mut stack_frames = vec![method.new_stack_frame()];
        let mut last_value: Option<i32> = None;
        let mut current_class_name: String;

        while !stack_frames.is_empty() {
            let stack_frame = stack_frames
                .last_mut()
                .ok_or(Error::new_execution("Error getting stack frame"))?;

            current_class_name = stack_frame.current_class_name().to_string();

            match stack_frame.get_bytecode_byte() {
                ICONST_0 => {
                    stack_frame.push(0);
                    stack_frame.incr_pc();
                    println!("ICONST_0");
                }
                ICONST_1 => {
                    stack_frame.push(1);
                    stack_frame.incr_pc();
                    println!("ICONST_1");
                }
                ICONST_2 => {
                    stack_frame.push(2);
                    stack_frame.incr_pc();
                    println!("ICONST_2");
                }
                ICONST_3 => {
                    stack_frame.push(3);
                    stack_frame.incr_pc();
                    println!("ICONST_3");
                }
                ICONST_4 => {
                    stack_frame.push(4);
                    stack_frame.incr_pc();
                    println!("ICONST_4");
                }
                ICONST_5 => {
                    stack_frame.push(5);
                    stack_frame.incr_pc();
                    println!("ICONST_5");
                }
                BIPUSH => {
                    stack_frame.incr_pc();
                    let value = stack_frame.get_bytecode_byte() as i32;
                    stack_frame.push(value);
                    stack_frame.incr_pc();
                    println!("BIPUSH -> value={value}");
                }
                SIPUSH => {
                    let value = Self::extract_two_bytes(stack_frame) as i32;
                    stack_frame.push(value);
                    stack_frame.incr_pc();
                    println!("SIPUSH -> value={value}");
                }
                LDC => {
                    stack_frame.incr_pc();
                    let cpoolindex = stack_frame.get_bytecode_byte() as usize;

                    let java_class = self
                        .method_area
                        .loaded_classes
                        .get(current_class_name.as_str())
                        .unwrap();

                    // todo add support of other types
                    let value = get_cpool_integer(&java_class.class_file, cpoolindex).unwrap();

                    stack_frame.push(value);

                    stack_frame.incr_pc();
                    println!("LDC -> cpoolindex={cpoolindex}, value={value}");
                }
                ILOAD => {
                    stack_frame.incr_pc();
                    let pos = stack_frame.get_bytecode_byte() as usize;

                    let value = stack_frame.get_local(pos);
                    stack_frame.push(value);

                    stack_frame.incr_pc();
                    println!("ILOAD -> pos={pos}, value={value}");
                }
                ALOAD => {
                    stack_frame.incr_pc();
                    let index = stack_frame.get_bytecode_byte() as usize;

                    let objectref = stack_frame.get_local(index);
                    stack_frame.push(objectref);

                    stack_frame.incr_pc();
                    println!("ALOAD -> index={index}, objectref={objectref}");
                }
                ILOAD_0 => {
                    let value = stack_frame.get_local(0);
                    stack_frame.push(value);
                    stack_frame.incr_pc();
                    println!("ILOAD_0 -> value={value}");
                }
                ILOAD_1 => {
                    let value = stack_frame.get_local(1);
                    stack_frame.push(value);

                    stack_frame.incr_pc();
                    println!("ILOAD_1 -> value={value}");
                }
                ILOAD_2 => {
                    let value = stack_frame.get_local(2);
                    stack_frame.push(value);
                    stack_frame.incr_pc();
                    println!("ILOAD_2 -> value={value}");
                }
                ILOAD_3 => {
                    let value = stack_frame.get_local(3);
                    stack_frame.push(value);
                    stack_frame.incr_pc();
                    println!("ILOAD_3 -> value={value}");
                }
                ALOAD_0 => {
                    let reference = stack_frame.get_local(0);
                    stack_frame.push(reference);
                    stack_frame.incr_pc();
                    println!("ALOAD_0 -> reference={reference}");
                }
                ALOAD_1 => {
                    let reference = stack_frame.get_local(1);
                    stack_frame.push(reference);
                    stack_frame.incr_pc();
                    println!("ALOAD_1 -> reference={reference}");
                }
                ALOAD_2 => {
                    let reference = stack_frame.get_local(2);
                    stack_frame.push(reference);
                    stack_frame.incr_pc();
                    println!("ALOAD_2 -> reference={reference}");
                }
                ALOAD_3 => {
                    let reference = stack_frame.get_local(3);
                    stack_frame.push(reference);
                    stack_frame.incr_pc();
                    println!("ALOAD_3 -> reference={reference}");
                }
                IALOAD => {
                    let index = stack_frame.pop();
                    let arrayref = stack_frame.pop();
                    let value = self.heap.get_array_value(arrayref, index)?;

                    stack_frame.push(value);
                    stack_frame.incr_pc();
                    println!("IALOAD -> arrayref={arrayref}, index={index}, value={value}");
                }
                AALOAD => {
                    let index = stack_frame.pop();
                    let arrayref = stack_frame.pop();
                    let objref = self.heap.get_array_value(arrayref, index)?;

                    stack_frame.push(objref);
                    stack_frame.incr_pc();
                    println!("AALOAD -> arrayref={arrayref}, index={index}, objref={objref}");
                }
                ISTORE => {
                    stack_frame.incr_pc();
                    let pos = stack_frame.get_bytecode_byte() as usize;
                    let value = stack_frame.pop();

                    stack_frame.set_local(pos, value);
                    stack_frame.incr_pc();
                    println!("ISTORE -> pos={pos}, value={value}");
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
                    let value = stack_frame.pop();
                    stack_frame.set_local(0, value);

                    stack_frame.incr_pc();
                    println!("ISTORE_0 -> value={value}");
                }
                ISTORE_1 => {
                    let value = stack_frame.pop();
                    stack_frame.set_local(1, value);

                    stack_frame.incr_pc();
                    println!("ISTORE_1 -> value={value}");
                }
                ISTORE_2 => {
                    let value = stack_frame.pop();
                    stack_frame.set_local(2, value);

                    stack_frame.incr_pc();
                    println!("ISTORE_2 -> value={value}");
                }
                ISTORE_3 => {
                    let value = stack_frame.pop();
                    stack_frame.set_local(3, value);

                    stack_frame.incr_pc();
                    println!("ISTORE_3 -> value={value}");
                }
                ASTORE_0 => {
                    let objectref = stack_frame.pop();
                    stack_frame.set_local(0, objectref);

                    stack_frame.incr_pc();
                    println!("ASTORE_0 -> objectref={objectref}");
                }
                ASTORE_1 => {
                    let objectref = stack_frame.pop();
                    stack_frame.set_local(1, objectref);

                    stack_frame.incr_pc();
                    println!("ASTORE_1 -> objectref={objectref}");
                }
                ASTORE_2 => {
                    let objectref = stack_frame.pop();
                    stack_frame.set_local(2, objectref);

                    stack_frame.incr_pc();
                    println!("ASTORE_2 -> objectref={objectref}");
                }
                ASTORE_3 => {
                    let objectref = stack_frame.pop();
                    stack_frame.set_local(3, objectref);

                    stack_frame.incr_pc();
                    println!("ASTORE_3 -> objectref={objectref}");
                }
                IASTORE => {
                    let value = stack_frame.pop();
                    let index = stack_frame.pop();
                    let arrayref = stack_frame.pop();

                    self.heap.set_array_value(arrayref, index, value)?;

                    stack_frame.incr_pc();
                    println!("IASTORE -> arrayref={arrayref}, index={index}, value={value}");
                }
                AASTORE => {
                    let objref = stack_frame.pop();
                    let index = stack_frame.pop();
                    let arrayref = stack_frame.pop();

                    self.heap.set_array_value(arrayref, index, objref)?;

                    stack_frame.incr_pc();
                    println!("AASTORE -> arrayref={arrayref}, index={index}, objref={objref}");
                }
                POP => {
                    stack_frame.pop();

                    stack_frame.incr_pc();
                    println!("POP");
                }
                DUP => {
                    let value = stack_frame.pop();
                    stack_frame.push(value);
                    stack_frame.push(value);

                    stack_frame.incr_pc();
                    println!("DUP -> value={value}");
                }
                DUP2 => {
                    let value1 = stack_frame.pop();
                    let value2 = stack_frame.pop();
                    stack_frame.push(value2);
                    stack_frame.push(value1);
                    stack_frame.push(value2);
                    stack_frame.push(value1);

                    stack_frame.incr_pc();
                    println!("DUP2 -> value1={value1}, value2={value2}");
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
                IDIV => {
                    let b = stack_frame.pop();
                    let a = stack_frame.pop();
                    let result = a / b; //todo add check for ArithmeticException here
                    stack_frame.push(result);

                    stack_frame.incr_pc();
                    println!("IDIV -> {a} / {b} = {result}");
                }
                IREM => {
                    let b = stack_frame.pop();
                    let a = stack_frame.pop();
                    let result = a % b;
                    stack_frame.push(result);

                    stack_frame.incr_pc();
                    println!("IREM -> {a} % {b} = {result}");
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
                    let value = stack_frame.pop();
                    let offset = Self::get_two_bytes_ahead(stack_frame);
                    stack_frame.advance_pc(if value == 0 { offset } else { 3 });
                    println!("IFEQ -> value={value}, offset={offset}");
                }
                IFNE => {
                    let value = stack_frame.pop();
                    let offset = Self::get_two_bytes_ahead(stack_frame);
                    stack_frame.advance_pc(if value != 0 { offset } else { 3 });
                    println!("IFNE -> value={value}, offset={offset}");
                }
                IF_ICMPEQ => {
                    Self::branch(|a: i32, b| a == b, stack_frame, "IF_ICMPEQ");
                }
                IF_ICMPNE => {
                    Self::branch(|a: i32, b| a != b, stack_frame, "IF_ICMPNE");
                }
                IF_ICMPLT => {
                    Self::branch(|a: i32, b| a < b, stack_frame, "IF_ICMPLT");
                }
                IF_ICMPGE => {
                    Self::branch(|a: i32, b| a >= b, stack_frame, "IF_ICMPGE");
                }
                IF_ICMPGT => {
                    Self::branch(|a: i32, b| a > b, stack_frame, "IF_ICMPGT");
                }
                IF_ICMPLE => {
                    Self::branch(|a: i32, b| a <= b, stack_frame, "IF_ICMPLE");
                }
                GOTO => {
                    let offset = Self::get_two_bytes_ahead(stack_frame);
                    stack_frame.advance_pc(offset);
                    println!("GOTO -> offset={offset}");
                }
                IRETURN => {
                    let ret = stack_frame.pop();
                    stack_frames.pop();
                    stack_frames
                        .last_mut()
                        .ok_or(Error::new_execution("Error getting stack last value"))?
                        .push(ret);
                    println!("IRETURN -> ret={ret}");
                }
                ARETURN => {
                    let objref = stack_frame.pop();

                    stack_frames.pop();
                    stack_frames
                        .last_mut()
                        .ok_or(Error::new_execution("Error getting stack last value"))?
                        .push(objref);
                    println!("IRETURN -> objref={objref}");
                }
                RETURN => {
                    println!("RETURN -> stack_frame.locals={:?}", stack_frame.locals);
                    last_value = stack_frames
                        .last()
                        .ok_or(Error::new_execution("Error getting stack last value"))?
                        .locals
                        .last()
                        .copied();
                    stack_frames.pop(); // Return from method, pop the current frame
                                        // add more logic here
                }
                GETSTATIC => {
                    let fieldref_constpool_index = Self::extract_two_bytes(stack_frame);

                    let java_class = self
                        .method_area
                        .loaded_classes
                        .get(current_class_name.as_str())
                        .unwrap();

                    let (class_name, field_name) =
                        self.method_area.get_fieldname_by_fieldref_cpool_index(
                            java_class,
                            fieldref_constpool_index,
                        )?;

                    let value = self
                        .method_area
                        .loaded_classes
                        .get(&class_name)
                        .unwrap()
                        .fields
                        .field_by_name
                        .get(&field_name)
                        .unwrap()
                        .borrow()
                        .value();
                    stack_frame.push(value);

                    stack_frame.incr_pc();
                    println!("GETSTATIC -> {class_name}.{field_name} is {value}");
                }
                PUTSTATIC => {
                    let fieldref_constpool_index = Self::extract_two_bytes(stack_frame);

                    let java_class = self
                        .method_area
                        .loaded_classes
                        .get(current_class_name.as_str())
                        .unwrap();

                    let (class_name, field_name) =
                        self.method_area.get_fieldname_by_fieldref_cpool_index(
                            java_class,
                            fieldref_constpool_index,
                        )?;

                    let value = stack_frame.pop();

                    self.method_area
                        .set_static_field_value(&class_name, &field_name, value)?;

                    stack_frame.incr_pc();
                    println!("PUTSTATIC -> {class_name}.{field_name} = {value}");
                }
                GETFIELD => {
                    let fieldref_constpool_index = Self::extract_two_bytes(stack_frame);

                    let java_class = self
                        .method_area
                        .loaded_classes
                        .get(current_class_name.as_str())
                        .unwrap();

                    let objectref = stack_frame.pop();
                    let (class_name, field_name) =
                        self.method_area.get_fieldname_by_fieldref_cpool_index(
                            java_class,
                            fieldref_constpool_index,
                        )?;

                    let value = self
                        .heap
                        .get_object_field_value(objectref, field_name.as_str())?;

                    stack_frame.push(value);

                    stack_frame.incr_pc();
                    println!("GETFIELD -> objectref={objectref}, class_name={class_name}, field_name={field_name}, value={value}");
                }
                PUTFIELD => {
                    let fieldref_constpool_index = Self::extract_two_bytes(stack_frame);

                    let java_class = self
                        .method_area
                        .loaded_classes
                        .get(current_class_name.as_str())
                        .unwrap();

                    let (class_name, field_name) =
                        self.method_area.get_fieldname_by_fieldref_cpool_index(
                            java_class,
                            fieldref_constpool_index,
                        )?;
                    let value = stack_frame.pop();
                    let objectref = stack_frame.pop();

                    self.heap
                        .set_object_field_value(objectref, field_name.as_str(), value)?;

                    stack_frame.incr_pc();
                    println!("PUTFIELD -> objectref={objectref}, class_name={class_name}, field_name={field_name} value={value}");
                }
                INVOKEVIRTUAL => {
                    let methodref_constpool_index = Self::extract_two_bytes(stack_frame);

                    let java_class = self
                        .method_area
                        .loaded_classes
                        .get(current_class_name.as_str())
                        .unwrap();

                    let (class_name, method_name, virtual_method) =
                        self.method_area.get_method_by_methodref_cpool_index(
                            java_class,
                            methodref_constpool_index,
                        )?;

                    let mut next_frame = virtual_method.new_stack_frame();
                    let arg_num = virtual_method.get_signature().parameter_types().len();

                    for i in (0..arg_num).rev() {
                        let val = stack_frame.pop();
                        next_frame.set_local(i + 1, val);
                    }
                    let reference = stack_frame.pop();
                    next_frame.set_local(0, reference);

                    stack_frame.incr_pc(); //incr here because of borrowing problem

                    stack_frames.push(next_frame);

                    println!("INVOKEVIRTUAL -> {class_name}.{method_name}({reference}, ...)");
                }
                INVOKESPECIAL => {
                    let methodref_constpool_index = Self::extract_two_bytes(stack_frame);

                    let java_class = self
                        .method_area
                        .loaded_classes
                        .get(current_class_name.as_str())
                        .unwrap();

                    let (class_name, method_name, special_method) =
                        self.method_area.get_method_by_methodref_cpool_index(
                            java_class,
                            methodref_constpool_index,
                        )?;

                    let mut next_frame = special_method.new_stack_frame();
                    let arg_num = special_method.get_signature().parameter_types().len();

                    for i in (0..arg_num).rev() {
                        let val = stack_frame.pop();
                        next_frame.set_local(i + 1, val);
                    }
                    let reference = stack_frame.pop();
                    next_frame.set_local(0, reference);

                    stack_frame.incr_pc(); //incr here because of borrowing problem

                    stack_frames.push(next_frame);
                    println!("INVOKESPECIAL -> {class_name}.{method_name}({reference}, ...)");
                }
                INVOKESTATIC => {
                    let methodref_constpool_index = Self::extract_two_bytes(stack_frame);
                    let java_class = self
                        .method_area
                        .loaded_classes
                        .get(current_class_name.as_str())
                        .unwrap();

                    let (class_name, method_name, static_method) =
                        self.method_area.get_method_by_methodref_cpool_index(
                            java_class,
                            methodref_constpool_index,
                        )?;

                    let mut next_frame = static_method.new_stack_frame();
                    let arg_num = static_method.get_signature().parameter_types().len();

                    for i in (0..arg_num).rev() {
                        let val = stack_frame.pop();
                        next_frame.set_local(i, val);
                    }

                    stack_frame.incr_pc(); //incr here because of borrowing problem
                    stack_frames.push(next_frame);
                    println!("INVOKESTATIC -> {class_name}.{method_name}(...)");
                }
                NEW => {
                    let class_constpool_index = Self::extract_two_bytes(stack_frame) as usize;

                    let java_class = self
                        .method_area
                        .loaded_classes
                        .get(current_class_name.as_str())
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

                    println!("NEW -> class={class_to_invoke_new_for}, reference={instanceref}");
                    stack_frame.incr_pc();
                }
                NEWARRAY => {
                    stack_frame.incr_pc();
                    let atype = stack_frame.get_bytecode_byte();

                    let length = stack_frame.pop();

                    let arrayref = self.heap.create_array(length);
                    stack_frame.push(arrayref);

                    stack_frame.incr_pc();
                    println!("NEWARRAY -> atype={atype}, length={length}, arrayref={arrayref}");
                }
                ANEWARRAY => {
                    let length = stack_frame.pop();

                    let class_constpool_index = Self::extract_two_bytes(stack_frame) as usize;
                    let java_class = self
                        .method_area
                        .loaded_classes
                        .get(current_class_name.as_str())
                        .unwrap();
                    let class_of_array = get_class_name_by_cpool_class_index(
                        class_constpool_index,
                        &java_class.class_file,
                    )
                    .unwrap();

                    let arrayref = self.heap.create_array(length);
                    stack_frame.push(arrayref);

                    stack_frame.incr_pc();
                    println!("ANEWARRAY -> class_of_array={class_of_array}, length={length}, arrayref={arrayref}");
                }
                ARRAYLENGTH => {
                    let arrayref = stack_frame.pop();

                    let len = self.heap.get_array_len(arrayref)?;
                    stack_frame.push(len);

                    stack_frame.incr_pc();
                    println!("ARRAYLENGTH -> arrayref={arrayref}, len={len}");
                }
                _ => unreachable!("{}", format! {"xxx = {}", stack_frame.get_bytecode_byte()}),
            }
        }

        Ok(last_value)
    }

    fn get_two_bytes_ahead(stack_frame: &mut StackFrame) -> i16 {
        let branchbyte1 = stack_frame.get_bytecode_byte_1() as u16;
        let branchbyte2 = stack_frame.get_bytecode_byte_2() as u16;

        ((branchbyte1 << 8) | branchbyte2) as i16
    }

    fn extract_two_bytes(stack_frame: &mut StackFrame) -> u16 {
        stack_frame.incr_pc();
        let high = stack_frame.get_bytecode_byte() as u16;
        stack_frame.incr_pc();
        let low = stack_frame.get_bytecode_byte() as u16;

        (high << 8) | low
    }

    fn branch<T>(op: impl Fn(T, T) -> bool, stack_frame: &mut StackFrame, op_code: &str)
    where
        T: PartialOrd + TryFrom<i32> + Copy + std::fmt::Display,
    {
        let value2 =
            T::try_from(stack_frame.pop()).unwrap_or_else(|_| panic!("Invalid conversion"));
        let value1 =
            T::try_from(stack_frame.pop()).unwrap_or_else(|_| panic!("Invalid conversion"));

        let offset = Self::get_two_bytes_ahead(stack_frame);

        stack_frame.advance_pc(if op(value1, value2) { offset } else { 3 });

        println!("{op_code} -> value1={value1}, value2={value2}, offset={offset}");
    }

    pub fn new(method_area: &'a MethodArea) -> Self {
        Self {
            method_area,
            heap: Heap::new(),
        }
    }
}
