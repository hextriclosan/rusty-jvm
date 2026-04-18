pub mod concurrency;
mod commons;
mod error;
mod exception;
mod execution_engine;
mod heap;
mod helper;
mod jni;
mod launcher;
mod method_area;
mod perf_data;
mod properties;
mod stack;
mod system_native;
mod validation;

use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::executor::Executor;
use crate::vm::execution_engine::static_init::StaticInit;
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
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

    // Execute the synchronous initialization prelude.
    // Nested async calls will automatically spin up temporary lightweight runtimes.
    prelude()?;

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(|e| Error::new_execution(&format!("Failed to build Tokio runtime: {e}")))?;

    let launch_mode = if *arguments.jar_mode() {
        LmJar
    } else {
        LmClass
    };

    let system_thread_id = crate::vm::method_area::method_area::with_method_area(|area| area.system_thread_id()).unwrap_or(0);
    
    // Execute the primary main method using the global Tokio runtime.
    let result = runtime.block_on(async {
        crate::vm::concurrency::task_local::CURRENT_JAVA_THREAD_ID.scope(system_thread_id, async {
            resolve_and_execute_main_method(entry_point, launch_mode, arguments.program_args()).await
        }).await
    });

    // After the main Java thread finishes, execute shutdown hooks.
    // Again, these are run outside the main global runtime block.
    match result {
        Ok(()) => {
            invoke_shutdown_hooks()?;
            Ok(())
        }
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

fn invoke_uncaught_exception_handler(throwable_ref: i32) -> Result<()> {
    let system_thread_group_ref = with_method_area(|area| area.system_thread_group_id())?;
    let system_thread_ref = with_method_area(|area| area.system_thread_id())?;
    crate::vm::concurrency::block_on_async(
        Executor::invoke_non_static_method(
            "java/lang/ThreadGroup",
            "uncaughtException:(Ljava/lang/Thread;Ljava/lang/Throwable;)V",
            system_thread_group_ref,
            &[system_thread_ref.into(), throwable_ref.into()],
        )
    )?;
    Ok(())
}

fn invoke_shutdown_hooks() -> Result<()> {
    crate::vm::concurrency::block_on_async(
        Executor::invoke_static_method("java/lang/Shutdown", "shutdown:()V", &[])
    )?;
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

    // create primordial ThreadGroup and Thread
    let tg_obj_ref = crate::vm::concurrency::block_on_async(Executor::invoke_default_constructor("java/lang/ThreadGroup"))?;
    with_method_area(|area| area.set_system_thread_group_id(tg_obj_ref))?;
    let string_obj_ref = StringPoolHelper::get_string("system")?; 
    let _thread_obj_ref = crate::vm::concurrency::block_on_async(
        Executor::create_primordial_thread(&[tg_obj_ref.into(), string_obj_ref.into()])
    )?;

    StaticInit::initialize("java/lang/reflect/Method")?;
    crate::vm::concurrency::block_on_async(Executor::invoke_static_method("java/lang/System", "initPhase1:()V", &[]))?;
    let init_phase2_result = crate::vm::concurrency::block_on_async(
        Executor::invoke_static_method(
            "java/lang/System",
            "initPhase2:(ZZ)I",
            &[1.into(), 1.into()],
        )
    )?[0];
    if init_phase2_result != 0 {
        return Err(Error::new_execution(&format!(
            "System.initPhase2 returned error code {init_phase2_result}"
        )));
    }
    crate::vm::concurrency::block_on_async(Executor::invoke_static_method("java/lang/System", "initPhase3:()V", &[]))?;

    PLATFORM_CLASSLOADER_REF
        .set(
            crate::vm::concurrency::block_on_async(
                Executor::invoke_static_method(
                    "java/lang/ClassLoader",
                    "getPlatformClassLoader:()Ljava/lang/ClassLoader;",
                    &[],
                )
            )?[0],
        )
        .map_err(|_e| Error::new_execution("value already set"))?;
    SYSTEM_CLASSLOADER_REF
        .set(
            crate::vm::concurrency::block_on_async(
                Executor::invoke_static_method(
                    "java/lang/ClassLoader",
                    "getSystemClassLoader:()Ljava/lang/ClassLoader;",
                    &[],
                )
            )?[0],
        )
        .map_err(|_e| Error::new_execution("value already set"))?;

    let system_classloader_ref = SYSTEM_CLASSLOADER_REF
        .get()
        .copied()
        .ok_or_else(|| Error::new_execution("SYSTEM_CLASSLOADER_REF not set"))?;
    let module_ref = crate::vm::concurrency::block_on_async(
        Executor::invoke_args_constructor(
            "java/lang/Module",
            "<init>:(Ljava/lang/ClassLoader;)V",
            &[system_classloader_ref.into()],
            Some("module for reflection class"),
        )
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
