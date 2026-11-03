use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::common::typed_slots;
use crate::vm::execution_engine::executor::Executor;
use crate::vm::helper::clazz_ref;
use crate::vm::jni::handles;
use crate::vm::jni::java_thread::JavaThread;
use crate::vm::jni::jni_env::get_jni_env;
use crate::vm::jni::local_frames::NativeFrame;
use crate::vm::safepoint::{Blocking, Reason};
use crate::vm::stack::rooted_slots::RootedSlots;
use crate::vm::stack::slot::Slot;
use crate::vm::stack::stack_value::StackValue;
use crate::vm::system_native::dispatcher::args::build_args;
use crate::vm::system_native::dispatcher::builtin_natives::find_builtin_native;
use crate::vm::system_native::dispatcher::cif_cache::get_cif;
use crate::vm::system_native::dispatcher::utils::get_symbol_address;
use jdescriptor::{MethodDescriptor, TypeDescriptor};
use jni_sys::jobject;
use libffi::high::CodePtr;
use libffi::middle::{Arg, Cif};
use std::ffi::c_void;

pub(crate) fn invoke(
    method_signature: &str,
    args: &[Slot],
    is_static: bool,
    destination: &mut RootedSlots,
) -> Result<()> {
    let split: Vec<_> = method_signature.split(":").collect();
    if split.len() != 3 {
        return Err(Error::new_native(&format!(
            "Dynamic JNI dispatch requires full method descriptor (expected class:method:descriptor, got: {method_signature})"
        )));
    }
    let class_name = split[0];
    let method_name = split[1];
    let descriptor = split[2];

    // Frame and argument locals precede class/symbol resolution, which can call Java.
    let Some(_native_frame) = NativeFrame::enter() else {
        return Ok(());
    };
    let method_descriptor: MethodDescriptor = descriptor.parse()?;
    let raw_args: Vec<_> = args.iter().map(|slot| slot.value()).collect();
    let storage = build_args(&raw_args, method_descriptor.parameter_types(), is_static);
    if JavaThread::get_pending_exception().is_some() {
        return Ok(());
    }
    let (short_name, long_name) =
        jniname::c_name(class_name, method_name, descriptor).map_err(|_| {
            Error::new_native(&format!("Failed to convert {method_signature} to C name"))
        })?;
    let clazz_ref = clazz_ref(class_name)?;
    let clazz_handle = handles::new_local(clazz_ref);
    if JavaThread::get_pending_exception().is_some() {
        return Ok(());
    }
    let (symbol_address, external) = match find_builtin_native(method_signature) {
        Some(address) => (address, false),
        None => (
            resolve_library_symbol(clazz_ref, &short_name, &long_name)?,
            true,
        ),
    };

    let fun_ptr: *mut c_void =
        std::ptr::with_exposed_provenance_mut::<c_void>(symbol_address as usize);

    let env = get_jni_env();
    let mut ffi_args = vec![Arg::new(&env)];
    if is_static {
        ffi_args.push(Arg::new(&clazz_handle));
    }

    let ffi_args_from_descriptor = storage.iter().map(|s| s.as_arg()).collect::<Vec<_>>();
    ffi_args.extend(ffi_args_from_descriptor);

    let code_ptr = CodePtr(fun_ptr);

    let safe_cif = get_cif(&method_descriptor)?;
    let result = call_native(
        &safe_cif.0,
        code_ptr,
        &ffi_args,
        method_descriptor.return_type(),
        external,
    );
    let raw = match result {
        NativeReturn::Object(handle) => vec![handles::resolve(handle).unwrap_or(0)],
        NativeReturn::Slots(raw) => raw,
    };
    destination.replace(typed_slots(method_descriptor.return_type(), &raw)?);
    Ok(())
}

/// Resolves the address of a native symbol exported by a loaded native library,
/// trying the overloaded (long) name first and then the simple (short) name.
fn resolve_library_symbol(clazz_ref: i32, short_name: &str, long_name: &str) -> Result<i64> {
    let class_loader = Executor::invoke_non_static_method(
        "java/lang/Class",
        "getClassLoader:()Ljava/lang/ClassLoader;",
        clazz_ref,
        &[],
    )?;
    let native_libraries = Executor::invoke_static_method(
        "java/lang/ClassLoader",
        "nativeLibrariesFor:(Ljava/lang/ClassLoader;)Ljdk/internal/loader/NativeLibraries;",
        &[class_loader[0].into()],
    )?;
    let native_libraries_ref = native_libraries[0].value();

    match get_symbol_address(long_name, native_libraries_ref)? {
        Some(v) => Ok(v),
        None => get_symbol_address(short_name, native_libraries_ref)?.ok_or_else(|| {
            Error::new_native(&format!(
                "Failed to find symbol for both short and long names: {short_name}, {long_name}"
            ))
        }),
    }
}

