use crate::error::Error;
use crate::helper::i64_to_vec;
use crate::system_native::class::{
    get_modifiers_wrp, get_primitive_class_wrp, is_array_wrp, is_primitive_wrp,
};
use crate::system_native::file_descriptor::file_descriptor_close0_wrp;
use crate::system_native::file_output_stream::{
    file_output_stream_open0_wrp, file_output_stream_write_bytes_wrp, file_output_stream_write_wrp,
};
use crate::system_native::object::{clone_wrp, get_class_wrp};
use crate::system_native::string::intern_wrp;
use crate::system_native::system::{arraycopy_wrp, current_time_millis_wrp};
use crate::system_native::system_props_raw::{platform_properties_wrp, vm_properties_wrp};
use crate::system_native::thread::current_thread_wrp;
use crate::system_native::unsafe_::{
    compare_and_set_int_wrp, compare_and_set_long_wrp, get_long_volatile_wrp,
    get_reference_volatile_wrp, object_field_offset_1_wrp, put_reference_volatile_wrp,
};
use once_cell::sync::Lazy;
use std::collections::HashMap;

static SYSTEM_NATIVE_TABLE: Lazy<
    HashMap<&'static str, fn(&[i32]) -> crate::error::Result<Vec<i32>>>,
> = Lazy::new(|| {
    let mut table = HashMap::new();
    table.insert(
        "java/lang/Object:getClass:()Ljava/lang/Class;",
        get_class_wrp as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "java/lang/Object:clone:()Ljava/lang/Object;",
        clone_wrp as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "java/lang/Object:notifyAll:()V",
        void_stub as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "java/lang/System:currentTimeMillis:()J",
        current_time_millis_wrp as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "java/lang/System:arraycopy:(Ljava/lang/Object;ILjava/lang/Object;II)V",
        arraycopy_wrp as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "java/lang/System:registerNatives:()V",
        void_stub as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "java/lang/System:setIn0:(Ljava/io/InputStream;)V",
        void_stub as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "java/lang/System:setOut0:(Ljava/io/PrintStream;)V",
        void_stub as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "java/lang/System:setErr0:(Ljava/io/PrintStream;)V",
        void_stub as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "java/lang/Class:getModifiers:()I",
        get_modifiers_wrp as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "java/lang/Class:getPrimitiveClass:(Ljava/lang/String;)Ljava/lang/Class;",
        get_primitive_class_wrp as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "java/lang/Class:desiredAssertionStatus0:(Ljava/lang/Class;)Z",
        bool_stub as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "java/lang/Class:isPrimitive:()Z",
        is_primitive_wrp as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "java/lang/Class:isArray:()Z",
        is_array_wrp as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "jdk/internal/misc/Unsafe:registerNatives:()V",
        void_stub as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "jdk/internal/misc/Unsafe:arrayBaseOffset0:(Ljava/lang/Class;)I",
        |_args: &[i32]| return_argument_stub(&vec![0]),
    );
    table.insert(
        "jdk/internal/misc/Unsafe:objectFieldOffset1:(Ljava/lang/Class;Ljava/lang/String;)J",
        object_field_offset_1_wrp as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "jdk/internal/misc/Unsafe:compareAndSetInt:(Ljava/lang/Object;JII)Z",
        compare_and_set_int_wrp as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "jdk/internal/misc/Unsafe:compareAndSetReference:(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z",
        compare_and_set_int_wrp as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "jdk/internal/misc/Unsafe:compareAndSetLong:(Ljava/lang/Object;JJJ)Z",
        compare_and_set_long_wrp as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "jdk/internal/misc/Unsafe:getReferenceVolatile:(Ljava/lang/Object;J)Ljava/lang/Object;",
        get_reference_volatile_wrp as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "jdk/internal/misc/Unsafe:getLongVolatile:(Ljava/lang/Object;J)J",
        get_long_volatile_wrp as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "jdk/internal/misc/Unsafe:arrayIndexScale0:(Ljava/lang/Class;)I",
        |_args: &[i32]| return_argument_stub(&vec![1]),
    );
    table.insert(
        "jdk/internal/misc/Unsafe:fullFence:()V",
        void_stub as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "jdk/internal/misc/Unsafe:getReference:(Ljava/lang/Object;J)Ljava/lang/Object;",
        get_reference_volatile_wrp as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "jdk/internal/misc/Unsafe:putReferenceVolatile:(Ljava/lang/Object;JLjava/lang/Object;)V",
        put_reference_volatile_wrp as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "java/lang/String:intern:()Ljava/lang/String;",
        intern_wrp as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "java/lang/Float:floatToRawIntBits:(F)I",
        return_argument_stub as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "java/lang/Double:doubleToRawLongBits:(D)J",
        |args: &[i32]| {
            let mut vec = args.to_vec();
            vec.reverse();
            return_argument_stub(&vec)
        },
    );
    table.insert(
        "java/lang/Double:longBitsToDouble:(J)D",
        |args: &[i32]| {
            let mut vec = args.to_vec();
            vec.reverse();
            return_argument_stub(&vec)
        },
    );
    table.insert(
        "jdk/internal/misc/CDS:initializeFromArchive:(Ljava/lang/Class;)V",
        void_stub as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "jdk/internal/misc/VM:initialize:()V",
        void_stub as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert("java/lang/Runtime:maxMemory:()J", |_args: &[i32]| {
        return_argument_stub(&i64_to_vec(i64::MAX))
    });
    table.insert(
        "java/lang/Runtime:availableProcessors:()I",
        |_args: &[i32]| return_argument_stub(&vec![14]),
    );
    table.insert(
        "jdk/internal/util/SystemProps$Raw:platformProperties:()[Ljava/lang/String;",
        platform_properties_wrp as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "jdk/internal/util/SystemProps$Raw:vmProperties:()[Ljava/lang/String;",
        vm_properties_wrp as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "java/io/FileDescriptor:initIDs:()V",
        void_stub as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert("java/io/FileDescriptor:getHandle:(I)J", |args: &[i32]| {
        return_argument_stub(&vec![0, args[0]])
    });
    table.insert("java/io/FileDescriptor:getAppend:(I)Z", |args: &[i32]| {
        let _fd = args[0];
        return_argument_stub(&vec![1])
    });
    table.insert(
        "java/io/FileDescriptor:close0:()V",
        file_descriptor_close0_wrp as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "jdk/internal/misc/ScopedMemoryAccess:registerNatives:()V",
        void_stub as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "jdk/internal/misc/Signal:findSignal0:(Ljava/lang/String;)I",
        |args: &[i32]| {
            let _fd = args[0];
            return_argument_stub(&vec![0])
        },
    );
    table.insert(
        "jdk/internal/misc/Signal:handle0:(IJ)J",
        |args: &[i32]| {
            let _fd = args[0];
            return_argument_stub(&vec![0, 0])
        },
    );
    table.insert(
        "java/lang/Thread:currentThread:()Ljava/lang/Thread;",
        current_thread_wrp as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "java/lang/Thread:registerNatives:()V",
        void_stub as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "java/lang/Thread:getNextThreadIdOffset:()J",
        |_args: &[i32]| {
            return_argument_stub(&vec![0, 1]) // it's always 1L, for spawning new threads real one should be incremented
        },
    );
    table.insert(
        "java/lang/Thread:setPriority0:(I)V",
        void_stub as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "java/lang/Thread:start0:()V",
        void_stub as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "java/lang/ref/Finalizer:isFinalizationEnabled:()Z",
        |_args: &[i32]| {
            return_argument_stub(&vec![0]) // false
        },
    );
    table.insert(
        "java/security/AccessController:getStackAccessControlContext:()Ljava/security/AccessControlContext;",
        |_args: &[i32]| {
            return_argument_stub(&vec![0]) // null
        }
    );
    table.insert(
        "java/io/FileOutputStream:open0:(Ljava/lang/String;Z)V",
        file_output_stream_open0_wrp as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "java/io/FileOutputStream:initIDs:()V",
        void_stub as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "java/io/FileOutputStream:write:(IZ)V",
        file_output_stream_write_wrp as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "java/io/FileOutputStream:writeBytes:([BIIZ)V",
        file_output_stream_write_bytes_wrp as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "java/lang/ref/Reference:clear0:()V",
        void_stub as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );

    table
});

pub(crate) fn invoke_native_method(
    method_signature: &str,
    args: &[i32],
) -> crate::error::Result<Vec<i32>> {
    let native_method = SYSTEM_NATIVE_TABLE.get(method_signature).ok_or_else(|| {
        Error::new_native(&format!("Native method {method_signature} not found"))
    })?;

    let result = native_method(args)?;

    Ok(result)
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
