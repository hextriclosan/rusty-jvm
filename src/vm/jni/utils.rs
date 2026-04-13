use crate::vm::execution_engine::static_init::StaticInit;
use crate::vm::helper::{clazz_ref, klass};
use crate::vm::method_area::java_method::JavaMethod;
use crate::vm::method_area::lookup;
use crate::vm::stack::stack_value::StackValueKind;
use jdescriptor::TypeDescriptor;
use jni_sys::{jclass, jmethodID, jvalue};
use std::ffi::c_char;
use std::ptr::null_mut;
use std::sync::Arc;

#[macro_export]
macro_rules! from_mutf8_ptr {
    ($ptr:expr) => {{
        use std::ffi::CStr;
        // Convert the raw pointer to a byte slice
        let ptr = $ptr;
        assert!(!ptr.is_null(), "from_mutf8_ptr! received a null pointer");
        cesu8::from_java_cesu8(unsafe { CStr::from_ptr(ptr) }.to_bytes())
    }};
}

#[macro_export]
macro_rules! to_mutf8_data {
    ($string_ref:expr) => {{
        let raw_data = get_string_raw_data($string_ref);
        let data = String::from_utf16(&raw_data).expect("Failed to build string from UTF-16 data");
        let mut mutf8_data = to_java_cesu8(&data).to_vec();
        mutf8_data.push(0); // null terminator
        mutf8_data
    }};
}

pub(super) fn get_method_id_impl(
    clazz: jclass,
    name: *const c_char,
    sig: *const c_char,
) -> jmethodID {
    let name_str = from_mutf8_ptr!(name).expect("Failed to convert method name from CESU-8");
    let sig_str = from_mutf8_ptr!(sig).expect("Failed to convert method signature from CESU-8");
    let declaring_klass = klass(clazz as i32).expect("Failed to get class from reference");
    StaticInit::initialize_java_class(&declaring_klass)
        .expect("Failed to initialize class before getting method ID"); // todo: throw ExceptionInInitializerError here
    let full_signature = format!("{}:{}", name_str, sig_str);

    // Look up the method implementation in the class/interface hierarchy.
    let klass_name = declaring_klass.this_class_name().clone();
    lookup::lookup_method(&klass_name, &full_signature)
        .unwrap_or_else(|e| panic!("Failed to find implementation of {full_signature}: {e}"))
        .and_then(|method| {
            let found_class_name = method.class_name();
            let found_clazz_ref = clazz_ref(found_class_name).ok()?;
            let found_klass = klass(found_clazz_ref).ok()?;
            let (idx, _) = found_klass.get_method_full(&full_signature)?;
            Some(encode_method_id(found_clazz_ref, idx) as jmethodID)
        })
        .unwrap_or(null_mut()) // todo: throw NoSuchMethodError here
}

#[cfg(not(target_pointer_width = "64"))]
compile_error!("src/vm/jni/utils.rs requires a 64-bit target because jmethodID values are encoded into usize using a 64-bit layout");
/// Encode a (class_ref, method_index) pair into a single `usize` suitable for use as a
/// `jmethodID`.
///
/// Layout (64-bit):
///   bits 63–32: declaring class heap reference (i32, zero-extended to u32)
///   bits 31–0: per-class method index (truncated to u32)
fn encode_method_id(class_ref: i32, method_index: usize) -> usize {
    let class_bits = (class_ref as u32 as u64) << 32;
    let index_bits = method_index as u32 as u64; // mask to 32 bits to keep encoding contract
    (class_bits | index_bits) as usize
}

/// Decode the declaring class reference and method index from the encoded jmethodID.
/// Layout (see encode_method_id in utils.rs):
///   bits 63–32: declaring class heap reference (u32)
///   bits 31–0: per-class method index (u32)
/// Use u64 arithmetic to avoid signed arithmetic right-shift ambiguity.
pub(super) fn decode_method_id(method_id: usize) -> (i32, i64) {
    let raw = method_id as u64;
    let declaring_class_ref = (raw >> 32) as u32 as i32;
    let method_index = (raw & 0xFFFF_FFFF) as i64;
    (declaring_class_ref, method_index)
}

pub(super) fn transform_args_to_vec(
    method: &Arc<JavaMethod>,
    args: *const jvalue,
) -> Vec<StackValueKind> {
    let md = method.get_method_descriptor();
    let pt = md.parameter_types();
    let args_count = pt.len();

    if args_count == 0 {
        return vec![];
    }

    if args.is_null() {
        panic!("Null pointer passed as arguments for method that expects {args_count} arguments");
    }

    let args = unsafe { std::slice::from_raw_parts(args, args_count) };
    args.iter()
        .zip(pt.iter())
        .map(|(arg, pt)| resolve_stack_kind_value(*arg, pt))
        .collect::<Vec<_>>()
}

fn resolve_stack_kind_value(value: jvalue, type_descriptor: &TypeDescriptor) -> StackValueKind {
    match type_descriptor {
        TypeDescriptor::Boolean => (if unsafe { value.z } { 1 } else { 0 }).into(),
        TypeDescriptor::Byte => (unsafe { value.b } as i32).into(),
        TypeDescriptor::Char => (unsafe { value.c } as i32).into(),
        TypeDescriptor::Short => (unsafe { value.s } as i32).into(),
        TypeDescriptor::Integer => (unsafe { value.i } as i32).into(),
        TypeDescriptor::Long => (unsafe { value.j } as i64).into(),
        TypeDescriptor::Float => (unsafe { value.f } as f32).into(),
        TypeDescriptor::Double => (unsafe { value.d } as f64).into(),
        TypeDescriptor::Object(_) | TypeDescriptor::Array(_, _) => {
            (unsafe { value.l } as i32).into()
        }
        TypeDescriptor::Void => panic!("Void type cannot be used as argument"),
    }
}
