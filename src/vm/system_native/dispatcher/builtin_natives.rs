use crate::vm::error::{Error, Result};
use crate::vm::jni::set_pending_internal_error;
use crate::vm::method_area::lookup::lookup_method;
use crate::vm::system_native as sn;
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
    stack_trace_element_array : ref   | i32  | jobjectArray | "[Ljava/lang/StackTraceElement;";
    network_interface_array   : ref   | i32  | jobjectArray | "[Ljava/net/NetworkInterface;";
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
    unix_file_attrs           : ref   | i32  | jobject      | "Lsun/nio/fs/UnixFileAttributes;";
    volume_info               : ref   | i32  | jobject      | "Lsun/nio/fs/WindowsNativeDispatcher$VolumeInformation;";
    first_file                : ref   | i32  | jobject      | "Lsun/nio/fs/WindowsNativeDispatcher$FirstFile;";
    protection_domain         : ref   | i32  | jobject      | "Ljava/security/ProtectionDomain;";
    thread                    : ref   | i32  | jobject      | "Ljava/lang/Thread;";
    member_name               : ref   | i32  | jobject      | "Ljava/lang/invoke/MemberName;";
    method_handle             : ref   | i32  | jobject      | "Ljava/lang/invoke/MethodHandle;";
    call_site                 : ref   | i32  | jobject      | "Ljava/lang/invoke/CallSite;";
    native_library_impl       : ref   | i32  | jobject      | "Ljdk/internal/loader/NativeLibraries$NativeLibraryImpl;";
    throwable                 : ref   | i32  | jobject      | "Ljava/lang/Throwable;";
    method                    : ref   | i32  | jobject      | "Ljava/lang/reflect/Method;";
    constructor               : ref   | i32  | jobject      | "Ljava/lang/reflect/Constructor;";
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
    "java/lang/System": static fn currentTimeMillis() -> long => sn::system::current_time_millis;
    "java/lang/System": static fn nanoTime() -> long => sn::system::nano_time;
    "java/lang/System": static fn arraycopy(src: object, src_pos: int, dest: object, dest_pos: int, length: int) -> void => sn::system::arraycopy;
    "java/lang/System": static fn setIn0(input_stream: input_stream) -> void => sn::system::set_in0; // todo: implement me
    "java/lang/System": static fn setOut0(print_stream: print_stream) -> void => sn::system::set_out0;
    "java/lang/System": static fn setErr0(print_stream: print_stream) -> void => sn::system::set_err0;
    "java/lang/System": static fn identityHashCode(obj: object) -> int => sn::object::identity_hashcode;
    "java/lang/System": static fn mapLibraryName(lib: string) -> string => sn::system::map_library_name;
    "java/lang/System": static fn registerNatives() -> void => sn::system::register_natives; // todo: implement me

    "java/lang/Object": instance fn hashCode() -> int => sn::object::identity_hashcode;
    "java/lang/Object": instance fn getClass() -> class => sn::object::get_class;
    "java/lang/Object": instance fn clone() -> object => sn::object::clone;
    "java/lang/Object": instance fn notifyAll() -> void => sn::object::notify_all; // todo: implement me

    "java/lang/Class": instance fn getSuperclass() -> class => sn::class::get_superclass;
    "java/lang/Class": static fn getPrimitiveClass(content: string) -> class => sn::class::get_primitive_class;
    "java/lang/Class": static fn desiredAssertionStatus0(clazz: class) -> boolean => sn::class::desired_assertion_status0; // setting all classes to have assertions enabled. todo: implement -ea and -da flags
    "java/lang/Class": static fn forName0(name: string, initialize: boolean, loader: class_loader, caller: class) -> class => sn::class::for_name0;
    "java/lang/Class": static fn registerNatives() -> void => sn::class::register_natives; // todo: implement me
    "java/lang/Class": instance fn initClassName() -> string => sn::class::init_class_name;
    "java/lang/Class": instance fn getInterfaces0() -> class_object_array => sn::class::get_interfaces0;
    "java/lang/Class": instance fn getDeclaringClass0() -> class => sn::class::get_declaring_class0;
    "java/lang/Class": instance fn getDeclaredFields0(flag: boolean) -> field_object_array => sn::class::get_declared_fields0;
    "java/lang/Class": instance fn getDeclaredMethods0(flag: boolean) -> method_object_array => sn::class::get_declared_methods0;
    "java/lang/Class": instance fn getDeclaredConstructors0(flag: boolean) -> constructor_object_array => sn::class::get_declared_constructors0;
    "java/lang/Class": instance fn getRawAnnotations() -> byte_array => sn::class::get_raw_annotations;
    "java/lang/Class": instance fn getEnclosingMethod0() -> object_array => sn::class::get_enclosing_method0;
    "java/lang/Class": instance fn getSimpleBinaryName0() -> string => sn::class::get_simple_binary_name0;
    "java/lang/Class": instance fn isAssignableFrom(clazz: class) -> boolean => sn::class::is_assignable_from;
    "java/lang/Class": instance fn isHidden() -> boolean => sn::class::is_hidden;  // we are treating all classes as non-hidden since we don't have a way to mark them as hidden (yet)
    "java/lang/Class": instance fn isInstance(obj: object) -> boolean => sn::class::class_is_instance;
    "java/lang/Class": instance fn getConstantPool() -> constant_pool => sn::class::get_constant_pool;
    "java/lang/Class": instance fn getNestHost0() -> class => sn::class::get_nest_host0;
    "java/lang/Class": instance fn isRecord0() -> boolean => sn::class::is_record0;

    "java/lang/Module": static fn addReads0(from: module, to: module) -> void => sn::module::add_reads0; // todo: implement me?
    "java/lang/Module": static fn defineModule0(module: module, is_open: boolean, version: string, location: string, pns: object_array) -> void => sn::module::define_module0;
    "java/lang/Module": static fn addExportsToAll0(from: module, pn: string) -> void => sn::module::add_exports_to_all0; // todo: implement me?
    "java/lang/Module": static fn addExports0(from: module, pn: string, to: module) -> void => sn::module::add_exports0; // todo: implement me?

    "java/lang/Shutdown": static fn beforeHalt() -> void => sn::shutdown::before_halt; // todo: implement me
    "java/lang/Shutdown": static fn halt0(status: int) -> void => sn::shutdown::halt0; // fixme: by doing this we skip destructors and other shutdown hooks, later it might be an issue

    "jdk/internal/misc/Unsafe": static fn registerNatives() -> void => sn::unsafe_::register_natives; // todo: implement me
    "jdk/internal/misc/Unsafe": instance fn arrayBaseOffset0(array_class: class) -> int => sn::unsafe_::array_base_offset0;
    "jdk/internal/misc/Unsafe": instance fn objectFieldOffset0(f: field) -> long => sn::unsafe_::object_field_offset0;
    "jdk/internal/misc/Unsafe": instance fn objectFieldOffset1(clazz: class, name: string) -> long => sn::unsafe_::object_field_offset1;
    "jdk/internal/misc/Unsafe": instance fn staticFieldOffset0(f: field) -> long => sn::unsafe_::static_field_offset0;
    "jdk/internal/misc/Unsafe": instance fn staticFieldBase0(f: field) -> object => sn::unsafe_::static_field_base0;
    "jdk/internal/misc/Unsafe": instance fn compareAndSetInt(obj: object, offset: long, expected: int, x: int) -> boolean => sn::unsafe_::compare_and_set_int;
    "jdk/internal/misc/Unsafe": instance fn compareAndSetReference(obj: object, offset: long, expected: object, x: object) -> boolean => sn::unsafe_::compare_and_set_reference;
    "jdk/internal/misc/Unsafe": instance fn compareAndSetLong(obj: object, offset: long, expected: long, x: long) -> boolean => sn::unsafe_::compare_and_set_long;
    "jdk/internal/misc/Unsafe": instance fn compareAndExchangeLong(obj: object, offset: long, expected: long, x: long) -> long => sn::unsafe_::compare_and_exchange_long;
    "jdk/internal/misc/Unsafe": instance fn getReferenceVolatile(obj: object, offset: long) -> object => sn::unsafe_::get_reference_volatile;
    "jdk/internal/misc/Unsafe": instance fn getByte(obj: object, offset: long) -> byte => sn::unsafe_::get_byte;
    "jdk/internal/misc/Unsafe": instance fn getShort(obj: object, offset: long) -> short => sn::unsafe_::get_short;
    "jdk/internal/misc/Unsafe": instance fn getChar(obj: object, offset: long) -> char => sn::unsafe_::get_char;
    "jdk/internal/misc/Unsafe": instance fn getInt(obj: object, offset: long) -> int => sn::unsafe_::get_int;
    "jdk/internal/misc/Unsafe": instance fn getIntVolatile(obj: object, offset: long) -> int => sn::unsafe_::get_int_volatile;
    "jdk/internal/misc/Unsafe": instance fn getBooleanVolatile(obj: object, offset: long) -> boolean => sn::unsafe_::get_boolean_volatile;
    "jdk/internal/misc/Unsafe": instance fn getLong(obj: object, offset: long) -> long => sn::unsafe_::get_long;
    "jdk/internal/misc/Unsafe": instance fn getLongVolatile(obj: object, offset: long) -> long => sn::unsafe_::get_long_volatile;
    "jdk/internal/misc/Unsafe": instance fn arrayIndexScale0(clazz: class) -> int => sn::unsafe_::array_index_scale0;
    "jdk/internal/misc/Unsafe": instance fn fullFence() -> void => sn::unsafe_::full_fence; // todo: implement me
    "jdk/internal/misc/Unsafe": instance fn getReference(obj: object, offset: long) -> object => sn::unsafe_::get_reference;
    "jdk/internal/misc/Unsafe": instance fn putReference(obj: object, offset: long, x: object) -> void => sn::unsafe_::put_reference;
    "jdk/internal/misc/Unsafe": instance fn putReferenceVolatile(obj: object, offset: long, x: object) -> void => sn::unsafe_::put_reference_volatile;
    "jdk/internal/misc/Unsafe": instance fn putChar(obj: object, offset: long, x: char) -> void => sn::unsafe_::put_char;
    "jdk/internal/misc/Unsafe": instance fn putByte(obj: object, offset: long, x: byte) -> void => sn::unsafe_::put_byte;
    "jdk/internal/misc/Unsafe": instance fn putShort(obj: object, offset: long, x: short) -> void => sn::unsafe_::put_short;
    "jdk/internal/misc/Unsafe": instance fn putInt(obj: object, offset: long, x: int) -> void => sn::unsafe_::put_int;
    "jdk/internal/misc/Unsafe": instance fn putIntVolatile(obj: object, offset: long, x: int) -> void => sn::unsafe_::put_int_volatile;
    "jdk/internal/misc/Unsafe": instance fn putLong(obj: object, offset: long, x: long) -> void => sn::unsafe_::put_long;
    "jdk/internal/misc/Unsafe": instance fn ensureClassInitialized0(clazz: class) -> void => sn::unsafe_::ensure_class_initialized0;
    "jdk/internal/misc/Unsafe": instance fn shouldBeInitialized0(clazz: class) -> boolean => sn::unsafe_::should_be_initialized0;
    "jdk/internal/misc/Unsafe": instance fn copyMemory0(src: object, srcOffset: long, dest: object, destOffset: long, bytes: long) -> void => sn::unsafe_::copy_memory0;
    "jdk/internal/misc/Unsafe": instance fn copySwapMemory0(src: object, srcOffset: long, dest: object, destOffset: long, bytes: long, swap: long) -> void => sn::unsafe_::copy_swap_memory0;
    "jdk/internal/misc/Unsafe": instance fn setMemory0(obj: object, offset: long, bytes: long, value: byte) -> void => sn::unsafe_::set_memory0;
    "jdk/internal/misc/Unsafe": instance fn allocateMemory0(bytes: long) -> long => sn::unsafe_::allocate_memory0;

    "java/lang/String": instance fn intern() -> string => sn::string::intern;

    "java/lang/Float": static fn floatToRawIntBits(f: float) -> int => sn::float::float_to_raw_int_bits;

    "java/lang/Double": static fn doubleToRawLongBits(d: double) -> long => sn::double::double_to_raw_long_bits;
    "java/lang/Double": static fn longBitsToDouble(l: long) -> double => sn::double::long_bits_to_double;

    "jdk/internal/misc/CDS": static fn initializeFromArchive(clazz: class) -> void => sn::cds::initialize_from_archive; // todo: implement me
    "jdk/internal/misc/CDS": static fn getRandomSeedForDumping() -> long => sn::cds::get_random_seed_for_dumping; // Should return a predictable "random" seed derived from the VM's build ID and version, we return constant value for now
    "jdk/internal/misc/CDS": static fn getCDSConfigStatus() -> int => sn::cds::get_cds_config_status;  // Class Data Sharing (CDS) is disabled

    "jdk/internal/misc/VM": static fn initialize() -> void => sn::vm::initialize; // todo: implement me

    "java/lang/ClassLoader": static fn registerNatives() -> void => sn::class_loader::register_natives; // todo: implement me
    "java/lang/ClassLoader": static fn defineClass0(cl: class_loader, lookup: class, name: string, buf: byte_array, off: int, len: int, pd: protection_domain, init: boolean, flags: int, class_data: object) -> class => sn::class_loader::define_class0;
    "java/lang/ClassLoader": static fn defineClass1(cl: class_loader, name: string, buf: byte_array, off: int, len: int, pd: protection_domain, source: string) -> class => sn::class_loader::define_class1;
    "java/lang/ClassLoader": static fn defineClass2(cl: class_loader, name: string, buf: byte_buffer, off: int, len: int, pd: protection_domain, source: string) -> class => sn::class_loader::define_class2;
    "java/lang/ClassLoader": static fn findBootstrapClass(name: string) -> class => sn::class_loader::find_bootstrap_class;
    "java/lang/ClassLoader": instance fn findLoadedClass0(name: string) -> class => sn::class_loader::find_loaded_class0;

    "jdk/internal/jimage/NativeImageBuffer": instance fn getNativeMap(name: string) -> byte_buffer => sn::native_image_buffer::get_native_map;

    "java/lang/invoke/MethodHandleNatives": static fn registerNatives() -> void => sn::method_handle_natives::register_natives; // todo: implement me
    "java/lang/invoke/MethodHandleNatives": static fn init(mn: member_name, obj_ref: object) -> void => sn::method_handle_natives::init;
    "java/lang/invoke/MethodHandleNatives": static fn resolve(mn: member_name, caller: class, lookup_mode: int, speculative_resolve: boolean) -> member_name => sn::method_handle_natives::resolve;
    "java/lang/invoke/MethodHandleNatives": static fn objectFieldOffset(mn: member_name) -> long => sn::method_handle_natives::object_field_offset;
    "java/lang/invoke/MethodHandleNatives": static fn staticFieldOffset(mn: member_name) -> long => sn::method_handle_natives::static_field_offset;
    "java/lang/invoke/MethodHandleNatives": static fn staticFieldBase(mn: member_name) -> object => sn::method_handle_natives::static_field_base;
    "java/lang/invoke/MethodHandleNatives": static fn getNamedCon(which: int, name: object_array) -> int => sn::method_handle_natives::get_named_con; // todo: implement me
    "java/lang/invoke/MethodHandleNatives": static fn getMemberVMInfo(mn: member_name) -> object => sn::method_handle_natives::get_member_vm_info;
    "java/lang/invoke/MethodHandleNatives": static fn setCallSiteTargetNormal(site: call_site, target: method_handle) -> void => sn::method_handle_natives::set_call_site_target_normal;

    "java/lang/Runtime": instance fn maxMemory() -> long => sn::runtime::max_memory; // todo: use meaningful value, maybe use `sysinfo` crate to get the actual memory size
    "java/lang/Runtime": instance fn availableProcessors() -> int => sn::runtime::available_processors;
    "java/lang/Runtime": instance fn totalMemory() -> long => sn::runtime::total_memory; // todo: implement me with GC
    "java/lang/Runtime": instance fn freeMemory() -> long => sn::runtime::free_memory; // todo: implement me with GC

    "jdk/internal/util/SystemProps$Raw": static fn platformProperties() -> string_array => sn::system_props_raw::platform_properties;
    "jdk/internal/util/SystemProps$Raw": static fn vmProperties() -> string_array => sn::system_props_raw::vm_properties;

    "java/io/FileDescriptor": static fn initIDs() -> void => sn::file_descriptor::init_ids; // todo: implement me
    "java/io/FileDescriptor": static fn getHandle(fd: int) -> long => sn::file_descriptor::get_handle;
    "java/io/FileDescriptor": static fn getAppend(fd: int) -> boolean => sn::file_descriptor::get_append;
    "java/io/FileDescriptor": instance fn close0() -> void => sn::file_descriptor::close0;

    "java/io/FileInputStream": static fn initIDs() -> void => sn::file_input_stream::init_ids;
    "java/io/FileInputStream": instance fn open0(name: string) -> void => sn::file_input_stream::open0;
    "java/io/FileInputStream": instance fn length0() -> long => sn::file_input_stream::length0;
    "java/io/FileInputStream": instance fn position0() -> long => sn::file_input_stream::position0;
    "java/io/FileInputStream": instance fn available0() -> int => sn::file_input_stream::available0;
    "java/io/FileInputStream": instance fn readBytes(b: byte_array, off: int, len: int) -> int => sn::file_input_stream::read_bytes;
    "java/io/FileInputStream": instance fn read0() -> int => sn::file_input_stream::read0;
    "java/io/FileInputStream": instance fn isRegularFile0(fd: file_descriptor) -> boolean => sn::file_input_stream::is_regular_file0;

    "java/io/FileOutputStream": static fn initIDs() -> void => sn::file_output_stream::init_ids;
    "java/io/FileOutputStream": instance fn open0(name: string, append: boolean) -> void => sn::file_output_stream::open0;
    "java/io/FileOutputStream": instance fn write(byte: int, append: boolean) -> void => sn::file_output_stream::write;
    "java/io/FileOutputStream": instance fn writeBytes(b: byte_array, off: int, len: int, append: boolean) -> void => sn::file_output_stream::write_bytes;

    "java/io/RandomAccessFile": static fn initIDs() -> void => sn::random_access_file::init_ids;
    "java/io/RandomAccessFile": instance fn open0(name: string, mode: int) -> void => sn::random_access_file::open0;
    "java/io/RandomAccessFile": instance fn seek0(offset: long) -> void => sn::random_access_file::seek0;
    "java/io/RandomAccessFile": instance fn writeBytes0(b: byte_array, off: int, len: int) -> void => sn::random_access_file::write_bytes0;
    "java/io/RandomAccessFile": instance fn readBytes0(b: byte_array, off: int, len: int) -> int => sn::random_access_file::read_bytes0;
    "java/io/RandomAccessFile": instance fn length0() -> long => sn::random_access_file::length0;

    "java/nio/MappedMemoryUtils": static fn registerNatives() -> void => sn::mapped_memory_utils::register_natives; // todo: implement me
    "java/nio/MappedMemoryUtils": static fn force0(fd: file_descriptor, addr: long, len: long) -> void => sn::mapped_memory_utils::force0;

    "java/lang/Thread": static fn registerNatives() -> void => sn::thread::register_natives;
    "java/lang/Thread": static fn currentThread() -> thread => sn::thread::current_thread;
    "java/lang/Thread": static fn currentCarrierThread() -> thread => sn::thread::current_carrier_thread;  //todo: use current carrier thread here (no matter what it is)
    "java/lang/Thread": static fn holdsLock(o: object) -> boolean => sn::thread::holds_lock; // todo: implement me
    "java/lang/Thread": static fn getNextThreadIdOffset() -> long => sn::thread::get_next_threadid_offset; // todo: `NEXT_TID_OFFSET` should have volatile semantics
    "java/lang/Thread": instance fn setPriority0(p: int) -> void => sn::thread::set_priority0; // todo: implement me
    "java/lang/Thread": instance fn start0() -> void => sn::thread::start0; // todo: implement me

    "java/util/zip/Deflater": static fn init(level: int, strategy: int, nowrap: boolean) -> long => sn::zip::deflater::init;
    "java/util/zip/Deflater": instance fn deflateBytesBytes(addr: long, inpt: byte_array, ioff: int, ilen: int, outpt: byte_array, ooff: int, olen: int, flush: int, params: int) -> long => sn::zip::deflater::deflate_bytes_bytes;
    "java/util/zip/Deflater": static fn end(addr: long) -> void => sn::zip::deflater::end;

    "java/util/zip/CRC32": static fn updateBytes0(crc: int, b: byte_array, off: int, len: int) -> int => sn::zip::crc32::updatebytes0;

    "java/util/zip/Inflater": static fn initIDs() -> void => sn::zip::inflater::init_ids; // todo: implement me
    "java/util/zip/Inflater": static fn init(nowrap: boolean) -> long => sn::zip::inflater::init;
    "java/util/zip/Inflater": instance fn inflateBytesBytes(addr: long, inpt: byte_array, ioff: int, ilen: int, outpt: byte_array, ooff: int, olen: int) -> long => sn::zip::inflater::inflate_bytes_bytes;
    "java/util/zip/Inflater": static fn end(addr: long) -> void => sn::zip::inflater::end;
    "java/util/zip/Inflater": static fn reset(addr: long) -> void => sn::zip::inflater::reset;

    "jdk/internal/perf/Perf": static fn registerNatives() -> void => sn::perf::register_natives; // todo: implement me
    "jdk/internal/perf/Perf": instance fn createLong(name: string, var: int, units: int, value: long) -> byte_buffer => sn::perf::create_long;
    "jdk/internal/perf/Perf": instance fn createByteArray(name: string, var: int, units: int, value: byte_array, mlen: int) -> byte_buffer => sn::perf::create_byte_array;

    "jdk/internal/reflect/Reflection": static fn getCallerClass() -> class => sn::reflecton::get_caller_class;
    "jdk/internal/reflect/Reflection": static fn getClassAccessFlags(c: class) -> int => sn::reflecton::get_class_access_flags;
    "jdk/internal/reflect/Reflection": static fn areNestMates(current: class, member: class) -> boolean => sn::reflecton::are_nest_mates;

    "jdk/internal/reflect/ConstantPool": instance fn getUTF8At0(cp: object, index: int) -> string => sn::constant_pool::get_utf8_at0;
    "jdk/internal/reflect/ConstantPool": instance fn getSize0(cp: object) -> int => sn::constant_pool::get_size0;
    "jdk/internal/reflect/ConstantPool": instance fn getTagAt0(cp: object, index: int) -> byte => sn::constant_pool::get_tag_at0;

    "sun/nio/ch/IOUtil": static fn initIDs() -> void => sn::io_util::init_ids; // todo: implement me
    "sun/nio/ch/IOUtil": static fn iovMax() -> int => sn::io_util::iov_max;
    "sun/nio/ch/IOUtil": static fn writevMax() -> long => sn::io_util::writev_max;

    "jdk/internal/misc/ScopedMemoryAccess": static fn registerNatives() -> void => sn::scoped_memory_access::register_natives; // todo: implement me

    "jdk/internal/misc/Signal": static fn findSignal0(sig_name: string) -> int => sn::signal::find_signal0; // todo: implement me
    "jdk/internal/misc/Signal": static fn handle0(sig: int, native_h: long) -> long => sn::signal::handle0; // todo: implement me

    "java/lang/ref/Finalizer": static fn isFinalizationEnabled() -> boolean => sn::finalizer::is_finalization_enabled; // todo: this should be implemented with GC

    "java/lang/ref/Reference": instance fn clear0() -> void => sn::reference::clear0; // todo: this should be implemented with GC
    "java/lang/ref/Reference": instance fn refersTo0(o: object) -> boolean => sn::reference::refers_to0; // todo: this should be implemented with GC

    "java/lang/ref/PhantomReference": instance fn clear0() -> void => sn::phantom_reference::clear0; // todo: this should be implemented with GC

    "java/lang/reflect/Array": static fn newArray(component_type: class, length: int) -> object => sn::reflect_array::new_array;

    "sun/nio/ch/NativeThread": static fn current0() -> long => sn::native_thread::current0; // todo: implement this (by 0 we say that the platform can not signal native threads)

    "jdk/internal/loader/NativeLibraries": static fn findBuiltinLib(name: string) -> string => sn::native_libraries::find_builtin_lib;
    "jdk/internal/loader/NativeLibraries": static fn load(imp: native_library_impl, name: string, builtin: boolean, throw: boolean) -> boolean => sn::native_libraries::load;
    "jdk/internal/loader/NativeLibraries$NativeLibraryImpl": static fn findEntry0(handle: long, name: string) -> long => sn::native_libraries::find_entry0;

    "jdk/internal/loader/BootLoader": static fn setBootLoaderUnnamedModule0(m: module) -> void => sn::bootloader::set_bootloader_unnamed_module0;

    "java/lang/StackTraceElement": static fn initStackTraceElements(elements: stack_trace_element_array, x: object, depth: int) -> void => sn::stack_trace_element::init_stack_trace_elements;

    "jdk/internal/reflect/DirectMethodHandleAccessor$NativeAccessor": static fn invoke0(m: method, obj: object, args: object_array) -> object => sn::method_handle_natives::native_accessor::invoke0;
    "jdk/internal/reflect/DirectConstructorHandleAccessor$NativeAccessor": static fn newInstance0(c: constructor, args: object_array) -> object => sn::method_handle_natives::native_accessor::new_instance0;

    "jdk/internal/vm/ContinuationSupport": static fn isSupported0() -> boolean => sn::continuation_support::is_supported0; // We do not support Loom continuations (yet)

    "java/lang/NullPointerException": instance fn getExtendedNPEMessage() -> string => sn::null_pointer_exception::get_extended_npe_message; // todo: https://github.com/hextriclosan/rusty-jvm/issues/521

    "java/net/NetworkInterface": static fn init() -> void => sn::network_interface::init; // todo: implement me
    "java/net/NetworkInterface": static fn getAll() -> network_interface_array => sn::network_interface::get_all; // fixme: https://github.com/hextriclosan/rusty-jvm/issues/539

    "jdk/internal/misc/PreviewFeatures": static fn isPreviewEnabled() -> boolean => sn::preview_features::is_preview_enabled;  // todo: implement me
    "java/util/TimeZone": static fn getSystemTimeZoneID(java_home: string) -> string => sn::time_zone::get_system_time_zone_id;
    }

    #[cfg(unix)]
    {
    "java/io/UnixFileSystem": static fn initIDs() -> void => sn::file_system::init_ids; // this method is for caching `path` field from java/io/File for faster access in other native methods
    "java/io/UnixFileSystem": instance fn canonicalize0(name: string) -> string => sn::file_system::canonicalize0;
    "java/io/UnixFileSystem": instance fn createFileExclusively0(name: string) -> boolean => sn::file_system::create_file_exclusively0;
    "java/io/UnixFileSystem": instance fn getBooleanAttributes0(file: file) -> int => sn::file_system::get_boolean_attributes0;
    "java/io/UnixFileSystem": instance fn checkAccess0(file: file, mode: int) -> boolean => sn::file_system::check_access0;
    "java/io/UnixFileSystem": instance fn delete0(file: file) -> boolean => sn::file_system::delete0;
    "java/io/UnixFileSystem": instance fn getNameMax0(name: string) -> long => sn::file_system::unix::get_name_max0;
    "java/io/UnixFileSystem": instance fn getLength0(file: file) -> long => sn::file_system::get_length0;
    }

    #[cfg(windows)]
    {
    "java/io/WinNTFileSystem": static fn initIDs() -> void => sn::file_system::init_ids; // this method is for caching `path` field from java/io/File for faster access in other native methods
    "java/io/WinNTFileSystem": instance fn canonicalize0(name: string) -> string => sn::file_system::canonicalize0;
    "java/io/WinNTFileSystem": instance fn createFileExclusively0(name: string) -> boolean => sn::file_system::create_file_exclusively0;
    "java/io/WinNTFileSystem": instance fn getBooleanAttributes0(file: file) -> int => sn::file_system::get_boolean_attributes0;
    "java/io/WinNTFileSystem": instance fn checkAccess0(file: file, mode: int) -> boolean => sn::file_system::check_access0;
    "java/io/WinNTFileSystem": instance fn getFinalPath0(name: string) -> string => sn::file_system::winnt::get_final_path0;
    "java/io/WinNTFileSystem": instance fn delete0(file: file, allow_delete_readonly: boolean) -> boolean => sn::file_system::winnt::delete0;
    "java/io/WinNTFileSystem": instance fn getNameMax0(name: string) -> int => sn::file_system::winnt::get_name_max0;
    "java/io/WinNTFileSystem": instance fn getLength0(file: file) -> long => sn::file_system::get_length0;
    }

    #[cfg(unix)]
    {
    "sun/nio/fs/UnixNativeDispatcher": static fn getcwd() -> byte_array => sn::platform_native_dispatcher::unix::get_cwd;
    "sun/nio/fs/UnixNativeDispatcher": static fn init() -> int => sn::platform_native_dispatcher::unix::init; // todo: return real capability flags
    "sun/nio/fs/UnixNativeDispatcher": static fn open0(path_ptr: long, flags: int, mode: int) -> int => sn::platform_native_dispatcher::unix::open0;
    "sun/nio/fs/UnixNativeDispatcher": static fn access0(path_ptr: long, mode: int) -> int => sn::platform_native_dispatcher::unix::access0;
    "sun/nio/fs/UnixNativeDispatcher": static fn stat0(path_ptr: long, attrs: unix_file_attrs) -> int => sn::platform_native_dispatcher::unix::stat0;
    "sun/nio/fs/UnixNativeDispatcher": static fn lstat0(path_ptr: long, attrs: unix_file_attrs) -> void => sn::platform_native_dispatcher::unix::lstat0;
    "sun/nio/fs/UnixNativeDispatcher": static fn mkdir0(path_ptr: long, mode: int) -> void => sn::platform_native_dispatcher::unix::mkdir0;
    "sun/nio/fs/UnixNativeDispatcher": static fn unlink0(path_ptr: long) -> void => sn::platform_native_dispatcher::unix::unlink0;
    "sun/nio/fs/UnixNativeDispatcher": static fn rmdir0(path_ptr: long) -> void => sn::platform_native_dispatcher::unix::rmdir0;
    "sun/nio/fs/UnixNativeDispatcher": static fn realpath0(path_ptr: long) -> byte_array => sn::platform_native_dispatcher::unix::realpath0;
    }

    #[cfg(windows)]
    {
    "sun/nio/fs/WindowsNativeDispatcher": static fn initIDs() -> void => sn::platform_native_dispatcher::win::init_ids; // todo: implement me
    "sun/nio/fs/WindowsNativeDispatcher": static fn CreateDirectory0(path_ptr: long, sd_ptr: long) -> void => sn::platform_native_dispatcher::win::create_directory0;
    "sun/nio/fs/WindowsNativeDispatcher": static fn GetFileAttributesEx0(name_ptr: long, output_ptr: long) -> void => sn::platform_native_dispatcher::win::get_file_attributes_ex0;
    "sun/nio/fs/WindowsNativeDispatcher": static fn DeleteFile0(name_ptr: long) -> void => sn::platform_native_dispatcher::win::delete_file0;
    "sun/nio/fs/WindowsNativeDispatcher": static fn RemoveDirectory0(name_ptr: long) -> void => sn::platform_native_dispatcher::win::remove_directory0;
    "sun/nio/fs/WindowsNativeDispatcher": static fn CreateFile0(name_ptr: long, access: int, share: int, sd_ptr: long, creation_dsp: int, flags: int) -> long => sn::platform_native_dispatcher::win::create_file0;
    "sun/nio/fs/WindowsNativeDispatcher": static fn SetEndOfFile(handle_ptr: long) -> void => sn::platform_native_dispatcher::win::set_end_of_file;
    "sun/nio/fs/WindowsNativeDispatcher": static fn GetFileSecurity0(name_ptr: long, req_info: int, descr_ptr: long, length: int) -> int => sn::platform_native_dispatcher::win::get_file_security0;
    "sun/nio/fs/WindowsNativeDispatcher": static fn GetCurrentProcess() -> long => sn::platform_native_dispatcher::win::get_current_process;
    "sun/nio/fs/WindowsNativeDispatcher": static fn OpenProcessToken(proc_handle: long, access: int) -> long => sn::platform_native_dispatcher::win::open_process_token;
    "sun/nio/fs/WindowsNativeDispatcher": static fn GetCurrentThread() -> long => sn::platform_native_dispatcher::win::get_current_thread;
    "sun/nio/fs/WindowsNativeDispatcher": static fn OpenThreadToken(thread_handle: long, access: int, open_as_self: boolean) -> long => sn::platform_native_dispatcher::win::open_thread_token;
    "sun/nio/fs/WindowsNativeDispatcher": static fn DuplicateTokenEx(token_handle: long, access: int) -> long => sn::platform_native_dispatcher::win::duplicate_token_ex;
    "sun/nio/fs/WindowsNativeDispatcher": static fn AccessCheck(token: long, sd_ptr: long, mask: int, read: int, write: int, exec: int, all: int) -> boolean => sn::platform_native_dispatcher::win::access_check;
    "sun/nio/fs/WindowsNativeDispatcher": static fn CloseHandle(handle: long) -> void => sn::platform_native_dispatcher::win::close_handle;
    "sun/nio/fs/WindowsNativeDispatcher": static fn GetVolumePathName0(address_ptr: long) -> string => sn::platform_native_dispatcher::win::get_volume_path_name0;
    "sun/nio/fs/WindowsNativeDispatcher": static fn GetVolumeInformation0(address_ptr: long, info_ref: volume_info) -> void => sn::platform_native_dispatcher::win::get_volume_information0;
    "sun/nio/fs/WindowsNativeDispatcher": static fn GetDriveType0(address_handle: long) -> int => sn::platform_native_dispatcher::win::get_drive_type0;
    "sun/nio/fs/WindowsNativeDispatcher": static fn FormatMessage(error_code: int) -> string => sn::platform_native_dispatcher::win::format_message;
    "sun/nio/fs/WindowsNativeDispatcher": static fn GetFullPathName0(address_ptr: long) -> string => sn::platform_native_dispatcher::win::get_full_path_name0;
    "sun/nio/fs/WindowsNativeDispatcher": static fn FindFirstFile0(name_ptr: long, first_file: first_file) -> void => sn::platform_native_dispatcher::win::find_first_file0;
    "sun/nio/fs/WindowsNativeDispatcher": static fn FindNextFile0(handle: long, addr: long) -> string => sn::platform_native_dispatcher::win::find_next_file0;
    "sun/nio/fs/WindowsNativeDispatcher": static fn FindClose(handle: long) -> void => sn::platform_native_dispatcher::win::find_close;
    }

    #[cfg(unix)]
    {
    "sun/nio/ch/UnixFileDispatcherImpl": static fn write0(fd: file_descriptor, address: long, len: int) -> int => sn::platform_file_dispatcher::unix::write0;
    "sun/nio/ch/UnixFileDispatcherImpl": static fn read0(fd: file_descriptor, address: long, len: int) -> int => sn::platform_file_dispatcher::unix::read0;
    "sun/nio/ch/UnixFileDispatcherImpl": static fn pread0(fd: file_descriptor, address: long, len: int, pos: long) -> int => sn::platform_file_dispatcher::unix::pread0;
    "sun/nio/ch/UnixFileDispatcherImpl": static fn size0(fd: file_descriptor) -> long => sn::platform_file_dispatcher::unix::size0;
    "sun/nio/ch/UnixFileDispatcherImpl": static fn allocationGranularity0() -> long => sn::platform_file_dispatcher::allocation_granularity0;
    "sun/nio/ch/UnixFileDispatcherImpl": static fn truncate0(fd: file_descriptor, size: long) -> int => sn::platform_file_dispatcher::unix::truncate0;
    "sun/nio/ch/UnixFileDispatcherImpl": static fn map0(fd: file_descriptor, prot: int, pos: long, len: long, is_sync: boolean) -> long => sn::platform_file_dispatcher::map0;
    "sun/nio/ch/UnixFileDispatcherImpl": static fn isOther0(fd: file_descriptor) -> boolean => sn::platform_file_dispatcher::is_other0;
    "sun/nio/ch/UnixFileDispatcherImpl": static fn seek0(fd: file_descriptor, off: long) -> long => sn::platform_file_dispatcher::seek0;
    }

    #[cfg(windows)]
    {
    "sun/nio/ch/FileDispatcherImpl": static fn allocationGranularity0() -> long => sn::platform_file_dispatcher::allocation_granularity0;
    "sun/nio/ch/FileDispatcherImpl": static fn maxDirectTransferSize0() -> int => sn::platform_file_dispatcher::win::max_direct_transfer_size0; // Integer.MAX_VALUE - 1 is the maximum transfer size for TransmitFile()
    "sun/nio/ch/FileDispatcherImpl": static fn write0(fd: file_descriptor, address: long, len: int, append: boolean) -> int => sn::platform_file_dispatcher::win::write0;
    "sun/nio/ch/FileDispatcherImpl": static fn read0(fd: file_descriptor, address: long, len: int) -> int => sn::platform_file_dispatcher::win::read0;
    "sun/nio/ch/FileDispatcherImpl": static fn pread0(fd: file_descriptor, address: long, len: int, offset: long) -> int => sn::platform_file_dispatcher::win::pread0;
    "sun/nio/ch/FileDispatcherImpl": static fn size0(fd: file_descriptor) -> long => sn::platform_file_dispatcher::win::size0;
    "sun/nio/ch/FileDispatcherImpl": static fn truncate0(fd: file_descriptor, size: long) -> int => sn::platform_file_dispatcher::win::truncate0;
    "sun/nio/ch/FileDispatcherImpl": static fn map0(fd: file_descriptor, prot: int, pos: long, len: long, is_sync: boolean) -> long => sn::platform_file_dispatcher::map0;
    "sun/nio/ch/FileDispatcherImpl": static fn isOther0(fd: file_descriptor) -> boolean => sn::platform_file_dispatcher::is_other0;
    "sun/nio/ch/FileDispatcherImpl": static fn seek0(fd: file_descriptor, off: long) -> long => sn::platform_file_dispatcher::seek0;
    "sun/nio/ch/FileDispatcherImpl": static fn duplicateHandle(handle: long) -> long => sn::platform_file_dispatcher::win::duplicate_handle;
    }

    #[cfg(target_os = "linux")]
    {
    "sun/nio/ch/FileDispatcherImpl": static fn init0() -> void => sn::platform_file_dispatcher::linux::init0;
    }

    #[cfg(windows)]
    {
    "sun/io/Win32ErrorMode": static fn setErrorMode(mode: long) -> long => sn::win32_error_mode::set_error_mode;
    "sun/security/provider/NativeSeedGenerator": static fn nativeGenerateSeed(bytes: byte_array) -> boolean => sn::native_seed_generator::native_generate_seed;
    }

    #[cfg(unix)] {
    "sun/nio/ch/NativeThread": static fn init() -> void => sn::native_thread::init;  // todo: implement me
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
