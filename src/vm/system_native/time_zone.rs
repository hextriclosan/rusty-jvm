use crate::vm::error::Result;
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;

pub(crate) fn get_system_time_zone_id_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let java_home_string_ref = args[0];
    let time_zone_id_string_ref = get_system_time_zone_id(java_home_string_ref)?;

    Ok(vec![time_zone_id_string_ref])
}
// the idea of _java_home_string_ref is to use TZ db from <java_home>/lib/ (we don't do it).
fn get_system_time_zone_id(_java_home_string_ref: i32) -> Result<i32> {
    let time_zone = iana_time_zone::get_timezone()?;
    StringPoolHelper::get_string(&time_zone)
}
