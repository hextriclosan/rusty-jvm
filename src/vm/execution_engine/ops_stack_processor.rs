//! The stack-shuffle opcodes: `pop`, `dup` and `swap` in their several widths.
//!
//! These are the only opcodes that move operands without knowing what they are — `dup` duplicates
//! whatever is on top, reference or `int` alike. They therefore work in [`Slot`]s, which carry the
//! reference tag along with the value. Popping and re-pushing as `i32` would strip that tag off
//! every value they touch, so a reference duplicated by `dup` would stop being visible as a
//! garbage-collection root.
//!
//! Category-2 values (`long`, `double`) occupy two slots and are shuffled as two untagged halves,
//! which is why these handlers can treat every slot uniformly.

use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::common::last_frame_mut;
use crate::vm::execution_engine::opcode::*;
use crate::vm::stack::slot::Slot;
use crate::vm::stack::stack_frame::StackFrames;
use tracing::trace;

pub(crate) fn process(code: u8, stack_frames: &mut StackFrames) -> Result<()> {
    let stack_frame = last_frame_mut(stack_frames)?;
    match code {
        POP => {
            let _value: Slot = stack_frame.pop();

            stack_frame.incr_pc();
            trace!("POP");
        }
        POP2 => {
            let _value: Slot = stack_frame.pop();
            let _value: Slot = stack_frame.pop();

            stack_frame.incr_pc();
            trace!("POP2");
        }
        DUP => {
            let value: Slot = stack_frame.pop();
            stack_frame.push(value)?;
            stack_frame.push(value)?;

            stack_frame.incr_pc();
            trace!("DUP -> value={value}");
        }
        DUP_X1 => {
            let value1: Slot = stack_frame.pop();
            let value2: Slot = stack_frame.pop();
            stack_frame.push(value1)?;
            stack_frame.push(value2)?;
            stack_frame.push(value1)?;

            stack_frame.incr_pc();
            trace!("DUP_X1 -> value1={value1}, value2={value2}, value1={value1}");
        }
        DUP_X2 => {
            let value1: Slot = stack_frame.pop();
            let value2: Slot = stack_frame.pop();
            let value3: Slot = stack_frame.pop();
            stack_frame.push(value1)?;
            stack_frame.push(value3)?;
            stack_frame.push(value2)?;
            stack_frame.push(value1)?;

            stack_frame.incr_pc();
            trace!("DUP_X2 -> value1={value1}, value2={value2}, value3={value3}, value1={value1}");
        }
        DUP2 => {
            let value1: Slot = stack_frame.pop();
            let value2: Slot = stack_frame.pop();
            stack_frame.push(value2)?;
            stack_frame.push(value1)?;
            stack_frame.push(value2)?;
            stack_frame.push(value1)?;

            stack_frame.incr_pc();
            trace!("DUP2 -> value1={value1}, value2={value2}");
        }
        DUP2_X1 => {
            let value1: Slot = stack_frame.pop();
            let value2: Slot = stack_frame.pop();
            let value3: Slot = stack_frame.pop();
            stack_frame.push(value2)?;
            stack_frame.push(value1)?;
            stack_frame.push(value3)?;
            stack_frame.push(value2)?;
            stack_frame.push(value1)?;

            stack_frame.incr_pc();
            trace!("DUP2_X1 -> value1={value1}, value2={value2}, value3={value3}");
        }
        DUP2_X2 => {
            let value1: Slot = stack_frame.pop();
            let value2: Slot = stack_frame.pop();
            let value3: Slot = stack_frame.pop();
            let value4: Slot = stack_frame.pop();
            stack_frame.push(value2)?;
            stack_frame.push(value1)?;
            stack_frame.push(value4)?;
            stack_frame.push(value3)?;
            stack_frame.push(value2)?;
            stack_frame.push(value1)?;

            stack_frame.incr_pc();
            trace!(
                "DUP2_X2 -> value1={value1}, value2={value2}, value3={value3}, value4={value4}"
            );
        }
        SWAP => {
            let value1: Slot = stack_frame.pop();
            let value2: Slot = stack_frame.pop();
            stack_frame.push(value1)?;
            stack_frame.push(value2)?;

            stack_frame.incr_pc();
            trace!("SWAP -> value1={value1}, value2={value2}");
        }
        _ => {
            return Err(Error::new_execution(&format!(
                "Unknown stack opcode: {}",
                code
            )));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::stack::stack_frame::StackFrame;

    /// A single frame whose operand stack holds `operands`, bottom-first.
    fn frames_with(operands: &[Slot]) -> StackFrames {
        let mut frame = StackFrame::for_test(0, operands.len() * 3); // headroom for the widest dup
        for operand in operands {
            frame.push(*operand).unwrap();
        }
        StackFrames::new(vec![frame])
    }

    /// Runs one opcode over `operands` and returns the resulting operand stack, top-first.
    fn shuffle(code: u8, operands: &[Slot], depth: usize) -> Vec<Slot> {
        let mut frames = frames_with(operands);
        process(code, &mut frames).unwrap();

        let frame = frames.last_mut().expect("frame");
        (0..depth).map(|_| frame.pop::<Slot>()).collect()
    }

    #[test]
    fn should_duplicate_a_reference_as_a_reference() {
        assert_eq!(
            shuffle(DUP, &[Slot::Ref(5)], 2),
            vec![Slot::Ref(5), Slot::Ref(5)]
        );
    }

    #[test]
    fn should_keep_primitives_untagged_when_duplicating() {
        assert_eq!(
            shuffle(DUP, &[Slot::Value(5)], 2),
            vec![Slot::Value(5), Slot::Value(5)]
        );
    }

    /// The case that a `dup`/`swap` written in terms of `i32` gets wrong without anyone noticing:
    /// the reference and the `int` are the same 32 bits, and only the tag tells them apart.
    #[test]
    fn should_not_confuse_a_reference_with_an_equal_int() {
        assert_eq!(
            shuffle(SWAP, &[Slot::Value(5), Slot::Ref(5)], 2),
            vec![Slot::Value(5), Slot::Ref(5)]
        );
    }

    #[test]
    fn should_carry_tags_through_dup_x1() {
        assert_eq!(
            shuffle(DUP_X1, &[Slot::Value(1), Slot::Ref(5)], 3),
            vec![Slot::Ref(5), Slot::Value(1), Slot::Ref(5)]
        );
    }

    #[test]
    fn should_carry_tags_through_the_widest_shuffle() {
        assert_eq!(
            shuffle(
                DUP2_X2,
                &[Slot::Value(1), Slot::Value(2), Slot::Ref(5), Slot::Ref(6)],
                6
            ),
            vec![
                Slot::Ref(6),
                Slot::Ref(5),
                Slot::Value(2),
                Slot::Value(1),
                Slot::Ref(6),
                Slot::Ref(5),
            ]
        );
    }

    #[test]
    fn should_drop_both_roots_on_pop2() {
        let mut frames = frames_with(&[Slot::Ref(5), Slot::Ref(6)]);
        process(POP2, &mut frames).unwrap();

        assert_eq!(frames.last().expect("frame").ref_slots().count(), 0);
    }
}
