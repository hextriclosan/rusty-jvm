use crate::vm::system_native::system::Java_java_lang_System_currentTimeMillis;
use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::LazyLock;

/// Registry of built-in native methods implemented in Rust, keyed by their JNI
/// C-name and resolving to the function address invoked via libffi.
///
/// Built-ins live in the running executable rather than in a loaded shared
/// library, so their address is taken directly here instead of being resolved
/// through the platform loader (`dlsym`/`GetProcAddress`) — the latter cannot
/// see executable symbols on Windows and is fragile on Linux. Dispatch itself is
/// unchanged: the address flows through the very same JNI/libffi path used for
/// functions from real native libraries.
///
/// Entries are described with plain Java coordinates (class, method, descriptor)
/// so there are no hand-written JNI-mangled strings; the lookup keys are derived
/// with [`jniname::c_name`].
static BUILTIN_NATIVE_TABLE: LazyLock<HashMap<String, i64>> = LazyLock::new(|| {
    let mut table = HashMap::new();
    register(
        &mut table,
        "java/lang/System",
        "currentTimeMillis",
        "()J",
        Java_java_lang_System_currentTimeMillis as *const c_void as i64,
    );
    table
});

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
