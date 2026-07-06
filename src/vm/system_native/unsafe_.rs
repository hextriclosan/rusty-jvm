use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::static_init::StaticInit;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::{i32toi64, i64_to_vec, klass, vec_to_i64};
use crate::vm::method_area::java_class::InnerState::Initialized;
use crate::vm::method_area::java_class::STATIC_FIELDS_START;
use crate::vm::method_area::loaded_classes::CLASSES;
use crate::vm::system_native::object_offset::offset_utils::{
    object_field_offset_by_names, object_field_offset_by_refs, static_field_offset_by_names,
};
use crate::vm::system_native::string::get_utf8_string_by_ref;
use std::alloc::{alloc, Layout};
use std::ptr;

trait ValueTypeTrait {
    fn size(&self) -> usize;
    fn to_vec(&self) -> Vec<i32>;
}

impl ValueTypeTrait for i64 {
    fn size(&self) -> usize {
        size_of::<i64>()
    }

    fn to_vec(&self) -> Vec<i32> {
        i64_to_vec(*self)
    }
}

impl ValueTypeTrait for i32 {
    fn size(&self) -> usize {
        size_of::<i32>()
    }

    fn to_vec(&self) -> Vec<i32> {
        vec![*self]
    }
}

impl ValueTypeTrait for i16 {
    fn size(&self) -> usize {
        size_of::<i16>()
    }

    fn to_vec(&self) -> Vec<i32> {
        vec![*self as i32]
    }
}

impl ValueTypeTrait for u16 {
    fn size(&self) -> usize {
        size_of::<u16>()
    }

    fn to_vec(&self) -> Vec<i32> {
        vec![*self as i32]
    }
}

impl ValueTypeTrait for i8 {
    fn size(&self) -> usize {
        size_of::<i8>()
    }

    fn to_vec(&self) -> Vec<i32> {
        vec![*self as i32]
    }
}

/// `jdk.internal.misc.Unsafe.registerNatives()V`
pub(crate) fn register_natives() -> Result<()> {
    // todo: implement me
    Ok(())
}

/// `jdk.internal.misc.Unsafe.arrayBaseOffset0(Ljava/lang/Class;)I`
pub(crate) fn array_base_offset0(_this: i32, _array_class: i32) -> Result<i32> {
    // our implementation does not have array header so the offset is 0
    Ok(0)
}

/// `jdk.internal.misc.Unsafe.objectFieldOffset0(Ljava/lang/reflect/Field;)J`
pub(crate) fn object_field_offset0(_this: i32, field_ref: i32) -> Result<i64> {
    let class_ref = HEAP.get_object_field_value(field_ref, "java/lang/reflect/Field", "clazz")?[0];
    let field_name_ref =
        HEAP.get_object_field_value(field_ref, "java/lang/reflect/Field", "name")?[0];

    let klass = klass(class_ref)?;
    let field_name = get_utf8_string_by_ref(field_name_ref)?;

    object_field_offset_by_names(klass.this_class_name(), &field_name)
}

/// `jdk.internal.misc.Unsafe.objectFieldOffset1(Ljava/lang/Class;Ljava/lang/String;)J`
pub(crate) fn object_field_offset1(_this: i32, class_ref: i32, string_ref: i32) -> Result<i64> {
    object_field_offset_by_refs(class_ref, string_ref)
}

/// `jdk.internal.misc.Unsafe.staticFieldOffset0(Ljava/lang/reflect/Field;)J`
pub(crate) fn static_field_offset0(_this: i32, field_ref: i32) -> Result<i64> {
    let class_ref = HEAP.get_object_field_value(field_ref, "java/lang/reflect/Field", "clazz")?[0];
    let field_name_ref =
        HEAP.get_object_field_value(field_ref, "java/lang/reflect/Field", "name")?[0];

    let klass = klass(class_ref)?;
    let field_name = get_utf8_string_by_ref(field_name_ref)?;

    static_field_offset_by_names(klass.this_class_name(), &field_name)
}

/// `jdk.internal.misc.Unsafe.staticFieldBase0(Ljava/lang/reflect/Field;)Ljava/lang/Object;`
pub(crate) fn static_field_base0(_this: i32, field_ref: i32) -> Result<i32> {
    let class_ref = HEAP.get_object_field_value(field_ref, "java/lang/reflect/Field", "clazz")?[0];
    Ok(class_ref)
}

