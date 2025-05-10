use crate::error::Error;
use crate::execution_engine::system_native_table::NativeMethod::WithMutStackFrames;
use crate::execution_engine::system_native_table::NativeMethod::{Basic, WithStackFrames};
use crate::helper::i64_to_vec;
use crate::stack::stack_frame::StackFrames;
use crate::system_native::class::{
    class_init_class_name_wrp, class_is_instance_wrp, for_name0_wrp, get_constant_pool_wrp,
    get_declared_constructors0_wrp, get_declared_methods0_wrp, get_declaring_class0_wrp,
    get_enclosing_method0_wrp, get_interfaces0_wrp, get_modifiers_wrp, get_nest_host0_wrp,
    get_primitive_class_wrp, get_raw_annotations_wrp, get_simple_binary_name0_wrp,
    get_superclass_wrp, is_array_wrp, is_assignable_from_wrp, is_interface_wrp, is_primitive_wrp,
};
use crate::system_native::class_loader::{define_class0_wrp, find_bootstrap_class_wrp};
use crate::system_native::constant_pool::{
    constant_pool_get_size0_wrp, constant_pool_get_tag_at0_wrp, constant_pool_get_utf8_at0_wrp,
};
use crate::system_native::file_descriptor::{file_descriptor_close0_wrp, get_handle_wrp};
use crate::system_native::file_output_stream::{
    file_output_stream_open0_wrp, file_output_stream_write_bytes_wrp, file_output_stream_write_wrp,
};
use crate::system_native::io_file_system::{
    canonicalize0_wrp, check_access0_wrp, create_file_exclusively0_wrp,
};
use crate::system_native::io_file_system::{delete0_wrp, get_boolean_attributes0_wrp};
use crate::system_native::io_util::{iov_max_wrp, writev_max_wrp};
use crate::system_native::method_handle_natives::wrappers::{
    method_handle_invoke_basic_wrp, method_handle_invoke_exact_wrp,
    method_handle_natives_get_member_vm_info_wrp, method_handle_natives_get_named_con_wrp,
    method_handle_natives_object_field_offset_wrp, method_handle_natives_resolve_wrp,
    method_handle_natives_static_field_base_wrp, method_handle_natives_static_field_offset_wrp,
};
use crate::system_native::native_libraries::find_builtin_lib_wrp;
use crate::system_native::object::{clone_wrp, get_class_wrp, object_hashcode_wrp};
use crate::system_native::reflect_array::new_array_wrp;
use crate::system_native::reflecton::{
    reflection_are_nest_mates_wrp, reflection_get_caller_class_wrp,
    reflection_get_class_access_flags_wrp,
};
use crate::system_native::string::intern_wrp;
use crate::system_native::system::{
    arraycopy_wrp, current_time_millis_wrp, set_out0_wrp, system_identity_hashcode_wrp,
    system_map_library_name_wrp,
};
use crate::system_native::system_props_raw::{platform_properties_wrp, vm_properties_wrp};
use crate::system_native::thread::current_thread_wrp;
use crate::system_native::unsafe_::{
    allocate_memory0_wrp, array_index_scale0_wrp, compare_and_set_int_wrp,
    compare_and_set_long_wrp, copy_memory0_wrp, ensure_class_initialized0_wrp, get_byte_wrp,
    get_int_volatile_wrp, get_int_wrp, get_long_volatile_wrp, get_long_wrp,
    get_reference_volatile_wrp, get_short_wrp, object_field_offset_1_wrp, put_byte_wrp,
    put_char_wrp, put_reference_volatile_wrp, put_reference_wrp, set_memory0_wrp,
    should_be_initialized0_wrp,
};
use once_cell::sync::Lazy;
use std::collections::HashMap;

type BasicNativeMethod = fn(&[i32]) -> crate::error::Result<Vec<i32>>;
type WithStackFramesNativeMethod = fn(&[i32], &StackFrames) -> crate::error::Result<Vec<i32>>;
type WithMutStackFramesNativeMethod =
    fn(&[i32], &mut StackFrames) -> crate::error::Result<Vec<i32>>;

