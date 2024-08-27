use crate::error::Error;
use crate::execution_engine::opcode::*;
use crate::method_area::java_method::JavaMethod;
use crate::method_area::method_area::MethodArea;

pub(crate) struct Engine<'a> {
    method_area: &'a MethodArea,
}

impl<'a> Engine<'a> {
    pub(crate) fn execute(&self, method: &JavaMethod) -> crate::error::Result<Option<i32>> {
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
                ISTORE => {
                    println!("ISTORE");
                    stack_frame.incr_pc();
                    let pos = stack_frame.get_bytecode_byte() as usize;

                    let val = stack_frame.pop();

                    stack_frame.set_local(pos, val);

                    stack_frame.incr_pc();
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
                IADD => {
                    println!("IADD");
                    let b = stack_frame.pop();
                    let a = stack_frame.pop();
                    stack_frame.push(a + b);

                    stack_frame.incr_pc();
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
                    let static_method = self
                        .method_area
                        .get_method_by_cpool_index(methodref_constpool_index)?;

                    let mut next_frame = static_method.new_stack_frame();
                    let arg_num = static_method.get_signature().get_arg_num();

                    for i in (0..arg_num).rev() {
                        let val = stack_frame.pop();
                        next_frame.set_local(i, val);
                    }

                    stack_frame.incr_pc(); //incr here because of borrowing problem

                    stack_frames.push(next_frame);
                }
                _ => unreachable!("{}", format! {"xxx = {}", stack_frame.get_bytecode_byte()}),
            }
        }

        Ok(last_value)
    }

    pub fn new(method_area: &'a MethodArea) -> Self {
        Self { method_area }
    }
}
