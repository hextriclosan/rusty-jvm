use crate::vm::error::{Error, Result};
use crate::vm::jni::set_pending_internal_error;
use crate::vm::method_area::lookup::lookup_method;
use crate::vm::system_native::class::{
    class_is_instance, desired_assertion_status0, for_name0, get_constant_pool,
    get_declared_constructors0, get_declared_fields0, get_declared_methods0, get_declaring_class0,
    get_enclosing_method0, get_interfaces0, get_nest_host0, get_primitive_class,
    get_raw_annotations, get_simple_binary_name0, get_superclass, init_class_name,
    is_assignable_from, is_hidden, is_record0, register_natives as register_natives_class,
};
use crate::vm::system_native::module::{
    add_exports0, add_exports_to_all0, add_reads0, define_module0,
};
use crate::vm::system_native::object::{
    clone as clone_object, get_class, identity_hashcode, notify_all,
};
use crate::vm::system_native::shutdown::{before_halt, halt0 as halt0_impl};
use crate::vm::system_native::system::{
    arraycopy as arraycopy_impl, current_time_millis, map_library_name, nano_time,
    register_natives as register_natives_system, set_err0, set_in0, set_out0,
};
use crate::vm::system_native::zip::crc32::updatebytes0;
#[allow(unused_imports)]
use jni_sys::{
    jarray, jboolean, jbyte, jbyteArray, jchar, jclass, jdouble, jfloat, jint, jlong, jobject,
    jobjectArray, jshort, jstring, JNIEnv,
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
    (object_array) => {
        jobjectArray
    };
    (class_object_array) => {
        jobjectArray
    };
    (field_object_array) => {
        jobjectArray
    };
    (method_object_array) => {
        jobjectArray
    };
    (constructor_object_array) => {
        jobjectArray
    };
    (object) => {
        jobject
    };
    (class) => {
        jclass
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
    (class_loader) => {
        jobject
    };
    (constant_pool) => {
        jobject
    };
    (module) => {
        jobject
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
    (class_object_array) => {
        "[Ljava/lang/Class;"
    };
    (field_object_array) => {
        "[Ljava/lang/reflect/Field;"
    };
    (method_object_array) => {
        "[Ljava/lang/reflect/Method;"
    };
    (constructor_object_array) => {
        "[Ljava/lang/reflect/Constructor;"
    };
    (object) => {
        "Ljava/lang/Object;"
    };
    (class) => {
        "Ljava/lang/Class;"
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
    (class_loader) => {
        "Ljava/lang/ClassLoader;"
    };
    (constant_pool) => {
        "Ljdk/internal/reflect/ConstantPool;"
    };
    (module) => {
        "Ljava/lang/Module;"
    };
    (void) => {
        "V"
    };
}

/// Casts an incoming JNI argument to the value type expected by the pure impl.
/// Reference types (`object`, arrays) are carried as the VM's `i32` object ref.
macro_rules! jni_arg_cast {
    (boolean, $e:expr) => {
        $e as i32 != 0
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
    (class_object_array, $e:expr) => {
        $e as i32
    };
    (field_object_array, $e:expr) => {
        $e as i32
    };
    (method_object_array, $e:expr) => {
        $e as i32
    };
    (constructor_object_array, $e:expr) => {
        $e as i32
    };
    (object, $e:expr) => {
        $e as i32
    };
    (class, $e:expr) => {
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
    (class_loader, $e:expr) => {
        $e as i32
    };
    (constant_pool, $e:expr) => {
        $e as i32
    };
    (module, $e:expr) => {
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
    (object_array, $e:expr) => {
        $e as usize as jobjectArray
    };
    (class_object_array, $e:expr) => {
        $e as usize as jobjectArray
    };
    (field_object_array, $e:expr) => {
        $e as usize as jobjectArray
    };
    (method_object_array, $e:expr) => {
        $e as usize as jobjectArray
    };
    (constructor_object_array, $e:expr) => {
        $e as usize as jobjectArray
    };
    (object, $e:expr) => {
        $e as usize as jobject
    };
    (class, $e:expr) => {
        $e as usize as jclass
    };
    (string, $e:expr) => {
        $e as usize as jstring
    };
    (class_loader, $e:expr) => {
        $e as usize as jobject
    };
    (constant_pool, $e:expr) => {
        $e as usize as jobject
    };
    (module, $e:expr) => {
        $e as usize as jobject
    }
}

/// The zero/null value of the wrapper's JNI return type, returned after an error is
/// recorded as a pending exception (never panics across the FFI boundary).
macro_rules! jni_ret_default {
    (void) => {
        ()
    };
    (boolean) => {
        false
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
    (object_array) => {
        std::ptr::null_mut()
    };
    (class_object_array) => {
        std::ptr::null_mut()
    };
    (field_object_array) => {
        std::ptr::null_mut()
    };
    (method_object_array) => {
        std::ptr::null_mut()
    };
    (constructor_object_array) => {
        std::ptr::null_mut()
    };
    (object) => {
        std::ptr::null_mut()
    };
    (class) => {
        std::ptr::null_mut()
    };
    (string) => {
        std::ptr::null_mut()
    };
    (class_loader) => {
        std::ptr::null_mut()
    };
    (constant_pool) => {
        std::ptr::null_mut()
    };
    (module) => {
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
        static BUILTIN_NATIVE_TABLE: LazyLock<HashMap<String, i64>> = LazyLock::new(|| {
            let mut table = HashMap::new();
            $(
                {
                    // Generate the wrapper in its own scope so that identical method
                    // names across different classes (e.g. `registerNatives`) don't
                    // collide at module level.
                    builtin_wrapper!($recv $method ( $($pname : $pty),* ) -> $ret => $inner);
                    register(
                        &mut table,
                        $class,
                        stringify!($method),
                        &build_descriptor(&[$(jni_desc_frag!($pty)),*], jni_desc_frag!($ret)),
                        $method as *const c_void as usize as i64,
                    );
                }
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
    "java/lang/System": static fn setIn0(input_stream: input_stream) -> void => set_in0; // todo: implement me
    "java/lang/System": static fn setOut0(print_stream: print_stream) -> void => set_out0;
    "java/lang/System": static fn setErr0(print_stream: print_stream) -> void => set_err0;
    "java/lang/System": static fn identityHashCode(obj: object) -> int => identity_hashcode;
    "java/lang/System": static fn mapLibraryName(lib: string) -> string => map_library_name;
    "java/lang/System": static fn registerNatives() -> void => register_natives_system; // todo: implement me

    "java/lang/Object": instance fn hashCode() -> int => identity_hashcode;
    "java/lang/Object": instance fn getClass() -> class => get_class;
    "java/lang/Object": instance fn clone() -> object => clone_object;
    "java/lang/Object": instance fn notifyAll() -> void => notify_all; // todo: implement me

    "java/lang/Class": instance fn getSuperclass() -> class => get_superclass;
    "java/lang/Class": static fn getPrimitiveClass(content: string) -> class => get_primitive_class;
    "java/lang/Class": static fn desiredAssertionStatus0(clazz: class) -> boolean => desired_assertion_status0; // setting all classes to have assertions enabled. todo: implement -ea and -da flags
    "java/lang/Class": static fn forName0(name: string, initialize: boolean, loader: class_loader, caller: class) -> class => for_name0;
    "java/lang/Class": static fn registerNatives() -> void => register_natives_class; // todo: implement me
    "java/lang/Class": instance fn initClassName() -> string => init_class_name;
    "java/lang/Class": instance fn getInterfaces0() -> class_object_array => get_interfaces0;
    "java/lang/Class": instance fn getDeclaringClass0() -> class => get_declaring_class0;
    "java/lang/Class": instance fn getDeclaredFields0(flag: boolean) -> field_object_array => get_declared_fields0;
    "java/lang/Class": instance fn getDeclaredMethods0(flag: boolean) -> method_object_array => get_declared_methods0;
    "java/lang/Class": instance fn getDeclaredConstructors0(flag: boolean) -> constructor_object_array => get_declared_constructors0;
    "java/lang/Class": instance fn getRawAnnotations() -> byte_array => get_raw_annotations;
    "java/lang/Class": instance fn getEnclosingMethod0() -> object_array => get_enclosing_method0;
    "java/lang/Class": instance fn getSimpleBinaryName0() -> string => get_simple_binary_name0;
    "java/lang/Class": instance fn isAssignableFrom(clazz: class) -> boolean => is_assignable_from;
    "java/lang/Class": instance fn isHidden() -> boolean => is_hidden;  // we are treating all classes as non-hidden since we don't have a way to mark them as hidden (yet)
    "java/lang/Class": instance fn isInstance(obj: object) -> boolean => class_is_instance;
    "java/lang/Class": instance fn getConstantPool() -> constant_pool => get_constant_pool;
    "java/lang/Class": instance fn getNestHost0() -> class => get_nest_host0;
    "java/lang/Class": instance fn isRecord0() -> boolean => is_record0;

    "java/lang/Module": static fn addReads0(from: module, to: module) -> void => add_reads0; // todo: implement me?
    "java/lang/Module": static fn defineModule0(module: module, is_open: boolean, version: string, location: string, pns: object_array) -> void => define_module0;
    "java/lang/Module": static fn addExportsToAll0(from: module, pn: string) -> void => add_exports_to_all0; // todo: implement me?
    "java/lang/Module": static fn addExports0(from: module, pn: string, to: module) -> void => add_exports0; // todo: implement me?

    "java/lang/Shutdown": static fn beforeHalt() -> void => before_halt; // todo: implement me
    "java/lang/Shutdown": static fn halt0(status: int) -> void => halt0_impl; // fixme: by doing this we skip destructors and other shutdown hooks, later it might be an issue

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