enum NativeMethod {
    Basic(BasicNativeMethod),
    WithStackFrames(WithStackFramesNativeMethod),
    WithMutStackFrames(WithMutStackFramesNativeMethod),
}

static SYSTEM_NATIVE_TABLE: Lazy<HashMap<&'static str, NativeMethod>> = Lazy::new(|| {
    let mut table = HashMap::new();
    table.insert(
        "java/lang/Object:getClass:()Ljava/lang/Class;",
        Basic(get_class_wrp),
    );
    table.insert(
        "java/lang/Object:clone:()Ljava/lang/Object;",
        Basic(clone_wrp),
    );
    table.insert("java/lang/Object:notifyAll:()V", Basic(void_stub));
    table.insert("java/lang/Object:hashCode:()I", Basic(object_hashcode_wrp));
    table.insert(
        "java/lang/System:currentTimeMillis:()J",
        Basic(current_time_millis_wrp),
    );
    table.insert(
        "java/lang/System:arraycopy:(Ljava/lang/Object;ILjava/lang/Object;II)V",
        Basic(arraycopy_wrp),
    );
    table.insert("java/lang/System:registerNatives:()V", Basic(void_stub));
    table.insert(
        "java/lang/System:setIn0:(Ljava/io/InputStream;)V",
        Basic(void_stub),
    );
    table.insert(
        "java/lang/System:setOut0:(Ljava/io/PrintStream;)V",
        Basic(set_out0_wrp),
    );
    table.insert(
        "java/lang/System:setErr0:(Ljava/io/PrintStream;)V",
        Basic(void_stub),
    );
    table.insert(
        "java/lang/System:identityHashCode:(Ljava/lang/Object;)I",
        Basic(system_identity_hashcode_wrp),
    );
    table.insert(
        "java/lang/System:mapLibraryName:(Ljava/lang/String;)Ljava/lang/String;",
        Basic(system_map_library_name_wrp),
    );
    table.insert("java/lang/Class:getModifiers:()I", Basic(get_modifiers_wrp));
    table.insert(
        "java/lang/Class:getSuperclass:()Ljava/lang/Class;",
        Basic(get_superclass_wrp),
    );
    table.insert(
        "java/lang/Class:getPrimitiveClass:(Ljava/lang/String;)Ljava/lang/Class;",
        Basic(get_primitive_class_wrp),
    );
    table.insert(
        "java/lang/Class:desiredAssertionStatus0:(Ljava/lang/Class;)Z",
        Basic(|_args: &[i32]| Ok(vec![1])), // setting all classes to have assertions enabled. todo: implement -ea and -da flags
    );
    table.insert("java/lang/Class:isPrimitive:()Z", Basic(is_primitive_wrp));
    table.insert("java/lang/Class:isArray:()Z", Basic(is_array_wrp));
    table.insert("java/lang/Class:isInterface:()Z", Basic(is_interface_wrp));
    table.insert("java/lang/Class:forName0:(Ljava/lang/String;ZLjava/lang/ClassLoader;Ljava/lang/Class;)Ljava/lang/Class;", Basic(for_name0_wrp));
    table.insert("java/lang/Class:registerNatives:()V", Basic(void_stub));
    table.insert(
        "java/lang/Class:initClassName:()Ljava/lang/String;",
        Basic(class_init_class_name_wrp),
    );
    table.insert(
        "java/lang/Class:getInterfaces0:()[Ljava/lang/Class;",
        Basic(get_interfaces0_wrp),
    );
    table.insert(
        "java/lang/Class:getDeclaringClass0:()Ljava/lang/Class;",
        Basic(get_declaring_class0_wrp),
    );
    table.insert(
        "java/lang/Class:getDeclaredMethods0:(Z)[Ljava/lang/reflect/Method;",
        Basic(get_declared_methods0_wrp),
    );
    table.insert(
        "java/lang/Class:getDeclaredConstructors0:(Z)[Ljava/lang/reflect/Constructor;",
        Basic(get_declared_constructors0_wrp),
    );
    table.insert(
        "java/lang/Class:getRawAnnotations:()[B",
        Basic(get_raw_annotations_wrp),
    );
    table.insert(
        "java/lang/Class:getEnclosingMethod0:()[Ljava/lang/Object;",
        Basic(get_enclosing_method0_wrp),
    );
    table.insert(
        "java/lang/Class:getSimpleBinaryName0:()Ljava/lang/String;",
        Basic(get_simple_binary_name0_wrp),
    );
    table.insert(
        "java/lang/Class:isAssignableFrom:(Ljava/lang/Class;)Z",
        Basic(is_assignable_from_wrp),
    );
    table.insert(
        "java/lang/Class:isHidden:()Z",
        Basic(|_args: &[i32]| Ok(vec![0])), // we are treating all classes as non-hidden since we don't have a way to mark them as hidden (yet)
    );
    table.insert(
        "java/lang/Class:isInstance:(Ljava/lang/Object;)Z",
        Basic(class_is_instance_wrp),
    );
    table.insert(
        "java/lang/Class:getConstantPool:()Ljdk/internal/reflect/ConstantPool;",
        Basic(get_constant_pool_wrp),
    );
    table.insert(
        "java/lang/Class:getNestHost0:()Ljava/lang/Class;",
        Basic(get_nest_host0_wrp),
    );
    table.insert(
        "jdk/internal/misc/Unsafe:registerNatives:()V",
        Basic(void_stub),
    );
    table.insert(
        "jdk/internal/misc/Unsafe:arrayBaseOffset0:(Ljava/lang/Class;)I",
        Basic(|_args: &[i32]| Ok(vec![0])),
    );
    table.insert(
        "jdk/internal/misc/Unsafe:objectFieldOffset1:(Ljava/lang/Class;Ljava/lang/String;)J",
        Basic(object_field_offset_1_wrp),
    );
    table.insert(
        "jdk/internal/misc/Unsafe:compareAndSetInt:(Ljava/lang/Object;JII)Z",
        Basic(compare_and_set_int_wrp),
    );
    table.insert(
        "jdk/internal/misc/Unsafe:compareAndSetReference:(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z",
        Basic(compare_and_set_int_wrp)
    );
    table.insert(
        "jdk/internal/misc/Unsafe:compareAndSetLong:(Ljava/lang/Object;JJJ)Z",
        Basic(compare_and_set_long_wrp),
    );
    table.insert(
        "jdk/internal/misc/Unsafe:getReferenceVolatile:(Ljava/lang/Object;J)Ljava/lang/Object;",
        Basic(get_reference_volatile_wrp),
    );
    table.insert(
        "jdk/internal/misc/Unsafe:getByte:(Ljava/lang/Object;J)B",
        Basic(get_byte_wrp),
    );
    table.insert(
        "jdk/internal/misc/Unsafe:getShort:(Ljava/lang/Object;J)S",
        Basic(get_short_wrp),
    );
    table.insert(
        "jdk/internal/misc/Unsafe:getInt:(Ljava/lang/Object;J)I",
        Basic(get_int_wrp),
    );
    table.insert(
        "jdk/internal/misc/Unsafe:getIntVolatile:(Ljava/lang/Object;J)I",
        Basic(get_int_volatile_wrp),
    );
    table.insert(
        "jdk/internal/misc/Unsafe:getLong:(Ljava/lang/Object;J)J",
        Basic(get_long_wrp),
    );
    table.insert(
        "jdk/internal/misc/Unsafe:getLongVolatile:(Ljava/lang/Object;J)J",
        Basic(get_long_volatile_wrp),
    );
    table.insert(
        "jdk/internal/misc/Unsafe:arrayIndexScale0:(Ljava/lang/Class;)I",
        Basic(array_index_scale0_wrp),
    );
    table.insert("jdk/internal/misc/Unsafe:fullFence:()V", Basic(void_stub));
    table.insert(
        "jdk/internal/misc/Unsafe:getReference:(Ljava/lang/Object;J)Ljava/lang/Object;",
        Basic(get_reference_volatile_wrp),
    );
    table.insert(
        "jdk/internal/misc/Unsafe:putReference:(Ljava/lang/Object;JLjava/lang/Object;)V",
        Basic(put_reference_wrp),
    );
    table.insert(
        "jdk/internal/misc/Unsafe:putReferenceVolatile:(Ljava/lang/Object;JLjava/lang/Object;)V",
        Basic(put_reference_volatile_wrp),
    );
    table.insert(
        "jdk/internal/misc/Unsafe:putChar:(Ljava/lang/Object;JC)V",
        Basic(put_char_wrp),
    );
    table.insert(
        "jdk/internal/misc/Unsafe:putByte:(Ljava/lang/Object;JB)V",
        Basic(put_byte_wrp),
    );
    table.insert(
        "jdk/internal/misc/Unsafe:ensureClassInitialized0:(Ljava/lang/Class;)V",
        Basic(ensure_class_initialized0_wrp),
    );
    table.insert(
        "jdk/internal/misc/Unsafe:shouldBeInitialized0:(Ljava/lang/Class;)Z",
        Basic(should_be_initialized0_wrp),
    );
    table.insert(
        "jdk/internal/misc/Unsafe:copyMemory0:(Ljava/lang/Object;JLjava/lang/Object;JJ)V",
        Basic(copy_memory0_wrp),
    );
    table.insert(
        "jdk/internal/misc/Unsafe:setMemory0:(Ljava/lang/Object;JJB)V",
        Basic(set_memory0_wrp),
    );
    table.insert(
        "jdk/internal/misc/Unsafe:allocateMemory0:(J)J",
        Basic(allocate_memory0_wrp),
    );
    table.insert(
        "java/lang/String:intern:()Ljava/lang/String;",
        Basic(intern_wrp),
    );
    table.insert(
        "java/lang/Float:floatToRawIntBits:(F)I",
        Basic(|args: &[i32]| Ok(args.to_vec())),
    );
    table.insert(
        "java/lang/Double:doubleToRawLongBits:(D)J",
        Basic(|args: &[i32]| {
            let mut vec = args.to_vec();
            vec.reverse();
            Ok(vec)
        }),
    );
    table.insert(
        "java/lang/Double:longBitsToDouble:(J)D",
        Basic(|args: &[i32]| {
            let mut vec = args.to_vec();
            vec.reverse();
            Ok(vec)
        }),
    );
    table.insert(
        "jdk/internal/misc/CDS:initializeFromArchive:(Ljava/lang/Class;)V",
        Basic(void_stub),
    );
    table.insert(
        "jdk/internal/misc/CDS:getRandomSeedForDumping:()J",
        Basic(|_args: &[i32]| Ok(vec![1337, 42])), // Should return a predictable "random" seed derived from the VM's build ID and version, we return constant value for now
    );
    table.insert(
        "jdk/internal/misc/CDS:getCDSConfigStatus:()I",
        Basic(|_args: &[i32]| Ok(vec![0])), // Class Data Sharing (CDS) is disabled
    );
    table.insert("jdk/internal/misc/VM:initialize:()V", Basic(void_stub));
    table.insert(
        "java/lang/Runtime:maxMemory:()J",
        Basic(|_args: &[i32]| Ok(i64_to_vec(i64::MAX))),
    );
    table.insert(
        "java/lang/Runtime:availableProcessors:()I",
        Basic(|_args: &[i32]| Ok(vec![14])),
    );
    table.insert(
        "jdk/internal/util/SystemProps$Raw:platformProperties:()[Ljava/lang/String;",
        Basic(platform_properties_wrp),
    );
    table.insert(
        "jdk/internal/util/SystemProps$Raw:vmProperties:()[Ljava/lang/String;",
        Basic(vm_properties_wrp),
    );
    table.insert("java/io/FileDescriptor:initIDs:()V", Basic(void_stub));
    table.insert(
        "java/io/FileDescriptor:getHandle:(I)J",
        Basic(get_handle_wrp),
    );
    table.insert(
        "java/io/FileDescriptor:getAppend:(I)Z",
        Basic(|args: &[i32]| {
            let _fd = args[0];
            Ok(vec![1])
        }),
    );
    table.insert(
        "java/io/FileDescriptor:close0:()V",
        Basic(file_descriptor_close0_wrp),
    );
    table.insert("java/io/UnixFileSystem:initIDs:()V", Basic(void_stub));
    table.insert("java/io/FileInputStream:initIDs:()V", Basic(void_stub));
    table.insert(
        "jdk/internal/misc/ScopedMemoryAccess:registerNatives:()V",
        Basic(void_stub),
    );
    table.insert(
        "jdk/internal/misc/Signal:findSignal0:(Ljava/lang/String;)I",
        Basic(|args: &[i32]| {
            let _fd = args[0];
            Ok(vec![0])
        }),
    );
    table.insert(
        "jdk/internal/misc/Signal:handle0:(IJ)J",
        Basic(|args: &[i32]| {
            let _fd = args[0];
            Ok(vec![0, 0])
        }),
    );
    table.insert(
        "java/lang/Thread:currentThread:()Ljava/lang/Thread;",
        Basic(current_thread_wrp),
    );
    table.insert(
        "java/lang/Thread:currentCarrierThread:()Ljava/lang/Thread;",
        Basic(current_thread_wrp), //todo: use current carrier thread here (no matter what it is)
    );
    table.insert("java/lang/Thread:registerNatives:()V", Basic(void_stub));
    table.insert(
        "java/lang/Thread:getNextThreadIdOffset:()J",
        Basic(|_args: &[i32]| {
            Ok(vec![0, 1]) // it's always 1L, for spawning new threads real one should be incremented
        }),
    );
    table.insert("java/lang/Thread:setPriority0:(I)V", Basic(void_stub));
    table.insert("java/lang/Thread:start0:()V", Basic(void_stub));
    table.insert(
        "java/lang/ref/Finalizer:isFinalizationEnabled:()Z",
        Basic(|_args: &[i32]| {
            Ok(vec![0]) // false
        }),
    );
    table.insert(
        "java/security/AccessController:getStackAccessControlContext:()Ljava/security/AccessControlContext;",
        Basic(|_args: &[i32]| {
            Ok(vec![0]) // null
        }
    ));
    table.insert(
        "java/io/FileOutputStream:open0:(Ljava/lang/String;Z)V",
        Basic(file_output_stream_open0_wrp),
    );
    table.insert("java/io/FileOutputStream:initIDs:()V", Basic(void_stub));
    table.insert(
        "java/io/FileOutputStream:write:(IZ)V",
        Basic(file_output_stream_write_wrp),
    );
    table.insert(
        "java/io/FileOutputStream:writeBytes:([BIIZ)V",
        Basic(file_output_stream_write_bytes_wrp),
    );
    table.insert("java/lang/ref/Reference:clear0:()V", Basic(void_stub));
    table.insert(
        "java/lang/ref/Reference:refersTo0:(Ljava/lang/Object;)Z",
        Basic(|_args: &[i32]| {
            Ok(vec![0]) // todo: this should be implemented with GC
        }),
    );
    table.insert(
        "jdk/internal/reflect/Reflection:getCallerClass:()Ljava/lang/Class;",
        WithStackFrames(reflection_get_caller_class_wrp),
    );
    table.insert(
        "jdk/internal/reflect/Reflection:getClassAccessFlags:(Ljava/lang/Class;)I",
        Basic(reflection_get_class_access_flags_wrp),
    );
    table.insert(
        "jdk/internal/reflect/Reflection:areNestMates:(Ljava/lang/Class;Ljava/lang/Class;)Z",
        Basic(reflection_are_nest_mates_wrp),
    );
    table.insert(
        "java/security/AccessController:ensureMaterializedForStackWalk:(Ljava/lang/Object;)V",
        Basic(void_stub),
    );
    table.insert(
        "java/lang/reflect/Array:newArray:(Ljava/lang/Class;I)Ljava/lang/Object;",
        Basic(new_array_wrp),
    );
    table.insert(
        "java/lang/invoke/MethodHandleNatives:resolve:(Ljava/lang/invoke/MemberName;Ljava/lang/Class;IZ)Ljava/lang/invoke/MemberName;",
        Basic(method_handle_natives_resolve_wrp),
    );
    table.insert(
        "java/lang/invoke/MethodHandleNatives:registerNatives:()V",
        Basic(void_stub),
    );
    table.insert(
        "java/lang/invoke/MethodHandleNatives:objectFieldOffset:(Ljava/lang/invoke/MemberName;)J",
        Basic(method_handle_natives_object_field_offset_wrp),
    );
    table.insert(
        "java/lang/invoke/MethodHandleNatives:staticFieldOffset:(Ljava/lang/invoke/MemberName;)J",
        Basic(method_handle_natives_static_field_offset_wrp),
    );
    table.insert(
        "java/lang/invoke/MethodHandleNatives:staticFieldBase:(Ljava/lang/invoke/MemberName;)Ljava/lang/Object;",
        Basic(method_handle_natives_static_field_base_wrp),
    );
    table.insert(
        "java/lang/invoke/MethodHandleNatives:getNamedCon:(I[Ljava/lang/Object;)I",
        Basic(method_handle_natives_get_named_con_wrp),
    );
    table.insert(
        "java/lang/invoke/MethodHandleNatives:getMemberVMInfo:(Ljava/lang/invoke/MemberName;)Ljava/lang/Object;",
        Basic(method_handle_natives_get_member_vm_info_wrp),
    );
    table.insert(
        "java/lang/invoke/MethodHandle:invokeExact", // this is a normalized polymorphic signature
        WithMutStackFrames(method_handle_invoke_exact_wrp),
    );
    table.insert(
        "java/lang/invoke/MethodHandle:invokeBasic", // this is a normalized polymorphic signature
        WithMutStackFrames(method_handle_invoke_basic_wrp),
    );
    table.insert(
        "java/lang/ClassLoader:defineClass0:(Ljava/lang/ClassLoader;Ljava/lang/Class;Ljava/lang/String;[BIILjava/security/ProtectionDomain;ZILjava/lang/Object;)Ljava/lang/Class;",
        Basic(define_class0_wrp),
    );
    table.insert(
        "java/lang/ClassLoader:registerNatives:()V",
        Basic(void_stub),
    );
    table.insert(
        "java/lang/ClassLoader:findBootstrapClass:(Ljava/lang/String;)Ljava/lang/Class;",
        Basic(find_bootstrap_class_wrp),
    );
    table.insert(
        "jdk/internal/reflect/ConstantPool:getUTF8At0:(Ljava/lang/Object;I)Ljava/lang/String;",
        Basic(constant_pool_get_utf8_at0_wrp),
    );
    table.insert(
        "jdk/internal/reflect/ConstantPool:getSize0:(Ljava/lang/Object;)I",
        Basic(constant_pool_get_size0_wrp),
    );
    table.insert(
        "jdk/internal/reflect/ConstantPool:getTagAt0:(Ljava/lang/Object;I)B",
        Basic(constant_pool_get_tag_at0_wrp),
    );
    table.insert(
        "jdk/internal/loader/BootLoader:setBootLoaderUnnamedModule0:(Ljava/lang/Module;)V",
        Basic(void_stub),
    );
    table.insert(
        "jdk/internal/loader/NativeLibraries:findBuiltinLib:(Ljava/lang/String;)Ljava/lang/String;",
        Basic(find_builtin_lib_wrp),
    );
    table.insert(
        "jdk/internal/loader/NativeLibraries:load:(Ljdk/internal/loader/NativeLibraries$NativeLibraryImpl;Ljava/lang/String;ZZ)Z", // todo: should be implemented with shared library dynamic loading
        Basic(|_args: &[i32]| {
            Ok(vec![1]) // true
        }),
    );
    table.insert(
        "java/io/WinNTFileSystem:initIDs:()V", // this method is for caching `path` field from java/io/File for faster access in other native methods
        Basic(void_stub),
    );
    table.insert(
        "java/io/WinNTFileSystem:canonicalize0:(Ljava/lang/String;)Ljava/lang/String;",
        Basic(canonicalize0_wrp),
    );
    table.insert(
        "java/io/UnixFileSystem:canonicalize0:(Ljava/lang/String;)Ljava/lang/String;",
        Basic(canonicalize0_wrp),
    );
    table.insert(
        "java/io/WinNTFileSystem:createFileExclusively0:(Ljava/lang/String;)Z",
        Basic(create_file_exclusively0_wrp),
    );
    table.insert(
        "java/io/UnixFileSystem:createFileExclusively0:(Ljava/lang/String;)Z",
        Basic(create_file_exclusively0_wrp),
    );
    table.insert(
        "java/io/WinNTFileSystem:delete0:(Ljava/io/File;)Z",
        Basic(delete0_wrp),
    );
    table.insert(
        "java/io/UnixFileSystem:delete0:(Ljava/io/File;)Z",
        Basic(delete0_wrp),
    );
    table.insert(
        "java/io/WinNTFileSystem:getBooleanAttributes0:(Ljava/io/File;)I",
        Basic(get_boolean_attributes0_wrp),
    );
    table.insert(
        "java/io/UnixFileSystem:getBooleanAttributes0:(Ljava/io/File;)I",
        Basic(get_boolean_attributes0_wrp),
    );
    table.insert(
        "java/io/WinNTFileSystem:checkAccess0:(Ljava/io/File;I)Z",
        Basic(check_access0_wrp),
    );
    table.insert(
        "java/io/UnixFileSystem:checkAccess0:(Ljava/io/File;I)Z",
        Basic(check_access0_wrp),
    );
    table.insert("sun/nio/ch/IOUtil:initIDs:()V", Basic(void_stub));
    table.insert("sun/nio/ch/IOUtil:iovMax:()I", Basic(iov_max_wrp));
    table.insert("sun/nio/ch/IOUtil:writevMax:()J", Basic(writev_max_wrp));
    table.insert("sun/nio/ch/NativeThread:init:()V", Basic(void_stub));
    table.insert(
        "sun/nio/ch/NativeThread:current0:()J",
        Basic(|_args: &[i32]| Ok(i64_to_vec(0))), // todo: implement this (by 0 we say that the platform can not signal native threads)
    );
    table.insert(
        "java/lang/Throwable:fillInStackTrace:(I)Ljava/lang/Throwable;",
        Basic(|args| {
            let throwable_ref = args[0];
            Ok(vec![throwable_ref]) // fixme: this should be implemented with stack trace
        }),
    );

    platform_specific(&mut table);

    table
});

