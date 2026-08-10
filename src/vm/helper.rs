use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::heap::heap::HEAP;
use crate::vm::method_area::java_class::JavaClass;
use crate::vm::method_area::loaded_classes::CLASSES;
use crate::vm::method_area::primitives_helper::PRIMITIVE_TYPE_BY_CODE;
use crate::vm::stack::slot::Slot;
use crate::vm::stack::stack_value::StackValue;
use jdescriptor::{MethodDescriptor, TypeDescriptor};
use std::sync::Arc;

pub fn i32toi64(high: i32, low: i32) -> i64 {
    let high_converted = (high as i64) << 32;
    let low_converted = low as u32/*to prevent sign extension*/ as i64;

    high_converted | low_converted
}

pub fn i64_to_vec(value: i64) -> Vec<i32> {
    let low = value as i32;
    let high = (value >> 32) as i32;

    vec![high, low]
}

pub fn vec_to_i64(value: &[i32]) -> i64 {
    match value.len() {
        1 => value[0] as i64,
        2 => i32toi64(value[0], value[1]),
        _ => panic!("Invalid value length: {}", value.len()),
    }
}

pub fn clazz_ref(class_name: &str) -> Result<i32> {
    CLASSES.get(class_name)?.mirror_clazz_ref()
}

pub fn klass(clazz_ref: i32) -> Result<Arc<JavaClass>> {
    HEAP.get_mirror_klass_id(clazz_ref)
        .map(|klass_id| CLASSES.get_by_id(klass_id))?
}

pub fn get_length(type_descriptor: &TypeDescriptor) -> Result<i32> {
    match type_descriptor {
        TypeDescriptor::Byte
        | TypeDescriptor::Char
        | TypeDescriptor::Integer
        | TypeDescriptor::Short
        | TypeDescriptor::Boolean
        | TypeDescriptor::Float
        | TypeDescriptor::Array(_, _)
        | TypeDescriptor::Object(_) => Ok(1),
        TypeDescriptor::Long | TypeDescriptor::Double => Ok(2),
        TypeDescriptor::Void => Err(Error::new_execution("Void type doesn't have a length")),
    }
}

/// Whether a declared type is a reference, and so occupies a slot a garbage collector must treat as
/// a root. Object and array types are references; every primitive is not.
///
/// A reference always occupies exactly one slot ([`get_length`] returns 1 for both variants), which
/// is why callers can tag a single chunk rather than a range.
pub fn is_reference(type_descriptor: &TypeDescriptor) -> bool {
    matches!(
        type_descriptor,
        TypeDescriptor::Array(_, _) | TypeDescriptor::Object(_)
    )
}

/// Which chunks of a method's arguments are references, in the order the interpreter collects them
/// off the operand stack (receiver first when `has_this`, then each parameter, `long` and `double`
/// contributing two chunks each).
///
/// Lets a caller that has only raw `i32` chunks and a descriptor rebuild the tags the operand stack
/// was carrying before the arguments were popped.
pub fn ref_arg_chunks(method_descriptor: &MethodDescriptor, has_this: bool) -> Result<Vec<bool>> {
    let mut chunks = Vec::new();
    if has_this {
        chunks.push(true); // the receiver is always a reference
    }
    for parameter in method_descriptor.parameter_types() {
        let is_ref = is_reference(parameter);
        for _ in 0..get_length(parameter)? {
            chunks.push(is_ref);
        }
    }

    Ok(chunks)
}

/// The reference mask for a call whose parameter types cannot be read off a descriptor — a
/// `@PolymorphicSignature` intrinsic, whose declared descriptor is the placeholder
/// `(Object[])Object`.
///
/// The parameters are genuinely unknown and stay untagged, but the receiver is not in doubt: an
/// instance method's local 0 is the object the call was made on — the `MethodHandle` or `VarHandle`
/// itself — and that is a reference whatever the signature claims. Tagging it costs nothing and
/// keeps it a root for the call, so only what is actually unknown is given up.
pub fn receiver_only_chunks(has_this: bool) -> Vec<bool> {
    if has_this {
        vec![true]
    } else {
        Vec::new()
    }
}