// todo: thread-safety - not atomic
/// `jdk.internal.misc.Unsafe.compareAndSetInt(Ljava/lang/Object;JII)Z`
pub(crate) fn compare_and_set_int(
    _this: i32,
    obj_ref: i32,
    offset: i64,
    expected: i32,
    x: i32,
) -> Result<bool> {
    let class_name = HEAP.get_instance_name(obj_ref)?;
    let updated = if class_name.starts_with("[") {
        let result = HEAP.get_array_value_by_raw_offset(obj_ref, offset as usize, 4)?[0];
        if result == expected {
            HEAP.set_array_value_by_raw_offset(
                obj_ref,
                offset as usize,
                vec![x],
                size_of::<i32>(),
            )?;
            Ok(true)
        } else {
            Ok(false)
        }
    } else {
        let klass = CLASSES.get(class_name.as_str())?;
        let (class_name, field_name) = klass.get_field_name_by_offset(offset)?;
        let result = HEAP.get_object_field_value(obj_ref, &class_name, &field_name)?[0];

        if result == expected {
            HEAP.set_object_field_value(obj_ref, &class_name, &field_name, vec![x])?;
            Ok(true)
        } else {
            Ok(false)
        }
    };

    updated
}

/// `jdk.internal.misc.Unsafe.compareAndSetReference(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z`
pub(crate) fn compare_and_set_reference(
    this: i32,
    obj_ref: i32,
    offset: i64,
    expected: i32,
    x: i32,
) -> Result<bool> {
    compare_and_set_int(this, obj_ref, offset, expected, x)
}

/// `jdk.internal.misc.Unsafe.getReference(Ljava/lang/Object;J)Ljava/lang/Object;`
pub(crate) fn get_reference(_this: i32, obj_ref: i32, offset: i64) -> Result<i32> {
    let class_name = HEAP.get_instance_name(obj_ref)?;
    let raw_value = if class_name.starts_with("[") {
        HEAP.get_array_value_by_raw_offset(obj_ref, offset as usize, 4)?
    } else {
        if class_name == "java/lang/Class" {
            // Special case for java/lang/Class<T>, in fact it is getting of static field of T
            let t_jc = klass(obj_ref)?;
            let static_field = t_jc.get_static_field_by_offset(offset)?;
            static_field.raw_value()?
        } else {
            let klass = CLASSES.get(class_name.as_str())?;
            let (class_name, field_name) = klass.get_field_name_by_offset(offset)?;
            HEAP.get_object_field_value(obj_ref, &class_name, &field_name)?
        }
    };

    Ok(raw_value[0])
}

// todo: thread-safety - not volatile
/// `jdk.internal.misc.Unsafe.getReferenceVolatile(Ljava/lang/Object;J)Ljava/lang/Object;`
pub(crate) fn get_reference_volatile(this: i32, obj_ref: i32, offset: i64) -> Result<i32> {
    get_reference(this, obj_ref, offset)
}

/// `jdk.internal.misc.Unsafe.getByte(Ljava/lang/Object;J)B`
pub(crate) fn get_byte(_this: i32, obj_ref: i32, offset: i64) -> Result<i8> {
    if obj_ref != 0 {
        let class_name = HEAP.get_instance_name(obj_ref)?;
        if class_name.starts_with("[") {
            let result = HEAP.get_array_value_by_raw_offset(obj_ref, offset as usize, 1)?;
            Ok(result[0] as i8)
        } else {
            todo!("implement get_byte for class field");
        }
    } else {
        let addr = offset as usize as *const u8;
        unsafe { Ok(ptr::read(addr) as i8) }
    }
}

/// `jdk.internal.misc.Unsafe.getShort(Ljava/lang/Object;J)S`
pub(crate) fn get_short(_this: i32, obj_ref: i32, offset: i64) -> Result<i16> {
    if obj_ref != 0 {
        let class_name = HEAP.get_instance_name(obj_ref)?;
        if class_name.starts_with("[") {
            let result = HEAP.get_array_value_by_raw_offset(obj_ref, offset as usize, 2)?;
            Ok(result[0] as i16)
        } else {
            todo!("implement get_short for class field");
        }
    } else {
        let addr = offset as usize as *const i16;
        unsafe { Ok(ptr::read(addr)) }
    }
}

