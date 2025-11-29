use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::executor::Executor;
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::helper::create_array_of_strings;
use crate::vm::method_area::method_area::with_method_area;

// refer: sun.launcher.LauncherHelper
const CLASS: i32 = 1;

const PRINT_TO_STDERR: bool = true;

pub fn resolve_and_execute_main_method(class_name: &str, args: &[String]) -> Result<()> {
    let class_name_ref = StringPoolHelper::get_string(class_name)?;
    let _app_class = Executor::invoke_static_method(
        "sun/launcher/LauncherHelper",
        "checkAndLoadMain:(ZILjava/lang/String;)Ljava/lang/Class;",
        &[
            (PRINT_TO_STDERR as i32).into(),
            CLASS.into(),
            class_name_ref.into(),
        ],
    )?[0];

    let launcher_helper = with_method_area(|a| a.get("sun/launcher/LauncherHelper"))?;
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
            Executor::invoke_static_method(class_name, "main:()V", &[])?;
        } else {
            let args_array_ref = create_array_of_strings(args)?;
            Executor::invoke_static_method(
                class_name,
                "main:([Ljava/lang/String;)V",
                &[args_array_ref.into()],
            )?;
        }
    } else {
        let main_instance_ref = construct_main_class(class_name)?;
        if no_arg_main {
            Executor::invoke_non_static_method(class_name, "main:()V", main_instance_ref, &[])?;
        } else {
            let args_array_ref = create_array_of_strings(args)?;
            Executor::invoke_non_static_method(
                class_name,
                "main:([Ljava/lang/String;)V",
                main_instance_ref,
                &[args_array_ref.into()],
            )?;
        }
    }

    Ok(())
}

fn construct_main_class(class_name: &str) -> Result<i32> {
    Executor::invoke_default_constructor(class_name)
}
