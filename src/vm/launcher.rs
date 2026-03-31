use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::executor::Executor;
use crate::vm::execution_engine::static_init::StaticInit;
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::helper::{create_array_of_strings, klass};
use crate::vm::method_area::java_class::JavaClass;
use crate::vm::method_area::loaded_classes::CLASSES;
use std::sync::Arc;

// refer: sun.launcher.LauncherHelper
#[repr(i32)]
#[derive(Debug)]
pub(crate) enum LaunchMode {
    LmClass = 1,
    LmJar = 2,
}

const PRINT_TO_STDERR: bool = true;

pub fn resolve_and_execute_main_method(
    class_name: &str,
    launch_mode: LaunchMode,
    args: &[String],
) -> Result<()> {
    let class_name_ref = StringPoolHelper::get_string(class_name)?;
    let app_clazz_ref = Executor::invoke_static_method(
        "sun/launcher/LauncherHelper",
        "checkAndLoadMain:(ZILjava/lang/String;)Ljava/lang/Class;",
        &[
            (PRINT_TO_STDERR as i32).into(),
            (launch_mode as i32).into(),
            class_name_ref.into(),
        ],
    )?[0];

    let jc = klass(app_clazz_ref)?;
    StaticInit::initialize_java_class(&jc)?;
    let launcher_helper = CLASSES.get("sun/launcher/LauncherHelper")?;
    let static_main = launcher_helper
        .static_field("isStaticMain")
        .ok_or_else(|| {
            Error::new_execution("Error getting isStaticMain field from LauncherHelper")
        })?
        .raw_value()?[0]
        != 0;
    let no_arg_main = launcher_helper
        .static_field("noArgMain")
        .ok_or_else(|| Error::new_execution("Error getting noArgMain field from LauncherHelper"))?
        .raw_value()?[0]
        != 0;

    if static_main {
        if no_arg_main {
            Executor::invoke_static_method_jc(&jc, "main:()V", &[])?;
        } else {
            let args_array_ref = create_array_of_strings(args)?;
            Executor::invoke_static_method_jc(
                &jc,
                "main:([Ljava/lang/String;)V",
                &[args_array_ref.into()],
            )?;
        }
    } else {
        let main_instance_ref = construct_main_class(&jc)?;
        if no_arg_main {
            Executor::invoke_non_static_method_jc(&jc, "main:()V", main_instance_ref, &[])?;
        } else {
            let args_array_ref = create_array_of_strings(args)?;
            Executor::invoke_non_static_method_jc(
                &jc,
                "main:([Ljava/lang/String;)V",
                main_instance_ref,
                &[args_array_ref.into()],
            )?;
        }
    }

    Ok(())
}

fn construct_main_class(java_class: &Arc<JavaClass>) -> Result<i32> {
    Executor::invoke_default_constructor_jc(java_class)
}
