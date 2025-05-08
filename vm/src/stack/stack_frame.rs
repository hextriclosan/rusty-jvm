use crate::error::Error;
use crate::method_area::instance_checker::InstanceChecker;
use crate::stack::stack::Stack;
use crate::stack::stack_value::StackValue;
use derive_new::new;
use std::collections::BTreeMap;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::Arc;
use tracing::trace;

static COUNTER: AtomicU32 = AtomicU32::new(0);

#[derive(Clone, Debug)]
pub(crate) struct StackFrame {
    #[allow(dead_code)]
    index: u32, // this field is only for debugging, it isn't involved in any program logic
    method_name: Arc<String>, // this field is only for debugging, it isn't involved in any program logic
    pc: usize,
    /// Stores the current program counter (pc) in `ex_pc` before invoking a method. This value is later used as the current `pc` if an exception is thrown.
    ex_pc: Option<usize>,
    locals: Box<[i32]>,
    operand_stack: Stack<i32>,
    bytecode_ref: Arc<Vec<u8>>,
    current_class_name: Arc<String>,
    line_numbers: Arc<BTreeMap<u16, u16>>,
    exception_table: Arc<ExceptionTable>,
}

#[derive(Debug, new, PartialEq)]
pub struct ExceptionTableRecord {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    /// The type of exception to catch. Use "any" to represent a catch-all handler.
    catch_type: String,
}

#[derive(Debug, new)]
pub struct ExceptionTable {
    table: Vec<ExceptionTableRecord>,
}

impl ExceptionTable {
    pub fn find_exception_handler(
        &self,
        exception_name: &str,
        pc: u16,
        method_name: &str,
    ) -> Option<u16> {
        if self.table.is_empty() {
            trace!("ATHROW -> exception table is empty: at pc={pc} for exception {exception_name} in method {method_name}");
        }
        const FINALLY_MARK: &'static str = "any";
        for record in self.table.iter() {
            trace!("ATHROW -> checking exception table record: {record:?} at pc={pc} for exception {exception_name} in method {method_name}");
            if pc < record.start_pc || pc >= record.end_pc {
                continue;
            }
            let eligible = record.catch_type == FINALLY_MARK
                || InstanceChecker::checkcast(exception_name, &record.catch_type)
                    .expect("Error in checkcast");
            if eligible {
                trace!("ATHROW -> found exception handler: {record:?} at pc={pc} for exception {exception_name} in method {method_name}");
                return Some(record.handler_pc);
            }
        }

        None
    }
}

#[derive(Debug, new)]
pub struct StackFrames {
    frames: Vec<StackFrame>,
}

impl StackFrames {
    pub fn iter(&self) -> std::slice::Iter<'_, StackFrame> {
        self.frames.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }

    pub fn last(&self) -> Option<&StackFrame> {
        self.frames.last()
    }

    pub fn last_mut(&mut self) -> Option<&mut StackFrame> {
        self.frames.last_mut()
    }

    /// Pushes a new stack frame onto the stack frames.
    pub fn new_frame(&mut self, frame: StackFrame) {
        self.frames.push(frame);
    }

    /// Pops the top stack frame from the stack frames and returns it.
    /// Resets the exception program counter (ex_pc) of the next frame if it exists since it is normal stack unwinding.
    pub fn exit_frame(&mut self) -> Option<StackFrame> {
        let popped_frame = self.pop();

        // Resetting the exception program counter (ex_pc) of the next frame is an intentional
        // part of the normal stack unwinding process.
        if let Some(next_frame) = self.frames.last_mut() {
            next_frame.reset_ex_pc()
        }

        popped_frame
    }

    /// Pops the top stack frame from the stack frames and returns it.
    /// Doesn't reset the exception program counter (ex_pc) of the next frame since it is not normal stack unwinding, and it will be used for correct exception handling.
    pub fn propagate_exception(&mut self) -> Option<StackFrame> {
        self.pop()
    }

    /// Pops the top stack frame from the stack frames and returns it.
    fn pop(&mut self) -> Option<StackFrame> {
        self.frames.pop()
    }
}

impl StackFrame {
    pub fn new(
        method_name: Arc<String>,
        locals_size: usize,
        stack_size: usize,
        bytecode_ref: Arc<Vec<u8>>,
        current_class_name: Arc<String>,
        line_numbers: Arc<BTreeMap<u16, u16>>,
        exception_table: Arc<ExceptionTable>,
    ) -> Self {
        StackFrame {
            index: COUNTER.fetch_add(1, SeqCst),
            method_name,
            pc: 0,
            ex_pc: None,
            locals: vec![0i32; locals_size].into_boxed_slice(),
            operand_stack: Stack::with_capacity(stack_size),
            bytecode_ref,
            current_class_name,
            line_numbers,
            exception_table,
        }
    }

    pub fn pc(&self) -> usize {
        self.pc
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

    pub fn set_pc(&mut self, pc: i16) {
        self.pc = pc as usize;
    }

    /// Stores the current program counter (PC) in the exception PC (ex_pc) field.
    pub fn store_ex_pc(&mut self) {
        self.ex_pc = Some(self.pc);
    }

    /// Resets the exception program counter (ex_pc) to None.
    pub fn reset_ex_pc(&mut self) {
        self.ex_pc = None;
    }

    pub fn clear_stack(&mut self) {
        self.operand_stack.clear()
    }

    pub fn push<T: StackValue>(&mut self, stack_value: T) -> crate::error::Result<()> {
        stack_value
            .push_onto(self)
            .map_err(|e| Error::new_execution(&format!("Reason: {e}; Current Frame: {self:?}")))
    }

    pub fn pop<T: StackValue>(&mut self) -> T {
        T::pop_from(self)
    }

    pub(crate) fn push_raw(&mut self, val: i32) -> crate::error::Result<()> {
        self.operand_stack
            .push(val)
            .map_err(|e| Error::new_execution(&e))
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

    pub fn line_numbers(&self) -> &BTreeMap<u16, u16> {
        &self.line_numbers
    }

    pub fn exception_table(&self) -> &Arc<ExceptionTable> {
        &self.exception_table
    }

    pub fn method_name(&self) -> &str {
        &self.method_name
    }

    /// Returns the exception program counter (ex_pc) if it is set, otherwise returns the current program counter (pc).
    pub fn ex_pc(&self) -> usize {
        self.ex_pc.unwrap_or(self.pc)
    }
}
