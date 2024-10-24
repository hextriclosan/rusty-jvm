use crate::helper::i32toi64;

#[derive(Clone)]
pub(crate) struct StackFrame {
    pc: usize,
    pub locals: Vec<i32>,
    pub(crate) operand_stack: Vec<i32>,
    bytecode_ref: Vec<u8>,
    current_class_name: String,
}

impl<'a> StackFrame {
    pub fn new(
        locals_size: usize,
        stack_size: usize,
        bytecode_ref: Vec<u8>,
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

    pub fn incr_pc(&mut self) {
        self.advance_pc(1)
    }

    pub fn adjust_pc_to_4(&mut self) {
        while (self.pc + 1) % 4 != 0 {
            self.pc += 1;
        }
    }

    pub fn pc(&self) -> usize {
        self.pc
    }

    pub(crate) fn advance_pc(&mut self, offset: i16) {
        if offset >= 0 {
            self.pc += offset as usize;
        } else {
            self.pc -= (-offset) as usize;
        }
    }

    pub fn get_bytecode_byte(&self) -> u8 {
        self.bytecode_ref[self.pc]
    }

    pub fn get_bytecode_byte_1(&self) -> u8 {
        self.bytecode_ref[self.pc + 1]
    }

    pub fn get_bytecode_byte_2(&self) -> u8 {
        self.bytecode_ref[self.pc + 2]
    }

    pub fn push(&mut self, val: i32) {
        self.operand_stack.push(val);
    }

    pub fn push_i64(&mut self, value: i64) {
        let low = value as i32;
        let high = (value >> 32) as i32;

        self.push(low);
        self.push(high);
    }

    pub(crate) fn push_f32(&mut self, value: f32) {
        self.push(value.to_bits() as i32);
    }

    pub(crate) fn push_f64(&mut self, value: f64) {
        self.push_i64(value.to_bits() as i64);
    }

    pub fn pop_i64(&mut self) -> i64 {
        let high = self.pop();
        let low = self.pop();

        i32toi64(high, low)
    }

    pub fn pop_f64(&mut self) -> f64 {
        let high = self.pop_i64();

        f64::from_bits(high as u64)
    }

    pub fn pop_f32(&mut self) -> f32 {
        let high = self.pop();

        f32::from_bits(high as u32)
    }

    pub fn pop(&mut self) -> i32 {
        self.operand_stack.pop().expect("Empty stack")
    }

    pub fn set_local(&mut self, index: usize, val: i32) {
        self.locals[index] = val;
    }

    pub fn get_local(&mut self, index: usize) -> i32 {
        *self.locals.get(index).expect("No value at index")
    }

    pub fn get_two_bytes_from_local(&mut self, index: usize) -> (i32, i32, i64) {
        let low = self.get_local(index);
        let high = self.get_local(index + 1);

        let value = i32toi64(high, low);

        (low, high, value)
    }

    pub fn current_class_name(&self) -> &str {
        &self.current_class_name
    }
}
