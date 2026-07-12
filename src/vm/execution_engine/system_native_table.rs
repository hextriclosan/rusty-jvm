use crate::vm::error::Result;
use crate::vm::execution_engine::system_native_table::NativeMethod::{
    Basic, WithMutStackFrames, WithStackFrames,
};
use crate::vm::helper::i64_to_vec;
use crate::vm::stack::stack_frame::StackFrames;
use crate::vm::system_native::class_loader::{
    define_class0_wrp, define_class1_wrp, define_class2_wrp, find_bootstrap_class_wrp,
    find_loaded_class_wrp,
};
use crate::vm::system_native::constant_pool::{
    constant_pool_get_size0_wrp, constant_pool_get_tag_at0_wrp, constant_pool_get_utf8_at0_wrp,
};
use crate::vm::system_native::dispatcher::invoke::invoke;
use crate::vm::system_native::io_util::{iov_max_wrp, writev_max_wrp};
use crate::vm::system_native::method_handle_natives::wrappers::{
    method_handle_invoke_basic_wrp, method_handle_invoke_exact_wrp, method_handle_invoke_wrp,
    method_handle_natives_get_member_vm_info_wrp, method_handle_natives_get_named_con_wrp,
    method_handle_natives_init_wrp, method_handle_natives_object_field_offset_wrp,
    method_handle_natives_resolve_wrp, method_handle_natives_static_field_base_wrp,
    method_handle_natives_static_field_offset_wrp, native_accessor_invoke0_wrp,
    native_accessor_newinstance0_wrp, set_call_site_target_normal_wrp,
    var_handle_compare_and_set_wrp, var_handle_get_wrp, var_handle_set_wrp,
};
use crate::vm::system_native::native_libraries::{
    find_builtin_lib_wrp, native_libraries_find_entry0_wrp, native_libraries_load_wrp,
};
use crate::vm::system_native::perf::{perf_create_byte_array_wrp, perf_create_long_wrp};
use crate::vm::system_native::reflect_array::new_array_wrp;
use crate::vm::system_native::reflecton::{
    reflection_are_nest_mates_wrp, reflection_get_caller_class_wrp,
    reflection_get_class_access_flags_wrp,
};
use crate::vm::system_native::stack_trace_element::init_stack_trace_elements_wrp;
use crate::vm::system_native::thread::{current_thread_wrp, get_next_threadid_offset_wrp};
use crate::vm::system_native::throwable::fill_in_stack_trace_wrp;
use crate::vm::system_native::time_zone::get_system_time_zone_id_wrp;
use crate::vm::system_native::zip::deflater::{
    java_util_zip_deflater_deflate_bytes_bytes_wrp, java_util_zip_deflater_end_wrp,
    java_util_zip_deflater_init_wrp,
};
use crate::vm::system_native::zip::inflater::{
    java_util_zip_inflater_end_wrp, java_util_zip_inflater_inflate_bytes_bytes_wrp,
    java_util_zip_inflater_init_wrp, java_util_zip_inflater_initids_wrp,
    java_util_zip_inflater_reset_wrp,
};
use once_cell::sync::Lazy;
use std::collections::HashMap;

type BasicNativeMethod = fn(&[i32]) -> Result<Vec<i32>>;
type WithStackFramesNativeMethod = fn(&[i32], &StackFrames) -> Result<Vec<i32>>;
type WithMutStackFramesNativeMethod = fn(&[i32], &mut StackFrames) -> Result<Vec<i32>>;

enum NativeMethod {
    Basic(BasicNativeMethod),
    WithStackFrames(WithStackFramesNativeMethod),
    WithMutStackFrames(WithMutStackFramesNativeMethod),
}

