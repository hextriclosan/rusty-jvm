use crate::error::Error;
use crate::execution_engine::system_native_table::NativeMethod::{Basic, WithStackFrames};
use crate::helper::i64_to_vec;
use crate::stack::stack_frame::StackFrames;
use crate::system_native::class::{
    class_init_class_name_wrp, for_name0_wrp, get_modifiers_wrp, get_primitive_class_wrp,
    is_array_wrp, is_interface_wrp, is_primitive_wrp,
};
use crate::system_native::file_descriptor::file_descriptor_close0_wrp;
use crate::system_native::file_output_stream::{
    file_output_stream_open0_wrp, file_output_stream_write_bytes_wrp, file_output_stream_write_wrp,
};
use crate::system_native::object::{clone_wrp, get_class_wrp, object_hashcode_wrp};
use crate::system_native::reflecton::reflection_get_caller_class_wrp;
use crate::system_native::string::intern_wrp;
use crate::system_native::system::{
    arraycopy_wrp, current_time_millis_wrp, system_identity_hashcode_wrp,
};
use crate::system_native::system_props_raw::{platform_properties_wrp, vm_properties_wrp};
use crate::system_native::thread::current_thread_wrp;
use crate::system_native::unsafe_::{
    compare_and_set_int_wrp, compare_and_set_long_wrp, get_long_volatile_wrp,
    get_reference_volatile_wrp, object_field_offset_1_wrp, put_reference_volatile_wrp,
};
use once_cell::sync::Lazy;
use std::collections::HashMap;

type BasicNativeMethod = fn(&[i32]) -> crate::error::Result<Vec<i32>>;
type WithStackFramesNativeMethod = fn(&[i32], &StackFrames) -> crate::error::Result<Vec<i32>>;

