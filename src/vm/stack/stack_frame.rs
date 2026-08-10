use crate::vm::error::{Error, Result};
use crate::vm::method_area::instance_checker::InstanceChecker;
use crate::vm::stack::slot::Slot;
use crate::vm::stack::stack::Stack;
use crate::vm::stack::stack_value::StackValue;
use derive_new::new;
use jclassmodel::exception_table::ExceptionTableRecord;
use jclassmodel::CatchType;
use std::collections::BTreeMap;
use std::fmt::Display;
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
    /// Start pc of the instruction currently being dispatched, stamped by the interpreter loop
    /// before the opcode's operands are decoded. Unlike `pc` (which the handler advances while
    /// reading operands) this stays put at the opcode boundary, so it is the reliable bytecode index
    /// of the faulting instruction for helpful `NullPointerException` messages (JEP 358).
    bci: usize,
    /// Stores the current program counter (pc) in `ex_pc` before invoking a method. This value is later used as the current `pc` if an exception is thrown.
    ex_pc: Option<usize>,
    locals: Box<[Slot]>,
    operand_stack: Stack<Slot>,
    bytecode_ref: Arc<Vec<u8>>,
    current_class_name: Arc<String>,
    line_numbers: Arc<BTreeMap<u16, u16>>,
    exception_table: Arc<ExceptionTable>,
    /// `true` for the synthetic frame of a currently executing native method (see
    /// [`StackFrame::new_native`]). Such frames carry no bytecode and are skipped by stack walks
    /// that only care about interpreted frames (e.g. caller-class resolution).
    is_native: bool,
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
        current_class_name: &str,
    ) -> Result<Option<u16>> {
        if self.table.is_empty() {
            trace!("ATHROW -> exception table is empty: at pc={pc} for exception {exception_name} in method {current_class_name}.{method_name}");
        }
        for record in self.table.iter() {
            trace!("ATHROW -> checking exception table record: {record:?} at pc={pc} for exception {exception_name} in method {current_class_name}.{method_name}");
            if pc < record.start_pc() || pc >= record.end_pc() {
                continue;
            }
            let eligible = match record.catch_type() {
                CatchType::Any => true,
                CatchType::Class(catch_type) => {
                    InstanceChecker::checkcast(exception_name, catch_type)?
                }
            };
            if eligible {
                trace!("ATHROW -> found exception handler: {record:?} at pc={pc} for exception {exception_name} in method {current_class_name}.{method_name}");
                return Ok(Some(record.handler_pc()));
            }
        }

        Ok(None)
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
            bci: 0,
            ex_pc: None,
            locals: vec![Slot::Value(0); locals_size].into_boxed_slice(),
            operand_stack: Stack::with_capacity(stack_size),
            bytecode_ref,
            current_class_name,
            line_numbers,
            exception_table,
            is_native: false,
        }
    }

    /// A bare frame with no bytecode, for tests that exercise slot handling rather than execution.
    #[cfg(test)]
    pub(crate) fn for_test(locals_size: usize, stack_size: usize) -> Self {
        Self::new(
            Arc::new("test:()V".to_string()),
            locals_size,
            stack_size,
            Arc::new(Vec::new()),
            Arc::new("Test".to_string()),
            Arc::new(BTreeMap::new()),
            Arc::new(ExceptionTable::new(Vec::new())),
        )
    }

    /// Builds a synthetic frame representing a native method that is currently executing.
    ///
    /// Native methods have no bytecode, line numbers or exception handlers, so those are left
    /// empty. The frame is never run by the interpreter loop; it exists so that a native method
    /// appears on the thread's stack chain (e.g. as `(Native Method)` in stack traces) while it
    /// runs, including when it calls back into Java.
    ///
    /// It does carry the call's arguments as locals. They were popped off the caller's operand
    /// stack before dispatch, so this frame is the only place a reference argument — the receiver
    /// included — can be seen as a root while the native runs. That window is not brief: resolving
    /// a native symbol re-enters Java, and any native may call back.
    pub fn new_native(
        method_name: Arc<String>,
        current_class_name: Arc<String>,
        args: Vec<Slot>,
    ) -> Self {
        StackFrame {
            index: COUNTER.fetch_add(1, SeqCst),
            method_name,
            pc: 0,
            bci: 0,
            ex_pc: None,
            locals: args.into_boxed_slice(),
            operand_stack: Stack::with_capacity(0),
            bytecode_ref: Arc::new(Vec::new()),
            current_class_name,
            line_numbers: Arc::new(BTreeMap::new()),
            exception_table: Arc::new(ExceptionTable::new(Vec::new())),
            is_native: true,
        }
    }

    pub fn is_native(&self) -> bool {
        self.is_native
    }

    pub fn pc(&self) -> usize {
        self.pc
    }

    /// Start pc of the instruction currently being dispatched. See the `bci` field.
    pub fn bci(&self) -> usize {
        self.bci
    }

    /// Records the start pc of the instruction about to be dispatched. Called by the interpreter
    /// loop before the opcode's operands are read.
    pub fn set_bci(&mut self, bci: usize) {
        self.bci = bci;
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

    pub fn get_four_bytes_ahead(&self) -> i32 {
        let byte1 = self.bytecode_ref[self.pc + 1] as u32;
        let byte2 = self.bytecode_ref[self.pc + 2] as u32;
        let byte3 = self.bytecode_ref[self.pc + 3] as u32;
        let byte4 = self.bytecode_ref[self.pc + 4] as u32;

        ((byte1 << 24) | (byte2 << 16) | (byte3 << 8) | byte4) as i32
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

    pub fn advance_pc_wide(&mut self, offset: i32) {
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

    pub fn push<T: StackValue>(&mut self, stack_value: T) -> Result<()> {
        stack_value
            .push_onto(self)
            .map_err(|e| Error::new_execution(&format!("Reason: {e}; Current Frame: {self}")))
    }

    pub fn pop<T: StackValue>(&mut self) -> T {
        T::pop_from(self)
    }

    /// Pushes non-reference bits, leaving the slot untagged so a root scan skips it.
    pub(crate) fn push_raw(&mut self, val: i32) -> Result<()> {
        self.push_slot(Slot::Value(val))
    }

    /// Pops a slot's raw bits, discarding its tag. Popping never needs the tag: the opcode already
    /// knows what it asked for, and the value leaves the frame either way.
    pub(crate) fn pop_raw(&mut self) -> i32 {
        self.pop_slot().value()
    }

    /// Pushes a slot **with its tag intact**, the slot-level counterpart of [`Self::push_raw`].
    ///
    /// Like `push_raw` this is the low-level primitive reached through a [`StackValue`] impl, and
    /// it reports a stack overflow bare. Call [`Self::push`] instead: it routes here via
    /// `Slot`'s impl and adds the failing frame to the error. Opcodes that relocate values whose
    /// type they do not know (`dup`, `swap`, ...) push a `Slot` for exactly that reason — going
    /// through `push_raw` would strip the reference tag off every value they touch.
    pub(crate) fn push_slot(&mut self, slot: Slot) -> Result<()> {
        self.operand_stack
            .push(slot)
            .map_err(|e| Error::new_execution(&e))
    }

    /// Pops a slot with its tag intact; the counterpart of [`Self::push_slot`], and likewise
    /// reached through [`Self::pop`] rather than called directly.
    pub(crate) fn pop_slot(&mut self) -> Slot {
        self.operand_stack.pop().expect("Empty stack")
    }

    pub fn set_local<T: StackValue>(&mut self, index: usize, stack_value: T) {
        stack_value.set(index, self);
    }

    pub fn get_local<T: StackValue>(&mut self, index: usize) -> T {
        T::get(index, self)
    }

    /// Stores non-reference bits in a local. See [`Self::push_raw`].
    pub fn set_local_raw(&mut self, index: usize, val: i32) {
        self.locals[index] = Slot::Value(val);
    }

    pub fn get_local_raw(&self, index: usize) -> i32 {
        self.get_local_slot(index).value()
    }

    /// Stores a local with its tag intact; the locals counterpart of [`Self::push_slot`], reached
    /// through [`Self::set_local`].
    pub(crate) fn set_local_slot(&mut self, index: usize, slot: Slot) {
        self.locals[index] = slot;
    }

    /// Reads a local with its tag intact; the counterpart of [`Self::set_local_slot`], reached
    /// through [`Self::get_local`].
    pub(crate) fn get_local_slot(&self, index: usize) -> Slot {
        *self.locals.get(index).expect("No value at index")
    }

    /// Every heap reference this frame holds, in locals and on the operand stack — the frame's
    /// contribution to a garbage collector's root set.
    ///
    /// Only slots the interpreter tagged are reported, so an `int` that happens to equal a live
    /// reference is not mistaken for one.
    #[allow(dead_code)] // consumed by the root scan added in a later step
    pub(crate) fn ref_slots(&self) -> impl Iterator<Item = i32> + '_ {
        self.locals
            .iter()
            .chain(self.operand_stack.iter())
            .filter(|slot| slot.is_ref())
            .map(|slot| slot.value())
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

impl Display for StackFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"StackFrame {{ method_name: "{}", pc: {}, ex_pc: {:?}, locals: {:?}, operand_stack: {:?}, bytecode_ref: {:?}, current_class_name: "{}", line_numbers: {:?}, exception_table: {:?} }}"#,
            self.method_name,
            self.pc,
            self.ex_pc,
            self.locals,
            self.operand_stack,
            self.bytecode_ref,
            self.current_class_name,
            self.line_numbers,
            self.exception_table,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(locals_size: usize, stack_size: usize) -> StackFrame {
        StackFrame::for_test(locals_size, stack_size)
    }

    #[test]
    fn should_report_no_roots_for_a_fresh_frame() {
        assert_eq!(frame(4, 4).ref_slots().count(), 0);
    }

    #[test]
    fn should_report_only_tagged_slots_as_roots() {
        let mut frame = frame(2, 2);
        frame.set_local(0, Slot::Ref(7));
        frame.set_local(1, 7i32);
        frame.push(Slot::Ref(9)).unwrap();
        frame.push(9i32).unwrap();

        // The untagged `7` and `9` are ordinary ints that merely look like references.
        assert_eq!(frame.ref_slots().collect::<Vec<_>>(), vec![7, 9]);
    }

    #[test]
    fn should_treat_null_as_a_reference_slot() {
        let mut frame = frame(1, 1);
        frame.push(Slot::Ref(0)).unwrap();
        assert_eq!(frame.ref_slots().collect::<Vec<_>>(), vec![0]);
    }

    #[test]
    fn should_preserve_the_tag_across_a_slot_move() {
        let mut frame = frame(0, 2);
        frame.push(Slot::Ref(5)).unwrap();

        // What `dup` does: relocate a value of unknown type.
        let slot = frame.pop_slot();
        frame.push_slot(slot).unwrap();
        frame.push_slot(slot).unwrap();

        assert_eq!(frame.ref_slots().collect::<Vec<_>>(), vec![5, 5]);
    }

    #[test]
    fn should_drop_the_tag_when_a_primitive_overwrites_a_local() {
        let mut frame = frame(1, 0);
        frame.set_local(0, Slot::Ref(5));
        frame.set_local(0, 5i32);

        assert_eq!(frame.ref_slots().count(), 0);
    }

    /// `astore` then `aload`: the tag survives a trip through a local.
    #[test]
    fn should_carry_the_tag_between_stack_and_locals() {
        let mut frame = frame(1, 1);
        frame.push(Slot::Ref(-1)).unwrap();

        let stored: Slot = frame.pop();
        frame.set_local(0, stored);
        let loaded: Slot = frame.get_local(0);
        frame.push(loaded).unwrap();

        assert_eq!(loaded, Slot::Ref(-1));
        // Reported twice: the local still holds it, and so does the stack.
        assert_eq!(frame.ref_slots().collect::<Vec<_>>(), vec![-1, -1]);
    }

    /// A native method's arguments live nowhere else while it runs: the caller popped them before
    /// dispatch. The synthetic frame is what keeps them reachable.
    #[test]
    fn should_report_a_native_frames_arguments_as_roots() {
        let frame = StackFrame::new_native(
            Arc::new("read:(Ljava/lang/Object;I)I".to_string()),
            Arc::new("Test".to_string()),
            vec![Slot::Ref(3), Slot::Ref(8), Slot::Value(64)],
        );

        assert!(frame.is_native());
        assert_eq!(frame.ref_slots().collect::<Vec<_>>(), vec![3, 8]);
    }

    /// Raw bits from the heap carry no tag, and a `Slot` must never invent one. `aaload` and
    /// friends build `Slot::Ref` themselves instead.
    #[test]
    #[should_panic(expected = "cannot be rebuilt from untagged bits")]
    fn should_refuse_to_rebuild_a_slot_from_untagged_bits() {
        let _ = Slot::from_vec(&[5]);
    }

    /// Overflowing on a `Slot` must report the frame like every other push does. `dup`/`swap` move
    /// slots, so pushing one through `push` rather than `push_slot` is what keeps their overflows
    /// diagnosable.
    #[test]
    fn should_report_the_frame_when_a_slot_overflows_the_stack() {
        let mut frame = frame(0, 1);
        frame.push(Slot::Ref(1)).unwrap();

        let error = frame.push(Slot::Ref(2)).unwrap_err().to_string();

        assert!(error.contains("Exceeded max stack size"), "{error}");
        assert!(error.contains("Current Frame:"), "{error}");
        // The tagged slot already on the stack shows up in the dump.
        assert!(error.contains("#1"), "{error}");
    }

    /// The same overflow taken through the low-level primitive, which by design carries no frame
    /// context — `push` is what adds it, and adding it here too would double-wrap the message.
    #[test]
    fn should_report_a_bare_error_when_a_slot_overflows_via_the_primitive() {
        let mut frame = frame(0, 1);
        frame.push_slot(Slot::Ref(1)).unwrap();

        let error = frame.push_slot(Slot::Ref(2)).unwrap_err().to_string();

        assert!(error.contains("Exceeded max stack size"), "{error}");
        assert!(!error.contains("Current Frame:"), "{error}");
    }

    #[test]
    fn should_not_tag_the_halves_of_a_wide_value() {
        let mut frame = frame(2, 2);
        frame.push(i64::MAX).unwrap();
        frame.set_local(0, f64::MAX);

        assert_eq!(frame.ref_slots().count(), 0);
    }
}
