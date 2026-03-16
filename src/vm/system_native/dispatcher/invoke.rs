use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::executor::Executor;
use crate::vm::helper::clazz_ref;
use crate::vm::stack::stack_value::StackValue;
use crate::vm::system_native::dispatcher::args::build_args;
use crate::vm::system_native::dispatcher::cif_cache::get_cif;
use crate::vm::system_native::dispatcher::utils::get_symbol_address;
use jdescriptor::{MethodDescriptor, TypeDescriptor};
use libffi::high::CodePtr;
use libffi::middle::{Arg, Cif};
use std::ffi::c_void;

pub(crate) fn invoke(method_signature: &str, args: &[i32], is_static: bool) -> Result<Vec<i32>> {
    let split: Vec<_> = method_signature.split(":").collect();
    if split.len() != 3 {
        return Err(Error::new_native(&format!(
            "Dynamic JNI dispatch requires full method descriptor (expected class:method:descriptor, got: {method_signature})"
        )));
    }
    let class_name = split[0];
    let method_name = split[1];
    let descriptor = split[2];

    let (short_name, long_name) =
        jniname::c_name(class_name, method_name, descriptor).map_err(|_| {
            Error::new_native(&format!("Failed to convert {method_signature} to C name"))
        })?;

    let clazz_ref = clazz_ref(class_name)?;

    let class_loader_ref = Executor::invoke_non_static_method(
        "java/lang/Class",
        "getClassLoader:()Ljava/lang/ClassLoader;",
        clazz_ref,
        &[],
    )?[0];
    let native_libraries_ref = Executor::invoke_static_method(
        "java/lang/ClassLoader",
        "nativeLibrariesFor:(Ljava/lang/ClassLoader;)Ljdk/internal/loader/NativeLibraries;",
        &[class_loader_ref.into()],
    )?[0];

    let symbol_address = match get_symbol_address(&long_name, native_libraries_ref)? {
        Some(v) => v,
        None => get_symbol_address(&short_name, native_libraries_ref)?.ok_or_else(|| {
            Error::new_native(&format!(
                "Failed to find symbol for both short and long names: {short_name}, {long_name}"
            ))
        })?,
    };

    let fun_ptr: *mut c_void =
        std::ptr::with_exposed_provenance_mut::<c_void>(symbol_address as usize);
    let env: *mut i32 = std::ptr::null_mut(); // todo add real JNIEnv*
    let clazz: *mut i32 = std::ptr::null_mut(); // todo add real jclass

    let mut ffi_args = vec![Arg::new(&env)];
    if is_static {
        ffi_args.push(Arg::new(&clazz));
    }

    let method_descriptor: MethodDescriptor = descriptor.parse()?;
    let storage = build_args(args, method_descriptor.parameter_types(), is_static);
    let ffi_args_from_descriptor = (&storage).iter().map(|s| s.as_arg()).collect::<Vec<_>>();
    ffi_args.extend(ffi_args_from_descriptor);

    let code_ptr = CodePtr(fun_ptr);

    let safe_cif = get_cif(&method_descriptor)?;
    call_native(
        &safe_cif.0,
        code_ptr,
        &ffi_args,
        method_descriptor.return_type(),
    )
}

fn call_native(
    cif: &Cif,
    code_ptr: CodePtr,
    args: &[Arg],
    ret: &TypeDescriptor,
) -> Result<Vec<i32>> {
    unsafe {
        match ret {
            TypeDescriptor::Byte
            | TypeDescriptor::Char
            | TypeDescriptor::Integer
            | TypeDescriptor::Short
            | TypeDescriptor::Boolean
            | TypeDescriptor::Array(_, _)
            | TypeDescriptor::Object(_) => {
                let result: i32 = cif.call(code_ptr, args); // look like cif correctly truncates result to the corresponding return type, so no need to cast
                Ok(result.to_vec())
            }
            TypeDescriptor::Float => {
                let result: f32 = cif.call(code_ptr, args);
                Ok(result.to_vec())
            }
            TypeDescriptor::Long => {
                let result: i64 = cif.call(code_ptr, args);
                Ok(result.to_vec())
            }
            TypeDescriptor::Double => {
                let result: f64 = cif.call(code_ptr, args);
                Ok(result.to_vec())
            }
            TypeDescriptor::Void => {
                cif.call::<()>(code_ptr, args);
                Ok(vec![])
            }
        }
    }
}
