use crate::helper::i32toi64;
use crate::stack::stack_frame::StackFrame;

#[derive(Clone)]
pub enum StackValueKind {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

impl From<i32> for StackValueKind {
    fn from(value: i32) -> Self {
        StackValueKind::I32(value)
    }
}

impl From<i64> for StackValueKind {
    fn from(value: i64) -> Self {
        StackValueKind::I64(value)
    }
}

impl From<f32> for StackValueKind {
    fn from(value: f32) -> Self {
        StackValueKind::F32(value)
    }
}

impl From<f64> for StackValueKind {
    fn from(value: f64) -> Self {
        StackValueKind::F64(value)
    }
}

pub trait StackValue {
    fn push_onto(&self, stack_frame: &mut StackFrame);
    fn pop_from(stack_frame: &mut StackFrame) -> Self;

    fn set(&self, index: usize, stack_frame: &mut StackFrame);
    fn get(index: usize, stack_frame: &mut StackFrame) -> Self;
}

impl StackValue for i32 {
    fn push_onto(&self, stack_frame: &mut StackFrame) {
        stack_frame.push_raw(*self);
    }

    fn pop_from(stack_frame: &mut StackFrame) -> Self {
        stack_frame.pop_raw()
    }

    fn set(&self, index: usize, stack_frame: &mut StackFrame) {
        stack_frame.set_local_raw(index, *self);
    }

    fn get(index: usize, stack_frame: &mut StackFrame) -> Self {
        stack_frame.get_local_raw(index)
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

    fn set(&self, index: usize, stack_frame: &mut StackFrame) {
        let low = *self as i32;
        let high = (*self >> 32) as i32;

        stack_frame.set_local_raw(index, low);
        stack_frame.set_local_raw(index + 1, high);
    }

    fn get(index: usize, stack_frame: &mut StackFrame) -> Self {
        let low = stack_frame.get_local_raw(index);
        let high = stack_frame.get_local_raw(index + 1);

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

    fn set(&self, index: usize, stack_frame: &mut StackFrame) {
        stack_frame.set_local(index, self.to_bits() as i32);
    }

    fn get(index: usize, stack_frame: &mut StackFrame) -> Self {
        let value: i32 = stack_frame.get_local(index);
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

    fn set(&self, index: usize, stack_frame: &mut StackFrame) {
        stack_frame.set_local(index, self.to_bits() as i64);
    }

    fn get(index: usize, stack_frame: &mut StackFrame) -> Self {
        let value: i64 = stack_frame.get_local(index);
        f64::from_bits(value as u64)
    }
}
