use crate::vm::error::{Error, Result};
use crate::vm::jni::set_pending_internal_error;
use crate::vm::method_area::lookup::lookup_method;
use crate::vm::system_native::object::identity_hashcode;
use crate::vm::system_native::system::{
    arraycopy as arraycopy_impl, current_time_millis, map_library_name, nano_time,
    register_natives, set_err0, set_in0, set_out0,
};
use crate::vm::system_native::zip::crc32::updatebytes0;
#[allow(unused_imports)]
use jni_sys::{
    jarray, jboolean, jbyte, jbyteArray, jchar, jclass, jdouble, jfloat, jint, jlong, jobject,
    jshort, jstring, JNIEnv,
};
use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::LazyLock;

/// Maps a JNI type token to its `extern "system"` FFI parameter/return type.
macro_rules! jni_ffi_ty {
    (boolean) => {
        jboolean
    };
    (byte) => {
        jbyte
    };
    (char) => {
        jchar
    };
    (short) => {
        jshort
    };
    (int) => {
        jint
    };
    (long) => {
        jlong
    };
    (float) => {
        jfloat
    };
    (double) => {
        jdouble
    };
    (byte_array) => {
        jbyteArray
    };
    (array) => {
        jarray
    };
    (object) => {
        jobject
    };
    (input_stream) => {
        jobject
    };
    (print_stream) => {
        jobject
    };
    (string) => {
        jstring
    };
    (void) => {
        ()
    };
}

/// Maps a JNI type token to its JVM descriptor fragment (source of truth for the CIF).
macro_rules! jni_desc_frag {
    (boolean) => {
        "Z"
    };
    (byte) => {
        "B"
    };
    (char) => {
        "C"
    };
    (short) => {
        "S"
    };
    (int) => {
        "I"
    };
    (long) => {
        "J"
    };
    (float) => {
        "F"
    };
    (double) => {
        "D"
    };
    (byte_array) => {
        "[B"
    };
    (object_array) => {
        "[Ljava/lang/Object;"
    };
    (object) => {
        "Ljava/lang/Object;"
    };
    (print_stream) => {
        "Ljava/io/PrintStream;"
    };
    (input_stream) => {
        "Ljava/io/InputStream;"
    };
    (string) => {
        "Ljava/lang/String;"
    };
    (void) => {
        "V"
    };
}

/// Casts an incoming JNI argument to the value type expected by the pure impl.
/// Reference types (`object`, arrays) are carried as the VM's `i32` object ref.
macro_rules! jni_arg_cast {
    (boolean, $e:expr) => {
        $e as i32
    };
    (byte, $e:expr) => {
        $e as i32
    };
    (char, $e:expr) => {
        $e as i32
    };
    (short, $e:expr) => {
        $e as i32
    };
    (int, $e:expr) => {
        $e as i32
    };
    (long, $e:expr) => {
        $e as i64
    };
    (float, $e:expr) => {
        $e as f32
    };
    (double, $e:expr) => {
        $e as f64
    };
    (byte_array, $e:expr) => {
        $e as i32
    };
    (object_array, $e:expr) => {
        $e as i32
    };
    (object, $e:expr) => {
        $e as i32
    };
    (print_stream, $e:expr) => {
        $e as i32
    };
    (input_stream, $e:expr) => {
        $e as i32
    };
    (string, $e:expr) => {
        $e as i32
    };
}

/// Converts the pure impl's success value into the wrapper's JNI return type.
/// Scalars map directly; reference types are the VM's `i32` object ref and must be
/// widened back into the JNI pointer handle expected by the FFI return slot.
macro_rules! jni_ret_conv {
    (void, $e:expr) => {
        $e
    };
    (boolean, $e:expr) => {
        $e as jboolean
    };
    (byte, $e:expr) => {
        $e as jbyte
    };
    (char, $e:expr) => {
        $e as jchar
    };
    (short, $e:expr) => {
        $e as jshort
    };
    (int, $e:expr) => {
        $e as jint
    };
    (long, $e:expr) => {
        $e as jlong
    };
    (float, $e:expr) => {
        $e as jfloat
    };
    (double, $e:expr) => {
        $e as jdouble
    };
    (byte_array, $e:expr) => {
        $e as usize as jbyteArray
    };
    (array, $e:expr) => {
        $e as usize as jarray
    };
    (object, $e:expr) => {
        $e as usize as jobject
    };
    (string, $e:expr) => {
        $e as usize as jstring
    };
}

