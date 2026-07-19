use crate::vm::error::Result;
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;

// the idea of _java_home_string_ref is to use TZ db from <java_home>/lib/ (we don't do it).
/// `java.util.TimeZone.getSystemTimeZoneID(Ljava/lang/String;)Ljava/lang/String;`
pub(crate) fn get_system_time_zone_id(_java_home_string_ref: i32) -> Result<i32> {
    match iana_time_zone::get_timezone() {
        Ok(time_zone) => StringPoolHelper::get_string(&time_zone),
        Err(_) => Ok(0),
    }
}
