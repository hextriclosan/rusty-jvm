use crate::vm::system_native::object::object_hashcode_wrp;
use crate::vm::system_native::system::current_time_millis_wrp;
use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::LazyLock;

/// Declaratively builds the built-in native registry.
///
/// Each entry maps a Java method — written as `class, method, descriptor` — to
/// its Rust JNI function. The JNI-mangled lookup keys (simple + overloaded) are
/// derived automatically via [`jniname::c_name`], so there are no hand-written
/// mangled strings. To migrate a method off `SYSTEM_NATIVE_TABLE`, add one line
/// here.
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
/// C-name and resolving to the function address invoked via libffi.
///
/// Built-ins live in the running executable rather than in a loaded shared
/// library, so their address is taken directly here instead of being resolved
/// through the platform loader (`dlsym`/`GetProcAddress`) — the latter cannot
/// see executable symbols on Windows and is fragile on Linux. Dispatch itself is
/// unchanged: the address flows through the very same JNI/libffi path used for
/// functions from real native libraries.
static BUILTIN_NATIVE_TABLE: LazyLock<HashMap<String, i64>> = builtin_natives! {
    "java/lang/System", "currentTimeMillis", "()J" => current_time_millis_wrp;
    "java/lang/Object", "hashCode",          "()I" => object_hashcode_wrp;
};

/// Registers a built-in native under both its simple and overloaded JNI names,
/// derived from the Java method coordinates.
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

/// Resolves a built-in native by its JNI C-names, preferring the overloaded
/// (long) name over the simple (short) name, matching JNI resolution order.
///
/// Returns the function address to be invoked via libffi, or `None` when the
/// method is not a built-in native.
pub(super) fn find_builtin_native(short_name: &str, long_name: &str) -> Option<i64> {
    BUILTIN_NATIVE_TABLE
        .get(long_name)
        .or_else(|| BUILTIN_NATIVE_TABLE.get(short_name))
        .copied()
}
