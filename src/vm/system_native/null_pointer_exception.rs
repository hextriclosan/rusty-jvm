//! Helpful `NullPointerException` messages — JEP 358.
//!
//! When the VM raises an implicit `NullPointerException` it is constructed **without** a detail
//! message (see [`throw_null_pointer_exception`](crate::vm::exception::helpers::throw_null_pointer_exception)).
//! The JDK's `NullPointerException.getMessage()` then calls
//! [`get_extended_npe_message`] (the native backing `getExtendedNPEMessage()`), which reconstructs a
//! description of *what* was null from the faulting bytecode.
//!
//! The throwable's backtrace records, for its top frame, the method (as a raw [`JavaMethod`]
//! pointer) and the bytecode index of the faulting instruction (see
//! [`build_backtrace`](crate::vm::system_native::throwable::build_backtrace)). From those we decode
//! the failing opcode to produce the *failed action* (`Cannot invoke ...`, `Cannot read field ...`, ...)
//! and, via a linear operand-stack simulation, the *null cause* (`... because "s" is null`).
//!
//! The cause is described only when the null reference was produced
//! by a direct `aload`/`aload_<n>` (the common case: a null local or parameter). More elaborate
//! reconstruction (field chains, array elements, method-return values, branch-aware simulation) is
//! left for later; when the cause cannot be described the action-only message is returned, matching
//! OpenJDK's behavior.

use crate::vm::error::Result;
use crate::vm::execution_engine::opcode::*;
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::vec_to_i64;
use crate::vm::method_area::cpool_helper::CPoolHelperTrait;
use crate::vm::method_area::java_method::JavaMethod;
use crate::vm::method_area::loaded_classes::CLASSES;
use crate::vm::system_native::throwable::NATIVE_METHOD;
use std::collections::BTreeSet;

const ACC_STATIC: i32 = 0x0008;

/// `java.lang.NullPointerException.getExtendedNPEMessage()Ljava/lang/String;`
///
/// Returns the reference of the computed message string, or `0` (Java `null`) when no helpful
/// message can be produced (missing/native top frame, out-of-range bci, unrecognized opcode, ...), in
/// which case the JDK falls back to a plain `NullPointerException` with no message.
pub(crate) fn get_extended_npe_message(this: i32) -> Result<i32> {
    match build_message_from_throwable(this) {
        Some(message) => StringPoolHelper::get_string(&message),
        None => Ok(0),
    }
}

/// Reads the throwable's backtrace, resolves its top frame's method + bci, and builds the message.
fn build_message_from_throwable(this: i32) -> Option<String> {
    let instance_name = HEAP.get_instance_name(this).ok()?;
    let backtrace_ref = *HEAP
        .get_object_field_value(this, &instance_name, "backtrace")
        .ok()?
        .first()?;
    if backtrace_ref == 0 {
        return None;
    }

    let method_array_ref = *HEAP.get_array_value(backtrace_ref, 1).ok()?.first()?;
    let tag_array_ref = *HEAP.get_array_value(backtrace_ref, 3).ok()?.first()?;
    let bci_array_ref = *HEAP.get_array_value(backtrace_ref, 4).ok()?.first()?;

    if HEAP.get_array_len(method_array_ref).ok()? < 1 {
        return None;
    }

    // Frame 0 is the frame that raised the NPE (the throwable's own constructor frames are excluded
    // when the backtrace is collected).
    if *HEAP.get_array_value(tag_array_ref, 0).ok()?.first()? == NATIVE_METHOD {
        return None;
    }
    let method_ptr = vec_to_i64(&HEAP.get_array_value(method_array_ref, 0).ok()?);
    let bci = *HEAP.get_array_value(bci_array_ref, 0).ok()?.first()? as usize;

    // SAFETY: the pointer was recorded in the backtrace as `Arc::as_ptr(&JavaMethod)` while the
    // method's class was loaded; loaded classes live for the lifetime of the VM, so it stays valid.
    let method = unsafe { &*(method_ptr as *const JavaMethod) };

    build_message(method, bci)
}