fn platform_specific(table: &mut HashMap<&'static str, NativeMethod>) {
    #[cfg(windows)]
    {
        use crate::system_native::platform_file_dispatcher::windows_file_dispatcher::{
            allocation_granularity0_wrp, windows_file_dispatcher_write0_wrp,
        };
        use crate::system_native::platform_native_dispatcher::windows_native_dispatcher::{
            create_directory0_wrp, create_file0_wrp, delete_file0_wrp,
            get_file_attributes_ex0_wrp, remove_directory0_wrp, set_end_of_file_wrp,
        };

        table.insert(
            "sun/nio/fs/WindowsNativeDispatcher:initIDs:()V",
            Basic(void_stub),
        );
        table.insert(
            "sun/nio/fs/WindowsNativeDispatcher:CreateDirectory0:(JJ)V",
            WithMutStackFrames(create_directory0_wrp),
        );
        table.insert(
            "sun/nio/fs/WindowsNativeDispatcher:GetFileAttributesEx0:(JJ)V",
            WithMutStackFrames(get_file_attributes_ex0_wrp),
        );
        table.insert(
            "sun/nio/fs/WindowsNativeDispatcher:DeleteFile0:(J)V",
            WithMutStackFrames(delete_file0_wrp),
        );
        table.insert(
            "sun/nio/fs/WindowsNativeDispatcher:RemoveDirectory0:(J)V",
            WithMutStackFrames(remove_directory0_wrp),
        );
        table.insert(
            "sun/nio/fs/WindowsNativeDispatcher:CreateFile0:(JIIJII)J",
            WithMutStackFrames(create_file0_wrp),
        );
        table.insert(
            "sun/nio/fs/WindowsNativeDispatcher:SetEndOfFile:(J)V",
            WithMutStackFrames(set_end_of_file_wrp),
        );
        table.insert(
            "sun/nio/ch/WindowsFileDispatcherImpl:allocationGranularity0:()J",
            Basic(allocation_granularity0_wrp),
        );
        table.insert(
            "sun/nio/ch/WindowsFileDispatcherImpl:maxDirectTransferSize0:()I", // Integer.MAX_VALUE - 1 is the maximum transfer size for TransmitFile()
            Basic(|_args| Ok(vec![i32::MAX - 1])),
        );
        table.insert(
            "sun/nio/ch/WindowsFileDispatcherImpl:write0:(Ljava/io/FileDescriptor;JIZ)I",
            WithMutStackFrames(windows_file_dispatcher_write0_wrp),
        );
    }

    #[cfg(unix)]
    {
        use crate::system_native::platform_file_dispatcher::unix_file_dispatcher::{
            unix_file_dispatcher_impl_read0_wrp, unix_file_dispatcher_impl_size0_wrp,
            unix_file_dispatcher_impl_write0_wrp,
        };
        use crate::system_native::platform_native_dispatcher::unix_native_dispatcher::{
            get_access0_wrp, get_cwd_wrp, lstat0_wrp, mkdir0_wrp, rmdir0_wrp, stat0_wrp,
            unix_native_dispatcher_open0_wrp, unlink0_wrp,
        };

        table.insert(
            "sun/nio/fs/UnixNativeDispatcher:getcwd:()[B",
            WithMutStackFrames(get_cwd_wrp),
        );
        table.insert(
            "sun/nio/fs/UnixNativeDispatcher:init:()I", // todo: return real capability flags
            Basic(|_args: &[i32]| Ok(vec![0])),
        );
        table.insert(
            "sun/nio/fs/UnixNativeDispatcher:open0:(JII)I",
            WithMutStackFrames(unix_native_dispatcher_open0_wrp),
        );
        table.insert(
            "sun/nio/fs/UnixNativeDispatcher:access0:(JI)I",
            Basic(get_access0_wrp),
        );
        table.insert(
            "sun/nio/fs/UnixNativeDispatcher:stat0:(JLsun/nio/fs/UnixFileAttributes;)I",
            Basic(stat0_wrp),
        );
        table.insert(
            "sun/nio/fs/UnixNativeDispatcher:lstat0:(JLsun/nio/fs/UnixFileAttributes;)V",
            WithMutStackFrames(lstat0_wrp),
        );
        table.insert(
            "sun/nio/fs/UnixNativeDispatcher:mkdir0:(JI)V",
            WithMutStackFrames(mkdir0_wrp),
        );
        table.insert(
            "sun/nio/fs/UnixNativeDispatcher:unlink0:(J)V",
            WithMutStackFrames(unlink0_wrp),
        );
        table.insert(
            "sun/nio/fs/UnixNativeDispatcher:rmdir0:(J)V",
            WithMutStackFrames(rmdir0_wrp),
        );

        table.insert(
            "sun/nio/ch/UnixFileDispatcherImpl:write0:(Ljava/io/FileDescriptor;JI)I",
            WithMutStackFrames(unix_file_dispatcher_impl_write0_wrp),
        );
        table.insert(
            "sun/nio/ch/UnixFileDispatcherImpl:read0:(Ljava/io/FileDescriptor;JI)I",
            WithMutStackFrames(unix_file_dispatcher_impl_read0_wrp),
        );
        table.insert(
            "sun/nio/ch/UnixFileDispatcherImpl:size0:(Ljava/io/FileDescriptor;)J",
            WithMutStackFrames(unix_file_dispatcher_impl_size0_wrp),
        );
    }

    #[cfg(target_os = "linux")]
    {
        table.insert(
            "sun/nio/ch/LinuxFileDispatcherImpl:init0:()V",
            Basic(void_stub),
        );
    }
}

pub(crate) fn invoke_native_method(
    method_signature: &str,
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> crate::error::Result<Vec<i32>> {
    let native_method = SYSTEM_NATIVE_TABLE.get(method_signature).ok_or_else(|| {
        Error::new_native(&format!("Native method {method_signature} not found"))
    })?;

    match native_method {
        Basic(method) => method(args),
        WithStackFrames(method) => method(args, stack_frames),
        WithMutStackFrames(method) => method(args, stack_frames),
    }
}

fn void_stub(_args: &[i32]) -> crate::error::Result<Vec<i32>> {
    Ok(vec![])
}