/// `jdk.internal.misc.Unsafe.getChar(Ljava/lang/Object;J)C`
pub(crate) fn get_char(_this: i32, obj_ref: i32, offset: i64) -> Result<u16> {
    if obj_ref != 0 {
        let class_name = HEAP.get_instance_name(obj_ref)?;
        if class_name.starts_with("[") {
            let result = HEAP.get_array_value_by_raw_offset(obj_ref, offset as usize, 2)?;
            Ok(result[0] as u16)
        } else {
            todo!("implement get_char for class field");
        }
    } else {
        todo!("implement get_char for null object");
    }
}

/// `jdk.internal.misc.Unsafe.getInt(Ljava/lang/Object;J)I`
pub(crate) fn get_int(_this: i32, obj_ref: i32, offset: i64) -> Result<i32> {
    if obj_ref == 0 {
        get_int_raw(offset)
    } else {
        get_int_via_object(obj_ref, offset)
    }
}

fn get_int_raw(address: i64) -> Result<i32> {
    let ptr = address as usize as *const i32;
    unsafe {
        let ptr = ptr.add(0);
        Ok(ptr::read(ptr))
    }
}
fn get_int_via_object(obj_ref: i32, offset: i64) -> Result<i32> {
    if obj_ref != 0 {
        let class_name = HEAP.get_instance_name(obj_ref)?;
        if class_name.starts_with("[") {
            let result = HEAP.get_array_value_by_raw_offset(obj_ref, offset as usize, 4)?;
            Ok(result[0])
        } else {
            let class_name = HEAP.get_instance_name(obj_ref)?;

            let klass = CLASSES.get(class_name.as_str())?;

            let (class_name, field_name) = klass.get_field_name_by_offset(offset)?;

            let result = HEAP.get_object_field_value(obj_ref, &class_name, &field_name)?;
            Ok(result[0])
        }
    } else {
        todo!("implement get_int for null object");
    }
}

// todo: thread-safety - not volatile
/// `jdk.internal.misc.Unsafe.getIntVolatile(Ljava/lang/Object;J)I`
pub(crate) fn get_int_volatile(this: i32, obj_ref: i32, offset: i64) -> Result<i32> {
    get_int(this, obj_ref, offset)
}

// todo: thread-safety - not volatile
/// `jdk.internal.misc.Unsafe.getBooleanVolatile(Ljava/lang/Object;J)Z`
pub(crate) fn get_boolean_volatile(this: i32, obj_ref: i32, offset: i64) -> Result<bool> {
    let ret = get_int(this, obj_ref, offset)?;
    Ok(ret != 0)
}

/// `jdk.internal.misc.Unsafe.getLong(Ljava/lang/Object;J)J`
pub(crate) fn get_long(_this: i32, obj_ref: i32, offset: i64) -> Result<i64> {
    if obj_ref != 0 {
        let class_name = HEAP.get_instance_name(obj_ref)?;
        if class_name.starts_with("[") {
            let bytes = HEAP.get_array_value_by_raw_offset(obj_ref, offset as usize, 8)?;
            Ok(vec_to_i64(&bytes))
        } else {
            let class_name = HEAP.get_instance_name(obj_ref)?;

            let klass = CLASSES.get(class_name.as_str())?;

            let (class_name, field_name) = klass.get_field_name_by_offset(offset)?;

            let result = HEAP.get_object_field_value(obj_ref, &class_name, &field_name)?;
            Ok(vec_to_i64(&result))
        }
    } else {
        Ok(read_raw(offset))
    }
}

// todo: thread-safety - not volatile
/// `jdk.internal.misc.Unsafe.getLongVolatile(Ljava/lang/Object;J)J`
pub(crate) fn get_long_volatile(this: i32, obj_ref: i32, offset: i64) -> Result<i64> {
    get_long(this, obj_ref, offset)
}

// todo: thread-safety - not atomic
/// `jdk.internal.misc.Unsafe.compareAndSetLong(Ljava/lang/Object;JJJ)Z`
pub(crate) fn compare_and_set_long(
    this: i32,
    obj_ref: i32,
    offset: i64,
    expected: i64,
    x: i64,
) -> Result<bool> {
    let (updated, _old_value) = compare_and_x_long(this, obj_ref, offset, expected, x)?;
    Ok(updated)
}