enum NativeReturn {
    Object(jobject),
    Slots(Vec<i32>),
}

fn call_native(
    cif: &Cif,
    code_ptr: CodePtr,
    args: &[Arg],
    ret: &TypeDescriptor,
    external: bool,
) -> NativeReturn {
    let result = {
        // Builtin wrappers can enter Safe themselves while performing blocking work.
        // Only third-party code needs this outer transition.
        let _safe = external.then(|| Blocking::enter(Reason::Native));
        unsafe {
            match ret {
                TypeDescriptor::Byte
                | TypeDescriptor::Char
                | TypeDescriptor::Integer
                | TypeDescriptor::Short
                | TypeDescriptor::Boolean => {
                    let result: i32 = cif.call(code_ptr, args); // look like cif correctly truncates result to the corresponding return type, so no need to cast
                    NativeReturn::Slots(result.to_vec())
                }
                TypeDescriptor::Array(_, _) | TypeDescriptor::Object(_) => {
                    let result: jobject = cif.call(code_ptr, args);
                    NativeReturn::Object(result)
                }
                TypeDescriptor::Float => {
                    let result: f32 = cif.call(code_ptr, args);
                    NativeReturn::Slots(result.to_vec())
                }
                TypeDescriptor::Long => {
                    let result: i64 = cif.call(code_ptr, args);
                    NativeReturn::Slots(result.to_vec())
                }
                TypeDescriptor::Double => {
                    let result: f64 = cif.call(code_ptr, args);
                    NativeReturn::Slots(result.to_vec())
                }
                TypeDescriptor::Void => {
                    cif.call::<()>(code_ptr, args);
                    NativeReturn::Slots(vec![])
                }
            }
        }
    };
    // Dropping `_safe` above reacquires managed access before any JNI handle decode.
    result
}

#[cfg(test)]
mod jni_handles_tests {
    use super::*;
    use crate::vm::heap::heap::HEAP;
    use libffi::middle::Type;
    use std::sync::mpsc::channel;
    use std::sync::Barrier;

    static NATIVE_ENTERED: Barrier = Barrier::new(2);
    static NATIVE_RELEASE: Barrier = Barrier::new(2);

    extern "system" fn return_object(object: jobject) -> jobject {
        object
    }

    extern "system" fn wait_inside_native() {
        NATIVE_ENTERED.wait();
        NATIVE_RELEASE.wait();
    }

    #[test]
    fn jni_handles_libffi_preserves_pointer_width_object_return() {
        let _frame = NativeFrame::enter().unwrap();
        let target = HEAP.create_array("[I", 0);
        let object = handles::new_wide_test_local(target);
        if usize::BITS > 32 {
            assert!(u64::try_from(object as usize).unwrap() > u64::from(u32::MAX));
        }
        let cif = Cif::new([Type::pointer()], Type::pointer());
        let result = call_native(
            &cif,
            CodePtr(return_object as *mut c_void),
            &[Arg::new(&object)],
            &TypeDescriptor::Object("java/lang/Object".into()),
            false,
        );
        let NativeReturn::Object(result) = result else {
            panic!("object return keeps its pointer-width value");
        };
        assert_eq!(handles::resolve(result), Some(target));
    }

    #[test]
    fn gc_coordinator_external_native_call_is_published_safe() {
        let _serial = crate::vm::safepoint::tests::SERIAL.lock().unwrap();
        crate::vm::safepoint::attach();
        let (participant, registered) = channel();
        let worker = std::thread::spawn(move || {
            crate::vm::safepoint::attach();
            participant
                .send(crate::vm::safepoint::current().unwrap())
                .unwrap();
            let cif = Cif::new([], Type::void());
            let result = call_native(
                &cif,
                CodePtr(wait_inside_native as *mut c_void),
                &[],
                &TypeDescriptor::Void,
                true,
            );
            assert!(matches!(result, NativeReturn::Slots(values) if values.is_empty()));
            crate::vm::safepoint::retire();
        });

        let worker_id = registered.recv().unwrap();
        NATIVE_ENTERED.wait();
        let safely_parked = !crate::vm::safepoint::COORDINATOR.running(worker_id);
        NATIVE_RELEASE.wait();
        worker.join().unwrap();
        assert!(safely_parked, "native execution must be reported Safe");
        crate::vm::safepoint::retire();
    }
}
