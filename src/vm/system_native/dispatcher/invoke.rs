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
        None => {
            let address = resolve_library_symbol(clazz_ref, &short_name, &long_name)?;
            #[cfg(test)]
            crate::vm::safepoint::event(crate::vm::safepoint::Event::ExternalNativeResolved);
            (address, true)
        }
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
    crate::vm::safepoint::poll();
    crate::vm::gc::trigger::at_safepoint();
    let result = call_native(
        &safe_cif.0,
        code_ptr,
        &ffi_args,
        method_descriptor.return_type(),
        external,
    );
    #[cfg(test)]
    crate::vm::safepoint::event(crate::vm::safepoint::Event::NativeCallReturned);
    if JavaThread::get_pending_exception().is_some() {
        return Ok(());
    }
    let raw = match result {
        NativeReturn::Object(handle) => vec![handles::resolve(handle).unwrap_or(0)],
        NativeReturn::Slots(raw) => raw,
    };
    destination.replace(typed_slots(method_descriptor.return_type(), &raw)?);
    crate::vm::safepoint::poll();
    crate::vm::gc::trigger::at_safepoint();
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
        let target = HEAP
            .create_array(
                "[I",
                0,
                &mut crate::vm::stack::rooted_slots::RootedSlots::new(),
            )
            .unwrap();
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

    #[test]
    fn gc_collect_completes_rendezvous_and_sweep_while_libffi_worker_is_held() {
        use crate::vm::gc::{collector, pending, roots};
        use crate::vm::safepoint::{self, COORDINATOR};
        use std::time::Duration;
        let _serial = safepoint::tests::SERIAL.lock().unwrap();
        let target = HEAP.create_array("[I", 1, &mut RootedSlots::new()).unwrap();
        let dead = HEAP.create_array("[I", 1, &mut RootedSlots::new()).unwrap();
        let worker = std::thread::spawn(move || {
            safepoint::attach();
            let _frame = NativeFrame::enter().unwrap();
            assert!(!handles::new_local(target).is_null());
            let cif = Cif::new([], Type::void());
            call_native(
                &cif,
                CodePtr(wait_inside_native as *mut c_void),
                &[],
                &TypeDescriptor::Void,
                true,
            );
            assert!(HEAP.contains(target));
            safepoint::retire();
        });
        NATIVE_ENTERED.wait();
        let (done, completion) = channel();
        let collector = std::thread::spawn(move || {
            safepoint::attach();
            let operation = COORDINATOR
                .request_collection(safepoint::current().unwrap(), JavaThread::snapshot_roots())
                .unwrap();
            assert_eq!(operation.snapshots.len(), 2);
            let snapshot = roots::snapshot(&operation).unwrap();
            assert!(snapshot.contains(&target));
            collector::collect_snapshot(
                &HEAP,
                &snapshot,
                &pending::PENDING,
                collector::Cause::Explicit,
                &operation,
            )
            .unwrap();
            operation.complete();
            done.send((HEAP.contains(target), HEAP.contains(dead)))
                .unwrap();
            safepoint::retire();
        });
        let result = completion.recv_timeout(Duration::from_secs(5));
        // Release even on failure: the timeout detects a stuck rendezvous, not ordering.
        NATIVE_RELEASE.wait();
        worker.join().unwrap();
        collector.join().unwrap();
        assert_eq!(result.unwrap(), (true, false));
    }

    #[test]
    fn gc_collect_real_library_resolution_takes_the_external_safe_path() {
        use crate::vm::safepoint::{self, Event};
        use std::cell::Cell;
        use std::rc::Rc;
        let status = std::process::Command::new("cargo")
            .args(["build", "-p", "jni_test_lib"])
            .status()
            .unwrap();
        assert!(status.success());
        let resolved = Rc::new(Cell::new(false));
        let safe = Rc::new(Cell::new(false));
        let calls = Rc::new(Cell::new(0));
        let observed = Rc::clone(&calls);
        safepoint::observe(move |event| match event {
            Event::ExternalNativeResolved => {
                resolved.set(true);
                safe.set(false);
            }
            Event::SnapshotPublished if resolved.get() => {
                assert!(!safepoint::COORDINATOR.running(safepoint::current().unwrap()));
                safe.set(true);
            }
            Event::NativeCallReturned if resolved.replace(false) => {
                assert!(
                    safe.get(),
                    "a real library hit must enter Safe inside invoke"
                );
                calls.set(calls.get() + 1);
            }
            _ => {}
        });
        let target = std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".into());
        let arguments = crate::Arguments::new(
            "samples.gc.GcJniIdentityDemo".into(),
            vec![],
            vec![],
            vec![],
            [("java.library.path".into(), env!("JNI_TEST_LIB_PATH").into())]
                .into_iter()
                .collect(),
            vec!["-Xmx64m".into()],
            vec![],
            Some(format!("{target}/java_classes_for_tests")),
            false,
        );
        crate::run(
            &arguments,
            std::path::Path::new(&std::env::var("JAVA_HOME").unwrap()),
        )
        .unwrap();
        assert!(
            observed.get() >= 8,
            "exercise real external library hits, not a hand-passed flag"
        );
    }
}
