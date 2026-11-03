use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::executor::Executor;
use crate::vm::execution_engine::static_init::StaticInit;
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::helper::{clazz_ref, klass};
use crate::vm::jni::handles;
use crate::vm::jni::java_thread::JavaThread;
use crate::vm::method_area::java_method::JavaMethod;
use crate::vm::method_area::lookup;
use crate::vm::stack::stack_value::StackValueKind;
use jdescriptor::TypeDescriptor;
use jni_sys::{jclass, jmethodID, jsize, jvalue};
use std::ffi::c_char;
use std::ptr::null_mut;
use std::sync::Arc;
use tracing::warn;

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

pub(super) fn get_instance_method_id_impl(
    clazz: jclass,
    name: *const c_char,
    sig: *const c_char,
) -> jmethodID {
    get_method_id_impl(clazz, name, sig, lookup::lookup_instance_method)
}

pub(super) fn get_static_method_id_impl(
    clazz: jclass,
    name: *const c_char,
    sig: *const c_char,
) -> jmethodID {
    get_method_id_impl(clazz, name, sig, lookup::lookup_static_method)
}

fn get_method_id_impl(
    clazz: jclass,
    name: *const c_char,
    sig: *const c_char,
    lookup_method: fn(&str, &str) -> Result<Option<Arc<JavaMethod>>>,
) -> jmethodID {
    let name_str = from_mutf8_ptr!(name).expect("Failed to convert method name from CESU-8");
    let sig_str = from_mutf8_ptr!(sig).expect("Failed to convert method signature from CESU-8");
    let declaring_klass =
        klass(handles::resolve_or_return!(clazz)).expect("Failed to get class from reference");
    if let Err(error) = StaticInit::initialize_java_class(&declaring_klass) {
        set_pending_vm_error(&error);
        return null_mut();
    }
    let full_signature = format!("{}:{}", name_str, sig_str);

    // Look up the method implementation in the class/interface hierarchy.
    let klass_name = declaring_klass.this_class_name().clone();
    let method_id = lookup_method(&klass_name, &full_signature).and_then(|method| {
        method
            .map(|method| -> Result<_> {
                let found_class_name = method.class_name();
                let found_clazz_ref = clazz_ref(found_class_name)?;
                let found_klass = klass(found_clazz_ref)?;
                Ok(found_klass
                    .get_method_full(&full_signature)
                    .map(|(idx, _)| encode_method_id(found_clazz_ref, idx) as jmethodID))
            })
            .transpose()
            .map(Option::flatten)
    });

    match method_id {
        Ok(Some(id)) => id,
        Ok(None) => {
            set_pending_no_such_method_error(&full_signature);
            null_mut()
        }
        Err(error) => {
            set_pending_vm_error(&error);
            null_mut()
        }
    }
}

fn set_pending_no_such_method_error(signature: &str) {
    set_pending_error("java/lang/NoSuchMethodError", signature);
}

pub(super) fn set_pending_no_such_field_error(field_name: &str) {
    set_pending_error("java/lang/NoSuchFieldError", field_name);
}

pub(super) fn set_pending_instantiation_exception(class_name: &str) {
    set_pending_error("java/lang/InstantiationException", class_name);
}

pub(super) fn set_pending_illegal_access_exception(class_name: &str) {
    set_pending_error("java/lang/IllegalAccessException", class_name);
}

pub(super) fn set_pending_no_class_def_found_error(message: &str) {
    set_pending_error("java/lang/NoClassDefFoundError", message);
}

pub(super) fn set_pending_class_format_error(message: &str) {
    set_pending_error("java/lang/ClassFormatError", message);
}

pub(super) fn set_pending_linkage_error(message: &str) {
    set_pending_error("java/lang/LinkageError", message);
}

pub(super) fn set_pending_null_pointer_exception(message: &str) {
    set_pending_error("java/lang/NullPointerException", message);
}

pub(super) fn set_pending_negative_array_size_exception(len: jsize) {
    set_pending_error("java/lang/NegativeArraySizeException", &len.to_string());
}

pub(super) fn set_pending_array_index_out_of_bounds_exception(index: jsize, len: jsize) {
    set_pending_error(
        "java/lang/ArrayIndexOutOfBoundsException",
        &format!("Index {index} out of bounds for length {len}"),
    );
}

pub(super) fn set_pending_array_region_out_of_bounds_exception(
    start: jsize,
    len: jsize,
    array_len: jsize,
) {
    // Same messages as HotSpot's check_bounds in jni.cpp
    let message = if len < 0 {
        format!("Length {len} is negative")
    } else {
        let end = start as i64 + len as i64;
        format!("Array region {start}..{end} out of bounds for length {array_len}")
    };
    set_pending_error("java/lang/ArrayIndexOutOfBoundsException", &message);
}