/// Builds the helpful message for the given faulting method/bci, or `None` when it cannot.
fn build_message(method: &JavaMethod, bci: usize) -> Option<String> {
    let bytecode = method.code_context()?.bytecode();
    let opcode = *bytecode.get(bci)?;

    let klass = CLASSES.get(method.class_name()).ok()?;
    let cpool = klass.cpool_helper();

    let Fault { action, null_depth } = decode_fault(opcode, bytecode, bci, cpool)?;

    // Simulate only the basic block containing the fault, starting from its leader with an empty
    // operand stack. The result is trusted only when the null reference was produced *within* this
    // block (the simulated height exceeds `null_depth`); otherwise the value entered from a
    // predecessor block and its origin is left undetermined. This never misidentifies the slot - at
    // worst it omits the cause - while still handling control-flow convergence before the fault.
    let cause = block_leader(bytecode, bci)
        .and_then(|leader| simulate_block(bytecode, leader, bci, cpool))
        .filter(|stack| stack.len() > null_depth)
        .and_then(|stack| stack.get(stack.len() - 1 - null_depth).copied())
        .and_then(|slot| match slot {
            Slot::Aload { slot, load_bci } => Some(describe_local(method, slot, load_bci)),
            Slot::Other => None,
        });

    Some(match cause {
        Some(name) => format!("{action} because \"{name}\" is null"),
        None => action,
    })
}

/// The result of decoding the faulting instruction.
struct Fault {
    /// The "failed action" clause, e.g. `Cannot invoke "String.length()"`.
    action: String,
    /// Depth (in operand-stack slots, counting from the top) of the null reference the instruction
    /// dereferenced. `0` is the top of the stack.
    null_depth: usize,
}

fn decode_fault<T: CPoolHelperTrait>(
    opcode: u8,
    bytecode: &[u8],
    bci: usize,
    cpool: &T,
) -> Option<Fault> {
    let action = match opcode {
        // Field access.
        GETFIELD => {
            let (_, field_name, _) = cpool.get_full_field_info(operand_u16(bytecode, bci)?)?;
            return Some(Fault {
                action: format!("Cannot read field \"{field_name}\""),
                null_depth: 0,
            });
        }
        PUTFIELD => {
            let (_, field_name, descriptor) =
                cpool.get_full_field_info(operand_u16(bytecode, bci)?)?;
            return Some(Fault {
                action: format!("Cannot assign field \"{field_name}\""),
                // Stack is [objectref, value...]; the value occupies 1 or 2 slots above the objectref.
                null_depth: slots_of_field_type(descriptor.as_bytes()),
            });
        }
        // Method invocation - the receiver sits below all the arguments.
        INVOKEVIRTUAL | INVOKESPECIAL | INVOKEINTERFACE => {
            let (class_name, method_name, descriptor) =
                cpool.get_full_method_info(operand_u16(bytecode, bci)?)?;
            // A constructor is only ever invoked on a freshly allocated object (via `new`/`dup`) or
            // on `this`/`super`, never on a null reference - so an `invokespecial <init>` can never
            // be an implicit NPE. This is the site of an *explicit* `new NullPointerException()`
            // (e.g. `Objects.requireNonNull`); like OpenJDK, produce no extended message so the
            // JDK's `getMessage()` returns null.
            if method_name == "<init>" {
                return None;
            }
            let (params, arg_slots) = parse_method_params(&descriptor)?;
            return Some(Fault {
                action: format!(
                    "Cannot invoke \"{}.{method_name}({})\"",
                    render_class_name(&class_name),
                    params.join(", ")
                ),
                null_depth: arg_slots,
            });
        }
        // Arrays.
        ARRAYLENGTH => "Cannot read the array length".to_string(),
        IALOAD | LALOAD | FALOAD | DALOAD | AALOAD | BALOAD | CALOAD | SALOAD => {
            // Stack is [arrayref, index]; the array is one slot below the index.
            return Some(Fault {
                action: format!("Cannot load from {} array", array_element_type(opcode)),
                null_depth: 1,
            });
        }
        IASTORE | LASTORE | FASTORE | DASTORE | AASTORE | BASTORE | CASTORE | SASTORE => {
            // Stack is [arrayref, index, value…]; value is 1 or 2 slots, index is 1.
            let value_slots = if matches!(opcode, LASTORE | DASTORE) {
                2
            } else {
                1
            };
            return Some(Fault {
                action: format!("Cannot store to {} array", array_element_type(opcode)),
                null_depth: 1 + value_slots,
            });
        }
        // Miscellaneous.
        ATHROW => "Cannot throw exception".to_string(),
        MONITORENTER => "Cannot enter synchronized block".to_string(),
        MONITOREXIT => "Cannot exit synchronized block".to_string(),
        _ => return None,
    };
    Some(Fault {
        action,
        null_depth: 0,
    })
}

