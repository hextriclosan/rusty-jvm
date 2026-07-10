use crate::vm::error::{Error, Result};
use crate::vm::jni::set_pending_internal_error;
use crate::vm::method_area::lookup::lookup_method;
use crate::vm::system_native::cds::{
    get_cds_config_status, get_random_seed_for_dumping, initialize_from_archive,
};
use crate::vm::system_native::class::{
    class_is_instance, desired_assertion_status0, for_name0, get_constant_pool,
    get_declared_constructors0, get_declared_fields0, get_declared_methods0, get_declaring_class0,
    get_enclosing_method0, get_interfaces0, get_nest_host0, get_primitive_class,
    get_raw_annotations, get_simple_binary_name0, get_superclass, init_class_name,
    is_assignable_from, is_hidden, is_record0, register_natives as register_natives_class,
};
use crate::vm::system_native::double::{double_to_raw_long_bits, long_bits_to_double};
use crate::vm::system_native::file_descriptor::{
    file_descriptor_close0, get_append, get_handle, init_ids as init_ids_file_descriptor,
};
use crate::vm::system_native::file_input_stream::{
    available0 as available0_file_input_stream, init_ids as init_ids_file_input_stream,
    is_regular_file0 as is_regular_file0_file_input_stream, length0 as length0_file_input_stream,
    open0 as open0_file_input_stream, position0 as position0_file_input_stream,
    read0 as read0_file_input_stream, read_bytes as read_bytes_file_input_stream,
};
use crate::vm::system_native::file_output_stream::{
    init_ids as init_ids_file_output_stream, open0 as open0_file_output_stream,
    write as write_file_output_stream, write_bytes as write_bytes_file_output_stream,
};
use crate::vm::system_native::file_system::{
    canonicalize0 as canonicalize0_file_system, check_access0, create_file_exclusively0,
    delete0 as delete_impl, get_boolean_attributes0, get_length0,
    init_ids as init_ids_file_system,
};
use crate::vm::system_native::float::float_to_raw_int_bits;
use crate::vm::system_native::module::{
    add_exports0, add_exports_to_all0, add_reads0, define_module0,
};
use crate::vm::system_native::native_image_buffer::get_native_map;
use crate::vm::system_native::object::{
    clone as clone_object, get_class, identity_hashcode, notify_all,
};
use crate::vm::system_native::random_access_file::{
    init_ids as init_ids_random_access_file, length0 as length0_random_access_file,
    open0_random_access_file, read_bytes0 as read_bytes0_random_access_file,
    seek0 as seek0_random_access_file, write_bytes0 as write_bytes0_random_access_file,
};
use crate::vm::system_native::runtime::{available_processors, max_memory};
use crate::vm::system_native::shutdown::{before_halt, halt0 as halt0_impl};
use crate::vm::system_native::string::intern as intern_string;
use crate::vm::system_native::system::{
    arraycopy as arraycopy_impl, current_time_millis, map_library_name, nano_time,
    register_natives as register_natives_system, set_err0, set_in0, set_out0,
};
use crate::vm::system_native::system_props_raw::{platform_properties, vm_properties};
use crate::vm::system_native::unsafe_::{
    allocate_memory0, array_base_offset0, array_index_scale0, compare_and_exchange_long,
    compare_and_set_int, compare_and_set_long, compare_and_set_reference, copy_memory0,
    copy_swap_memory0, ensure_class_initialized0, full_fence, get_boolean_volatile, get_byte,
    get_char, get_int, get_int_volatile, get_long, get_long_volatile, get_reference,
    get_reference_volatile, get_short, object_field_offset0, object_field_offset1, put_byte,
    put_char, put_int, put_int_volatile, put_long, put_reference, put_reference_volatile,
    put_short, register_natives as register_natives_unsafe, set_memory0, should_be_initialized0,
    static_field_base0, static_field_offset0,
};
use crate::vm::system_native::vm::initialize as initialize_vm;
use crate::vm::system_native::zip::crc32::updatebytes0;
#[allow(unused_imports)]
use jni_sys::{
    jarray, jboolean, jbyte, jbyteArray, jchar, jclass, jdouble, jfloat, jint, jlong, jobject,
    jobjectArray, jshort, jstring, JNIEnv,
};
use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::LazyLock;
// ---------------------------------------------------------------------------
// JNI type-token mapping
//
// Every built-in native parameter/return type is described by a single token
// (e.g. `int`, `object`, `string`). All knowledge about a token lives in ONE
// place: the `jni_types!` table below. From that table we generate the five
// mapping macros used by `builtin_wrapper!`:
//
//   * jni_ffi_ty!(tok)          -> the `extern "system"` FFI param/return type
//   * jni_desc_frag!(tok)       -> the JVM descriptor fragment (CIF source)
//   * jni_arg_cast!(tok, e)     -> cast an incoming JNI arg to the impl's type
//   * jni_ret_conv!(tok, e)     -> convert the impl's result to the FFI return
//   * jni_ret_default!(tok)     -> the zero/null FFI value used after an error
//
// Each token belongs to a `kind` (bool/int/float/ref/void) that fully
// determines the arg-cast / ret-conv / ret-default shapes; the only per-token
// data is the VM-side Rust type, the FFI type, and the descriptor fragment.
// This makes adding a type a single line and keeps the five mappings from ever
// drifting apart.
// ---------------------------------------------------------------------------