// todo: thread-safety - not atomic
/// `jdk.internal.misc.Unsafe.compareAndExchangeLong(Ljava/lang/Object;JJJ)J`
pub(crate) fn compare_and_exchange_long(
    this: i32,
    obj_ref: i32,
    offset: i64,
    expected: i64,
    x: i64,
) -> Result<i64> {
    let (_updated, old_value) = compare_and_x_long(this, obj_ref, offset, expected, x)?;
    Ok(old_value)
}

/// `jdk.internal.misc.Unsafe.compareAndExchangeInt(Ljava/lang/Object;JII)I`
fn compare_and_x_long(
    _this: i32,
    obj_ref: i32,
    offset: i64,
    expected: i64,
    x: i64,
) -> Result<(bool, i64)> {
    if obj_ref == 0 {
        let old_value: i64 = read_raw(offset);
        return if old_value == expected {
            write_raw(offset, x);
            Ok((true, old_value))
        } else {
            Ok((false, old_value))
        };
    }

    let class_name = HEAP.get_instance_name(obj_ref)?;

    let klass = CLASSES.get(class_name.as_str())?;

    let (class_name, field_name) = klass.get_field_name_by_offset(offset)?;

    let bytes = HEAP.get_object_field_value(obj_ref, &class_name, &field_name)?;
    let old_value = i32toi64(bytes[0], bytes[1]);

    if old_value == expected {
        HEAP.set_object_field_value(obj_ref, &class_name, &field_name, i64_to_vec(x))?;
        Ok((true, old_value))
    } else {
        Ok((false, old_value))
    }
}

/// `jdk.internal.misc.Unsafe.putReference(Ljava/lang/Object;JLjava/lang/Object;)V`
pub(crate) fn put_reference(_this: i32, obj_ref: i32, offset: i64, ref_value: i32) -> Result<()> {
    let class_name = HEAP.get_instance_name(obj_ref)?;
    if class_name.starts_with("[") {
        HEAP.set_array_value_by_raw_offset(
            obj_ref,
            offset as usize,
            vec![ref_value],
            size_of::<i32>(),
        )
    } else {
        if class_name == "java/lang/Class" && offset >= STATIC_FIELDS_START {
            // Special case for java/lang/Class<T>, in fact it is modification of static field of T
            let t_jc = klass(obj_ref)?;
            let static_field = t_jc.get_static_field_by_offset(offset)?;
            static_field.set_raw_value(vec![ref_value])
        } else {
            let klass = CLASSES.get(class_name.as_str())?;
            let (class_name, field_name) = klass.get_field_name_by_offset(offset)?;

            HEAP.set_object_field_value(obj_ref, &class_name, &field_name, vec![ref_value])
        }
    }
}

// todo: thread-safety - not volatile
/// `jdk.internal.misc.Unsafe.putReferenceVolatile(Ljava/lang/Object;JLjava/lang/Object;)V`
pub(crate) fn put_reference_volatile(
    this: i32,
    obj_ref: i32,
    offset: i64,
    ref_value: i32,
) -> Result<()> {
    put_reference(this, obj_ref, offset, ref_value)
}

/// `jdk.internal.misc.Unsafe.putChar(Ljava/lang/Object;JC)V`
pub(crate) fn put_char(_this: i32, obj_ref: i32, offset: i64, x: u16) -> Result<()> {
    put_value(obj_ref, offset, x)
}

/// `jdk.internal.misc.Unsafe.putByte(Ljava/lang/Object;JB)V`
pub(crate) fn put_byte(_this: i32, obj_ref: i32, offset: i64, x: i8) -> Result<()> {
    put_value(obj_ref, offset, x)
}

/// `jdk.internal.misc.Unsafe.putShort(Ljava/lang/Object;JS)V`
pub(crate) fn put_short(_this: i32, obj_ref: i32, offset: i64, x: i16) -> Result<()> {
    put_value(obj_ref, offset, x)
}

/// `jdk.internal.misc.Unsafe.putInt(Ljava/lang/Object;JI)V`
pub(crate) fn put_int(_this: i32, obj_ref: i32, offset: i64, x: i32) -> Result<()> {
    put_value(obj_ref, offset, x)
}