/// The abstract origin of an operand-stack slot, tracked during simulation.
#[derive(Clone, Copy)]
enum Slot {
    /// Produced by an `aload`/`aload_<n>` of local variable `slot` at `load_bci`.
    Aload { slot: u16, load_bci: usize },
    /// Produced by anything else (its expression is not reconstructed at Tier 1).
    Other,
}

/// Linearly simulates the operand stack from `start` to `faulting_bci`, tracking which slots were
/// produced by `aload`. `start` is a basic-block leader, so the walk sees only straight-line code;
/// it returns `None` (the analysis bails) on any control-flow instruction or any opcode whose stack
/// effect is not modeled - never producing a wrong result, only a less detailed message.
fn simulate_block<T: CPoolHelperTrait>(
    bytecode: &[u8],
    start: usize,
    faulting_bci: usize,
    cpool: &T,
) -> Option<Vec<Slot>> {
    let mut stack: Vec<Slot> = Vec::new();
    let mut pc = start;

    while pc < faulting_bci {
        let op = *bytecode.get(pc)?;
        let mut push_other = 0usize;
        let mut len = 1usize;

        match op {
            NOP => {}
            // Category-1 constants / pushes.
            ACONST_NULL | ICONST_M1..=ICONST_5 | FCONST_0..=FCONST_2 => push_other = 1,
            BIPUSH | LDC => {
                len = 2;
                push_other = 1;
            }
            SIPUSH | LDC_W => {
                len = 3;
                push_other = 1;
            }
            // Category-2 constants.
            LCONST_0 | LCONST_1 | DCONST_0 | DCONST_1 => push_other = 2,
            LDC2_W => {
                len = 3;
                push_other = 2;
            }
            // Loads: `aload` variants carry their source; other loads are opaque.
            ILOAD | FLOAD => {
                len = 2;
                push_other = 1;
            }
            LLOAD | DLOAD => {
                len = 2;
                push_other = 2;
            }
            ALOAD => {
                let slot = *bytecode.get(pc + 1)? as u16;
                stack.push(Slot::Aload { slot, load_bci: pc });
                len = 2;
            }
            ILOAD_0..=ILOAD_3 | FLOAD_0..=FLOAD_3 => push_other = 1,
            LLOAD_0..=LLOAD_3 | DLOAD_0..=DLOAD_3 => push_other = 2,
            ALOAD_0..=ALOAD_3 => stack.push(Slot::Aload {
                slot: (op - ALOAD_0) as u16,
                load_bci: pc,
            }),
            // Array loads: pop arrayref + index, push the element (category depends on opcode).
            IALOAD | FALOAD | AALOAD | BALOAD | CALOAD | SALOAD => {
                pop_slots(&mut stack, 2)?;
                push_other = 1;
            }
            LALOAD | DALOAD => {
                pop_slots(&mut stack, 2)?;
                push_other = 2;
            }
            // Stores just consume the top of the stack; the local they write is irrelevant here
            // because a later `aload` re-derives its source from the load's own operand.
            ISTORE | FSTORE | ASTORE => {
                pop_slots(&mut stack, 1)?;
                len = 2;
            }
            LSTORE | DSTORE => {
                pop_slots(&mut stack, 2)?;
                len = 2;
            }
            ISTORE_0..=ISTORE_3 | FSTORE_0..=FSTORE_3 | ASTORE_0..=ASTORE_3 => {
                pop_slots(&mut stack, 1)?
            }
            LSTORE_0..=LSTORE_3 | DSTORE_0..=DSTORE_3 => pop_slots(&mut stack, 2)?,
            POP => pop_slots(&mut stack, 1)?,
            POP2 => pop_slots(&mut stack, 2)?,
            IINC => len = 3,
            DUP => {
                let top = *stack.last()?;
                stack.push(top);
            }
            DUP2 => {
                let n = stack.len();
                let (a, b) = (*stack.get(n.checked_sub(2)?)?, *stack.get(n - 1)?);
                stack.push(a);
                stack.push(b);
            }
            SWAP => {
                let n = stack.len();
                stack.swap(n.checked_sub(2)?, n - 1);
            }
            // `checkcast` leaves the reference (and thus its tracked source) in place.
            CHECKCAST => len = 3,
            INSTANCEOF => {
                pop_slots(&mut stack, 1)?;
                push_other = 1;
                len = 3;
            }
            NEW => {
                push_other = 1;
                len = 3;
            }
            NEWARRAY => {
                pop_slots(&mut stack, 1)?;
                push_other = 1;
                len = 2;
            }
            ANEWARRAY => {
                pop_slots(&mut stack, 1)?;
                push_other = 1;
                len = 3;
            }
            ARRAYLENGTH => {
                pop_slots(&mut stack, 1)?;
                push_other = 1;
            }
            GETSTATIC => {
                let (_, _, descriptor) = cpool.get_full_field_info(operand_u16(bytecode, pc)?)?;
                push_other = slots_of_field_type(descriptor.as_bytes());
                len = 3;
            }
            GETFIELD => {
                let (_, _, descriptor) = cpool.get_full_field_info(operand_u16(bytecode, pc)?)?;
                pop_slots(&mut stack, 1)?;
                push_other = slots_of_field_type(descriptor.as_bytes());
                len = 3;
            }
            INVOKEVIRTUAL | INVOKESPECIAL | INVOKESTATIC | INVOKEINTERFACE => {
                let (_, _, descriptor) = cpool.get_full_method_info(operand_u16(bytecode, pc)?)?;
                let (_, arg_slots) = parse_method_params(&descriptor)?;
                let receiver = if op == INVOKESTATIC { 0 } else { 1 };
                pop_slots(&mut stack, arg_slots + receiver)?;
                push_other = return_type_slots(&descriptor)?;
                len = if op == INVOKEINTERFACE { 5 } else { 3 };
            }
            // Any other opcode (stores, arithmetic, branches, switches, returns, ...): bail.
            _ => return None,
        }

        for _ in 0..push_other {
            stack.push(Slot::Other);
        }
        pc += len;
    }

    // Bail if we overshot into the middle of an instruction rather than landing on the fault.
    if pc == faulting_bci {
        Some(stack)
    } else {
        None
    }
}

