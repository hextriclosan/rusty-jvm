use crate::helper::i32toi64;
use crate::stack::stack_frame::StackFrame;

pub trait StackValue {
    fn push_onto(&self, stack_frame: &mut StackFrame);
    fn pop_from(stack_frame: &mut StackFrame) -> Self;
}

impl StackValue for i32 {
    fn push_onto(&self, stack_frame: &mut StackFrame) {
        stack_frame.push_raw(*self);
    }

    fn pop_from(stack_frame: &mut StackFrame) -> Self {
        stack_frame.pop_raw()
    }
}

impl StackValue for i64 {
    fn push_onto(&self, stack_frame: &mut StackFrame) {
        let low = *self as i32;
        let high = (self >> 32) as i32;

        stack_frame.push_raw(low);
        stack_frame.push_raw(high);
    }

    fn pop_from(stack_frame: &mut StackFrame) -> Self {
        let high = stack_frame.pop_raw();
        let low = stack_frame.pop_raw();

        i32toi64(high, low)
    }
}

impl StackValue for f32 {
    fn push_onto(&self, stack_frame: &mut StackFrame) {
        stack_frame.push(self.to_bits() as i32);
    }

    fn pop_from(stack_frame: &mut StackFrame) -> Self {
        let value: i32 = stack_frame.pop();
        f32::from_bits(value as u32)
    }
}

impl StackValue for f64 {
    fn push_onto(&self, stack_frame: &mut StackFrame) {
        stack_frame.push(self.to_bits() as i64);
    }

    fn pop_from(stack_frame: &mut StackFrame) -> Self {
        let value: i64 = stack_frame.pop();
        f64::from_bits(value as u64)
    }
}