// todo: thread-safety - not volatile
/// `jdk.internal.misc.Unsafe.putIntVolatile(Ljava/lang/Object;JI)V`
pub(crate) fn put_int_volatile(_this: i32, obj_ref: i32, offset: i64, x: i32) -> Result<()> {
    put_int(_this, obj_ref, offset, x)
}

/// `jdk.internal.misc.Unsafe.putLong(Ljava/lang/Object;JJ)V`
pub(crate) fn put_long(_this: i32, obj_ref: i32, offset: i64, x: i64) -> Result<()> {
    put_value(obj_ref, offset, x)
}

fn put_value<T: ValueTypeTrait + Copy>(obj_ref: i32, offset: i64, value: T) -> Result<()> {
    if obj_ref == 0 {
        write_raw(offset, value);
        Ok(())
    } else {
        let raw_value = value.to_vec();
        put_value_via_object(obj_ref, offset, raw_value, value.size())
    }
}

fn write_raw<T: Copy>(address: i64, value: T) {
    let ptr = address as usize as *mut u8;
    let src = &value as *const T as *const u8;
    unsafe { ptr::copy(src, ptr, size_of::<T>()) };
}

fn read_raw<T: Copy>(address: i64) -> T {
    let ptr = address as usize as *const T;
    unsafe {
        let ptr = ptr.add(0);
        ptr.read_unaligned()
    }
}

fn put_value_via_object(
    obj_ref: i32,
    offset: i64,
    raw_value: Vec<i32>,
    value_size: usize,
) -> Result<()> {
    let class_name = HEAP.get_instance_name(obj_ref)?;
    if class_name.starts_with('[') {
        HEAP.set_array_value_by_raw_offset(obj_ref, offset as usize, raw_value, value_size)
    } else {
        let klass = CLASSES.get(&class_name)?;
        let (class_name, field_name) = klass.get_field_name_by_offset(offset)?;
        HEAP.set_object_field_value(obj_ref, &class_name, &field_name, raw_value)
    }
}

/// `jdk.internal.misc.Unsafe.arrayIndexScale0(Ljava/lang/Class;)I`
pub(crate) fn array_index_scale0(_this: i32, class_ref: i32) -> Result<i32> {
    let klass = klass(class_ref)?;
    Ok(match klass.this_class_name().as_str() {
        "[B" => 1,
        "[C" => 2,
        "[D" => 8,
        "[F" => 4,
        "[I" => 4,
        "[J" => 8,
        "[S" => 2,
        "[Z" => 1,
        _ => 4,
    })
}

/// `jdk.internal.misc.Unsafe.fullFence()V`
pub(crate) fn full_fence(_this: i32) -> Result<()> {
    // todo: implement me
    Ok(())
}

/// `jdk.internal.misc.Unsafe.ensureClassInitialized0(Ljava/lang/Class;)V`
pub(crate) fn ensure_class_initialized0(_this: i32, class_ref: i32) -> Result<()> {
    let klass = klass(class_ref)?;
    StaticInit::initialize(klass.this_class_name())
}

/// `jdk.internal.misc.Unsafe.shouldBeInitialized0(Ljava/lang/Class;)Z`
pub(crate) fn should_be_initialized0(_this: i32, class_ref: i32) -> Result<bool> {
    let klass = klass(class_ref)?;
    let guard = klass.static_fields_init_state().lock();
    Ok(guard.get_inner_state() != Initialized)
}

/// `jdk.internal.misc.Unsafe.allocateMemory0(J)J`
pub(crate) fn allocate_memory0(_this: i32, bytes: i64) -> Result<i64> {
    let layout = Layout::array::<u8>(bytes as usize)
        .map_err(|_| Error::new_native("Invalid memory allocation"))?;
    let ptr = unsafe { alloc(layout) };
    if ptr.is_null() {
        return Err(Error::new_native("Failed to allocate memory"));
    }

    let address = ptr as usize as i64;

    Ok(address)
}