/// The zero/null value of the wrapper's JNI return type, returned after an error is
/// recorded as a pending exception (never panics across the FFI boundary).
macro_rules! jni_ret_default {
    (void) => {
        ()
    };
    (boolean) => {
        0
    };
    (byte) => {
        0
    };
    (char) => {
        0
    };
    (short) => {
        0
    };
    (int) => {
        0
    };
    (long) => {
        0
    };
    (float) => {
        0.0
    };
    (double) => {
        0.0
    };
    (byte_array) => {
        std::ptr::null_mut()
    };
    (array) => {
        std::ptr::null_mut()
    };
    (object) => {
        std::ptr::null_mut()
    };
    (string) => {
        std::ptr::null_mut()
    };
}

/// Generates a single `extern "system"` wrapper that adapts a pure impl to JNI.
///
/// The wrapper's signature is derived entirely from the same type tokens used to
/// build the JVM descriptor, so the descriptor and the Rust signature can never
/// drift apart, and the call to `$inner` is type-checked against those tokens.
macro_rules! builtin_wrapper {
    // Static native: 2nd JNI arg is the `jclass` and is ignored.
    (static $name:ident ( $($pname:ident : $pty:ident),* ) -> $ret:ident => $inner:path) => {
        #[allow(non_snake_case)]
        extern "system" fn $name(
            _env: *mut JNIEnv,
            _class: jclass,
            $($pname: jni_ffi_ty!($pty),)*
        ) -> jni_ffi_ty!($ret) {
            match $inner($(jni_arg_cast!($pty, $pname)),*) {
                Ok(__value) => jni_ret_conv!($ret, __value),
                Err(__error) => {
                    set_pending_internal_error(&__error.to_string());
                    jni_ret_default!($ret)
                }
            }
        }
    };
    // Instance native: 2nd JNI arg is the receiver, passed as the first impl arg.
    (instance $name:ident ( $($pname:ident : $pty:ident),* ) -> $ret:ident => $inner:path) => {
        #[allow(non_snake_case)]
        extern "system" fn $name(
            _env: *mut JNIEnv,
            this: jobject,
            $($pname: jni_ffi_ty!($pty),)*
        ) -> jni_ffi_ty!($ret) {
            match $inner(this as i32 $(, jni_arg_cast!($pty, $pname))*) {
                Ok(__value) => jni_ret_conv!($ret, __value),
                Err(__error) => {
                    set_pending_internal_error(&__error.to_string());
                    jni_ret_default!($ret)
                }
            }
        }
    };
}

/// Declaratively defines the built-in native registry from a single source of
/// truth. For every entry it generates a type-safe `extern "system"` wrapper and
/// registers its address under the JNI long (descriptor-mangled) C-name.
///
/// Grammar: `"<class>": <static|instance> fn <method>(<name>: <type>, ...) -> <type> => <impl>`
macro_rules! builtin_natives {
    (
        $(
            $class:literal : $recv:ident fn $method:ident ( $($pname:ident : $pty:ident),* $(,)? ) -> $ret:ident => $inner:path
        );* $(;)?
    ) => {
        $(
            builtin_wrapper!($recv $method ( $($pname : $pty),* ) -> $ret => $inner);
        )*

        static BUILTIN_NATIVE_TABLE: LazyLock<HashMap<String, i64>> = LazyLock::new(|| {
            let mut table = HashMap::new();
            $(
                register(
                    &mut table,
                    $class,
                    stringify!($method),
                    &build_descriptor(&[$(jni_desc_frag!($pty)),*], jni_desc_frag!($ret)),
                    $method as *const c_void as usize as i64,
                );
            )*
            table
        });

        /// The declared `(class, method, descriptor)` of every built-in native, used
        /// by [`validate_builtin_natives`] to check each declaration against the real
        /// JDK method contract. This is the only thing that can catch a self-consistent
        /// but wrong descriptor (e.g. co-editing the tokens and the impl together),
        /// which no Rust compile-time check can detect since the contract lives in
        /// JDK bytecode.
        static BUILTIN_SPECS: LazyLock<Vec<(&'static str, &'static str, String)>> =
            LazyLock::new(|| {
                vec![
                    $((
                        $class,
                        stringify!($method),
                        build_descriptor(&[$(jni_desc_frag!($pty)),*], jni_desc_frag!($ret)),
                    )),*
                ]
            });
    };
}