/// Per-kind argument cast: reference types are carried as the VM's `i32`
/// object ref; scalars are cast to their VM-side value type.
macro_rules! jni_arg_cast_impl {
    (bool, $rust:ty, $e:expr) => {
        $e as i32 != 0
    };
    (int, $rust:ty, $e:expr) => {
        $e as $rust
    };
    (float, $rust:ty, $e:expr) => {
        $e as $rust
    };
    (ref, $rust:ty, $e:expr) => {
        $e as i32
    };
    (void, $rust:ty, $e:expr) => {
        compile_error!("`void` cannot be used as an argument")
    };
}

/// Per-kind return conversion: reference types are the VM's `i32` object ref
/// and must be widened back into the JNI pointer handle; scalars map directly.
macro_rules! jni_ret_conv_impl {
    (void, $ffi:ty, $e:expr) => {
        $e
    };
    (bool, $ffi:ty, $e:expr) => {
        $e as $ffi
    };
    (int, $ffi:ty, $e:expr) => {
        $e as $ffi
    };
    (float, $ffi:ty, $e:expr) => {
        $e as $ffi
    };
    (ref, $ffi:ty, $e:expr) => {
        $e as usize as $ffi
    };
}

/// Per-kind zero/null value returned after an error is recorded as a pending
/// exception (never panics across the FFI boundary).
macro_rules! jni_ret_default_impl {
    (void, $ffi:ty) => {
        ()
    };
    (bool, $ffi:ty) => {
        false
    };
    (int, $ffi:ty) => {
        0
    };
    (float, $ffi:ty) => {
        0.0
    };
    (ref, $ffi:ty) => {
        std::ptr::null_mut()
    };
}

/// Generates the five `jni_*` mapping macros from a single type table.
///
/// `$dol` is the literal `$` token, threaded in so the *generated* macros can
/// declare their own `$e:expr` metavariable (the classic nested-`macro_rules!`
/// workaround on stable Rust).
macro_rules! jni_types {
    (
        $dol:tt
        $(
            $token:ident : $kind:ident | $rust:ty | $ffi:ty | $desc:literal
        );* $(;)?
    ) => {
        /// Maps a JNI type token to its `extern "system"` FFI param/return type.
        macro_rules! jni_ffi_ty {
            $( ($token) => { $ffi }; )*
        }
        /// Maps a JNI type token to its JVM descriptor fragment (CIF source of truth).
        macro_rules! jni_desc_frag {
            $( ($token) => { $desc }; )*
        }
        /// Casts an incoming JNI argument to the value type expected by the pure impl.
        macro_rules! jni_arg_cast {
            $( ($token, $dol e:expr) => { jni_arg_cast_impl!($kind, $rust, $dol e) }; )*
        }
        /// Converts the pure impl's success value into the wrapper's JNI return type.
        macro_rules! jni_ret_conv {
            $( ($token, $dol e:expr) => { jni_ret_conv_impl!($kind, $ffi, $dol e) }; )*
        }
        /// The zero/null value of the wrapper's JNI return type, used after an error.
        macro_rules! jni_ret_default {
            $( ($token) => { jni_ret_default_impl!($kind, $ffi) }; )*
        }
    };
}

