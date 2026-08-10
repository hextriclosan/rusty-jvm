use crate::vm::error::Result;
use crate::vm::helper::{i32toi64, i64_to_vec};
use crate::vm::stack::slot::Slot;
use crate::vm::stack::stack_frame::StackFrame;

#[derive(Clone, Debug)]
pub enum StackValueKind {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

impl StackValueKind {
    pub fn chunks(&self) -> usize {
        match self {
            StackValueKind::I32(_) | StackValueKind::F32(_) => 1,
            StackValueKind::I64(_) | StackValueKind::F64(_) => 2,
        }
    }
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
    fn push_onto(&self, stack_frame: &mut StackFrame) -> Result<()>;
    fn pop_from(stack_frame: &mut StackFrame) -> Self;

    fn set(&self, index: usize, stack_frame: &mut StackFrame);
    fn get(index: usize, stack_frame: &mut StackFrame) -> Self;

    fn from_vec(v: &[i32]) -> Self;
    fn to_vec(&self) -> Vec<i32>;
}

impl StackValue for i32 {
    fn push_onto(&self, stack_frame: &mut StackFrame) -> Result<()> {
        stack_frame.push_raw(*self)
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

    fn from_vec(v: &[i32]) -> Self {
        v[0]
    }

    fn to_vec(&self) -> Vec<i32> {
        vec![*self]
    }
}

/// Moves slots without disturbing their tags. Naming `Slot` as a generic opcode handler's type
/// parameter is what marks that opcode as reference-typed: `handle_load::<i32>` implements `iload`
/// and untags the slot it writes, while `handle_load::<Slot>` implements `aload` and carries the
/// reference tag through.
///
/// Every method here *moves* a tag rather than deciding one: transfers between the operand stack
/// and locals preserve whatever the value already carries, so `aload`/`astore` cannot manufacture a
/// reference out of a slot that was never tagged as one. Deciding the tag belongs to the opcode
/// that produces the reference — `new`, `getfield`, `aaload`, an invoke's return value — each of
/// which builds `Slot::Ref` explicitly, at the point where the type is actually known.
impl StackValue for Slot {
    fn push_onto(&self, stack_frame: &mut StackFrame) -> Result<()> {
        stack_frame.push_slot(*self)
    }

    fn pop_from(stack_frame: &mut StackFrame) -> Self {
        stack_frame.pop_slot()
    }

    fn set(&self, index: usize, stack_frame: &mut StackFrame) {
        stack_frame.set_local_slot(index, *self);
    }

    fn get(index: usize, stack_frame: &mut StackFrame) -> Self {
        stack_frame.get_local_slot(index)
    }

    /// Unsupported by design: a slot is never rebuilt from untagged bits.
    ///
    /// Every other method here moves a slot that already carries its tag. Raw `Vec<i32>`s coming
    /// back from the heap or a native return carry none, and nothing in this signature can say
    /// which variant they should become — so the answer has to come from the opcode, which builds
    /// `Slot::Ref` or `Slot::Value` itself.
    ///
    /// Neither guess is acceptable here: `Value` would silently drop a root and free a live object,
    /// while `Ref` would silently retain garbage and hide the mistake at whatever call site got it
    /// wrong. `aaload` is the one caller that would otherwise land here, and it constructs its slot
    /// directly for exactly this reason.
    fn from_vec(_v: &[i32]) -> Self {
        unreachable!(
            "a Slot cannot be rebuilt from untagged bits: construct Slot::Ref or Slot::Value \
             at the opcode that knows which it is"
        )
    }

    fn to_vec(&self) -> Vec<i32> {
        vec![self.value()]
    }
}

impl StackValue for i64 {
    fn push_onto(&self, stack_frame: &mut StackFrame) -> Result<()> {
        let low = *self as i32;
        let high = (self >> 32) as i32;

        stack_frame.push_raw(low)?;
        stack_frame.push_raw(high)
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

    fn from_vec(v: &[i32]) -> Self {
        let low = v[1];
        let high = v[0];

        i32toi64(high, low)
    }

    fn to_vec(&self) -> Vec<i32> {
        i64_to_vec(*self)
    }
}

impl StackValue for f32 {
    fn push_onto(&self, stack_frame: &mut StackFrame) -> Result<()> {
        stack_frame.push(self.to_bits() as i32)
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

    fn from_vec(v: &[i32]) -> Self {
        let value: i32 = StackValue::from_vec(v);
        f32::from_bits(value as u32)
    }

    fn to_vec(&self) -> Vec<i32> {
        vec![self.to_bits() as i32]
    }
}

impl StackValue for f64 {
    fn push_onto(&self, stack_frame: &mut StackFrame) -> Result<()> {
        stack_frame.push(self.to_bits() as i64)
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

    fn from_vec(v: &[i32]) -> Self {
        let value: i64 = StackValue::from_vec(v);
        f64::from_bits(value as u64)
    }

    fn to_vec(&self) -> Vec<i32> {
        StackValue::to_vec(&(self.to_bits() as i64))
    }
}
