use crate::error::Error;
use crate::execution_engine::ldc_resolution_manager::LdcResolutionManager;
use crate::execution_engine::opcode::*;
use crate::execution_engine::system_native_table::invoke_native_method;
use crate::heap::heap::{with_heap_read_lock, with_heap_write_lock};
use crate::helper::i32toi64;
use crate::method_area::instance_checker::InstanceChecker;
use crate::method_area::method_area::with_method_area;
use crate::stack::stack_frame::StackFrame;
use jdescriptor::get_length;

pub(crate) struct Engine {
    ldc_resolution_manager: LdcResolutionManager,
    instance_checker: InstanceChecker,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            ldc_resolution_manager: LdcResolutionManager::new(),
            instance_checker: InstanceChecker::new(),
        }
    }

    pub(crate) fn execute(
        &mut self,
        stack_frame: StackFrame,
    ) -> crate::error::Result<Option<Vec<i32>>> {
        let mut stack_frames = vec![stack_frame];
        let mut last_value: Option<Vec<i32>> = None;
        let mut current_class_name: String;

        while !stack_frames.is_empty() {
            let stack_frame = stack_frames
                .last_mut()
                .ok_or(Error::new_execution("Error getting stack frame"))?;

            current_class_name = stack_frame.current_class_name().to_string();

            match stack_frame.get_bytecode_byte() {
                ACONST_NULL => {
                    stack_frame.push(0);
                    stack_frame.incr_pc();
                    println!("ACONST_NULL");
                }
                ICONST_M1 => {
                    stack_frame.push(-1);
                    stack_frame.incr_pc();
                    println!("ICONST_M1");
                }
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
                LCONST_0 => {
                    stack_frame.push_i64(0i64);
                    stack_frame.incr_pc();
                    println!("LCONST_0");
                }
                LCONST_1 => {
                    stack_frame.push_i64(1i64);
                    stack_frame.incr_pc();
                    println!("LCONST_1");
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
                    let cpoolindex = stack_frame.get_bytecode_byte() as u16;

                    let value = self
                        .ldc_resolution_manager
                        .resolve_ldc(&current_class_name, cpoolindex)?;

                    stack_frame.push(value);

                    stack_frame.incr_pc();
                    println!("LDC -> cpoolindex={cpoolindex}, value={value}");
                }
                LDC_W => {
                    //todo: merge me with LDC
                    let cpoolindex = Self::extract_two_bytes(stack_frame) as u16;

                    let value = self
                        .ldc_resolution_manager
                        .resolve_ldc(&current_class_name, cpoolindex)?;

                    stack_frame.push(value);

                    stack_frame.incr_pc();
                    println!("LDC_W -> cpoolindex={cpoolindex}, value={value}");
                }
                LDC2_W => {
                    //todo: merge me with LDC
                    let cpoolindex = Self::extract_two_bytes(stack_frame) as u16;

                    let value = self
                        .ldc_resolution_manager
                        .resolve_ldc2_w(&current_class_name, cpoolindex)?;

                    stack_frame.push_i64(value);

                    stack_frame.incr_pc();
                    println!("LDC2_W -> cpoolindex={cpoolindex}, value={value}");
                }
                ILOAD => {
                    stack_frame.incr_pc();
                    let pos = stack_frame.get_bytecode_byte() as usize;

                    let value = stack_frame.get_local(pos);
                    stack_frame.push(value);

                    stack_frame.incr_pc();
                    println!("ILOAD -> pos={pos}, value={value}");
                }
                LLOAD => {
                    stack_frame.incr_pc();
                    let pos = stack_frame.get_bytecode_byte() as usize;

                    let (low, high, value) = stack_frame.get_two_bytes_from_local(pos);

                    stack_frame.push(low);
                    stack_frame.push(high);

                    stack_frame.incr_pc();
                    println!("LLOAD -> pos={pos}, value={value}");
                }
                DLOAD => {
                    stack_frame.incr_pc();
                    let pos = stack_frame.get_bytecode_byte() as usize;

                    let (low, high, value) = stack_frame.get_two_bytes_from_local(pos);

                    stack_frame.push(low);
                    stack_frame.push(high);

                    stack_frame.incr_pc();
                    println!("DLOAD -> pos={pos}, value={value}");
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
                LLOAD_0 => {
                    let (low, high, value) = stack_frame.get_two_bytes_from_local(0);

                    stack_frame.push(low);
                    stack_frame.push(high);

                    stack_frame.incr_pc();
                    println!("LLOAD_0 -> value={value}");
                }
                LLOAD_1 => {
                    let (low, high, value) = stack_frame.get_two_bytes_from_local(1);

                    stack_frame.push(low);
                    stack_frame.push(high);

                    stack_frame.incr_pc();
                    println!("LLOAD_1 -> value={value}");
                }
                LLOAD_2 => {
                    let (low, high, value) = stack_frame.get_two_bytes_from_local(2);

                    stack_frame.push(low);
                    stack_frame.push(high);

                    stack_frame.incr_pc();
                    println!("LLOAD_2 -> value={value}");
                }
                LLOAD_3 => {
                    let (low, high, value) = stack_frame.get_two_bytes_from_local(3);

                    stack_frame.push(low);
                    stack_frame.push(high);

                    stack_frame.incr_pc();
                    println!("LLOAD_3 -> value={value}");
                }
                FLOAD_1 => {
                    let value = stack_frame.get_local(1);
                    stack_frame.push(value);

                    stack_frame.incr_pc();
                    println!("FLOAD_1 -> value={value}");
                }
                DLOAD_0 => {
                    let (low, high, value) = stack_frame.get_two_bytes_from_local(0);

                    stack_frame.push(low);
                    stack_frame.push(high);

                    stack_frame.incr_pc();
                    println!("DLOAD_0 -> value={value}");
                }
                DLOAD_1 => {
                    let (low, high, value) = stack_frame.get_two_bytes_from_local(1);

                    stack_frame.push(low);
                    stack_frame.push(high);

                    stack_frame.incr_pc();
                    println!("DLOAD_1 -> value={value}");
                }
                DLOAD_3 => {
                    let (low, high, value) = stack_frame.get_two_bytes_from_local(3);

                    stack_frame.push(low);
                    stack_frame.push(high);

                    stack_frame.incr_pc();
                    println!("DLOAD_3 -> value={value}");
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
                    let value = with_heap_read_lock(|heap| {
                        heap.get_array_value(arrayref, index).cloned()
                    })?;

                    stack_frame.push(value[0]);
                    stack_frame.incr_pc();
                    println!(
                        "IALOAD -> arrayref={arrayref}, index={index}, value={}",
                        value[0]
                    );
                }
                LALOAD => {
                    let index = stack_frame.pop();
                    let arrayref = stack_frame.pop();
                    let value = with_heap_read_lock(|heap| {
                        heap.get_array_value(arrayref, index).cloned()
                    })?;

                    let high = value[0];
                    let low = value[1];

                    stack_frame.push(low);
                    stack_frame.push(high);
                    stack_frame.incr_pc();
                    println!("LALOAD -> arrayref={arrayref}, index={index}, value={value:?}");
                }
                AALOAD => {
                    let index = stack_frame.pop();
                    let arrayref = stack_frame.pop();
                    let objref = with_heap_read_lock(|heap| {
                        heap.get_array_value(arrayref, index).cloned()
                    })?;

                    stack_frame.push(objref[0]);
                    stack_frame.incr_pc();
                    println!(
                        "AALOAD -> arrayref={arrayref}, index={index}, objref={}",
                        objref[0]
                    );
                }
                BALOAD => {
                    let index = stack_frame.pop();
                    let arrayref = stack_frame.pop();
                    let value = with_heap_read_lock(|heap| {
                        heap.get_array_value(arrayref, index).cloned()
                    })?;

                    stack_frame.push(value[0]);
                    stack_frame.incr_pc();
                    println!(
                        "BALOAD -> arrayref={arrayref}, index={index}, value={}",
                        value[0]
                    );
                }
                CALOAD => {
                    let index = stack_frame.pop();
                    let arrayref = stack_frame.pop();
                    let value = with_heap_read_lock(|heap| {
                        heap.get_array_value(arrayref, index).cloned()
                    })?;

                    stack_frame.push(value[0]);
                    stack_frame.incr_pc();
                    println!(
                        "CALOAD -> arrayref={arrayref}, index={index}, value={}",
                        value[0]
                    );
                }
                ISTORE => {
                    stack_frame.incr_pc();
                    let pos = stack_frame.get_bytecode_byte() as usize;
                    let value = stack_frame.pop();

                    stack_frame.set_local(pos, value);
                    stack_frame.incr_pc();
                    println!("ISTORE -> pos={pos}, value={value}");
                }
                LSTORE => {
                    stack_frame.incr_pc();
                    let pos = stack_frame.get_bytecode_byte() as usize;
                    let high = stack_frame.pop();
                    let low = stack_frame.pop();

                    stack_frame.set_local(pos, low);
                    stack_frame.set_local(pos + 1, high);

                    stack_frame.incr_pc();
                    let value = i32toi64(high, low);
                    println!("LSTORE -> value={value}");
                }
                DSTORE => {
                    stack_frame.incr_pc();
                    let pos = stack_frame.get_bytecode_byte() as usize;
                    let high = stack_frame.pop();
                    let low = stack_frame.pop();

                    stack_frame.set_local(pos, low);
                    stack_frame.set_local(pos + 1, high);

                    stack_frame.incr_pc();
                    let value = i32toi64(high, low);
                    println!("DSTORE -> value={value}");
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
                LSTORE_1 => {
                    let high = stack_frame.pop();
                    let low = stack_frame.pop();

                    stack_frame.set_local(1, low);
                    stack_frame.set_local(2, high);

                    stack_frame.incr_pc();
                    let value = ((high as i64) << 32) | (low as i64);
                    println!("LSTORE_1 -> value={value}");
                }
                LSTORE_2 => {
                    let high = stack_frame.pop();
                    let low = stack_frame.pop();

                    stack_frame.set_local(2, low);
                    stack_frame.set_local(3, high);

                    stack_frame.incr_pc();
                    let value = ((high as i64) << 32) | (low as i64);
                    println!("LSTORE_2 -> value={value}");
                }
                LSTORE_3 => {
                    let high = stack_frame.pop();
                    let low = stack_frame.pop();

                    stack_frame.set_local(3, low);
                    stack_frame.set_local(4, high);

                    stack_frame.incr_pc();
                    let value = ((high as i64) << 32) | (low as i64);
                    println!("LSTORE_3 -> value={value}");
                }
                FSTORE_1 => {
                    let value = stack_frame.pop();
                    stack_frame.set_local(1, value);

                    stack_frame.incr_pc();
                    println!("FSTORE_1 -> value={value}");
                }
                FSTORE_3 => {
                    let value = stack_frame.pop();
                    stack_frame.set_local(3, value);

                    stack_frame.incr_pc();
                    println!("FSTORE_3 -> value={value}");
                }
                DSTORE_1 => {
                    let high = stack_frame.pop();
                    let low = stack_frame.pop();

                    stack_frame.set_local(1, low);
                    stack_frame.set_local(2, high);

                    stack_frame.incr_pc();
                    let value = ((high as i64) << 32) | (low as i64);
                    println!("DSTORE_1 -> value={value}");
                }
                DSTORE_3 => {
                    let high = stack_frame.pop();
                    let low = stack_frame.pop();

                    stack_frame.set_local(3, low);
                    stack_frame.set_local(4, high);

                    stack_frame.incr_pc();
                    let value = ((high as i64) << 32) | (low as i64);
                    println!("DSTORE_3 -> value={value}");
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

                    with_heap_write_lock(|heap| {
                        heap.set_array_value(arrayref, index, vec![value])
                    })?;

                    stack_frame.incr_pc();
                    println!("IASTORE -> arrayref={arrayref}, index={index}, value={value}");
                }
                LASTORE => {
                    let high = stack_frame.pop();
                    let low = stack_frame.pop();

                    let value = vec![high, low];
                    let index = stack_frame.pop();
                    let arrayref = stack_frame.pop();

                    with_heap_write_lock(|heap| {
                        heap.set_array_value(arrayref, index, value.clone())
                    })?;

                    stack_frame.incr_pc();
                    println!("LASTORE -> arrayref={arrayref}, index={index}, value={value:?}");
                }
                AASTORE => {
                    let objref = stack_frame.pop();
                    let index = stack_frame.pop();
                    let arrayref = stack_frame.pop();

                    with_heap_write_lock(|heap| {
                        heap.set_array_value(arrayref, index, vec![objref])
                    })?;

                    stack_frame.incr_pc();
                    println!("AASTORE -> arrayref={arrayref}, index={index}, objref={objref}");
                }
                BASTORE => {
                    let value = stack_frame.pop();
                    let index = stack_frame.pop();
                    let arrayref = stack_frame.pop();

                    with_heap_write_lock(|heap| {
                        heap.set_array_value(arrayref, index, vec![value])
                    })?;

                    stack_frame.incr_pc();
                    println!("BASTORE -> arrayref={arrayref}, index={index}, value={value}");
                }
                CASTORE => {
                    let value = stack_frame.pop();
                    let index = stack_frame.pop();
                    let arrayref = stack_frame.pop();

                    with_heap_write_lock(|heap| {
                        heap.set_array_value(arrayref, index, vec![value])
                    })?;

                    stack_frame.incr_pc();
                    println!("CASTORE -> arrayref={arrayref}, index={index}, value={value}");
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
                DUP_X1 => {
                    let value1 = stack_frame.pop();
                    let value2 = stack_frame.pop();
                    stack_frame.push(value1);
                    stack_frame.push(value2);
                    stack_frame.push(value1);

                    stack_frame.incr_pc();
                    println!("DUP_X1 -> value1={value1}, value2={value2}, value1={value1}");
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
                LADD => {
                    let b = stack_frame.pop_i64();
                    let a = stack_frame.pop_i64();

                    let result = a + b;

                    stack_frame.push_i64(result);

                    stack_frame.incr_pc();
                    println!("LADD -> {a} + {b} = {result}");
                }
                DADD => {
                    let b = f64::from_bits(stack_frame.pop_i64() as u64);
                    let a = f64::from_bits(stack_frame.pop_i64() as u64);

                    let result = a + b;

                    stack_frame.push_i64(result.to_bits() as i64);

                    stack_frame.incr_pc();
                    println!("DADD -> {a} + {b} = {result}");
                }
                ISUB => {
                    let b = stack_frame.pop();
                    let a = stack_frame.pop();
                    let result = a - b;
                    stack_frame.push(result);

                    stack_frame.incr_pc();
                    println!("ISUB -> {a} - {b} = {result}");
                }
                LSUB => {
                    let b = stack_frame.pop_i64();
                    let a = stack_frame.pop_i64();

                    let result = a - b;

                    stack_frame.push_i64(result);

                    stack_frame.incr_pc();
                    println!("LSUB -> {a} - {b} = {result}");
                }
                FSUB => {
                    let b = stack_frame.pop_f32();
                    let a = stack_frame.pop_f32();
                    let result = a - b;
                    stack_frame.push_f32(result);

                    stack_frame.incr_pc();
                    println!("FSUB -> {a} - {b} = {result}");
                }
                DSUB => {
                    let b = f64::from_bits(stack_frame.pop_i64() as u64);
                    let a = f64::from_bits(stack_frame.pop_i64() as u64);

                    let result = a - b;

                    stack_frame.push_i64(result.to_bits() as i64);

                    stack_frame.incr_pc();
                    println!("DSUB -> {a} - {b} = {result}");
                }
                IMUL => {
                    let b = stack_frame.pop();
                    let a = stack_frame.pop();
                    let result = a * b;
                    stack_frame.push(result);

                    stack_frame.incr_pc();
                    println!("IMUL -> {a} * {b} = {result}");
                }
                LMUL => {
                    let b = stack_frame.pop_i64();
                    let a = stack_frame.pop_i64();

                    let result = b.wrapping_mul(a);

                    stack_frame.push_i64(result);

                    stack_frame.incr_pc();
                    println!("LMUL -> {a} * {b} = {result}");
                }
                DMUL => {
                    let b = f64::from_bits(stack_frame.pop_i64() as u64);
                    let a = f64::from_bits(stack_frame.pop_i64() as u64);

                    let result = a * b;

                    stack_frame.push_i64(result.to_bits() as i64);

                    stack_frame.incr_pc();
                    println!("DMUL -> {a} * {b} = {result}");
                }
                IDIV => {
                    let b = stack_frame.pop();
                    let a = stack_frame.pop();
                    let result = a / b; //todo add check for ArithmeticException here
                    stack_frame.push(result);

                    stack_frame.incr_pc();
                    println!("IDIV -> {a} / {b} = {result}");
                }
                DDIV => {
                    let b = f64::from_bits(stack_frame.pop_i64() as u64);
                    let a = f64::from_bits(stack_frame.pop_i64() as u64);

                    let result = a / b;

                    stack_frame.push_i64(result.to_bits() as i64);

                    stack_frame.incr_pc();
                    println!("DDIV -> {a} / {b} = {result}");
                }
                IREM => {
                    let b = stack_frame.pop();
                    let a = stack_frame.pop();
                    let result = a % b;
                    stack_frame.push(result);

                    stack_frame.incr_pc();
                    println!("IREM -> {a} % {b} = {result}");
                }
                LREM => {
                    let b = stack_frame.pop_i64();
                    let a = stack_frame.pop_i64();

                    let result = a % b;

                    stack_frame.push_i64(result);

                    stack_frame.incr_pc();
                    println!("LREM -> {a} % {b} = {result}");
                }
                DREM => {
                    let b = f64::from_bits(stack_frame.pop_i64() as u64);
                    let a = f64::from_bits(stack_frame.pop_i64() as u64);

                    let result = a % b;

                    stack_frame.push_i64(result.to_bits() as i64);

                    stack_frame.incr_pc();
                    println!("DREM -> {a} % {b} = {result}");
                }
                ISHL => {
                    let b = stack_frame.pop();
                    let a = stack_frame.pop();

                    let b_trunc = b & 0b00011111;
                    let result = a << b_trunc;
                    stack_frame.push(result);

                    stack_frame.incr_pc();
                    println!("ISHL -> {a} << {b} = {result}");
                }
                ISHR => {
                    // todo: recheck spec
                    let b = stack_frame.pop() as u32;
                    let a = stack_frame.pop();

                    let b_trunc = b & 0b00011111u32;
                    let result = a >> b_trunc;
                    stack_frame.push(result);

                    stack_frame.incr_pc();
                    println!("ISHR -> {a} >> {b} = {result}");
                }
                IUSHR => {
                    let b = stack_frame.pop() as u32;
                    let a = stack_frame.pop() as u32;

                    let b_trunc = b & 0b00011111u32;
                    let result = a >> b_trunc;
                    stack_frame.push(result as i32);

                    stack_frame.incr_pc();
                    println!("IUSHR -> {a} % {b} = {result}");
                }
                IAND => {
                    let b = stack_frame.pop();
                    let a = stack_frame.pop();

                    let result = a & b;
                    stack_frame.push(result);

                    stack_frame.incr_pc();
                    println!("IAND -> {a} & {b} = {result}");
                }
                IOR => {
                    let b = stack_frame.pop();
                    let a = stack_frame.pop();

                    let result = a | b;
                    stack_frame.push(result);

                    stack_frame.incr_pc();
                    println!("IOR -> {a} | {b} = {result}");
                }
                IXOR => {
                    let b = stack_frame.pop();
                    let a = stack_frame.pop();

                    let result = a ^ b;
                    stack_frame.push(result);

                    stack_frame.incr_pc();
                    println!("IXOR -> {a} & {b} = {result}");
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
                I2L => {
                    let value = stack_frame.pop() as i64;

                    stack_frame.push_i64(value);

                    stack_frame.incr_pc();
                    println!("I2L -> {value}L");
                }
                I2F => {
                    let value = stack_frame.pop() as f32;

                    stack_frame.push_f32(value);

                    stack_frame.incr_pc();
                    println!("I2F -> {value}F");
                }
                I2D => {
                    let value = stack_frame.pop() as f64;

                    stack_frame.push_i64(value.to_bits() as i64);

                    stack_frame.incr_pc();
                    println!("I2D -> {value}D");
                }
                I2B => {
                    let value = stack_frame.pop() as i8;

                    stack_frame.push(value as i32);

                    stack_frame.incr_pc();
                    println!("I2B -> {value}B");
                }
                I2C => {
                    let value = stack_frame.pop() as u16;

                    stack_frame.push(value as i32);

                    stack_frame.incr_pc();
                    println!("I2C -> {value}C");
                }
                LCMP => {
                    let b = stack_frame.pop_i64();
                    let a = stack_frame.pop_i64();

                    if a > b {
                        stack_frame.push(1);
                    } else if a < b {
                        stack_frame.push(-1);
                    } else {
                        stack_frame.push(0);
                    }

                    stack_frame.incr_pc();
                    println!("LCMP -> {a} ? {b}");
                }
                DCMPL => {
                    let b = f64::from_bits(stack_frame.pop_i64() as u64);
                    let a = f64::from_bits(stack_frame.pop_i64() as u64);

                    let result = if a.is_nan() || b.is_nan() {
                        -1
                    } else if a < b {
                        -1
                    } else if a > b {
                        1
                    } else {
                        0
                    };

                    stack_frame.push(result);

                    stack_frame.incr_pc();
                    println!("DCMPL -> {a} ? {b}");
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
                IFLT => {
                    let value = stack_frame.pop();
                    let offset = Self::get_two_bytes_ahead(stack_frame);
                    stack_frame.advance_pc(if value < 0 { offset } else { 3 });
                    println!("IFLT -> value={value}, offset={offset}");
                }
                IFGE => {
                    let value = stack_frame.pop();
                    let offset = Self::get_two_bytes_ahead(stack_frame);
                    stack_frame.advance_pc(if value >= 0 { offset } else { 3 });
                    println!("IFGE -> value={value}, offset={offset}");
                }
                IFGT => {
                    let value = stack_frame.pop();
                    let offset = Self::get_two_bytes_ahead(stack_frame);
                    stack_frame.advance_pc(if value > 0 { offset } else { 3 });
                    println!("IFGT -> value={value}, offset={offset}");
                }
                IFLE => {
                    let value = stack_frame.pop();
                    let offset = Self::get_two_bytes_ahead(stack_frame);
                    stack_frame.advance_pc(if value <= 0 { offset } else { 3 });
                    println!("IFLE -> value={value}, offset={offset}");
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
                IF_ACMPEQ => {
                    Self::branch(|a: i32, b| a == b, stack_frame, "IF_ACMPEQ");
                }
                IF_ACMPNE => {
                    Self::branch(|a: i32, b| a != b, stack_frame, "IF_ACMPNE");
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
                LRETURN => {
                    let ret_high = stack_frame.pop();
                    let ret_low = stack_frame.pop();
                    stack_frames.pop();
                    let frame = stack_frames
                        .last_mut()
                        .ok_or(Error::new_execution("Error getting stack last value"))?;

                    frame.push(ret_low);
                    frame.push(ret_high);

                    let ret = i32toi64(ret_high, ret_low);
                    println!("LRETURN -> ret={ret}");
                }
                ARETURN => {
                    let objref = stack_frame.pop();

                    stack_frames.pop();
                    stack_frames
                        .last_mut()
                        .ok_or(Error::new_execution("Error getting stack last value"))?
                        .push(objref);
                    println!("ARETURN -> objref={objref}");
                }
                RETURN => {
                    println!("RETURN -> stack_frame.locals={:?}", stack_frame.locals);
                    last_value = Some(
                        stack_frames
                            .last()
                            .ok_or(Error::new_execution("Error getting stack last value"))?
                            .locals
                            .clone(),
                    );

                    stack_frames.pop();
                }
                GETSTATIC => {
                    let fieldref_constpool_index = Self::extract_two_bytes(stack_frame) as u16;

                    let java_class = with_method_area(|method_area| {
                        method_area.get(current_class_name.as_str())
                    })?;
                    let cpool_helper = java_class.cpool_helper();

                    let (class_name, field_name, _) =
                        cpool_helper.get_full_field_info(fieldref_constpool_index)
                            .ok_or_else(|| Error::new_constant_pool(&format!("Error getting full field info by index {fieldref_constpool_index}")))?;

                    let field = with_method_area(|method_area| {
                        method_area.lookup_for_static_field(&class_name, &field_name)
                    })?;

                    field
                        .raw_value()
                        .iter()
                        .rev()
                        .for_each(|x| stack_frame.push(*x));

                    println!(
                        "GETSTATIC -> {class_name}.{field_name} is {:?}",
                        field.raw_value()
                    );
                    stack_frame.incr_pc();
                }
                PUTSTATIC => {
                    let fieldref_constpool_index = Self::extract_two_bytes(stack_frame) as u16;

                    let rc = with_method_area(|method_area| {
                        method_area.get(current_class_name.as_str())
                    })?;

                    let cpool_helper = rc.cpool_helper();
                    let (class_name, field_name, _) =
                        cpool_helper.get_full_field_info(fieldref_constpool_index)
                            .ok_or_else(|| Error::new_constant_pool(&format!("Error getting full field info by index {fieldref_constpool_index}")))?;

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

                    println!("PUTSTATIC -> {class_name}.{field_name} = {value:?}");
                    stack_frame.incr_pc();
                }
                GETFIELD => {
                    let fieldref_constpool_index = Self::extract_two_bytes(stack_frame) as u16;

                    let rc = with_method_area(|method_area| {
                        method_area.get(current_class_name.as_str())
                    })?;
                    let cpool_helper = rc.cpool_helper();

                    let objectref = stack_frame.pop();
                    let (class_name, field_name, field_descriptor) =
                        cpool_helper.get_full_field_info(fieldref_constpool_index)
                            .ok_or_else(|| Error::new_constant_pool(&format!("Error getting full field info by index {fieldref_constpool_index}")))?;
                    let field_name_type = format!("{field_name}:{field_descriptor}");

                    let value = with_heap_write_lock(|heap| {
                        heap.get_object_field_value(
                            objectref,
                            class_name.as_str(),
                            field_name_type.as_str(),
                        )
                    })?;

                    value.iter().rev().for_each(|x| stack_frame.push(*x));

                    stack_frame.incr_pc();
                    println!("GETFIELD -> objectref={objectref}, class_name={class_name}, field_name_type={field_name_type}, value={value:?}");
                }
                PUTFIELD => {
                    let fieldref_constpool_index = Self::extract_two_bytes(stack_frame) as u16;

                    let rc = with_method_area(|method_area| {
                        method_area.get(current_class_name.as_str())
                    })?;
                    let cpool_helper = rc.cpool_helper();

                    let (class_name, field_name, field_descriptor) =
                        cpool_helper.get_full_field_info(fieldref_constpool_index)
                            .ok_or_else(|| Error::new_constant_pool(&format!("Error getting full field info by index {fieldref_constpool_index}")))?;

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

                    println!("PUTFIELD -> objectref={objectref}, class_name={class_name}, field_name_type={field_name_type} value={value:?}");
                    stack_frame.incr_pc();
                }
                INVOKEVIRTUAL => {
                    let methodref_constpool_index = Self::extract_two_bytes(stack_frame) as u16;
                    stack_frame.incr_pc();

                    let rc = with_method_area(|method_area| {
                        method_area.get(current_class_name.as_str())
                    })?;
                    let cpool_helper = rc.cpool_helper();

                    let (reference_type_class_name, method_name, method_descriptor) = cpool_helper
                        .get_full_method_info(methodref_constpool_index)
                        .ok_or_else(|| Error::new_constant_pool(&format!("Error getting full method info by index {methodref_constpool_index}")))?;
                    let full_signature = format!("{}:{}", method_name, method_descriptor);

                    let java_method = with_method_area(|method_area| method_area.lookup_for_implementation(&reference_type_class_name, &full_signature))
                            .ok_or_else(|| Error::new_constant_pool(&format!("Error getting instance type JavaMethod by class name {reference_type_class_name} and full signature {full_signature} invoking virtual")))?;
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
                        .ok_or_else(|| Error::new_constant_pool(&format!("Error getting instance type JavaMethod by class name {instance_type_class_name} and full signature {full_signature} invoking virtual")))
                    })?;

                    if virtual_method.is_native() {
                        let full_native_signature =
                            format!("{instance_type_class_name}:{full_signature}");
                        println!(
                            "<Calling native method> -> {full_native_signature} ({method_args:?})"
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
                    println!(
                        "INVOKEVIRTUAL -> {instance_type_class_name}.{method_name}({method_args:?})"
                    );
                }
                INVOKESPECIAL => {
                    let methodref_constpool_index = Self::extract_two_bytes(stack_frame) as u16;
                    stack_frame.incr_pc();

                    let rc = with_method_area(|method_area| {
                        method_area.get(current_class_name.as_str())
                    })?;
                    let cpool_helper = rc.cpool_helper();

                    let (class_name, method_name, method_descriptor) = cpool_helper
                        .get_full_method_info(methodref_constpool_index)
                        .ok_or_else(|| Error::new_constant_pool(&format!("Error getting full method info by index {methodref_constpool_index}")))?;
                    let full_signature = format!("{}:{}", method_name, method_descriptor);
                    let rc = with_method_area(|method_area| method_area.get(class_name.as_str()))?;
                    let special_method = rc
                        .methods
                        .method_by_signature
                        .get(&full_signature)
                        .ok_or_else(|| Error::new_constant_pool(&format!("Error getting JavaMethod by class name {class_name} and full signature {full_signature} invoking special")))?;
                    // ^^^ todo: implement lookup in parents

                    let mut next_frame = special_method.new_stack_frame()?;
                    let arg_num = special_method.get_method_descriptor().arguments_length();

                    let mut method_args = Vec::with_capacity(arg_num);
                    for _ in 0..arg_num {
                        let val = stack_frame.pop();
                        method_args.push(val);
                    }
                    let reference = stack_frame.pop();
                    method_args.push(reference);
                    method_args.reverse();

                    method_args
                        .iter()
                        .enumerate()
                        .for_each(|(index, val)| next_frame.set_local(index, *val));

                    stack_frames.push(next_frame);
                    println!("INVOKESPECIAL -> {class_name}.{method_name}({method_args:?})");
                }
                INVOKESTATIC => {
                    let methodref_constpool_index = Self::extract_two_bytes(stack_frame) as u16;
                    stack_frame.incr_pc();

                    let rc = with_method_area(|method_area| {
                        method_area.get(current_class_name.as_str())
                    })?;
                    let cpool_helper = rc.cpool_helper();

                    let (class_name, method_name, method_descriptor) = cpool_helper
                        .get_full_method_info(methodref_constpool_index)
                        .ok_or_else(|| Error::new_constant_pool(&format!("Error getting full method info by index {methodref_constpool_index}")))?;
                    let full_signature = format!("{}:{}", method_name, method_descriptor);
                    let rc = with_method_area(|method_area| method_area.get(class_name.as_str()))?;
                    let static_method = rc
                        .methods
                        .method_by_signature
                        .get(&full_signature)
                        .ok_or_else(|| Error::new_constant_pool(&format!("Error getting JavaMethod by class name {class_name} and full signature {full_signature} invoking static")))?;

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
                        println!(
                            "<Calling native method> -> {full_native_signature} ({method_args:?})"
                        );

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
                    println!("INVOKESTATIC -> {class_name}.{method_name}({method_args:?})");
                }
                INVOKEINTERFACE => {
                    let interfacemethodref_constpool_index =
                        Self::extract_two_bytes(stack_frame) as u16;
                    stack_frame.incr_pc();

                    let arg_count = stack_frame.get_bytecode_byte() as usize;
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
                        return Err(Error::new_execution(&format!("Error calling interface method by index {interfacemethodref_constpool_index}")));
                    }

                    let rc = with_method_area(|method_area| {
                        method_area.get(current_class_name.as_str())
                    })?;
                    let cpool_helper = rc.cpool_helper();

                    let (interface_class_name, method_name, method_descriptor) = cpool_helper
                        .get_full_interfacemethodref_info(interfacemethodref_constpool_index)
                        .ok_or_else(|| Error::new_constant_pool(&format!("Error getting full interfacemethodref info by index {interfacemethodref_constpool_index}")))?;

                    let instance_name =
                        with_heap_read_lock(|heap| heap.get_instance_name(reference))?;

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

                    println!("INVOKEINTERFACE -> {interface_class_name}.{method_name}{method_descriptor}({method_args:?}) on instance {instance_name}");
                }
                NEW => {
                    let class_constpool_index = Self::extract_two_bytes(stack_frame) as u16;

                    let rc = with_method_area(|method_area| {
                        method_area.get(current_class_name.as_str())
                    })?;
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

                    let instanceref = with_heap_write_lock(|heap| {
                        heap.create_instance(instance_with_default_fields)
                    });
                    stack_frame.push(instanceref);

                    println!("NEW -> class={class_to_invoke_new_for}, reference={instanceref}");
                    stack_frame.incr_pc();
                }
                NEWARRAY => {
                    stack_frame.incr_pc();
                    let atype = stack_frame.get_bytecode_byte();

                    let length = stack_frame.pop();

                    let arrayref = with_heap_write_lock(|heap| heap.create_array(length));
                    stack_frame.push(arrayref);

                    stack_frame.incr_pc();
                    println!("NEWARRAY -> atype={atype}, length={length}, arrayref={arrayref}");
                }
                ANEWARRAY => {
                    let length = stack_frame.pop();

                    let class_constpool_index = Self::extract_two_bytes(stack_frame) as u16;
                    let rc = with_method_area(|method_area| {
                        method_area.get(current_class_name.as_str())
                    })?;
                    let cpool_helper = rc.cpool_helper();

                    let class_of_array = cpool_helper
                        .get_class_name(class_constpool_index)
                        .ok_or_else(|| {
                            Error::new_constant_pool(&format!(
                                "Error getting class name by index {class_constpool_index}"
                            ))
                        })?;
                    let arrayref = with_heap_write_lock(|heap| heap.create_array(length));
                    stack_frame.push(arrayref);

                    stack_frame.incr_pc();
                    println!("ANEWARRAY -> class_of_array={class_of_array}, length={length}, arrayref={arrayref}");
                }
                ARRAYLENGTH => {
                    let arrayref = stack_frame.pop();

                    let len = with_heap_read_lock(|heap| heap.get_array_len(arrayref))?;
                    stack_frame.push(len);

                    stack_frame.incr_pc();
                    println!("ARRAYLENGTH -> arrayref={arrayref}, len={len}");
                }
                CHECKCAST => {
                    let class_constpool_index = Self::extract_two_bytes(stack_frame) as u16;
                    stack_frame.incr_pc();
                    let objectref = stack_frame.pop();

                    if objectref != 0 {
                        let rc = with_method_area(|method_area| {
                            method_area.get(current_class_name.as_str())
                        })?;
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

                        let possible_cast = self
                            .instance_checker
                            .checkcast(&instance_class_name, &class_name)?;
                        if !possible_cast {
                            return Err(Error::new_execution(&format!(
                                "Error casting {instance_class_name} to {class_name}"
                            ))); //todo: throw ClassCastException here
                        }
                    }

                    stack_frame.push(objectref);

                    println!("CHECKCAST -> class_constpool_index={class_constpool_index}, objectref={objectref}");
                }
                INSTANCEOF => {
                    // todo: merge me with CHECKCAST
                    let class_constpool_index = Self::extract_two_bytes(stack_frame) as u16;
                    stack_frame.incr_pc();
                    let mut objectref = stack_frame.pop();

                    if objectref != 0 {
                        let rc = with_method_area(|method_area| {
                            method_area.get(current_class_name.as_str())
                        })?;
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

                        let instanse_of = self
                            .instance_checker
                            .checkcast(&instance_class_name, &class_name)?;
                        objectref = if instanse_of { 1 } else { 0 };
                    }

                    stack_frame.push(objectref);

                    println!("INSTANCEOF -> class_constpool_index={class_constpool_index}, objectref={objectref}");
                }
                IFNULL => {
                    //todo: this one is opposite to IFNE ops code
                    let value = stack_frame.pop();
                    let offset = Self::get_two_bytes_ahead(stack_frame);
                    stack_frame.advance_pc(if value == 0 { offset } else { 3 });
                    println!("IFNULL -> value={value}, offset={offset}");
                }
                IFNONNULL => {
                    //todo: this one is opposite to IFNULL ops code
                    let value = stack_frame.pop();
                    let offset = Self::get_two_bytes_ahead(stack_frame);
                    stack_frame.advance_pc(if value != 0 { offset } else { 3 });
                    println!("IFNONNULL -> value={value}, offset={offset}");
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

    fn extract_two_bytes(stack_frame: &mut StackFrame) -> i16 {
        stack_frame.incr_pc();
        let high = stack_frame.get_bytecode_byte() as i16;
        stack_frame.incr_pc();
        let low = stack_frame.get_bytecode_byte() as i16;

        (high << 8) | (low)
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
}