fn pop_slots(stack: &mut Vec<Slot>, n: usize) -> Option<()> {
    if stack.len() < n {
        return None;
    }
    stack.truncate(stack.len() - n);
    Some(())
}

/// Returns the start pc of the basic block containing `faulting_bci` - the greatest leader `<=`
/// `faulting_bci`. Leaders are bci 0, every branch/switch target, and the instruction following any
/// block terminator (branch, `switch`, `return`, `athrow`, `jsr`/`ret`). Walking a block from its
/// leader therefore never crosses a branch before reaching the fault. Returns `None` if the code
/// cannot be fully parsed.
fn block_leader(code: &[u8], faulting_bci: usize) -> Option<usize> {
    let mut leaders = BTreeSet::new();
    leaders.insert(0usize);

    let mut pc = 0usize;
    while pc < code.len() {
        let op = *code.get(pc)?;
        let len = instruction_length(code, pc)?;
        let next = pc + len;

        match op {
            // Conditional branches, `goto`, `jsr`, `ifnull`/`ifnonnull`: 16-bit signed offset.
            IFEQ..=JSR | IFNULL | IFNONNULL => {
                leaders.insert((pc as isize + read_i16(code, pc + 1)? as isize) as usize);
                leaders.insert(next);
            }
            // Wide branches: 32-bit signed offset.
            GOTO_W | JSR_W => {
                leaders.insert((pc as isize + read_i32(code, pc + 1)? as isize) as usize);
                leaders.insert(next);
            }
            TABLESWITCH => {
                let p = pc + 1 + pad(pc + 1);
                leaders.insert((pc as isize + read_i32(code, p)? as isize) as usize); // default
                let low = read_i32(code, p + 4)?;
                let high = read_i32(code, p + 8)?;
                for i in 0..=(high - low) {
                    let off = read_i32(code, p + 12 + (i as usize) * 4)?;
                    leaders.insert((pc as isize + off as isize) as usize);
                }
                leaders.insert(next);
            }
            LOOKUPSWITCH => {
                let p = pc + 1 + pad(pc + 1);
                leaders.insert((pc as isize + read_i32(code, p)? as isize) as usize); // default
                let npairs = read_i32(code, p + 4)?.max(0) as usize;
                for i in 0..npairs {
                    let off = read_i32(code, p + 8 + i * 8 + 4)?;
                    leaders.insert((pc as isize + off as isize) as usize);
                }
                leaders.insert(next);
            }
            // Block terminators that fall through to a new block.
            IRETURN..=RETURN | ATHROW | RET => {
                leaders.insert(next);
            }
            _ => {}
        }

        pc = next;
    }

    leaders.range(..=faulting_bci).next_back().copied()
}