/// Best-effort classification of a polymorphic call's arguments, from the descriptor written at the
/// call site.
///
/// A `@PolymorphicSignature` method's own descriptor is a placeholder, but the constant pool entry
/// that invoked it carries the real types (JVMS §5.4.3.3), so `call_site_descriptor` classifies the
/// arguments exactly when it is the descriptor those arguments were popped against.
///
/// `fallback_chunks` is what the method itself can say — for a polymorphic target, its receiver and
/// nothing more.
///
/// `None` means the caller had no call-site descriptor to offer — VM-side MethodHandle dispatch
/// passes a resolved method's own signature, which for a polymorphic target is the placeholder.
/// That case falls back to [`receiver_only_chunks`]: the receiver is known either way, and tagging
/// from a descriptor that describes a different call would put `Slot::Ref` on the wrong values.
///
/// The fallback also catches a descriptor that parses but does not fit the arguments, though the
/// provenance check is what makes this sound — a placeholder whose chunk count happens to match
/// would otherwise be accepted, and its `Object` parameters would tag primitives as references.
pub fn polymorphic_arg_slots(
    call_site_descriptor: Option<&str>,
    has_this: bool,
    fallback_chunks: &[bool],
    method_args: &[i32],
) -> Vec<Slot> {
    let from_call_site = call_site_descriptor
        .and_then(|descriptor| descriptor.parse::<MethodDescriptor>().ok())
        .and_then(|descriptor| ref_arg_chunks(&descriptor, has_this).ok())
        .and_then(|chunks| arg_slots(&chunks, method_args).ok());

    from_call_site.unwrap_or_else(|| {
        method_args
            .iter()
            .enumerate()
            .map(|(index, &value)| {
                if fallback_chunks.get(index) == Some(&true) {
                    Slot::Ref(value)
                } else {
                    Slot::Value(value)
                }
            })
            .collect()
    })
}

/// Rebuilds the slots for a call's raw argument chunks, tagging the receiver and every reference
/// parameter.
///
/// Arguments reach a callee as bare `i32`s — popped off the caller's operand stack, or handed in by
/// VM-side code — with nothing left to say which of them are references. `ref_chunks` restores
/// that, and the resulting slots are what make those objects roots for the duration of the call.
/// Callers get the mask from [`JavaMethod::arg_ref_chunks`](crate::vm::method_area::java_method::JavaMethod::arg_ref_chunks),
/// which caches it per method.
pub fn arg_slots(ref_chunks: &[bool], method_args: &[i32]) -> Result<Vec<Slot>> {
    if ref_chunks.len() != method_args.len() {
        return Err(Error::new_execution(&format!(
            "argument mismatch: descriptor describes {} chunks but {} were supplied",
            ref_chunks.len(),
            method_args.len()
        )));
    }

    Ok(method_args
        .iter()
        .zip(ref_chunks)
        .map(|(&value, &is_ref)| {
            if is_ref {
                Slot::Ref(value)
            } else {
                Slot::Value(value)
            }
        })
        .collect())
}

pub fn default_value(type_descriptor: &TypeDescriptor) -> Result<Vec<i32>> {
    match type_descriptor {
        TypeDescriptor::Byte
        | TypeDescriptor::Char
        | TypeDescriptor::Integer
        | TypeDescriptor::Short
        | TypeDescriptor::Boolean => Ok(vec![0]),
        TypeDescriptor::Float => Ok(0.0f32.to_vec()),
        TypeDescriptor::Long => Ok(vec![0, 0]),
        TypeDescriptor::Double => Ok(0.0f64.to_vec()),
        TypeDescriptor::Array(_, _) => Ok(vec![0]),
        TypeDescriptor::Object(_) => Ok(vec![0]),
        TypeDescriptor::Void => Err(Error::new_execution("Void type doesn't have a value")),
    }
}

pub fn argument_length(args: &[TypeDescriptor]) -> Result<i32> {
    args.iter().map(get_length).sum()
}

pub fn strip_nest_host(class_name: &str) -> Option<&str> {
    class_name.find('$').map(|index| &class_name[..index])
}

pub fn create_array_of_strings(props: &[String]) -> Result<i32> {
    let class_of_array = "java/lang/String";
    let class_of_array = format!("[L{class_of_array};");
    let length = props.len() as i32;
    let array_ref = HEAP.create_array(&class_of_array, length);

    for (index, prop) in props.iter().enumerate() {
        let string_ref = StringPoolHelper::get_string(prop)?;
        HEAP.set_array_value(array_ref, index as i32, vec![string_ref])?
    }

    Ok(array_ref)
}