// Registry of built-in native methods implemented in Rust, keyed by their JNI
// C-name and resolving to the function address invoked via libffi.
builtin_natives! {
    "java/lang/System": static fn currentTimeMillis() -> long => current_time_millis;
    "java/lang/System": static fn nanoTime() -> long => nano_time;
    "java/lang/System": static fn arraycopy(src: object, src_pos: int, dest: object, dest_pos: int, length: int) -> void => arraycopy_impl;
    "java/lang/System": static fn setIn0(input_stream: input_stream) -> void => set_in0;
    "java/lang/System": static fn setOut0(print_stream: print_stream) -> void => set_out0;
    "java/lang/System": static fn setErr0(print_stream: print_stream) -> void => set_err0;
    "java/lang/System": static fn identityHashCode(obj: object) -> int => identity_hashcode;
    "java/lang/System": static fn mapLibraryName(lib: string) -> string => map_library_name;
    "java/lang/System": static fn registerNatives() -> void => register_natives;

    "java/lang/Object": instance fn hashCode() -> int => identity_hashcode;
    "java/util/zip/CRC32": static fn updateBytes0(crc: int, b: byte_array, off: int, len: int) -> int => updatebytes0;
}

fn build_descriptor(params: &[&str], ret: &str) -> String {
    format!("({}){}", params.concat(), ret)
}

fn register(
    table: &mut HashMap<String, i64>,
    class_name: &str,
    method_name: &str,
    descriptor: &str,
    address: i64,
) {
    let (_short_name, long_name) = jniname::c_name(class_name, method_name, descriptor)
        .expect("built-in native must have a valid class/method/descriptor");
    table.insert(long_name, address);
}

pub(super) fn find_builtin_native(long_name: &str) -> Option<i64> {
    BUILTIN_NATIVE_TABLE.get(long_name).copied()
}

/// Verifies that every built-in native declared in [`builtin_natives!`] matches a
/// real, `native` method of its JDK class, with the exact declared descriptor.
///
/// The `builtin_natives!` macro only guarantees the *descriptor tokens* and the
/// Rust *impl* signature agree with each other; it cannot know the real Java
/// method contract, which lives in JDK bytecode. So editing the tokens and the
/// impl together (e.g. `len: int`/`i32` -> `len: float`/`f32`) stays
/// self-consistent and compiles, yet no longer matches `CRC32.updateBytes0`.
///
/// This runs against loaded JDK classes and turns such a mistake into a clear,
/// deterministic error instead of a silent resolution failure at first use.
pub(crate) fn validate_builtin_natives() -> Result<()> {
    for (class_name, method_name, descriptor) in BUILTIN_SPECS.iter() {
        let signature = format!("{method_name}:{descriptor}");
        match lookup_method(class_name, &signature)? {
            Some(method) if method.is_native() => {}
            Some(_) => {
                return Err(Error::new_native(&format!(
                    "Built-in native '{class_name}.{signature}' resolves to a non-native JDK \
                     method; the declared descriptor likely does not match the real method"
                )));
            }
            None => {
                return Err(Error::new_native(&format!(
                    "Built-in native '{class_name}.{signature}' does not match any method of the \
                     JDK class '{class_name}'; the declared descriptor is wrong"
                )));
            }
        }
    }
    Ok(())
}