enum NativeMethod {
    Basic(BasicNativeMethod),
    WithStackFrames(WithStackFramesNativeMethod),
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
        Basic(void_stub),
    );
    table.insert(
        "java/lang/System:setErr0:(Ljava/io/PrintStream;)V",
        Basic(void_stub),
    );
    table.insert(
        "java/lang/System:identityHashCode:(Ljava/lang/Object;)I",
        Basic(system_identity_hashcode_wrp),
    );
    table.insert("java/lang/Class:getModifiers:()I", Basic(get_modifiers_wrp));
    table.insert(
        "java/lang/Class:getPrimitiveClass:(Ljava/lang/String;)Ljava/lang/Class;",
        Basic(get_primitive_class_wrp),
    );
    table.insert(
        "java/lang/Class:desiredAssertionStatus0:(Ljava/lang/Class;)Z",
        Basic(bool_stub),
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
        "jdk/internal/misc/Unsafe:registerNatives:()V",
        Basic(void_stub),
    );
    table.insert(
        "jdk/internal/misc/Unsafe:arrayBaseOffset0:(Ljava/lang/Class;)I",
        Basic(|_args: &[i32]| return_argument_stub(&vec![0])),
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
        "jdk/internal/misc/Unsafe:getLongVolatile:(Ljava/lang/Object;J)J",
        Basic(get_long_volatile_wrp),
    );
    table.insert(
        "jdk/internal/misc/Unsafe:arrayIndexScale0:(Ljava/lang/Class;)I",
        Basic(|_args: &[i32]| return_argument_stub(&vec![1])),
    );
    table.insert("jdk/internal/misc/Unsafe:fullFence:()V", Basic(void_stub));
    table.insert(
        "jdk/internal/misc/Unsafe:getReference:(Ljava/lang/Object;J)Ljava/lang/Object;",
        Basic(get_reference_volatile_wrp),
    );
    table.insert(
        "jdk/internal/misc/Unsafe:putReferenceVolatile:(Ljava/lang/Object;JLjava/lang/Object;)V",
        Basic(put_reference_volatile_wrp),
    );
    table.insert(
        "java/lang/String:intern:()Ljava/lang/String;",
        Basic(intern_wrp),
    );
    table.insert(
        "java/lang/Float:floatToRawIntBits:(F)I",
        Basic(return_argument_stub),
    );
    table.insert(
        "java/lang/Double:doubleToRawLongBits:(D)J",
        Basic(|args: &[i32]| {
            let mut vec = args.to_vec();
            vec.reverse();
            return_argument_stub(&vec)
        }),
    );
    table.insert(
        "java/lang/Double:longBitsToDouble:(J)D",
        Basic(|args: &[i32]| {
            let mut vec = args.to_vec();
            vec.reverse();
            return_argument_stub(&vec)
        }),
    );
    table.insert(
        "jdk/internal/misc/CDS:initializeFromArchive:(Ljava/lang/Class;)V",
        Basic(void_stub),
    );
    table.insert("jdk/internal/misc/VM:initialize:()V", Basic(void_stub));
    table.insert(
        "java/lang/Runtime:maxMemory:()J",
        Basic(|_args: &[i32]| return_argument_stub(&i64_to_vec(i64::MAX))),
    );
    table.insert(
        "java/lang/Runtime:availableProcessors:()I",
        Basic(|_args: &[i32]| return_argument_stub(&vec![14])),
    );
    table.insert(
        "jdk/internal/util/SystemProps$Raw:platformProperties:()[Ljava/lang/String;",
        Basic(platform_properties_wrp),
    );
    table.insert(
        "jdk/internal/util/SystemProps$Raw:vmProperties:()[Ljava/lang/String;",
        Basic(vm_properties_wrp),
    );
    table.insert(
        "java/io/FileDescriptor:initIDs:()V",
        Basic(void_stub),
    );
    table.insert(
        "java/io/FileDescriptor:getHandle:(I)J",
        Basic(|args: &[i32]| return_argument_stub(&vec![0, args[0]])),
    );
    table.insert(
        "java/io/FileDescriptor:getAppend:(I)Z",
        Basic(|args: &[i32]| {
            let _fd = args[0];
            return_argument_stub(&vec![1])
        }),
    );
    table.insert(
        "java/io/FileDescriptor:close0:()V",
        Basic(file_descriptor_close0_wrp),
    );
    table.insert(
        "jdk/internal/misc/ScopedMemoryAccess:registerNatives:()V",
        Basic(void_stub),
    );
    table.insert(
        "jdk/internal/misc/Signal:findSignal0:(Ljava/lang/String;)I",
        Basic(|args: &[i32]| {
            let _fd = args[0];
            return_argument_stub(&vec![0])
        }),
    );
    table.insert(
        "jdk/internal/misc/Signal:handle0:(IJ)J",
        Basic(|args: &[i32]| {
            let _fd = args[0];
            return_argument_stub(&vec![0, 0])
        }),
    );
    table.insert(
        "java/lang/Thread:currentThread:()Ljava/lang/Thread;",
        Basic(current_thread_wrp),
    );
    table.insert(
        "java/lang/Thread:registerNatives:()V",
        Basic(void_stub),
    );
    table.insert(
        "java/lang/Thread:getNextThreadIdOffset:()J",
        Basic(|_args: &[i32]| {
            return_argument_stub(&vec![0, 1]) // it's always 1L, for spawning new threads real one should be incremented
        }),
    );
    table.insert(
        "java/lang/Thread:setPriority0:(I)V",
        Basic(void_stub),
    );
    table.insert(
        "java/lang/Thread:start0:()V",
        Basic(void_stub),
    );
    table.insert(
        "java/lang/ref/Finalizer:isFinalizationEnabled:()Z",
        Basic(|_args: &[i32]| {
            return_argument_stub(&vec![0]) // false
        }),
    );
    table.insert(
        "java/security/AccessController:getStackAccessControlContext:()Ljava/security/AccessControlContext;",
        Basic(|_args: &[i32]| {
            return_argument_stub(&vec![0]) // null
        }
    ));
    table.insert(
        "java/io/FileOutputStream:open0:(Ljava/lang/String;Z)V",
        Basic(file_output_stream_open0_wrp),
    );
    table.insert(
        "java/io/FileOutputStream:initIDs:()V",
        Basic(void_stub),
    );
    table.insert(
        "java/io/FileOutputStream:write:(IZ)V",
        Basic(file_output_stream_write_wrp),
    );
    table.insert(
        "java/io/FileOutputStream:writeBytes:([BIIZ)V",
        Basic(file_output_stream_write_bytes_wrp),
    );
    table.insert(
        "java/lang/ref/Reference:clear0:()V",
        Basic(void_stub),
    );
    table.insert(
        "jdk/internal/reflect/Reflection:getCallerClass:()Ljava/lang/Class;",
        WithStackFrames(reflection_get_caller_class_wrp),
    );
    table.insert(
        "java/security/AccessController:ensureMaterializedForStackWalk:(Ljava/lang/Object;)V",
        Basic(void_stub),
    );

    table
});

pub(crate) fn invoke_native_method(
    method_signature: &str,
    args: &[i32],
    stack_frames: &StackFrames,
) -> crate::error::Result<Vec<i32>> {
    let native_method = SYSTEM_NATIVE_TABLE.get(method_signature).ok_or_else(|| {
        Error::new_native(&format!("Native method {method_signature} not found"))
    })?;

    match native_method {
        Basic(method) => method(args),
        WithStackFrames(method) => method(args, stack_frames),
    }
}

fn void_stub(_args: &[i32]) -> crate::error::Result<Vec<i32>> {
    Ok(vec![])
}

fn bool_stub(_args: &[i32]) -> crate::error::Result<Vec<i32>> {
    Ok(vec![false as i32])
}

fn return_argument_stub(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    Ok(args.to_vec())
}