pub(crate) fn set_pending_internal_error(message: &str) {
    set_pending_error("java/lang/InternalError", message);
}

/// Allocation errors already carry the permanent emergency throwable. Inspect it
/// before formatting a diagnostic or trying to construct another Java exception.
pub(super) fn set_pending_vm_error(error: &Error) {
    if let Some(reference) = error.throwable_ref() {
        JavaThread::set_pending_if_empty(reference);
    } else if JavaThread::get_pending_exception().is_none() {
        set_pending_internal_error(&error.to_string());
    }
}

fn set_pending_error(exception_class: &str, message: &str) {
    // First-wins semantics: if an exception is already pending, preserve it and
    // do nothing, like OpenJDK does. This must never panic — `set_pending_error` is reachable from
    // `extern "system"` natives, and unwinding across that FFI boundary would
    // abort the VM and violate JNI semantics (the original pending exception
    // must survive and propagate to Java).
    //
    // We check *before* constructing the throwable so we never run Java code
    // (the exception constructor) while an exception is already pending, which
    // is itself unsafe.
    if let Some(pending_ref) = JavaThread::get_pending_exception() {
        warn!(
            "Dropping {exception_class} ('{message}'): an exception (ref={pending_ref}) is already pending"
        );
        return;
    }

    let result = StringPoolHelper::get_string(message).and_then(|msg_ref| {
        Executor::invoke_args_constructor(
            exception_class,
            "<init>:(Ljava/lang/String;)V",
            &[StackValueKind::Ref(msg_ref)],
            None,
        )
    });
    match result {
        Ok(throwable_ref) => match JavaThread::try_set_pending_exception(throwable_ref[0].value())
        {
            Ok(()) => (),
            // Constructing the throwable ran Java code, which may itself have
            // raised (and left pending) an exception in the meantime. Honor
            // first-wins and drop this one rather than panicking across FFI.
            Err(e) => warn!("Dropping {exception_class} ('{message}'): {e}"),
        },
        Err(e) => {
            if let Some(throwable) = e.throwable_ref() {
                JavaThread::set_pending_if_empty(throwable);
            } else {
                handles::HandleError::OutOfMemory.set_pending();
            }
        }
    }
}

#[cfg(not(target_pointer_width = "64"))]
compile_error!("src/vm/jni/utils.rs requires a 64-bit target because jmethodID values are encoded into usize using a 64-bit layout");
/// Encode a (class_ref, method_index) pair into a single `usize` suitable for use as a
/// `jmethodID`.
///
/// Layout (64-bit):
///   bits 63–32: declaring class heap reference (i32, zero-extended to u32)
///   bits 31–0: per-class method index (truncated to u32)
pub(super) fn encode_method_id(class_ref: i32, method_index: usize) -> usize {
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
) -> Option<Vec<StackValueKind>> {
    let md = method.get_method_descriptor();
    let pt = md.parameter_types();
    let args_count = pt.len();

    if args_count == 0 {
        return Some(vec![]);
    }

    if args.is_null() {
        set_pending_null_pointer_exception("JNI method arguments are null");
        return None;
    }

    let args = unsafe { std::slice::from_raw_parts(args, args_count) };
    args.iter()
        .zip(pt.iter())
        .map(|(arg, pt)| resolve_stack_kind_value(*arg, pt))
        .collect()
}

fn resolve_stack_kind_value(
    value: jvalue,
    type_descriptor: &TypeDescriptor,
) -> Option<StackValueKind> {
    Some(match type_descriptor {
        TypeDescriptor::Boolean => (if unsafe { value.z } { 1 } else { 0 }).into(),
        TypeDescriptor::Byte => (unsafe { value.b } as i32).into(),
        TypeDescriptor::Char => (unsafe { value.c } as i32).into(),
        TypeDescriptor::Short => (unsafe { value.s } as i32).into(),
        TypeDescriptor::Integer => unsafe { value.i }.into(),
        TypeDescriptor::Long => unsafe { value.j }.into(),
        TypeDescriptor::Float => unsafe { value.f }.into(),
        TypeDescriptor::Double => unsafe { value.d }.into(),
        TypeDescriptor::Object(_) | TypeDescriptor::Array(_, _) => {
            StackValueKind::Ref(handles::resolve(unsafe { value.l })?)
        }
        TypeDescriptor::Void => return None,
    })
}
