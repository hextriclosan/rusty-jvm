mod error;
mod exception;
mod execution_engine;
mod heap;
mod helper;
mod method_area;
mod properties;
mod stack;
mod system_native;
mod validation;

use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::executor::Executor;
use crate::vm::execution_engine::static_init::StaticInit;
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::helper::create_array_of_strings;
use crate::vm::method_area::java_class::JavaClass;
use crate::vm::method_area::method_area::{with_method_area, MethodArea};
use crate::vm::properties::system_properties::init_system_properties;
use crate::vm::system_native::properties_provider::properties::is_bigendian;
use crate::vm::validation::validate_class_name;
use crate::Arguments;
use std::path::Path;
use std::sync::Arc;
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
/// * `std_dir` - The path to the standard library directory containing core Java classes.
pub fn run(arguments: &Arguments, java_home: &Path) -> Result<()> {
    let main_class_name = arguments.entry_point();
    validate_class_name(main_class_name)?;

    init_system_properties(arguments.system_properties().clone())?;

    prelude(java_home)?;

    let internal_name = &main_class_name.replace('.', "/");
    StaticInit::initialize(internal_name)?; // before invoking static main method, static fields should be initialized (JVMS requirement)

    let args_array_ref = create_array_of_strings(arguments.program_args())?;
    Executor::invoke_static_method(
        internal_name,
        "main:([Ljava/lang/String;)V",
        &[args_array_ref.into()],
    )
}

fn prelude(java_home: &Path) -> Result<()> {
    init_logger()?;

    MethodArea::init(java_home)?;

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
    StaticInit::initialize("jdk/internal/misc/UnsafeConstants")?;
    let lc = with_method_area(|area| area.get("jdk/internal/misc/UnsafeConstants"))?;
    let big_endian = lc.static_field("BIG_ENDIAN").unwrap();
    big_endian.set_raw_value(vec![if is_bigendian() { 1 } else { 0 }])?;

    let address_size0 = lc.static_field("ADDRESS_SIZE0").unwrap();
    address_size0.set_raw_value(vec![8])?;

    StaticInit::initialize("java/lang/reflect/AccessibleObject")?;

    put_synthetic_instance_field("java/lang/invoke/ResolvedMethodName", "vmtarget", "J", 0)?;

    // create primordial ThreadGroup and Thread
    let tg_obj_ref = Executor::invoke_default_constructor("java/lang/ThreadGroup")?;
    with_method_area(|area| area.set_system_thread_group_id(tg_obj_ref))?;
    let string_obj_ref = StringPoolHelper::get_string("system".to_string())?; // refactor candidate B: introduce and use here common string creator, not string pool one
    let _thread_obj_ref =
        Executor::create_primordial_thread(&vec![tg_obj_ref.into(), string_obj_ref.into()])?;

    Executor::invoke_static_method("java/lang/System", "initPhase1:()V", &[])?;

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
