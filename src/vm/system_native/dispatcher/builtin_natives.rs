use crate::vm::system_native::object::object_hashcode_wrp;
use crate::vm::system_native::system::current_time_millis_wrp;
use crate::vm::system_native::zip::crc32::java_util_zip_crc32_updatebytes0_wrp;
use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::LazyLock;

/// Declaratively builds the built-in native registry
macro_rules! builtin_natives {
    ($($class:literal, $method:literal, $descriptor:literal => $func:path);* $(;)?) => {
        LazyLock::new(|| {
            let mut table = HashMap::new();
            $(
                register(
                    &mut table,
                    $class,
                    $method,
                    $descriptor,
                    $func as *const c_void as usize as i64,
                );
            )*
            table
        })
    };
}

/// Registry of built-in native methods implemented in Rust, keyed by their JNI
/// C-name and resolving to the function address invoked via libffi
static BUILTIN_NATIVE_TABLE: LazyLock<HashMap<String, i64>> = builtin_natives! {
    "java/lang/System", "currentTimeMillis", "()J" => current_time_millis_wrp;
    "java/lang/Object", "hashCode",          "()I" => object_hashcode_wrp;
    "java/util/zip/CRC32", "updateBytes0", "(I[BII)I" => java_util_zip_crc32_updatebytes0_wrp;
};

fn register(
    table: &mut HashMap<String, i64>,
    class_name: &str,
    method_name: &str,
    descriptor: &str,
    address: i64,
) {
    let (short_name, long_name) = jniname::c_name(class_name, method_name, descriptor)
        .expect("built-in native must have a valid class/method/descriptor");
    table.insert(short_name, address);
    table.insert(long_name, address);
}

pub(super) fn find_builtin_native(short_name: &str, long_name: &str) -> Option<i64> {
    BUILTIN_NATIVE_TABLE
        .get(long_name)
        .or_else(|| BUILTIN_NATIVE_TABLE.get(short_name))
        .copied()
}
