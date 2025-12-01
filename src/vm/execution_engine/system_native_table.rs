use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::system_native_table::NativeMethod::{
    Basic, WithMutStackFrames, WithStackFrames,
};
use crate::vm::helper::i64_to_vec;
use crate::vm::stack::stack_frame::StackFrames;
use crate::vm::system_native::class::{
    class_init_class_name_wrp, class_is_instance_wrp, for_name0_wrp, get_constant_pool_wrp,
    get_declared_constructors0_wrp, get_declared_fields0_wrp, get_declared_methods0_wrp,
    get_declaring_class0_wrp, get_enclosing_method0_wrp, get_interfaces0_wrp, get_nest_host0_wrp,
    get_primitive_class_wrp, get_raw_annotations_wrp, get_simple_binary_name0_wrp,
    get_superclass_wrp, is_assignable_from_wrp, is_record0_wrp,
};
use crate::vm::system_native::class_loader::{
    define_class0_wrp, define_class2_wrp, find_bootstrap_class_wrp, find_loaded_class_wrp,
};
use crate::vm::system_native::constant_pool::{
    constant_pool_get_size0_wrp, constant_pool_get_tag_at0_wrp, constant_pool_get_utf8_at0_wrp,
};
use crate::vm::system_native::file_descriptor::{file_descriptor_close0_wrp, get_handle_wrp};
use crate::vm::system_native::file_input_stream::{
    file_input_stream_available0_wrp, file_input_stream_is_regular_file0_wrp,
    file_input_stream_length0_wrp, file_input_stream_open0_wrp, file_input_stream_position0_wrp,
    file_input_stream_read0_wrp, file_input_stream_read_bytes_wrp,
};
use crate::vm::system_native::file_output_stream::{
    file_output_stream_open0_wrp, file_output_stream_write_bytes_wrp, file_output_stream_write_wrp,
};
use crate::vm::system_native::io_file_system::{
    canonicalize0_wrp, check_access0_wrp, create_file_exclusively0_wrp,
    get_boolean_attributes0_wrp, get_length0_wrp,
};
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
use crate::vm::system_native::module::{
    add_exports0_wrp, add_exports_to_all0_wrp, add_reads0_wrp, define_module0_wrp,
};
use crate::vm::system_native::native_image_buffer::get_native_map_wrp;
use crate::vm::system_native::native_libraries::find_builtin_lib_wrp;
use crate::vm::system_native::object::{clone_wrp, get_class_wrp, object_hashcode_wrp};
use crate::vm::system_native::platform_file_dispatcher::{
    allocation_granularity0_wrp, file_dispatcher_impl_truncate0_wrp, file_dispatcher_map0_wrp,
    mapped_memory_utils_force0_wrp,
};
use crate::vm::system_native::random_access_file::{
    random_access_file_open0_wrp, random_access_file_read_bytes0_wrp,
    random_access_file_seek0_wrp, random_access_file_write_bytes0_wrp,
};
use crate::vm::system_native::reflect_array::new_array_wrp;
use crate::vm::system_native::reflecton::{
    reflection_are_nest_mates_wrp, reflection_get_caller_class_wrp,
    reflection_get_class_access_flags_wrp,
};
use crate::vm::system_native::stack_trace_element::init_stack_trace_elements_wrp;
use crate::vm::system_native::string::intern_wrp;
use crate::vm::system_native::system::{
    arraycopy_wrp, current_time_millis_wrp, nano_time_wrp, set_err0_wrp, set_out0_wrp,
    system_identity_hashcode_wrp, system_map_library_name_wrp,
};
use crate::vm::system_native::system_props_raw::{platform_properties_wrp, vm_properties_wrp};
use crate::vm::system_native::thread::{current_thread_wrp, get_next_threadid_offset_wrp};
use crate::vm::system_native::throwable::fill_in_stack_trace_wrp;
use crate::vm::system_native::time_zone::get_system_time_zone_id_wrp;
use crate::vm::system_native::unsafe_::{
    allocate_memory0_wrp, array_index_scale0_wrp, compare_and_exchange_long_wrp,
    compare_and_set_int_wrp, compare_and_set_long_wrp, copy_memory0_wrp,
    ensure_class_initialized0_wrp, get_boolean_volatile_wrp, get_byte_wrp, get_char_wrp,
    get_int_volatile_wrp, get_int_wrp, get_long_volatile_wrp, get_long_wrp,
    get_reference_volatile_wrp, get_short_wrp, object_field_offset_0_wrp,
    object_field_offset_1_wrp, put_byte_wrp, put_char_wrp, put_int_volatile_wrp, put_int_wrp,
    put_long_wrp, put_reference_volatile_wrp, put_reference_wrp, put_short_wrp, set_memory0_wrp,
    should_be_initialized0_wrp, static_field_base0_wrp, static_field_offset_0_wrp,
};
use crate::vm::system_native::zip::crc32::java_util_zip_crc32_updatebytes0_wrp;
use crate::vm::system_native::zip::deflater::{
    java_util_zip_deflater_deflate_bytes_bytes_wrp, java_util_zip_deflater_end_wrp,
    java_util_zip_deflater_init_wrp,
};
use crate::vm::system_native::zip::inflater::{
    java_util_zip_inflater_end_wrp, java_util_zip_inflater_inflate_bytes_bytes_wrp,
    java_util_zip_inflater_init_wrp, java_util_zip_inflater_initids_wrp,
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
    table.insert("java/lang/System:nanoTime:()J", Basic(nano_time_wrp));
    table.insert(
        "java/lang/System:arraycopy:(Ljava/lang/Object;ILjava/lang/Object;II)V",
        WithMutStackFrames(arraycopy_wrp),
    );
    table.insert("java/lang/System:registerNatives:()V", Basic(void_stub));
    table.insert(
        "java/lang/System:setIn0:(Ljava/io/InputStream;)V",
        Basic(void_stub), // todo: implement me
    );
    table.insert(
        "java/lang/System:setOut0:(Ljava/io/PrintStream;)V",
        Basic(set_out0_wrp),
    );
    table.insert(
        "java/lang/System:setErr0:(Ljava/io/PrintStream;)V",
        Basic(set_err0_wrp),
    );
    table.insert(
        "java/lang/System:identityHashCode:(Ljava/lang/Object;)I",
        Basic(system_identity_hashcode_wrp),
    );
    table.insert(
        "java/lang/System:mapLibraryName:(Ljava/lang/String;)Ljava/lang/String;",
        Basic(system_map_library_name_wrp),
    );
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
    table.insert("java/lang/Class:forName0:(Ljava/lang/String;ZLjava/lang/ClassLoader;Ljava/lang/Class;)Ljava/lang/Class;", WithMutStackFrames(for_name0_wrp));
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
        "java/lang/Class:getDeclaredFields0:(Z)[Ljava/lang/reflect/Field;",
        Basic(get_declared_fields0_wrp),
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
    table.insert("java/lang/Class:isRecord0:()Z", Basic(is_record0_wrp));
    table.insert("java/lang/Module:defineModule0:(Ljava/lang/Module;ZLjava/lang/String;Ljava/lang/String;[Ljava/lang/Object;)V", Basic(define_module0_wrp));
    table.insert(
        "java/lang/Module:addReads0:(Ljava/lang/Module;Ljava/lang/Module;)V",
        Basic(add_reads0_wrp),
    );
    table.insert(
        "java/lang/Module:addExportsToAll0:(Ljava/lang/Module;Ljava/lang/String;)V",
        Basic(add_exports_to_all0_wrp),
    );
    table.insert(
        "java/lang/Module:addExports0:(Ljava/lang/Module;Ljava/lang/String;Ljava/lang/Module;)V",
        Basic(add_exports0_wrp),
    );
    table.insert(
        "java/lang/Shutdown:beforeHalt:()V",
        Basic(void_stub), // todo: implement me
    );
    table.insert(
        "java/lang/Shutdown:halt0:(I)V",
        Basic(|args: &[i32]| {
            let status = args[0];
            std::process::exit(status); // fixme: by doing this we skip destructors and other shutdown hooks, later it might be an issue
        }),
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
        "jdk/internal/misc/Unsafe:objectFieldOffset0:(Ljava/lang/reflect/Field;)J",
        Basic(object_field_offset_0_wrp),
    );
    table.insert(
        "jdk/internal/misc/Unsafe:objectFieldOffset1:(Ljava/lang/Class;Ljava/lang/String;)J",
        Basic(object_field_offset_1_wrp),
    );
    table.insert(
        "jdk/internal/misc/Unsafe:staticFieldOffset0:(Ljava/lang/reflect/Field;)J",
        Basic(static_field_offset_0_wrp),
    );
    table.insert(
        "jdk/internal/misc/Unsafe:staticFieldBase0:(Ljava/lang/reflect/Field;)Ljava/lang/Object;",
        Basic(static_field_base0_wrp),
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
        "jdk/internal/misc/Unsafe:compareAndExchangeLong:(Ljava/lang/Object;JJJ)J",
        Basic(compare_and_exchange_long_wrp),
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
        "jdk/internal/misc/Unsafe:getChar:(Ljava/lang/Object;J)C",
        Basic(get_char_wrp),
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
        "jdk/internal/misc/Unsafe:getBooleanVolatile:(Ljava/lang/Object;J)Z",
        Basic(get_boolean_volatile_wrp),
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
        "jdk/internal/misc/Unsafe:putShort:(Ljava/lang/Object;JS)V",
        Basic(put_short_wrp),
    );
    table.insert(
        "jdk/internal/misc/Unsafe:putInt:(Ljava/lang/Object;JI)V",
        Basic(put_int_wrp),
    );
    table.insert(
        "jdk/internal/misc/Unsafe:putIntVolatile:(Ljava/lang/Object;JI)V",
        Basic(put_int_volatile_wrp),
    );
    table.insert(
        "jdk/internal/misc/Unsafe:putLong:(Ljava/lang/Object;JJ)V",
        Basic(put_long_wrp),
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
    table.insert("jdk/internal/jimage/NativeImageBuffer:getNativeMap:(Ljava/lang/String;)Ljava/nio/ByteBuffer;", Basic(get_native_map_wrp));
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
        "java/io/FileInputStream:open0:(Ljava/lang/String;)V",
        WithMutStackFrames(file_input_stream_open0_wrp),
    );
    table.insert(
        "java/io/FileInputStream:length0:()J",
        WithMutStackFrames(file_input_stream_length0_wrp),
    );
    table.insert(
        "java/io/FileInputStream:position0:()J",
        WithMutStackFrames(file_input_stream_position0_wrp),
    );
    table.insert(
        "java/io/FileInputStream:available0:()I",
        WithMutStackFrames(file_input_stream_available0_wrp),
    );
    table.insert(
        "java/io/FileInputStream:readBytes:([BII)I",
        WithMutStackFrames(file_input_stream_read_bytes_wrp),
    );
    table.insert(
        "java/io/FileInputStream:read0:()I",
        WithMutStackFrames(file_input_stream_read0_wrp),
    );
    table.insert(
        "java/io/FileInputStream:isRegularFile0:(Ljava/io/FileDescriptor;)Z",
        WithMutStackFrames(file_input_stream_is_regular_file0_wrp),
    );
    table.insert("java/io/RandomAccessFile:initIDs:()V", Basic(void_stub));
    table.insert(
        "java/io/RandomAccessFile:open0:(Ljava/lang/String;I)V",
        WithMutStackFrames(random_access_file_open0_wrp),
    );
    table.insert(
        "java/io/RandomAccessFile:seek0:(J)V",
        WithMutStackFrames(random_access_file_seek0_wrp),
    );
    table.insert(
        "java/io/RandomAccessFile:writeBytes0:([BII)V",
        WithMutStackFrames(random_access_file_write_bytes0_wrp),
    );
    table.insert(
        "java/io/RandomAccessFile:readBytes0:([BII)I",
        WithMutStackFrames(random_access_file_read_bytes0_wrp),
    );
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
    table.insert(
        "java/io/FileOutputStream:open0:(Ljava/lang/String;Z)V",
        WithMutStackFrames(file_output_stream_open0_wrp),
    );
    table.insert("java/io/FileOutputStream:initIDs:()V", Basic(void_stub));
    table.insert(
        "java/io/FileOutputStream:write:(IZ)V",
        WithMutStackFrames(file_output_stream_write_wrp),
    );
    table.insert(
        "java/io/FileOutputStream:writeBytes:([BIIZ)V",
        WithMutStackFrames(file_output_stream_write_bytes_wrp),
    );
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
        "java/util/zip/CRC32:updateBytes0:(I[BII)I",
        Basic(java_util_zip_crc32_updatebytes0_wrp),
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

    platform_specific(&mut table);

    table
});

