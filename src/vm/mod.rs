mod commons;
mod error;
mod exception;
mod execution_engine;
mod heap;
mod helper;
mod jni;
mod launcher;
mod method_area;
mod monitor;
mod safepoint;
mod perf_data;
mod properties;
mod stack;
mod system_native;
mod threads;
mod validation;

use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::executor::Executor;
use crate::vm::execution_engine::static_init::StaticInit;
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::jni::java_thread::JavaThread;
use crate::vm::launcher::resolve_and_execute_main_method;
use crate::vm::launcher::LaunchMode::{LmClass, LmJar};
use crate::vm::method_area::java_class::JavaClass;
use crate::vm::method_area::loaded_classes::CLASSES;
use crate::vm::method_area::method_area::{with_method_area, MethodArea};
use crate::vm::perf_data::init_perf_file;
use crate::vm::properties::resolve_classpath;
use crate::vm::properties::system_properties::init_system_properties;
use crate::vm::system_native::properties_provider::properties::is_bigendian;
use crate::vm::validation::validate_class_name;
use crate::Arguments;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

pub(crate) static PLATFORM_CLASSLOADER_REF: OnceLock<i32> = OnceLock::new();
pub(crate) static SYSTEM_CLASSLOADER_REF: OnceLock<i32> = OnceLock::new();

/// Launches the Rusty Java Virtual Machine with the given arguments.
///
/// This function initializes the JVM, loads the specified main class, and invokes its `main` method.
///
/// # Arguments
///
/// * `arguments` - The arguments for the Java program.
/// * `java_home` - The path to the Java home directory.
pub fn run(arguments: &Arguments, java_home: &Path) -> Result<()> {
    JAVA_HOME
        .set(java_home.to_path_buf())
        .map_err(|e| Error::new_execution(&format!("JAVA_HOME already set: {e:?}")))?;
    let entry_point = arguments.entry_point();
    validate_class_name(entry_point)?;

    resolve_classpath(arguments)?;

    init_system_properties(arguments.system_properties().clone())?;

    init_perf_file(arguments)?;

    let result = (|| -> Result<()> {
        prelude()?;

        let launch_mode = if *arguments.jar_mode() {
            LmJar
        } else {
            LmClass
        };
        let main_result =
            resolve_and_execute_main_method(entry_point, launch_mode, arguments.program_args());

        // The VM stays alive until the last non-daemon thread dies (JVMS), so wait for every
        // spawned non-daemon thread before shutting down — regardless of how `main` itself finished.
        threads::join_all_non_daemon();

        main_result?;

        invoke_shutdown_hooks()?;

        Ok(())
    })();

    match result {
        Ok(()) => Ok(()),
        Err(e) if e.is_uncaught_exception() => {
            if let Some(throwable_ref) = e.throwable_ref() {
                if let Err(handler_err) = invoke_uncaught_exception_handler(throwable_ref) {
                    tracing::error!("Failed to invoke uncaught exception handler: {handler_err}");
                }
            }
            if let Err(hook_err) = invoke_shutdown_hooks() {
                tracing::error!("Failed to invoke shutdown hooks: {hook_err}");
            }
            Err(e)
        }
        Err(e) => Err(e),
    }
}

pub(crate) fn invoke_uncaught_exception_handler(throwable_ref: i32) -> Result<()> {
    // Dispatch exactly as HotSpot does for an uncaught exception: hand it to the *offending* thread's
    // own `dispatchUncaughtException`, which routes to `getUncaughtExceptionHandler()` (defaulting to
    // the thread's group) and prints `Exception in thread "<name>" ...` using that thread's name.
    // Works for both the main thread and spawned threads via the per-thread identity; the main thread
    // ref is used as a fallback for any early path that has no thread-local identity yet.
    let thread_ref = match JavaThread::current_thread() {
        Some(thread_ref) => thread_ref,
        None => with_method_area(|area| area.system_thread_id())?,
    };
    Executor::invoke_non_static_method(
        "java/lang/Thread",
        "dispatchUncaughtException:(Ljava/lang/Throwable;)V",
        thread_ref,
        &[throwable_ref.into()],
    )?;
    Ok(())
}

fn invoke_shutdown_hooks() -> Result<()> {
    Executor::invoke_static_method("java/lang/Shutdown", "shutdown:()V", &[])?;
    Ok(())
}

fn prelude() -> Result<()> {
    init_logger()?;

    MethodArea::init()?;

    // preload essential classes including breaking circular dependencies
    CLASSES.pre_construct()?;
    CLASSES.post_construct()?;

    for java_class in MethodArea::generate_synthetic_classes() {
        CLASSES.insert_klass(Arc::clone(&java_class), None)?;
    }

    init()?;

    Ok(())
}

