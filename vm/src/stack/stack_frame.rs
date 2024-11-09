use crate::helper::i32toi64;
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

    pub fn extract_two_bytes(&mut self) -> i16 {
        self.incr_pc();
        let high = self.get_bytecode_byte() as i16;
        self.incr_pc();
        let low = self.get_bytecode_byte() as i16;

        (high << 8) | (low)
    }

    pub fn extract_four_bytes(&mut self) -> i32 {
        self.incr_pc();
        let byte1 = self.get_bytecode_byte() as u32;
        self.incr_pc();
        let byte2 = self.get_bytecode_byte() as u32;
        self.incr_pc();
        let byte3 = self.get_bytecode_byte() as u32;
        self.incr_pc();
        let byte4 = self.get_bytecode_byte() as u32;

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

    pub fn set_local(&mut self, index: usize, val: i32) {
        self.locals[index] = val;
    }

    pub fn get_local(&self, index: usize) -> i32 {
        *self.locals.get(index).expect("No value at index")
    }

    pub fn get_two_bytes_from_local(&self, index: usize) -> (i32, i32, i64) {
        let low = self.get_local(index);
        let high = self.get_local(index + 1);

        let value = i32toi64(high, low);

        (low, high, value)
    }
}