/// Number of padding bytes inserting before `switch` operands so they align to a 4-byte boundary.
fn pad(after_opcode: usize) -> usize {
    (4 - (after_opcode % 4)) % 4
}

/// Byte length of the instruction at `pc`, including operands (`tableswitch`/`lookupswitch`/`wide`
/// are variable). Returns `None` for a truncated or unknown instruction.
fn instruction_length(code: &[u8], pc: usize) -> Option<usize> {
    let op = *code.get(pc)?;
    let len = match op {
        WIDE => {
            if *code.get(pc + 1)? == IINC {
                6
            } else {
                4
            }
        }
        TABLESWITCH => {
            let p = pc + 1 + pad(pc + 1);
            let low = read_i32(code, p + 4)?;
            let high = read_i32(code, p + 8)?;
            let entries = (high - low).max(-1) as usize + 1;
            (p + 12 + entries * 4) - pc
        }
        LOOKUPSWITCH => {
            let p = pc + 1 + pad(pc + 1);
            let npairs = read_i32(code, p + 4)?.max(0) as usize;
            (p + 8 + npairs * 8) - pc
        }
        // Two-byte operand.
        SIPUSH
        | LDC_W
        | LDC2_W
        | IINC
        | GETSTATIC
        | PUTSTATIC
        | GETFIELD
        | PUTFIELD
        | INVOKEVIRTUAL
        | INVOKESPECIAL
        | INVOKESTATIC
        | NEW
        | ANEWARRAY
        | CHECKCAST
        | INSTANCEOF
        | IFEQ..=JSR
        | IFNULL
        | IFNONNULL => 3,
        // Four operand bytes.
        INVOKEINTERFACE | INVOKEDYNAMIC | GOTO_W | JSR_W => 5,
        MULTIANEWARRAY => 4,
        // One operand byte.
        BIPUSH | LDC | ILOAD | LLOAD | FLOAD | DLOAD | ALOAD | ISTORE | LSTORE | FSTORE
        | DSTORE | ASTORE | RET | NEWARRAY => 2,
        // Everything else is a single byte.
        _ => 1,
    };
    if pc + len <= code.len() {
        Some(len)
    } else {
        None
    }
}

fn read_i16(code: &[u8], at: usize) -> Option<i16> {
    Some(((*code.get(at)? as i16) << 8) | *code.get(at + 1)? as i16)
}

fn read_i32(code: &[u8], at: usize) -> Option<i32> {
    Some(
        ((*code.get(at)? as i32) << 24)
            | ((*code.get(at + 1)? as i32) << 16)
            | ((*code.get(at + 2)? as i32) << 8)
            | (*code.get(at + 3)? as i32),
    )
}

/// Renders the source name of local variable `slot` at `load_bci`, matching OpenJDK: the
/// `LocalVariableTable` name when present (`javac -g`), otherwise `this`, `<parameterN>`, or
/// `<localN>`.
fn describe_local(method: &JavaMethod, slot: u16, load_bci: usize) -> String {
    if let Some(context) = method.code_context() {
        for record in context.local_variable_table().iter() {
            let start = record.start_pc() as usize;
            let end = start + record.length() as usize;
            if record.slot() == slot && (start..end).contains(&load_bci) {
                return record.name().clone();
            }
        }
    }

    let is_static = method.access_flags() & ACC_STATIC != 0;
    if !is_static && slot == 0 {
        return "this".to_string();
    }

    // Walk the parameter slots to find whether `slot` is a parameter, and its 1-based ordinal.
    if let Some((_, descriptor)) = method.name_signature().split_once(':') {
        let mut cur = if is_static { 0u16 } else { 1 };
        for (ordinal, param_slots) in param_slots_iter(descriptor).into_iter().enumerate() {
            if cur == slot {
                return format!("<parameter{}>", ordinal + 1);
            }
            cur += param_slots as u16;
        }
    }

    format!("<local{slot}>")
}

