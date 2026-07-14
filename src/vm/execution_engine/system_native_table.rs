use crate::vm::error::Result;
use crate::vm::execution_engine::system_native_table::NativeMethod::{
    Basic, WithMutStackFrames, WithStackFrames,
};
use crate::vm::helper::i64_to_vec;
use crate::vm::stack::stack_frame::StackFrames;
use crate::vm::system_native::constant_pool::{
    constant_pool_get_size0_wrp, constant_pool_get_tag_at0_wrp, constant_pool_get_utf8_at0_wrp,
};
use crate::vm::system_native::dispatcher::invoke::invoke;
use crate::vm::system_native::io_util::{iov_max_wrp, writev_max_wrp};
use crate::vm::system_native::method_handle_natives::wrappers::{
    method_handle_invoke_basic_wrp, method_handle_invoke_exact_wrp, method_handle_invoke_wrp,
    native_accessor_invoke0_wrp, native_accessor_newinstance0_wrp, var_handle_compare_and_set_wrp,
    var_handle_get_wrp, var_handle_set_wrp,
};
use crate::vm::system_native::native_libraries::{
    find_builtin_lib_wrp, native_libraries_find_entry0_wrp, native_libraries_load_wrp,
};
use crate::vm::system_native::reflect_array::new_array_wrp;
use crate::vm::system_native::reflecton::{
    reflection_are_nest_mates_wrp, reflection_get_caller_class_wrp,
    reflection_get_class_access_flags_wrp,
};
use crate::vm::system_native::stack_trace_element::init_stack_trace_elements_wrp;
use crate::vm::system_native::throwable::fill_in_stack_trace_wrp;
use crate::vm::system_native::time_zone::get_system_time_zone_id_wrp;
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

    table
});

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
