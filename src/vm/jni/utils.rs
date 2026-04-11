use crate::vm::execution_engine::static_init::StaticInit;
use crate::vm::helper::klass;
use crate::vm::method_area::java_method::JavaMethod;
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
    let klass = klass(clazz as i32).expect("Failed to get class from reference");
    StaticInit::initialize_java_class(&klass)
        .expect("Failed to initialize class before getting method ID"); // todo: throw ExceptionInInitializerError here
    let full_signature = format!("{}:{}", name_str, sig_str);

    klass
        .get_method_full(&full_signature)
        .and_then(|(method_id, _method)| {
            // Encode both the declaring class reference and the method index into the jmethodID.
            // High 32 bits: class object reference (clazz as i32)
            // Low 32 bits: method index within that class's methods map
            let encoded = (((clazz as i32 as u32 as u64) << 32) | (method_id as u64)) as usize;
            Some(encoded as jmethodID)
        })
        .unwrap_or(null_mut()) // todo: throw NoSuchMethodError here
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
