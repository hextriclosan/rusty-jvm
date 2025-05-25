use crate::error::Error;
use crate::helper::create_array_of_strings;
use crate::properties::system_properties::{
    OVERRIDDEN_PLATFORM_PROPERTIES, OVERRIDDEN_VM_PROPERTIES,
};

pub(crate) fn platform_properties_wrp(_args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let string_array_ref = platform_properties()?;

    Ok(vec![string_array_ref])
}
fn platform_properties() -> crate::error::Result<i32> {
    create_array_of_strings(&get_platform_properties()?)
}

pub(crate) fn vm_properties_wrp(_args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let string_array_ref = vm_properties()?;

    Ok(vec![string_array_ref])
}
fn vm_properties() -> crate::error::Result<i32> {
    create_array_of_strings(&get_vm_properties()?)
}

fn get_platform_properties() -> crate::error::Result<Vec<String>> {
    Ok(OVERRIDDEN_PLATFORM_PROPERTIES
        .get()
        .ok_or(Error::new_execution(
            "Failed to get OVERRIDDEN_PLATFORM_PROPERTIES",
        ))?
        .iter()
        .map(|(_key, value)| value.clone())
        .collect())
}

fn get_vm_properties() -> crate::error::Result<Vec<String>> {
    Ok(OVERRIDDEN_VM_PROPERTIES
        .get()
        .ok_or(Error::new_execution(
            "Failed to get OVERRIDDEN_VM_PROPERTIES",
        ))?
        .iter()
        .flat_map(|(key, value)| vec![key.clone(), value.clone()])
        .collect())
}
