mod commons;
mod error;
mod exception;
mod execution_engine;
mod heap;
mod helper;
mod launcher;
mod method_area;
mod properties;
mod stack;
mod system_native;
mod validation;

use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::executor::Executor;
use crate::vm::execution_engine::static_init::StaticInit;
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::launcher::resolve_and_execute_main_method;
use crate::vm::method_area::java_class::JavaClass;
use crate::vm::method_area::method_area::{with_method_area, MethodArea};
use crate::vm::properties::system_properties::init_system_properties;
use crate::vm::system_native::properties_provider::properties::is_bigendian;
use crate::vm::validation::validate_class_name;
use crate::Arguments;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

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
    let main_class_name = arguments.entry_point();
    validate_class_name(main_class_name)?;

    init_system_properties(arguments.system_properties().clone())?;

    prelude()?;

    let internal_name = &main_class_name.replace('.', "/");
    StaticInit::initialize(internal_name)?; // before invoking static main method, static fields should be initialized (JVMS requirement)

    match resolve_and_execute_main_method(internal_name, arguments.program_args()) {
        Ok(_) => {
            invoke_shutdown_hooks()?;
            Ok(())
        }
        Err(e) if e.is_uncaught_exception() => {
            invoke_shutdown_hooks()?;
            Err(e)
        }
        Err(e) => Err(e),
    }
}

fn invoke_shutdown_hooks() -> Result<()> {
    Executor::invoke_static_method("java/lang/Shutdown", "shutdown:()V", &[])?;
    Ok(())
}

fn prelude() -> Result<()> {
    init_logger()?;

    MethodArea::init()?;

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

    let lc = with_method_area(|area| area.get("jdk/internal/misc/UnsafeConstants"))?;
    let big_endian = lc.static_field("BIG_ENDIAN").unwrap();
    big_endian.set_raw_value(vec![if is_bigendian() { 1 } else { 0 }])?;

    let address_size0 = lc.static_field("ADDRESS_SIZE0").unwrap();
    address_size0.set_raw_value(vec![8])?;

    let page_size = lc.static_field("PAGE_SIZE").unwrap();
    page_size.set_raw_value(vec![page_size::get() as i32])?;

    // create primordial ThreadGroup and Thread
    let tg_obj_ref = Executor::invoke_default_constructor("java/lang/ThreadGroup")?;
    with_method_area(|area| area.set_system_thread_group_id(tg_obj_ref))?;
    let string_obj_ref = StringPoolHelper::get_string("system")?; // refactor candidate B: introduce and use here common string creator, not string pool one
    let _thread_obj_ref =
        Executor::create_primordial_thread(&vec![tg_obj_ref.into(), string_obj_ref.into()])?;

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
    Ok(())
}

fn put_synthetic_instance_field(
    class_name: &str,
    field_name: &str,
    type_descriptor: &str,
    flags: u16,
) -> Result<()> {
    let lc = with_method_area(|area| area.get(class_name))?;
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