// Todo: for all *_ref/offset pairs:
//  *_ref is 0 means that corresponding offset represents absolute address, otherwise it is relative within the object
/// `jdk.internal.misc.Unsafe.copyMemory0(Ljava/lang/Object;JLjava/lang/Object;JJ)V`
pub(crate) fn copy_memory0(
    _this: i32,
    src_base_ref: i32,
    src_offset: i64,
    dest_base_ref: i32,
    dest_offset: i64,
    bytes: i64,
) -> Result<()> {
    if src_base_ref != 0 {
        // Collect source bytes into a local Vec before acquiring the dest guard to avoid
        // deadlock when src and dest are in the same DashMap shard.
        // todo: only arrays are supported so far (add check isArray)
        let to_copy = {
            let raw = HEAP.get_entire_raw_data(src_base_ref)?;
            raw.iter()
                .skip(src_offset as usize)
                .take(bytes as usize)
                .copied()
                .collect::<Vec<_>>()
        };

        if dest_base_ref == 0 {
            // dest_offset is absolute address
            unsafe {
                let src = to_copy.as_ptr();
                let dst = dest_offset as usize as *mut u8;
                let len = to_copy.len();
                ptr::copy(src, dst, len);
            }
        } else {
            let mut arr_copy_to = HEAP.get_entire_raw_data_mut(dest_base_ref)?; // todo: only arrays are supported so far (add check isArray)
            let input = &mut arr_copy_to[dest_offset as usize..(dest_offset + bytes) as usize];
            input.copy_from_slice(to_copy.as_slice());
        }
    } else {
        if dest_base_ref == 0 {
            unimplemented!("dest_base_ref == null not supported yet");
        } else {
            let ptr_copy_from = src_offset as usize as *const u8;
            let mut arr_copy_to = HEAP.get_entire_raw_data_mut(dest_base_ref)?; // todo: only arrays are supported so far (add check isArray)
            unsafe {
                let output =
                    &mut arr_copy_to[dest_offset as usize..(dest_offset + bytes) as usize];

                ptr::copy(ptr_copy_from, output.as_mut_ptr(), bytes as usize);
            }
        }
    }

    Ok(())
}

/// `jdk.internal.misc.Unsafe.copySwapMemory0(Ljava/lang/Object;JLjava/lang/Object;JJJ)V`
pub(crate) fn copy_swap_memory0(
    _this: i32,
    src_base_ref: i32,
    src_offset: i64,
    dest_base_ref: i32,
    dest_offset: i64,
    bytes: i64,
    elem_size: i64,
) -> Result<()> {
    let total_bytes = bytes as usize;
    let elem_size = elem_size as usize;

    if elem_size == 0 || total_bytes % elem_size != 0 {
        return Err(Error::new_execution("Invalid elem_size or bytes"));
    }

    // ---------------------------
    // Resolve source
    // ---------------------------
    if src_base_ref == 0 {
        unimplemented!("src_base_ref == 0 not supported yet");
    }

    // Collect source bytes into a local Vec before acquiring the dest guard to avoid
    // deadlock when src and dest are in the same DashMap shard.
    let src_data: Vec<u8> = {
        let src_raw = HEAP.get_entire_raw_data(src_base_ref)?;
        let src_start = src_offset as usize;
        src_raw[src_start..src_start + total_bytes].to_vec()
    };

    // ---------------------------
    // Resolve destination
    // ---------------------------
    if dest_base_ref == 0 {
        unimplemented!("dest_base_ref == 0 not supported yet");
    }

    let mut dest_raw = HEAP.get_entire_raw_data_mut(dest_base_ref)?;
    let dest_start = dest_offset as usize;

    // ---------------------------
    // Copy + swap
    // ---------------------------
    let mut byte_index = 0;

    while byte_index < total_bytes {
        let src_chunk_start = byte_index;
        let src_chunk_end = src_chunk_start + elem_size;

        let src_chunk = &src_data[src_chunk_start..src_chunk_end];

        for j in 0..elem_size {
            let value = src_chunk[elem_size - 1 - j]; // swap
            let dst_index = dest_start + byte_index + j;

            dest_raw[dst_index] = value;
        }

        byte_index += elem_size;
    }

    Ok(())
}

/// `jdk.internal.misc.Unsafe.setMemory0(Ljava/lang/Object;JJB)V`
pub(crate) fn set_memory0(
    _this: i32,
    obj_ref: i32,
    offset: i64,
    bytes: i64,
    value: i8,
) -> Result<()> {
    if obj_ref != 0 {
        unimplemented!("implement this for objects")
    }

    let dst_ptr = offset as *mut u8;
    unsafe {
        ptr::write_bytes(dst_ptr, value as u8, bytes as usize);
    }

    Ok(())
}