/// Renders a class's internal name (`java/util/List`) the way OpenJDK does inside these messages:
/// `java/lang/String` and `java/lang/Object` collapse to their simple names, array classes keep
/// their descriptor form (`[J`), everything else becomes dotted.
fn render_class_name(internal: &str) -> String {
    match internal {
        "java/lang/String" => "String".to_string(),
        "java/lang/Object" => "Object".to_string(),
        _ if internal.starts_with('[') => internal.to_string(),
        _ => internal.replace('/', "."),
    }
}

fn array_element_type(opcode: u8) -> &'static str {
    match opcode {
        IALOAD | IASTORE => "int",
        LALOAD | LASTORE => "long",
        FALOAD | FASTORE => "float",
        DALOAD | DASTORE => "double",
        AALOAD | AASTORE => "object",
        BALOAD | BASTORE => "byte",
        CALOAD | CASTORE => "char",
        SALOAD | SASTORE => "short",
        _ => "object",
    }
}

fn operand_u16(bytecode: &[u8], bci: usize) -> Option<u16> {
    let hi = *bytecode.get(bci + 1)? as u16;
    let lo = *bytecode.get(bci + 2)? as u16;
    Some((hi << 8) | lo)
}

/// Number of local-variable slots a field type occupies: `long`/`double` take two, everything else
/// one. `descriptor` is a field descriptor such as `I`, `J`, `Ljava/lang/String;`, `[I`.
fn slots_of_field_type(descriptor: &[u8]) -> usize {
    match descriptor.first() {
        Some(b'J') | Some(b'D') => 2,
        _ => 1,
    }
}

/// Parses a method descriptor's parameter list, returning the parameters' external type names and
/// their total slot count.
fn parse_method_params(descriptor: &str) -> Option<(Vec<String>, usize)> {
    let bytes = descriptor.as_bytes();
    if bytes.first() != Some(&b'(') {
        return None;
    }
    let mut i = 1;
    let mut names = Vec::new();
    let mut slots = 0usize;
    while i < bytes.len() && bytes[i] != b')' {
        let (name, cat2, next) = parse_field_type(bytes, i)?;
        slots += if cat2 { 2 } else { 1 };
        names.push(name);
        i = next;
    }
    Some((names, slots))
}

/// Yields the slot count (1 or 2) of each parameter, in order.
fn param_slots_iter(descriptor: &str) -> Vec<usize> {
    let bytes = descriptor.as_bytes();
    let mut out = Vec::new();
    let mut i = 1; // skip '('
    while i < bytes.len() && bytes[i] != b')' {
        match parse_field_type(bytes, i) {
            Some((_, cat2, next)) => {
                out.push(if cat2 { 2 } else { 1 });
                i = next;
            }
            None => break,
        }
    }
    out
}

fn return_type_slots(descriptor: &str) -> Option<usize> {
    let bytes = descriptor.as_bytes();
    let close = bytes.iter().position(|&b| b == b')')?;
    match bytes.get(close + 1)? {
        b'V' => Some(0),
        b'J' | b'D' => Some(2),
        _ => Some(1),
    }
}

/// Parses one field type starting at `i`, returning its external name, whether it is a category-2
/// type (`long`/`double`), and the index just past it.
fn parse_field_type(bytes: &[u8], i: usize) -> Option<(String, bool, usize)> {
    let mut dims = 0usize;
    let mut j = i;
    while bytes.get(j) == Some(&b'[') {
        dims += 1;
        j += 1;
    }
    let (base, cat2, next) = match *bytes.get(j)? {
        b'B' => ("byte".to_string(), false, j + 1),
        b'C' => ("char".to_string(), false, j + 1),
        b'D' => ("double".to_string(), true, j + 1),
        b'F' => ("float".to_string(), false, j + 1),
        b'I' => ("int".to_string(), false, j + 1),
        b'J' => ("long".to_string(), true, j + 1),
        b'S' => ("short".to_string(), false, j + 1),
        b'Z' => ("boolean".to_string(), false, j + 1),
        b'L' => {
            let end = bytes[j..].iter().position(|&b| b == b';')? + j;
            let internal = std::str::from_utf8(&bytes[j + 1..end]).ok()?;
            (render_class_name(internal), false, end + 1)
        }
        _ => return None,
    };
    // Array types are references (category 1) regardless of element type.
    let (name, cat2) = if dims > 0 {
        (format!("{base}{}", "[]".repeat(dims)), false)
    } else {
        (base, cat2)
    };
    Some((name, cat2, next))
}