static SYSTEM_NATIVE_TABLE: Lazy<HashMap<&'static str, NativeMethod>> = Lazy::new(|| {
    let mut table = HashMap::new();
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
        "jdk/internal/perf/Perf:registerNatives:()V",
        Basic(void_stub),
    );
    table.insert(
        "jdk/internal/perf/Perf:createLong:(Ljava/lang/String;IIJ)Ljava/nio/ByteBuffer;",
        WithMutStackFrames(perf_create_long_wrp),
    );
    table.insert(
        "jdk/internal/perf/Perf:createByteArray:(Ljava/lang/String;II[BI)Ljava/nio/ByteBuffer;",
        WithMutStackFrames(perf_create_byte_array_wrp),
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
        "java/lang/Thread:holdsLock:(Ljava/lang/Object;)Z",
        Basic(|_args: &[i32]| Ok(vec![1])),
    ); // todo: implement this properly
    table.insert(
        "java/lang/Thread:getNextThreadIdOffset:()J",
        Basic(get_next_threadid_offset_wrp),
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
    table.insert("java/lang/ref/Reference:clear0:()V", Basic(void_stub));
    table.insert(
        "java/lang/ref/Reference:refersTo0:(Ljava/lang/Object;)Z",
        Basic(|_args: &[i32]| {
            Ok(vec![0]) // todo: this should be implemented with GC
        }),
    );
    table.insert(
        "java/lang/ref/PhantomReference:clear0:()V",
        Basic(void_stub), // todo: this should be implemented with GC
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
        "java/lang/invoke/MethodHandleNatives:init:(Ljava/lang/invoke/MemberName;Ljava/lang/Object;)V",
        Basic(method_handle_natives_init_wrp),
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
        "java/lang/invoke/MethodHandleNatives:setCallSiteTargetNormal:(Ljava/lang/invoke/CallSite;Ljava/lang/invoke/MethodHandle;)V",
        Basic(set_call_site_target_normal_wrp),
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
        "java/lang/invoke/MethodHandle:invoke", // this is a normalized polymorphic signature
        WithMutStackFrames(method_handle_invoke_wrp),
    );
    table.insert(
        "java/lang/invoke/VarHandle:set", // this is a normalized polymorphic signature
        Basic(var_handle_set_wrp),
    );
    table.insert(
        "java/lang/invoke/VarHandle:get", // this is a normalized polymorphic signature
        Basic(var_handle_get_wrp),
    );
    table.insert(
        "java/lang/invoke/VarHandle:compareAndSet", // this is a normalized polymorphic signature
        Basic(var_handle_compare_and_set_wrp),
    );
    table.insert(
        "java/lang/ClassLoader:defineClass0:(Ljava/lang/ClassLoader;Ljava/lang/Class;Ljava/lang/String;[BIILjava/security/ProtectionDomain;ZILjava/lang/Object;)Ljava/lang/Class;",
        Basic(define_class0_wrp),
    );
    table.insert(
        "java/lang/ClassLoader:defineClass1:(Ljava/lang/ClassLoader;Ljava/lang/String;[BIILjava/security/ProtectionDomain;Ljava/lang/String;)Ljava/lang/Class;",
        Basic(define_class1_wrp),
    );
    table.insert(
        "java/lang/ClassLoader:defineClass2:(Ljava/lang/ClassLoader;Ljava/lang/String;Ljava/nio/ByteBuffer;IILjava/security/ProtectionDomain;Ljava/lang/String;)Ljava/lang/Class;",
        Basic(define_class2_wrp),
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
        "java/lang/ClassLoader:findLoadedClass0:(Ljava/lang/String;)Ljava/lang/Class;",
        Basic(find_loaded_class_wrp),
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
        "jdk/internal/loader/NativeLibraries:load:(Ljdk/internal/loader/NativeLibraries$NativeLibraryImpl;Ljava/lang/String;ZZ)Z",
        WithMutStackFrames(native_libraries_load_wrp),
    );
    table.insert(
        "jdk/internal/loader/NativeLibraries$NativeLibraryImpl:findEntry0:(JLjava/lang/String;)J",
        Basic(native_libraries_find_entry0_wrp),
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
        "java/nio/MappedMemoryUtils:registerNatives:()V",
        Basic(void_stub),
    );
    table.insert(
        "java/lang/Throwable:fillInStackTrace:(I)Ljava/lang/Throwable;",
        WithStackFrames(fill_in_stack_trace_wrp),
    );
    table.insert(
        "java/lang/StackTraceElement:initStackTraceElements:([Ljava/lang/StackTraceElement;Ljava/lang/Object;I)V",
        Basic(init_stack_trace_elements_wrp),
    );
    table.insert(
        "jdk/internal/reflect/DirectMethodHandleAccessor$NativeAccessor:invoke0:(Ljava/lang/reflect/Method;Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;",
        Basic(native_accessor_invoke0_wrp),
    );
    table.insert(
        "jdk/internal/reflect/DirectConstructorHandleAccessor$NativeAccessor:newInstance0:(Ljava/lang/reflect/Constructor;[Ljava/lang/Object;)Ljava/lang/Object;",
        Basic(native_accessor_newinstance0_wrp),
    );
    table.insert(
        "jdk/internal/vm/ContinuationSupport:isSupported0:()Z",
        Basic(|_args: &[i32]| Ok(vec![0])), // We do not support Loom continuations (yet)
    );
    table.insert(
        "java/util/zip/Deflater:init:(IIZ)J",
        Basic(java_util_zip_deflater_init_wrp),
    );
    table.insert(
        "java/util/zip/Deflater:deflateBytesBytes:(J[BII[BIIII)J",
        Basic(java_util_zip_deflater_deflate_bytes_bytes_wrp),
    );
    table.insert(
        "java/util/zip/Deflater:end:(J)V",
        Basic(java_util_zip_deflater_end_wrp),
    );
    table.insert(
        "java/util/zip/Inflater:initIDs:()V",
        Basic(java_util_zip_inflater_initids_wrp),
    );
    table.insert(
        "java/util/zip/Inflater:init:(Z)J",
        Basic(java_util_zip_inflater_init_wrp),
    );
    table.insert(
        "java/util/zip/Inflater:inflateBytesBytes:(J[BII[BII)J",
        Basic(java_util_zip_inflater_inflate_bytes_bytes_wrp),
    );
    table.insert(
        "java/util/zip/Inflater:end:(J)V",
        Basic(java_util_zip_inflater_end_wrp),
    );
    table.insert(
        "java/util/zip/Inflater:reset:(J)V",
        Basic(java_util_zip_inflater_reset_wrp),
    );
    table.insert(
        "java/lang/NullPointerException:getExtendedNPEMessage:()Ljava/lang/String;",
        Basic(|_args: &[i32]| Ok(vec![0])), // todo: https://github.com/hextriclosan/rusty-jvm/issues/521
    );
    table.insert(
        "java/io/Console:istty:()Z",
        Basic(|_args: &[i32]| Ok(vec![1])), // todo implement me
    );
    table.insert("java/net/NetworkInterface:init:()V", Basic(void_stub));
    table.insert(
        "java/net/NetworkInterface:getAll:()[Ljava/net/NetworkInterface;",
        Basic(|_args: &[i32]| Ok(vec![0])), // fixme: https://github.com/hextriclosan/rusty-jvm/issues/539
    );
    table.insert(
        "java/lang/Runtime:totalMemory:()J",
        Basic(|_args: &[i32]| Ok(i64_to_vec(i64::MAX))), // todo implement me with GC
    );
    table.insert(
        "java/lang/Runtime:freeMemory:()J",
        Basic(|_args: &[i32]| Ok(i64_to_vec(i64::MAX))), // todo implement me with GC
    );
    table.insert(
        "jdk/internal/misc/PreviewFeatures:isPreviewEnabled:()Z",
        Basic(|_args: &[i32]| Ok(vec![0])),
    );
    table.insert(
        "java/util/TimeZone:getSystemTimeZoneID:(Ljava/lang/String;)Ljava/lang/String;",
        Basic(get_system_time_zone_id_wrp),
    );

    #[cfg(windows)]
    platform_specific(&mut table);

    table
});

#[cfg(windows)]
fn platform_specific(table: &mut HashMap<&'static str, NativeMethod>) {
    use crate::vm::system_native::native_seed_generator::native_generate_seed_wrp;
    use crate::vm::system_native::platform_specific_files::win32_error_mode::set_error_mode_wrp;

    table.insert(
        "sun/io/Win32ErrorMode:setErrorMode:(J)J",
        Basic(set_error_mode_wrp),
    );
    table.insert(
        "sun/security/provider/NativeSeedGenerator:nativeGenerateSeed:([B)Z",
        Basic(native_generate_seed_wrp),
    );
}

pub(crate) fn invoke_native_method(
    method_signature: &str,
    args: &[i32],
    is_static: bool,
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let native_method = match SYSTEM_NATIVE_TABLE.get(method_signature) {
        Some(native_method) => native_method,
        None => return invoke(method_signature, args, is_static),
    };

    match native_method {
        Basic(method) => method(args),
        WithStackFrames(method) => method(args, stack_frames),
        WithMutStackFrames(method) => method(args, stack_frames),
    }
}

fn void_stub(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(vec![])
}