fn platform_specific(table: &mut HashMap<&'static str, NativeMethod>) {
    #[cfg(windows)]
    {
        use crate::vm::system_native::native_seed_generator::native_generate_seed_wrp;
        use crate::vm::system_native::platform_file_dispatcher::windows_file_dispatcher::{
            windows_file_dispatcher_duplicate_handle_wrp, windows_file_dispatcher_pread0_wrp,
            windows_file_dispatcher_read0_wrp, windows_file_dispatcher_size0_wrp,
            windows_file_dispatcher_write0_wrp,
        };
        use crate::vm::system_native::platform_native_dispatcher::windows_native_dispatcher::{
            access_check_wrp, close_handle_wrp, create_directory0_wrp, create_file0_wrp,
            delete_file0_wrp, duplicate_token_ex_wrp, find_close_wrp, find_first_file0_wrp,
            find_next_file0_wrp, format_message_wrp, get_current_process_wrp,
            get_current_thread_wrp, get_drive_type0_wrp, get_file_attributes_ex0_wrp,
            get_file_security0_wrp, get_full_path_name0_wrp, get_volume_information0_wrp,
            get_volume_path_name0_wrp, open_process_token_wrp, open_thread_token_wrp,
            remove_directory0_wrp, set_end_of_file_wrp,
        };
        use crate::vm::system_native::platform_specific_files::win32_error_mode::set_error_mode_wrp;
        use crate::vm::system_native::platform_specific_files::winnt_file_system::{
            get_final_path0_wrp, get_name_max0_wrp, winnt_file_system_delete0_wrp,
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
            "sun/nio/fs/WindowsNativeDispatcher:GetFileSecurity0:(JIJI)I",
            WithMutStackFrames(get_file_security0_wrp),
        );
        table.insert(
            "sun/nio/fs/WindowsNativeDispatcher:GetCurrentProcess:()J",
            Basic(get_current_process_wrp),
        );
        table.insert(
            "sun/nio/fs/WindowsNativeDispatcher:OpenProcessToken:(JI)J",
            WithMutStackFrames(open_process_token_wrp),
        );
        table.insert(
            "sun/nio/fs/WindowsNativeDispatcher:GetCurrentThread:()J",
            Basic(get_current_thread_wrp),
        );
        table.insert(
            "sun/nio/fs/WindowsNativeDispatcher:OpenThreadToken:(JIZ)J",
            WithMutStackFrames(open_thread_token_wrp),
        );
        table.insert(
            "sun/nio/fs/WindowsNativeDispatcher:DuplicateTokenEx:(JI)J",
            WithMutStackFrames(duplicate_token_ex_wrp),
        );
        table.insert(
            "sun/nio/fs/WindowsNativeDispatcher:AccessCheck:(JJIIIII)Z",
            WithMutStackFrames(access_check_wrp),
        );
        table.insert(
            "sun/nio/fs/WindowsNativeDispatcher:CloseHandle:(J)V",
            Basic(close_handle_wrp),
        );
        table.insert(
            "sun/nio/fs/WindowsNativeDispatcher:GetVolumePathName0:(J)Ljava/lang/String;",
            WithMutStackFrames(get_volume_path_name0_wrp),
        );
        table.insert(
            "sun/nio/fs/WindowsNativeDispatcher:GetVolumeInformation0:(JLsun/nio/fs/WindowsNativeDispatcher$VolumeInformation;)V",
            WithMutStackFrames(get_volume_information0_wrp),
        );
        table.insert(
            "sun/nio/fs/WindowsNativeDispatcher:GetDriveType0:(J)I",
            Basic(get_drive_type0_wrp),
        );
        table.insert(
            "sun/nio/fs/WindowsNativeDispatcher:FormatMessage:(I)Ljava/lang/String;",
            Basic(format_message_wrp),
        );
        table.insert(
            "sun/nio/fs/WindowsNativeDispatcher:GetFullPathName0:(J)Ljava/lang/String;",
            WithMutStackFrames(get_full_path_name0_wrp),
        );
        table.insert(
            "sun/nio/fs/WindowsNativeDispatcher:FindFirstFile0:(JLsun/nio/fs/WindowsNativeDispatcher$FirstFile;)V",
            WithMutStackFrames(find_first_file0_wrp),
        );
        table.insert(
            "sun/nio/fs/WindowsNativeDispatcher:FindNextFile0:(JJ)Ljava/lang/String;",
            WithMutStackFrames(find_next_file0_wrp),
        );
        table.insert(
            "sun/nio/fs/WindowsNativeDispatcher:FindClose:(J)V",
            WithMutStackFrames(find_close_wrp),
        );

        table.insert(
            "sun/nio/ch/FileDispatcherImpl:allocationGranularity0:()J",
            Basic(allocation_granularity0_wrp),
        );
        table.insert(
            "sun/nio/ch/FileDispatcherImpl:maxDirectTransferSize0:()I", // Integer.MAX_VALUE - 1 is the maximum transfer size for TransmitFile()
            Basic(|_args| Ok(vec![i32::MAX - 1])),
        );
        table.insert(
            "sun/nio/ch/FileDispatcherImpl:write0:(Ljava/io/FileDescriptor;JIZ)I",
            WithMutStackFrames(windows_file_dispatcher_write0_wrp),
        );
        table.insert(
            "sun/nio/ch/FileDispatcherImpl:read0:(Ljava/io/FileDescriptor;JI)I",
            WithMutStackFrames(windows_file_dispatcher_read0_wrp),
        );
        table.insert(
            "sun/nio/ch/FileDispatcherImpl:pread0:(Ljava/io/FileDescriptor;JIJ)I",
            WithMutStackFrames(windows_file_dispatcher_pread0_wrp),
        );
        table.insert(
            "sun/nio/ch/FileDispatcherImpl:size0:(Ljava/io/FileDescriptor;)J",
            WithMutStackFrames(windows_file_dispatcher_size0_wrp),
        );
        table.insert(
            "sun/nio/ch/FileDispatcherImpl:truncate0:(Ljava/io/FileDescriptor;J)I",
            WithMutStackFrames(file_dispatcher_impl_truncate0_wrp),
        );
        table.insert(
            "sun/nio/ch/FileDispatcherImpl:map0:(Ljava/io/FileDescriptor;IJJZ)J",
            WithMutStackFrames(file_dispatcher_map0_wrp),
        );
        table.insert(
            "sun/nio/ch/FileDispatcherImpl:duplicateHandle:(J)J",
            WithMutStackFrames(windows_file_dispatcher_duplicate_handle_wrp),
        );
        table.insert(
            "java/nio/MappedMemoryUtils:force0:(Ljava/io/FileDescriptor;JJ)V",
            WithMutStackFrames(mapped_memory_utils_force0_wrp),
        );
        table.insert(
            "sun/io/Win32ErrorMode:setErrorMode:(J)J",
            Basic(set_error_mode_wrp),
        );
        table.insert(
            "java/io/WinNTFileSystem:getFinalPath0:(Ljava/lang/String;)Ljava/lang/String;",
            WithMutStackFrames(get_final_path0_wrp),
        );
        table.insert(
            "java/io/WinNTFileSystem:delete0:(Ljava/io/File;Z)Z",
            Basic(winnt_file_system_delete0_wrp),
        );
        table.insert(
            "java/io/WinNTFileSystem:getNameMax0:(Ljava/lang/String;)I",
            WithMutStackFrames(get_name_max0_wrp),
        );
        table.insert(
            "java/io/WinNTFileSystem:getLength0:(Ljava/io/File;)J",
            Basic(get_length0_wrp),
        );

        table.insert(
            "sun/security/provider/NativeSeedGenerator:nativeGenerateSeed:([B)Z",
            Basic(native_generate_seed_wrp),
        );
    }

    #[cfg(unix)]
    {
        use crate::vm::system_native::io_file_system::delete0_wrp;
        use crate::vm::system_native::platform_file_dispatcher::unix_file_dispatcher::{
            unix_file_dispatcher_impl_pread0_wrp, unix_file_dispatcher_impl_read0_wrp,
            unix_file_dispatcher_impl_size0_wrp, unix_file_dispatcher_impl_write0_wrp,
        };
        use crate::vm::system_native::platform_native_dispatcher::unix_native_dispatcher::{
            get_access0_wrp, get_cwd_wrp, lstat0_wrp, mkdir0_wrp, realpath0_wrp, rmdir0_wrp,
            stat0_wrp, unix_native_dispatcher_open0_wrp, unlink0_wrp,
        };
        use crate::vm::system_native::platform_specific_files::unix_file_system::get_name_max0_wrp;

        table.insert(
            "java/io/UnixFileSystem:delete0:(Ljava/io/File;)Z",
            Basic(delete0_wrp),
        );
        table.insert(
            "java/io/UnixFileSystem:getNameMax0:(Ljava/lang/String;)J",
            Basic(get_name_max0_wrp),
        );
        table.insert(
            "java/io/UnixFileSystem:getLength0:(Ljava/io/File;)J",
            Basic(get_length0_wrp),
        );

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
            "sun/nio/fs/UnixNativeDispatcher:realpath0:(J)[B",
            WithMutStackFrames(realpath0_wrp),
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
            "sun/nio/ch/UnixFileDispatcherImpl:pread0:(Ljava/io/FileDescriptor;JIJ)I",
            WithMutStackFrames(unix_file_dispatcher_impl_pread0_wrp),
        );
        table.insert(
            "sun/nio/ch/UnixFileDispatcherImpl:size0:(Ljava/io/FileDescriptor;)J",
            WithMutStackFrames(unix_file_dispatcher_impl_size0_wrp),
        );
        table.insert(
            "sun/nio/ch/UnixFileDispatcherImpl:allocationGranularity0:()J",
            Basic(allocation_granularity0_wrp),
        );
        table.insert(
            "sun/nio/ch/UnixFileDispatcherImpl:truncate0:(Ljava/io/FileDescriptor;J)I",
            WithMutStackFrames(file_dispatcher_impl_truncate0_wrp),
        );
        table.insert(
            "sun/nio/ch/UnixFileDispatcherImpl:map0:(Ljava/io/FileDescriptor;IJJZ)J",
            WithMutStackFrames(file_dispatcher_map0_wrp),
        );
        table.insert(
            "java/nio/MappedMemoryUtils:force0:(Ljava/io/FileDescriptor;JJ)V",
            WithMutStackFrames(mapped_memory_utils_force0_wrp),
        );
    }

    #[cfg(target_os = "linux")]
    {
        table.insert("sun/nio/ch/FileDispatcherImpl:init0:()V", Basic(void_stub));
    }
}

pub(crate) fn invoke_native_method(
    method_signature: &str,
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let native_method = SYSTEM_NATIVE_TABLE.get(method_signature).ok_or_else(|| {
        Error::new_native(&format!("Native method {method_signature} not found"))
    })?;

    match native_method {
        Basic(method) => method(args),
        WithStackFrames(method) => method(args, stack_frames),
        WithMutStackFrames(method) => method(args, stack_frames),
    }
}

fn void_stub(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(vec![])
}
