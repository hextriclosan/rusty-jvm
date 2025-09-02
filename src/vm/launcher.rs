use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::executor::Executor;
use crate::vm::helper::create_array_of_strings;
use crate::vm::method_area::method_area::with_method_area;

pub fn resolve_and_execute_main_method(class_name: &str, args: &[String]) -> Result<()> {
    let jc = with_method_area(|a| a.get(class_name))?;
    if let Some((_, method)) = jc.get_method_full("main:([Ljava/lang/String;)V") {
        let args_array_ref = create_array_of_strings(args)?;
        if method.is_static() {
            Executor::invoke_static_method(
                class_name,
                "main:([Ljava/lang/String;)V",
                &[args_array_ref.into()],
            )?;
        } else {
            let main_instance_ref = construct_main_class(class_name)?;
            Executor::invoke_non_static_method(
                class_name,
                "main:([Ljava/lang/String;)V",
                main_instance_ref,
                &[],
            )?;
        }
    } else if let Some((_, method)) = jc.get_method_full("main:()V") {
        if method.is_static() {
            Executor::invoke_static_method(class_name, "main:()V", &[])?;
        } else {
            let main_instance_ref = construct_main_class(class_name)?;
            Executor::invoke_non_static_method(class_name, "main:()V", main_instance_ref, &[])?;
        }
    } else {
        return Err(Error::new_execution(&format!(
            "Main method not found in class {class_name}"
        )));
    }

    Ok(())
}

fn construct_main_class(class_name: &str) -> Result<i32> {
    Executor::invoke_default_constructor(class_name)
}