#[cfg(unix)]
pub fn get_handle(fd_ref: i32) -> Result<i32> {
    let raw = HEAP.get_object_field_value(fd_ref, "java/io/FileDescriptor", "fd")?;
    Ok(raw[0])
}
#[cfg(windows)]
pub fn get_handle(fd_ref: i32) -> Result<i64> {
    let raw = HEAP.get_object_field_value(fd_ref, "java/io/FileDescriptor", "handle")?;
    Ok(vec_to_i64(&raw))
}

pub fn decorate(type_name: String) -> String {
    if PRIMITIVE_TYPE_BY_CODE.contains_key(type_name.as_str()) // primitive type B, C, D, F, I, J, S, Z, V
        || (type_name.starts_with('[') && PRIMITIVE_TYPE_BY_CODE.contains_key(&type_name[1..])) // array of primitive types [B, [C, [D, [F, [I, [J, [S, [Z, [V
        || ((type_name.starts_with('L') || type_name.starts_with('[')) && type_name.ends_with(';'))
    // already decorated type Ljava/lang/String; or [Ljava/lang/String;
    {
        type_name
    } else {
        format!("L{};", type_name)
    }
}

pub fn undecorate(type_name: &str) -> &str {
    if type_name.starts_with('L') && type_name.ends_with(';') {
        &type_name[1..type_name.len() - 1]
    } else {
        type_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// What `JavaMethod::arg_ref_chunks` reports for an instance polymorphic method.
    const RECEIVER_ONLY: &[bool] = &[true];

    fn descriptor(text: &str) -> MethodDescriptor {
        text.parse().expect("descriptor")
    }

    #[test]
    fn should_recognise_reference_types() {
        assert!(is_reference(&TypeDescriptor::Object(
            "java/lang/String".to_string()
        )));
        assert!(is_reference(&TypeDescriptor::Array(
            Box::new(TypeDescriptor::Integer),
            1
        )));
        assert!(!is_reference(&TypeDescriptor::Integer));
        assert!(!is_reference(&TypeDescriptor::Long));
        assert!(!is_reference(&TypeDescriptor::Void));
    }

    #[test]
    fn should_mark_no_chunks_for_a_static_method_without_parameters() {
        assert_eq!(
            ref_arg_chunks(&descriptor("()V"), false).unwrap(),
            Vec::<bool>::new()
        );
    }

    #[test]
    fn should_mark_the_receiver_of_an_instance_method() {
        assert_eq!(
            ref_arg_chunks(&descriptor("()V"), true).unwrap(),
            vec![true]
        );
    }

    /// `long` and `double` each occupy two chunks, so a reference after one of them must not have
    /// its position shifted.
    #[test]
    fn should_account_for_wide_parameters_when_locating_references() {
        assert_eq!(
            ref_arg_chunks(&descriptor("(IJLjava/lang/String;D[I)V"), false).unwrap(),
            vec![
                false, // int
                false, false, // long
                true,  // String
                false, false, // double
                true,  // int[]
            ]
        );
    }

    #[test]
    fn should_place_the_receiver_before_the_parameters() {
        assert_eq!(
            ref_arg_chunks(&descriptor("(JLjava/lang/Object;)V"), true).unwrap(),
            vec![true, false, false, true]
        );
    }

    /// The return type never contributes a chunk; only parameters do.
    #[test]
    fn should_ignore_the_return_type() {
        assert_eq!(
            ref_arg_chunks(&descriptor("(I)Ljava/lang/String;"), false).unwrap(),
            vec![false]
        );
    }

    #[test]
    fn should_tag_the_receiver_and_reference_arguments() {
        assert_eq!(
            arg_slots(
                &ref_arg_chunks(&descriptor("(JLjava/lang/Object;I)V"), true).unwrap(),
                &[7, 0, 5, 9, 3]
            )
            .unwrap(),
            vec![
                Slot::Ref(7),   // receiver
                Slot::Value(0), // long, high
                Slot::Value(5), // long, low
                Slot::Ref(9),   // Object
                Slot::Value(3), // int
            ]
        );
    }

    /// When the call-site descriptor is the one the arguments were popped against, it classifies
    /// them fully — the placeholder on the method never could.
    #[test]
    fn should_classify_polymorphic_arguments_from_the_call_site() {
        assert_eq!(
            polymorphic_arg_slots(
                Some("(Ljava/lang/Object;I)Ljava/lang/Object;"),
                true,
                RECEIVER_ONLY,
                &[4, 9, 2]
            ),
            vec![Slot::Ref(4), Slot::Ref(9), Slot::Value(2)]
        );
    }

    /// VM-side MethodHandle dispatch has no call-site descriptor to offer, so only the receiver is
    /// certain.
    #[test]
    fn should_fall_back_to_the_receiver_without_a_call_site_descriptor() {
        assert_eq!(
            polymorphic_arg_slots(None, true, RECEIVER_ONLY, &[4, 9, 2]),
            vec![Slot::Ref(4), Slot::Value(9), Slot::Value(2)]
        );
        assert_eq!(
            polymorphic_arg_slots(None, false, &[], &[4, 9]),
            vec![Slot::Value(4), Slot::Value(9)]
        );
    }

    /// The reason provenance is carried rather than inferred: `invokeExact`'s placeholder
    /// `([Ljava/lang/Object;)Ljava/lang/Object;` parses cleanly and, for a receiver plus one
    /// argument, has exactly the right chunk count — so nothing about it looks wrong. Handed in as
    /// a call-site descriptor it would tag an `int` argument as a reference.
    #[test]
    fn should_show_why_a_placeholder_cannot_be_detected_by_inspection() {
        let placeholder = "([Ljava/lang/Object;)Ljava/lang/Object;";

        assert_eq!(
            polymorphic_arg_slots(Some(placeholder), true, RECEIVER_ONLY, &[4, 7]),
            vec![Slot::Ref(4), Slot::Ref(7)] // the 7 is really an int
        );
        assert_eq!(
            polymorphic_arg_slots(None, true, RECEIVER_ONLY, &[4, 7]),
            vec![Slot::Ref(4), Slot::Value(7)]
        );
    }

    /// A call-site descriptor that does not fit the arguments still falls back rather than tagging
    /// the wrong values.
    #[test]
    fn should_fall_back_when_the_descriptor_does_not_fit() {
        assert_eq!(
            polymorphic_arg_slots(Some("(I)V"), true, RECEIVER_ONLY, &[4, 9, 2]),
            vec![Slot::Ref(4), Slot::Value(9), Slot::Value(2)]
        );
        assert_eq!(
            polymorphic_arg_slots(Some("not a descriptor"), true, RECEIVER_ONLY, &[4, 9]),
            vec![Slot::Ref(4), Slot::Value(9)]
        );
    }

    /// A polymorphic signature hides the parameter types but not the receiver: `invokeExact` is an
    /// instance method, and local 0 is the `MethodHandle` it was called on.
    #[test]
    fn should_keep_the_receiver_of_an_unknown_signature() {
        assert_eq!(receiver_only_chunks(true), vec![true]);
    }

    #[test]
    fn should_mark_nothing_for_a_static_unknown_signature() {
        assert_eq!(receiver_only_chunks(false), Vec::<bool>::new());
    }

    /// A descriptor that disagrees with the supplied chunks means the tags would land on the wrong
    /// values, which is worse than not tagging at all.
    #[test]
    fn should_reject_arguments_that_do_not_match_the_descriptor() {
        let chunks = ref_arg_chunks(&descriptor("(Ljava/lang/Object;)V"), false).unwrap();
        assert!(arg_slots(&chunks, &[1, 2]).is_err());
    }

    /// The shape an `invokedynamic` bootstrap call takes: an argument array of constants passed to
    /// a static method. The array reference has to be a root for the whole call, or everything it
    /// holds becomes unreachable while the bootstrap method is still running.
    #[test]
    fn should_mark_the_bootstrap_argument_array_as_a_reference() {
        let bootstrap = "(Ljava/lang/Class;Ljava/lang/invoke/MethodHandle;Ljava/lang/String;\
                         Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Class;)Ljava/lang/Object;";

        assert_eq!(
            ref_arg_chunks(&descriptor(bootstrap), false).unwrap(),
            vec![true; 6]
        );
    }
}
