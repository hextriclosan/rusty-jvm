//! The unit of storage in a frame's locals array and operand stack.
//!
//! A JVM frame slot is 32 bits wide and the bytecode alone decides how those bits are read: the
//! same word is an `int`, the bit pattern of a `float`, half of a `long`, or a reference. The
//! interpreter recovers that from the opcode (`iload` vs `aload`), but nothing in the stored value
//! records it — and a garbage collector scanning a stopped thread has only the stored value to go
//! on. Reference `5` and the integer `5` are indistinguishable, so a collector would have to either
//! guess (retaining objects a program can no longer reach) or miss a live reference and free an
//! object still in use.
//!
//! [`Slot`] therefore records which of the two a slot holds. The interpreter sets it where the
//! opcode says so, and a root scan reads exactly the slots that are references and no others.

use std::fmt::{Debug, Display, Formatter};

/// One 32-bit frame slot, tagged with whether its bits are a heap reference.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Slot {
    /// Non-reference bits: an `int`, a `float`'s bit pattern, or half of a `long`/`double`.
    Value(i32),
    /// A heap reference (`0` for `null`), and so a root for a collector.
    // Constructed by the reference-typed opcodes converted in a later step.
    #[allow(dead_code)]
    Ref(i32),
}

impl Slot {
    /// The raw 32 bits, whatever they mean.
    pub(crate) fn value(&self) -> i32 {
        match self {
            Slot::Value(value) | Slot::Ref(value) => *value,
        }
    }

    /// Whether this slot holds a heap reference.
    pub(crate) fn is_ref(&self) -> bool {
        matches!(self, Slot::Ref(_))
    }
}

/// Prints references as `#5` and everything else as the bare number, so the interpreter's trace
/// logs stay readable while making the tag visible.
impl Debug for Slot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Slot::Value(value) => write!(f, "{value}"),
            Slot::Ref(reference) => write!(f, "#{reference}"),
        }
    }
}

impl Display for Slot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_tag_reference_slots_only() {
        assert!(!Slot::Value(5).is_ref());
        assert!(Slot::Ref(5).is_ref());
    }

    #[test]
    fn should_keep_value_regardless_of_tag() {
        assert_eq!(Slot::Value(-7).value(), -7);
        assert_eq!(Slot::Ref(-7).value(), -7);
    }

    #[test]
    fn should_distinguish_reference_from_equal_primitive() {
        assert_ne!(Slot::Value(5), Slot::Ref(5));
    }

    #[test]
    fn should_format_slots_marking_references() {
        assert_eq!(format!("{:?}", Slot::Value(5)), "5");
        assert_eq!(format!("{:?}", Slot::Ref(5)), "#5");
        assert_eq!(format!("{}", Slot::Value(5)), "5");
        assert_eq!(format!("{}", Slot::Ref(5)), "#5");
    }
}
