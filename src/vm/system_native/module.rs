use crate::vm::error::Error;
use crate::vm::execution_engine::executor::Executor;
use crate::vm::heap::heap::HEAP;
use crate::vm::method_area::method_area::with_method_area;
use crate::vm::system_native::string::get_utf8_string_by_ref;
use crate::vm::Result;

pub(crate) fn define_module0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let this_module_ref = args[0];
    let is_open = args[1] != 0;
    let version_str_ref = args[2];
    let location_str_ref = args[3];
    let packages_arr_ref = args[4];
    define_module0(
        this_module_ref,
        is_open,
        version_str_ref,
        location_str_ref,
        packages_arr_ref,
    )?;
    Ok(vec![])
}
fn define_module0(
    this_module_ref: i32,
    _is_open: bool,
    _version_str_ref: i32,
    _location_str_ref: i32,
    _packages_arr_ref: i32,
) -> Result<()> {
    let module_str_ref = Executor::invoke_non_static_method(
        "java/lang/Module",
        "getName:()Ljava/lang/String;",
        this_module_ref,
        &[],
    )?[0];

    if module_str_ref != 0 {
        let module_str = get_utf8_string_by_ref(module_str_ref)?;
        with_method_area(|a| {
            let modules = a.modules();
            let registry = modules.registry();
            if let Some(already_exist) = registry.insert(module_str.clone(), this_module_ref) {
                return Err(Error::new_execution(&format!(
                    "Module {module_str} already defined with ref {already_exist}",
                )));
            }
            Ok::<_, Error>(())
        })?;

        if module_str == "java.base" {
            // patch all previously loaded java.base module Class<?>-es
            let modules = with_method_area(|a| a.modules());
            let base_classes_to_patch = modules.base_classes_to_patch();
            let mut guard = base_classes_to_patch.lock();
            let to_patch = guard.take();
            if let Some(to_patch) = to_patch {
                for obj_ref in to_patch.iter() {
                    HEAP.set_object_field_value(
                        *obj_ref,
                        "java/lang/Class",
                        "module",
                        vec![this_module_ref],
                    )?;
                }
            } else {
                return Err(Error::new_execution("Patching has already been performed"));
            }
        }
    } else {
        return Err(Error::new_execution("Warning: module with null name"));
    }

    // todo: implement me?
    Ok(())
}

pub(crate) fn add_reads0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let module_from_ref = args[0];
    let module_to_ref = args[1];
    add_reads0(module_from_ref, module_to_ref)?;
    Ok(vec![])
}
fn add_reads0(_module_from_ref: i32, _module_to_ref: i32) -> Result<()> {
    // todo: implement me?
    Ok(())
}

pub(crate) fn add_exports_to_all0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let module_ref = args[0];
    let package_name_ref = args[1];
    add_exports_to_all0(module_ref, package_name_ref)?;
    Ok(vec![])
}
fn add_exports_to_all0(_module_ref: i32, _package_name_ref: i32) -> Result<()> {
    // todo: implement me?
    Ok(())
}

pub(crate) fn add_exports0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let module_from_ref = args[0];
    let package_name_ref = args[1];
    let module_to_ref = args[2];
    add_exports0(module_from_ref, package_name_ref, module_to_ref)?;
    Ok(vec![])
}
fn add_exports0(_module_from_ref: i32, _package_name_ref: i32, _module_to_ref: i32) -> Result<()> {
    // todo: implement me?
    Ok(())
}