// Single source of truth: `token : kind | vm-rust-type | ffi-type | descriptor`.
jni_types! {
    $
    boolean                   : bool  | bool | jboolean     | "Z";
    byte                      : int   | i8   | jbyte        | "B";
    char                      : int   | u16  | jchar        | "C";
    short                     : int   | i16  | jshort       | "S";
    int                       : int   | i32  | jint         | "I";
    long                      : int   | i64  | jlong        | "J";
    float                     : float | f32  | jfloat       | "F";
    double                    : float | f64  | jdouble      | "D";
    byte_array                : ref   | i32  | jbyteArray   | "[B";
    object_array              : ref   | i32  | jobjectArray | "[Ljava/lang/Object;";
    class_object_array        : ref   | i32  | jobjectArray | "[Ljava/lang/Class;";
    field_object_array        : ref   | i32  | jobjectArray | "[Ljava/lang/reflect/Field;";
    method_object_array       : ref   | i32  | jobjectArray | "[Ljava/lang/reflect/Method;";
    constructor_object_array  : ref   | i32  | jobjectArray | "[Ljava/lang/reflect/Constructor;";
    string_array              : ref   | i32  | jobjectArray | "[Ljava/lang/String;";
    object                    : ref   | i32  | jobject      | "Ljava/lang/Object;";
    class                     : ref   | i32  | jclass       | "Ljava/lang/Class;";
    input_stream              : ref   | i32  | jobject      | "Ljava/io/InputStream;";
    print_stream              : ref   | i32  | jobject      | "Ljava/io/PrintStream;";
    string                    : ref   | i32  | jstring      | "Ljava/lang/String;";
    class_loader              : ref   | i32  | jobject      | "Ljava/lang/ClassLoader;";
    constant_pool             : ref   | i32  | jobject      | "Ljdk/internal/reflect/ConstantPool;";
    module                    : ref   | i32  | jobject      | "Ljava/lang/Module;";
    field                     : ref   | i32  | jobject      | "Ljava/lang/reflect/Field;";
    byte_buffer               : ref   | i32  | jobject      | "Ljava/nio/ByteBuffer;";
    file_descriptor           : ref   | i32  | jobject      | "Ljava/io/FileDescriptor;";
    file                      : ref   | i32  | jobject      | "Ljava/io/File;";
    void                      : void  | ()   | ()           | "V";
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
/// The entries are organized into one or more **braced groups**. A group may
/// carry outer attributes (e.g. `#[cfg(windows)]`) that apply to *every* entry
/// inside it, so platform-specific natives can be gated once instead of per line:
///
/// ```ignore
/// builtin_natives! {
///     {
///         // common, always-compiled entries
///         "java/lang/System": static fn nanoTime() -> long => nano_time;
///     }
///
///     #[cfg(windows)]
///     {
///         "java/io/WinNTFileSystem": instance fn getLength0(file: file) -> long => get_length0;
///     }
/// }
/// ```
///
/// Individual entries may still carry their own attributes.
///
/// Entry grammar: `"<class>": <static|instance> fn <method>(<name>: <type>, ...) -> <type> => <impl>`
macro_rules! builtin_natives {
    (
        $(
            $(#[$gmeta:meta])*
            {
                $(
                    $(#[$imeta:meta])*
                    $class:literal : $recv:ident fn $method:ident ( $($pname:ident : $pty:ident),* $(,)? ) -> $ret:ident => $inner:path
                );* $(;)?
            }
        )*
    ) => {
        static BUILTIN_NATIVE_TABLE: LazyLock<HashMap<String, i64>> = LazyLock::new(|| {
            let mut table = HashMap::new();
            $(
                // Group-level attributes (e.g. `#[cfg(windows)]`) gate the whole block.
                $(#[$gmeta])*
                {
                    $(
                        // Per-entry attributes gate a single native.
                        $(#[$imeta])*
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
                let mut specs: Vec<(&'static str, &'static str, String)> = Vec::new();
                $(
                    // Apply the same cfg gating as the native table so that
                    // platform-specific specs (e.g. `WinNTFileSystem` vs
                    // `UnixFileSystem`) are only validated on their platform.
                    $(#[$gmeta])*
                    {
                        $(
                            $(#[$imeta])*
                            specs.push((
                                $class,
                                stringify!($method),
                                build_descriptor(&[$(jni_desc_frag!($pty)),*], jni_desc_frag!($ret)),
                            ));
                        )*
                    }
                )*
                specs
            });
    };
}

// Registry of built-in native methods implemented in Rust, keyed by their JNI
// C-name and resolving to the function address invoked via libffi.
builtin_natives! {
    {
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

    "jdk/internal/misc/Unsafe": static fn registerNatives() -> void => register_natives_unsafe; // todo: implement me
    "jdk/internal/misc/Unsafe": instance fn arrayBaseOffset0(array_class: class) -> int => array_base_offset0;
    "jdk/internal/misc/Unsafe": instance fn objectFieldOffset0(f: field) -> long => object_field_offset0;
    "jdk/internal/misc/Unsafe": instance fn objectFieldOffset1(clazz: class, name: string) -> long => object_field_offset1;
    "jdk/internal/misc/Unsafe": instance fn staticFieldOffset0(f: field) -> long => static_field_offset0;
    "jdk/internal/misc/Unsafe": instance fn staticFieldBase0(f: field) -> object => static_field_base0;
    "jdk/internal/misc/Unsafe": instance fn compareAndSetInt(obj: object, offset: long, expected: int, x: int) -> boolean => compare_and_set_int;
    "jdk/internal/misc/Unsafe": instance fn compareAndSetReference(obj: object, offset: long, expected: object, x: object) -> boolean => compare_and_set_reference;
    "jdk/internal/misc/Unsafe": instance fn compareAndSetLong(obj: object, offset: long, expected: long, x: long) -> boolean => compare_and_set_long;
    "jdk/internal/misc/Unsafe": instance fn compareAndExchangeLong(obj: object, offset: long, expected: long, x: long) -> long => compare_and_exchange_long;
    "jdk/internal/misc/Unsafe": instance fn getReferenceVolatile(obj: object, offset: long) -> object => get_reference_volatile;
    "jdk/internal/misc/Unsafe": instance fn getByte(obj: object, offset: long) -> byte => get_byte;
    "jdk/internal/misc/Unsafe": instance fn getShort(obj: object, offset: long) -> short => get_short;
    "jdk/internal/misc/Unsafe": instance fn getChar(obj: object, offset: long) -> char => get_char;
    "jdk/internal/misc/Unsafe": instance fn getInt(obj: object, offset: long) -> int => get_int;
    "jdk/internal/misc/Unsafe": instance fn getIntVolatile(obj: object, offset: long) -> int => get_int_volatile;
    "jdk/internal/misc/Unsafe": instance fn getBooleanVolatile(obj: object, offset: long) -> boolean => get_boolean_volatile;
    "jdk/internal/misc/Unsafe": instance fn getLong(obj: object, offset: long) -> long => get_long;
    "jdk/internal/misc/Unsafe": instance fn getLongVolatile(obj: object, offset: long) -> long => get_long_volatile;
    "jdk/internal/misc/Unsafe": instance fn arrayIndexScale0(clazz: class) -> int => array_index_scale0;
    "jdk/internal/misc/Unsafe": instance fn fullFence() -> void => full_fence; // todo: implement me
    "jdk/internal/misc/Unsafe": instance fn getReference(obj: object, offset: long) -> object => get_reference;
    "jdk/internal/misc/Unsafe": instance fn putReference(obj: object, offset: long, x: object) -> void => put_reference;
    "jdk/internal/misc/Unsafe": instance fn putReferenceVolatile(obj: object, offset: long, x: object) -> void => put_reference_volatile;
    "jdk/internal/misc/Unsafe": instance fn putChar(obj: object, offset: long, x: char) -> void => put_char;
    "jdk/internal/misc/Unsafe": instance fn putByte(obj: object, offset: long, x: byte) -> void => put_byte;
    "jdk/internal/misc/Unsafe": instance fn putShort(obj: object, offset: long, x: short) -> void => put_short;
    "jdk/internal/misc/Unsafe": instance fn putInt(obj: object, offset: long, x: int) -> void => put_int;
    "jdk/internal/misc/Unsafe": instance fn putIntVolatile(obj: object, offset: long, x: int) -> void => put_int_volatile;
    "jdk/internal/misc/Unsafe": instance fn putLong(obj: object, offset: long, x: long) -> void => put_long;
    "jdk/internal/misc/Unsafe": instance fn ensureClassInitialized0(clazz: class) -> void => ensure_class_initialized0;
    "jdk/internal/misc/Unsafe": instance fn shouldBeInitialized0(clazz: class) -> boolean => should_be_initialized0;
    "jdk/internal/misc/Unsafe": instance fn copyMemory0(src: object, srcOffset: long, dest: object, destOffset: long, bytes: long) -> void => copy_memory0;
    "jdk/internal/misc/Unsafe": instance fn copySwapMemory0(src: object, srcOffset: long, dest: object, destOffset: long, bytes: long, swap: long) -> void => copy_swap_memory0;
    "jdk/internal/misc/Unsafe": instance fn setMemory0(obj: object, offset: long, bytes: long, value: byte) -> void => set_memory0;
    "jdk/internal/misc/Unsafe": instance fn allocateMemory0(bytes: long) -> long => allocate_memory0;

    "java/lang/String": instance fn intern() -> string => intern_string;

    "java/lang/Float": static fn floatToRawIntBits(f: float) -> int => float_to_raw_int_bits;

    "java/lang/Double": static fn doubleToRawLongBits(d: double) -> long => double_to_raw_long_bits;
    "java/lang/Double": static fn longBitsToDouble(l: long) -> double => long_bits_to_double;

    "jdk/internal/misc/CDS": static fn initializeFromArchive(clazz: class) -> void => initialize_from_archive; // todo: implement me
    "jdk/internal/misc/CDS": static fn getRandomSeedForDumping() -> long => get_random_seed_for_dumping; // Should return a predictable "random" seed derived from the VM's build ID and version, we return constant value for now
    "jdk/internal/misc/CDS": static fn getCDSConfigStatus() -> int => get_cds_config_status;  // Class Data Sharing (CDS) is disabled

    "jdk/internal/misc/VM": static fn initialize() -> void => initialize_vm; // todo: implement me

    "jdk/internal/jimage/NativeImageBuffer": instance fn getNativeMap(name: string) -> byte_buffer => get_native_map;

    "java/lang/Runtime": instance fn maxMemory() -> long => max_memory; // todo: use meaningful value, maybe use `sysinfo` crate to get the actual memory size
    "java/lang/Runtime": instance fn availableProcessors() -> int => available_processors;

    "jdk/internal/util/SystemProps$Raw": static fn platformProperties() -> string_array => platform_properties;
    "jdk/internal/util/SystemProps$Raw": static fn vmProperties() -> string_array => vm_properties;

    "java/io/FileDescriptor": static fn initIDs() -> void => init_ids_file_descriptor; // todo: implement me
    "java/io/FileDescriptor": static fn getHandle(fd: int) -> long => get_handle;
    "java/io/FileDescriptor": static fn getAppend(fd: int) -> boolean => get_append;
    "java/io/FileDescriptor": instance fn close0() -> void => file_descriptor_close0;

    "java/io/FileInputStream": static fn initIDs() -> void => init_ids_file_input_stream;
    "java/io/FileInputStream": instance fn open0(name: string) -> void => open0_file_input_stream;
    "java/io/FileInputStream": instance fn length0() -> long => length0_file_input_stream;
    "java/io/FileInputStream": instance fn position0() -> long => position0_file_input_stream;
    "java/io/FileInputStream": instance fn available0() -> int => available0_file_input_stream;
    "java/io/FileInputStream": instance fn readBytes(b: byte_array, off: int, len: int) -> int => read_bytes_file_input_stream;
    "java/io/FileInputStream": instance fn read0() -> int => read0_file_input_stream;
    "java/io/FileInputStream": instance fn isRegularFile0(fd: file_descriptor) -> boolean => is_regular_file0_file_input_stream;

    "java/io/FileOutputStream": static fn initIDs() -> void => init_ids_file_output_stream;
    "java/io/FileOutputStream": instance fn open0(name: string, append: boolean) -> void => open0_file_output_stream;
    "java/io/FileOutputStream": instance fn write(byte: int, append: boolean) -> void => write_file_output_stream;
    "java/io/FileOutputStream": instance fn writeBytes(b: byte_array, off: int, len: int, append: boolean) -> void => write_bytes_file_output_stream;

    "java/io/RandomAccessFile": static fn initIDs() -> void => init_ids_random_access_file;
    "java/io/RandomAccessFile": instance fn open0(name: string, mode: int) -> void => open0_random_access_file;
    "java/io/RandomAccessFile": instance fn seek0(offset: long) -> void => seek0_random_access_file;
    "java/io/RandomAccessFile": instance fn writeBytes0(b: byte_array, off: int, len: int) -> void => write_bytes0_random_access_file;
    "java/io/RandomAccessFile": instance fn readBytes0(b: byte_array, off: int, len: int) -> int => read_bytes0_random_access_file;
    "java/io/RandomAccessFile": instance fn length0() -> long => length0_random_access_file;

    "java/util/zip/CRC32": static fn updateBytes0(crc: int, b: byte_array, off: int, len: int) -> int => updatebytes0;
    }

    #[cfg(unix)]
    {
    "java/io/UnixFileSystem": static fn initIDs() -> void => init_ids_file_system; // this method is for caching `path` field from java/io/File for faster access in other native methods
    "java/io/UnixFileSystem": instance fn canonicalize0(name: string) -> string => canonicalize0_file_system;
    "java/io/UnixFileSystem": instance fn createFileExclusively0(name: string) -> boolean => create_file_exclusively0;
    "java/io/UnixFileSystem": instance fn getBooleanAttributes0(file: file) -> int => get_boolean_attributes0;
    "java/io/UnixFileSystem": instance fn checkAccess0(file: file, mode: int) -> boolean => check_access0;
    "java/io/UnixFileSystem": instance fn delete0(file: file) -> boolean => delete_impl;
    "java/io/UnixFileSystem": instance fn getNameMax0(name: string) -> long => crate::vm::system_native::file_system::unix::get_name_max0;
    "java/io/UnixFileSystem": instance fn getLength0(file: file) -> long => get_length0;
    }

    #[cfg(windows)]
    {
    "java/io/WinNTFileSystem": static fn initIDs() -> void => init_ids_file_system; // this method is for caching `path` field from java/io/File for faster access in other native methods
    "java/io/WinNTFileSystem": instance fn canonicalize0(name: string) -> string => canonicalize0_file_system;
    "java/io/WinNTFileSystem": instance fn createFileExclusively0(name: string) -> boolean => create_file_exclusively0;
    "java/io/WinNTFileSystem": instance fn getBooleanAttributes0(file: file) -> int => get_boolean_attributes0;
    "java/io/WinNTFileSystem": instance fn checkAccess0(file: file, mode: int) -> boolean => check_access0;
    "java/io/WinNTFileSystem": instance fn getFinalPath0(name: string) -> string => crate::vm::system_native::file_system::winnt::get_final_path0;
    "java/io/WinNTFileSystem": instance fn delete0(file: file, allow_delete_readonly: boolean) -> boolean => crate::vm::system_native::file_system::winnt::winnt_file_system_delete0;
    "java/io/WinNTFileSystem": instance fn getNameMax0(name: string) -> int => crate::vm::system_native::file_system::winnt::get_name_max0;
    "java/io/WinNTFileSystem": instance fn getLength0(file: file) -> long => get_length0;
    }
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
