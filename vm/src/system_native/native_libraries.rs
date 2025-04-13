use crate::system_native::string::get_utf8_string_by_ref;
use tracing::{enabled, trace};

pub(crate) fn find_builtin_lib_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    if enabled!(tracing::Level::TRACE) {
        let name_ref = args[0];
        let name = get_utf8_string_by_ref(name_ref)?;
        trace!("findBuiltinLib: {name}");
    }

    Ok(vec![0]) // we don't have static libraries, so we always return null
}
