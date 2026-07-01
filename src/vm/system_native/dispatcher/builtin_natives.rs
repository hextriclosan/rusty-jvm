use crate::vm::system_native::system::Java_java_lang_System_currentTimeMillis;
#[cfg(unix)]
use libloading::os::unix::{Library, Symbol};
#[cfg(windows)]
use libloading::os::windows::{Library, Symbol};
use std::ffi::{c_void, CString};
use std::sync::LazyLock;

/// The running executable itself, viewed as a "native library".
///
/// Built-in native methods are ordinary exported symbols (`#[no_mangle]`), so
/// they can be resolved by name straight from the current process image, just
/// like functions from a loaded shared library. `Library::this` uses the
/// platform loader (`dlsym`/`GetProcAddress`) under the hood.
static BUILTIN_LIBRARY: LazyLock<Library> = LazyLock::new(open_self);

#[cfg(unix)]
fn open_self() -> Library {
    Library::this()
}

#[cfg(windows)]
fn open_self() -> Library {
    Library::this().expect("failed to load self-library")
}

/// Keeps built-in natives in the executable's symbol table.
///
/// `#[no_mangle]` alone does not guarantee the symbol survives dead-code
/// elimination when nothing references it directly; referencing every built-in
/// here keeps them present so `dlsym`/`GetProcAddress` can find them by name.
#[used]
static KEEP_ALIVE: [unsafe extern "system" fn(
    *mut jni_sys::JNIEnv,
    jni_sys::jclass,
) -> jni_sys::jlong; 1] = [Java_java_lang_System_currentTimeMillis];

/// Resolves a built-in native by its JNI C-names, preferring the overloaded
/// (long) name over the simple (short) name, matching JNI resolution order.
///
/// Returns the function address to be invoked via libffi, or `None` when the
/// method is not a built-in native.
pub(super) fn find_builtin_native(short_name: &str, long_name: &str) -> Option<i64> {
    resolve_symbol(long_name).or_else(|| resolve_symbol(short_name))
}

fn resolve_symbol(name: &str) -> Option<i64> {
    let c_name = CString::new(name).ok()?;
    let symbol: Result<Symbol<*const c_void>, libloading::Error> =
        unsafe { BUILTIN_LIBRARY.get(c_name.as_bytes_with_nul()) };
    let address = symbol.ok()?.as_raw_ptr() as i64;
    (address != 0).then_some(address)
}
