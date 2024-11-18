use crate::stack::sack_value::StackValue;
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct StackFrame {
    pc: usize,
    locals: Vec<i32>,
    operand_stack: Vec<i32>,
    bytecode_ref: Arc<Vec<u8>>,
    current_class_name: String,
}

impl StackFrame {
    pub fn new(
        locals_size: usize,
        stack_size: usize,
        bytecode_ref: Arc<Vec<u8>>,
        current_class_name: String,
    ) -> Self {
        StackFrame {
            pc: 0,
            locals: vec![0i32; locals_size],
            operand_stack: Vec::with_capacity(stack_size),
            bytecode_ref,
            current_class_name,
        }
    }

    pub fn pc(&self) -> usize {
        self.pc
    }

    pub fn locals(&self) -> &[i32] {
        &self.locals
    }

    pub fn current_class_name(&self) -> &str {
        &self.current_class_name
    }

    pub fn get_bytecode_byte(&self) -> u8 {
        self.bytecode_ref[self.pc]
    }

    pub fn get_two_bytes_ahead(&self) -> i16 {
        let branchbyte1 = self.bytecode_ref[self.pc + 1] as u16;
        let branchbyte2 = self.bytecode_ref[self.pc + 2] as u16;

        ((branchbyte1 << 8) | branchbyte2) as i16
    }

    pub fn extract_one_byte(&mut self) -> u8 {
        self.incr_pc();
        self.get_bytecode_byte()
    }

    pub fn extract_two_bytes(&mut self) -> i16 {
        let high = self.extract_one_byte() as i16;
        let low = self.extract_one_byte() as i16;

        (high << 8) | (low)
    }

    pub fn extract_four_bytes(&mut self) -> i32 {
        let byte1 = self.extract_one_byte() as u32;
        let byte2 = self.extract_one_byte() as u32;
        let byte3 = self.extract_one_byte() as u32;
        let byte4 = self.extract_one_byte() as u32;

        ((byte1 << 24) | (byte2 << 16) | (byte3 << 8) | byte4) as i32
    }

    pub fn incr_pc(&mut self) {
        self.advance_pc(1)
    }

    pub fn adjust_pc_to_4(&mut self) {
        while (self.pc + 1) % 4 != 0 {
            self.pc += 1;
        }
    }

    pub fn advance_pc(&mut self, offset: i16) {
        if offset >= 0 {
            self.pc += offset as usize;
        } else {
            self.pc -= (-offset) as usize;
        }
    }

    pub fn push<T: StackValue>(&mut self, stack_value: T) {
        stack_value.push_onto(self);
    }

    pub fn pop<T: StackValue>(&mut self) -> T {
        T::pop_from(self)
    }

    pub(crate) fn push_raw(&mut self, val: i32) {
        self.operand_stack.push(val);
    }

    pub(crate) fn pop_raw(&mut self) -> i32 {
        self.operand_stack.pop().expect("Empty stack")
    }

    pub fn set_local<T: StackValue>(&mut self, index: usize, stack_value: T) {
        stack_value.set(index, self);
    }

    pub fn get_local<T: StackValue>(&mut self, index: usize) -> T {
        T::get(index, self)
    }

    pub fn set_local_raw(&mut self, index: usize, val: i32) {
        self.locals[index] = val;
    }

    pub fn get_local_raw(&self, index: usize) -> i32 {
        *self.locals.get(index).expect("No value at index")
    }
}
