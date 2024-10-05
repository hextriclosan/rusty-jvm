use crate::error::Error;
use crate::system_native::system::current_time_millis_wrp;
use once_cell::sync::Lazy;
use std::collections::HashMap;

static SYSTEM_NATIVE_TABLE: Lazy<HashMap<&'static str, fn(&[i32]) -> Vec<i32>>> =
    Lazy::new(|| {
        let mut table = HashMap::new();
        table.insert(
            "java/lang/System:currentTimeMillis:()J",
            current_time_millis_wrp as fn(&[i32]) -> Vec<i32>,
        );

        table
    });

pub(crate) fn invoke_native_method(
    method_signature: &str,
    args: &[i32],
) -> crate::error::Result<Vec<i32>> {
    let native_method = SYSTEM_NATIVE_TABLE.get(method_signature).ok_or_else(|| {
        Error::new_native(&format!("Native method {method_signature} not found"))
    })?;

    let result = native_method(args);

    Ok(result)
}