fn init_logger() -> Result<()> {
    let fmt_layer = fmt::layer()
        .with_target(false)
        .without_time()
        .with_ansi(false);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .map_err(|e| Error::new_execution(&format!("Error creating EnvFilter: {e}")))?;
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();

    Ok(())
}

fn init() -> Result<()> {
    put_synthetic_instance_field("java/lang/invoke/ResolvedMethodName", "vmtarget", "J", 0)?;
    StaticInit::initialize("jdk/internal/misc/UnsafeConstants")?;

    let lc = CLASSES.get("jdk/internal/misc/UnsafeConstants")?;
    let big_endian = lc.static_field("BIG_ENDIAN").unwrap();
    big_endian.set_raw_value(vec![if is_bigendian() { 1 } else { 0 }])?;

    let address_size0 = lc.static_field("ADDRESS_SIZE0").unwrap();
    address_size0.set_raw_value(vec![8])?;

    let page_size = lc.static_field("PAGE_SIZE").unwrap();
    page_size.set_raw_value(vec![page_size::get() as i32])?;

    // Create the thread-group hierarchy and the main thread, mirroring OpenJDK: a root "system"
    // group, a "main" group as its child, and the primordial thread named "main" in that group.
    // The thread's name is what surfaces as `Exception in thread "main"` on an uncaught exception.
    let system_tg_ref = Executor::invoke_default_constructor("java/lang/ThreadGroup")?;
    with_method_area(|area| area.set_system_thread_group_id(system_tg_ref))?;
    let main_name_ref = StringPoolHelper::get_string("main")?; // refactor candidate B: introduce and use here common string creator, not string pool one
    let main_tg_ref = Executor::invoke_args_constructor(
        "java/lang/ThreadGroup",
        "<init>:(Ljava/lang/ThreadGroup;Ljava/lang/String;)V",
        &[system_tg_ref.into(), main_name_ref.into()],
        Some("main thread group creation"),
    )?;
    with_method_area(|area| area.set_main_thread_group_id(main_tg_ref))?;
    let _thread_obj_ref =
        Executor::create_primordial_thread(&[main_tg_ref.into(), main_name_ref.into()])?;

    StaticInit::initialize("java/lang/reflect/Method")?;
    Executor::invoke_static_method("java/lang/System", "initPhase1:()V", &[])?;
    let init_phase2_result = Executor::invoke_static_method(
        "java/lang/System",
        "initPhase2:(ZZ)I",
        &[1.into(), 1.into()],
    )?[0];
    if init_phase2_result != 0 {
        return Err(Error::new_execution(&format!(
            "System.initPhase2 returned error code {init_phase2_result}"
        )));
    }
    Executor::invoke_static_method("java/lang/System", "initPhase3:()V", &[])?;

    PLATFORM_CLASSLOADER_REF
        .set(
            Executor::invoke_static_method(
                "java/lang/ClassLoader",
                "getPlatformClassLoader:()Ljava/lang/ClassLoader;",
                &[],
            )?[0],
        )
        .map_err(|_e| Error::new_execution("value already set"))?;
    SYSTEM_CLASSLOADER_REF
        .set(
            Executor::invoke_static_method(
                "java/lang/ClassLoader",
                "getSystemClassLoader:()Ljava/lang/ClassLoader;",
                &[],
            )?[0],
        )
        .map_err(|_e| Error::new_execution("value already set"))?;

    let system_classloader_ref = SYSTEM_CLASSLOADER_REF
        .get()
        .copied()
        .ok_or_else(|| Error::new_execution("SYSTEM_CLASSLOADER_REF not set"))?;
    let module_ref = Executor::invoke_args_constructor(
        "java/lang/Module",
        "<init>:(Ljava/lang/ClassLoader;)V",
        &[system_classloader_ref.into()],
        Some("module for reflection class"),
    )?;
    UNNAMED_MODULE_REF
        .set(module_ref)
        .map_err(|e| Error::new_execution(&format!("MODULE_REF already set: {e:?}")))?;

    Ok(())
}

pub(crate) static UNNAMED_MODULE_REF: OnceLock<i32> = OnceLock::new();

fn put_synthetic_instance_field(
    class_name: &str,
    field_name: &str,
    type_descriptor: &str,
    flags: u16,
) -> Result<()> {
    let lc = CLASSES.get(class_name)?;
    let raw_java_class = Arc::into_raw(lc) as *mut JavaClass;
    let result = unsafe {
        (*raw_java_class).put_instance_field_descriptor(
            field_name.to_string(),
            str::parse(type_descriptor)?,
            flags,
            class_name,
        )?
    };
    match result {
        Some(field_property) => Err(Error::new_execution(&format!(
            "field {field_name}:{} already exists in {class_name}",
            field_property.type_descriptor()
        ))),
        None => Ok(()),
    }
}

static JAVA_HOME: OnceLock<PathBuf> = OnceLock::new();
