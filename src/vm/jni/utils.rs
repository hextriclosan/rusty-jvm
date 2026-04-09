use crate::vm::execution_engine::static_init::StaticInit;
use crate::vm::helper::{clazz_ref, klass};
use crate::vm::method_area::java_method::JavaMethod;
use crate::vm::method_area::loaded_classes::CLASSES;
use crate::vm::method_area::method_area::with_method_area;
use crate::vm::stack::stack_value::StackValueKind;
use jclassfile::methods::MethodFlags;
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
    expect_static: bool,
) -> jmethodID {
    let name_str = from_mutf8_ptr!(name).expect("Failed to convert method name from CESU-8");
    let sig_str = from_mutf8_ptr!(sig).expect("Failed to convert method signature from CESU-8");
    let klass = klass(clazz as i32).expect("Failed to get class from reference");
    let method_kind = if expect_static { "static" } else { "instance" };
    StaticInit::initialize_java_class(&klass).unwrap_or_else(|_| {
        panic!("Failed to initialize class before getting {method_kind} method ID")
    }); // todo: throw ExceptionInInitializerError here
    let full_signature = format!("{}:{}", name_str, sig_str);

    // First try to find the method directly in the specified class's own methods map.
    // If not found (e.g. abstract class that inherits an interface method without re-declaring it),
    // fall back to the class hierarchy (parent classes and interfaces).
    let found = klass
        .get_method_full(&full_signature)
        .filter(|(_, method)| {
            MethodFlags::from_bits_truncate(method.access_flags() as u16)
                .contains(MethodFlags::ACC_STATIC)
                == expect_static
        })
        .map(|(method_index, _method)| {
            // Method lives in the class we were given – encode that class's ref.
            let encoded: i64 = ((clazz as i32 as i64) << 32) | (method_index as i64);
            encoded as jmethodID
        })
        .or_else(|| {
            // Look up the class hierarchy to find which ancestor/interface owns the method.
            let class_name = klass.this_class_name();
            let owner = with_method_area(|method_area| {
                method_area
                    .lookup_for_implementation(class_name, &full_signature)
                    .or_else(|| {
                        method_area
                            .lookup_for_implementation_interface(class_name, &full_signature)
                    })
            })?;
            // Verify the found method has the expected static/instance kind.
            let owner_flags = MethodFlags::from_bits_truncate(owner.access_flags() as u16);
            if owner_flags.contains(MethodFlags::ACC_STATIC) != expect_static {
                return None;
            }
            // Get the owner class's mirror ref and method index.
            let owner_class_name = owner.class_name();
            let owner_clazz_ref = clazz_ref(owner_class_name).ok()?;
            let owner_klass = CLASSES.get(owner_class_name).ok()?;
            let (method_index, _) = owner_klass.get_method_full(&full_signature)?;
            let encoded: i64 = ((owner_clazz_ref as i64) << 32) | (method_index as i64);
            Some(encoded as jmethodID)
        })
        .unwrap_or(null_mut()); // todo: throw NoSuchMethodError here

    found
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
        TypeDescriptor::Boolean => (if unsafe { value.z } != 0 { 1 } else { 0 }).into(),
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
