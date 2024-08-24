pub(crate) struct StackFrame<'a> {
    pc: usize,
    pub locals: Vec<i32>,
    pub(crate) operand_stack: Vec<i32>,
    bytecode_ref: &'a [u8],
}

impl<'a> StackFrame<'a> {
    pub fn new(locals_size: usize, stack_size: usize, bytecode_ref: &'a [u8]) -> Self {
        StackFrame {
            pc: 0,
            locals: vec![0i32; locals_size],
            operand_stack: Vec::with_capacity(stack_size),
            bytecode_ref,
        }
    }

    pub fn incr_pc(&mut self) {
        self.advance_pc(1)
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

    pub fn pop(&mut self) -> i32 {
        self.operand_stack.pop().unwrap()
    }

    pub fn set_local(&mut self, index: usize, val: i32) {
        self.locals[index] = val;
    }

    pub fn get_local(&mut self, index: usize) -> i32 {
        *self.locals.get(index).unwrap()
    }
}
