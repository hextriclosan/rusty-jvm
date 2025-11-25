use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::static_init::StaticInit;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::{i32toi64, i64_to_vec, vec_to_i64};
use crate::vm::method_area::java_class::InnerState::Initialized;
use crate::vm::method_area::java_class::STATIC_FIELDS_START;
use crate::vm::method_area::method_area::with_method_area;
use crate::vm::system_native::object_offset::offset_utils::{
    object_field_offset_by_names, object_field_offset_by_refs, static_field_offset_by_names,
};
use crate::vm::system_native::string::get_utf8_string_by_ref;
use std::alloc::{alloc, Layout};
use std::ptr;

#[derive(Clone, Copy, Debug)]
enum ValueType {
    Char,
    Byte,
    Int,
    Long,
    Short,
}

impl From<ValueType> for usize {
    fn from(value: ValueType) -> Self {
        match value {
            ValueType::Byte => 1,
            ValueType::Char => 2,
            ValueType::Int => 4,
            ValueType::Long => 8,
            ValueType::Short => 2,
        }
    }
}

pub(crate) fn object_field_offset_0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let _unsafe_ref = args[0];
    let field_ref = args[1];
    let offset = object_field_offset_0(field_ref)?;

    let high = ((offset >> 32) & 0xFFFFFFFF) as i32;
    let low = (offset & 0xFFFFFFFF) as i32;

    Ok(vec![high, low])
}
fn object_field_offset_0(field_ref: i32) -> Result<i64> {
    let class_ref = HEAP.get_object_field_value(field_ref, "java/lang/reflect/Field", "clazz")?[0];
    let field_name_ref =
        HEAP.get_object_field_value(field_ref, "java/lang/reflect/Field", "name")?[0];

    let class_name = with_method_area(|area| area.get_from_reflection_table(class_ref))?;
    let field_name = get_utf8_string_by_ref(field_name_ref)?;

    object_field_offset_by_names(&class_name, &field_name)
}

pub(crate) fn object_field_offset_1_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let _unsafe_ref = args[0];
    let class_ref = args[1];
    let string_ref = args[2];
    let offset = object_field_offset_1(class_ref, string_ref)?;

    let high = ((offset >> 32) & 0xFFFFFFFF) as i32;
    let low = (offset & 0xFFFFFFFF) as i32;

    Ok(vec![high, low])
}
fn object_field_offset_1(class_ref: i32, string_ref: i32) -> Result<i64> {
    object_field_offset_by_refs(class_ref, string_ref)
}

pub(crate) fn static_field_offset_0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let _unsafe_ref = args[0];
    let field_ref = args[1];
    let offset = static_field_offset_0(field_ref)?;

    let high = ((offset >> 32) & 0xFFFFFFFF) as i32;
    let low = (offset & 0xFFFFFFFF) as i32;

    Ok(vec![high, low])
}
fn static_field_offset_0(field_ref: i32) -> Result<i64> {
    let class_ref = HEAP.get_object_field_value(field_ref, "java/lang/reflect/Field", "clazz")?[0];
    let field_name_ref =
        HEAP.get_object_field_value(field_ref, "java/lang/reflect/Field", "name")?[0];

    let class_name = with_method_area(|area| area.get_from_reflection_table(class_ref))?;
    let field_name = get_utf8_string_by_ref(field_name_ref)?;

    static_field_offset_by_names(&class_name, &field_name)
}

pub(crate) fn static_field_base0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let _unsafe_ref = args[0];
    let field_ref = args[1];
    let base_ref = static_field_base0(field_ref)?;

    Ok(vec![base_ref])
}
fn static_field_base0(field_ref: i32) -> Result<i32> {
    let clazz_ref = HEAP.get_object_field_value(field_ref, "java/lang/reflect/Field", "clazz")?[0];

    let class_name = with_method_area(|area| area.get_from_reflection_table(clazz_ref))?;
    let class_ref = with_method_area(|area| area.load_reflection_class(class_name.as_str()))?;

    Ok(class_ref)
}

pub(crate) fn compare_and_set_int_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let _unsafe_ref = args[0];
    let obj_ref = args[1];
    let offset = i32toi64(args[3], args[2]);
    let expected = args[4];
    let x = args[5];

    let result = compare_and_set_int(obj_ref, offset, expected, x)?;
    Ok(vec![result as i32])
}
fn compare_and_set_int(obj_ref: i32, offset: i64, expected: i32, x: i32) -> Result<bool> {
    let class_name = HEAP.get_instance_name(obj_ref)?;
    let updated = if class_name.starts_with("[") {
        let result = HEAP.get_array_value_by_raw_offset(obj_ref, offset as usize, 4)?[0];
        if result == expected {
            HEAP.set_array_value_by_raw_offset(
                obj_ref,
                offset as usize,
                vec![x],
                ValueType::Int.into(),
            )?;
            Ok::<bool, Error>(true)
        } else {
            Ok(false)
        }
    } else {
        let jc = with_method_area(|area| area.get(class_name.as_str()))?;
        let (class_name, field_name) = jc.get_field_name_by_offset(offset)?;
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

pub(crate) fn get_reference_volatile_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let _unsafe_ref = args[0];
    let obj_ref = args[1];
    let offset = i32toi64(args[3], args[2]);

    let result = get_reference_volatile(obj_ref, offset)?;
    Ok(vec![result])
}
fn get_reference_volatile(obj_ref: i32, offset: i64) -> Result<i32> {
    let class_name = HEAP.get_instance_name(obj_ref)?;
    let raw_value = if class_name.starts_with("[") {
        HEAP.get_array_value_by_raw_offset(obj_ref, offset as usize, 4)?
    } else {
        if class_name == "java/lang/Class" {
            // Special case for java/lang/Class<T>, in fact it is getting of static field of T
            let t_name = with_method_area(|area| area.get_from_reflection_table(obj_ref))?;
            let t_jc = with_method_area(|area| area.get(t_name.as_str()))?;
            let static_field = t_jc.get_static_field_by_offset(offset)?;
            static_field.raw_value()?
        } else {
            let jc = with_method_area(|area| area.get(class_name.as_str()))?;
            let (class_name, field_name) = jc.get_field_name_by_offset(offset)?;
            HEAP.get_object_field_value(obj_ref, &class_name, &field_name)?
        }
    };

    Ok(raw_value[0])
}

pub(crate) fn get_byte_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let _unsafe_ref = args[0];
    let obj_ref = args[1];
    let offset = i32toi64(args[3], args[2]);

    let byte = get_byte(obj_ref, offset)?;
    Ok(vec![byte as i32])
}
pub(crate) fn get_byte(obj_ref: i32, offset: i64) -> Result<i8> {
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

pub(crate) fn get_short_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let _unsafe_ref = args[0];
    let obj_ref = args[1];
    let offset = i32toi64(args[3], args[2]);

    let short = get_short(obj_ref, offset)?;
    Ok(vec![short as i32])
}
pub(crate) fn get_short(obj_ref: i32, offset: i64) -> Result<i16> {
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

pub(crate) fn get_char_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let _unsafe_ref = args[0];
    let obj_ref = args[1];
    let offset = i32toi64(args[3], args[2]);

    let char = get_char(obj_ref, offset)?;
    Ok(vec![char as i32])
}

pub(crate) fn get_char(obj_ref: i32, offset: i64) -> Result<u16> {
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

pub(crate) fn get_int_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let _unsafe_ref = args[0];
    let obj_ref = args[1];
    let offset = i32toi64(args[3], args[2]);

    let int = if obj_ref == 0 {
        get_int_raw(offset)?
    } else {
        get_int_via_object(obj_ref, offset)?
    };
    Ok(vec![int])
}
pub(crate) fn get_int_raw(address: i64) -> Result<i32> {
    let ptr = address as usize as *const i32;
    unsafe {
        let ptr = ptr.add(0);
        Ok(ptr::read(ptr))
    }
}
pub(crate) fn get_int_via_object(obj_ref: i32, offset: i64) -> Result<i32> {
    if obj_ref != 0 {
        let class_name = HEAP.get_instance_name(obj_ref)?;
        if class_name.starts_with("[") {
            let result = HEAP.get_array_value_by_raw_offset(obj_ref, offset as usize, 4)?;
            Ok(result[0])
        } else {
            let class_name = HEAP.get_instance_name(obj_ref)?;

            let jc = with_method_area(|area| area.get(class_name.as_str()))?;

            let (class_name, field_name) = jc.get_field_name_by_offset(offset)?;

            let result = HEAP.get_object_field_value(obj_ref, &class_name, &field_name)?;
            Ok(result[0])
        }
    } else {
        todo!("implement get_int for null object");
    }
}
pub(crate) fn get_int_volatile_wrp(args: &[i32]) -> Result<Vec<i32>> {
    get_int_wrp(args) // todo! make me volatile
}

pub(crate) fn get_boolean_volatile_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let ret = get_int_wrp(args)?; // todo! make me volatile (and boolean)
    Ok(vec![if ret[0] != 0 { 1 } else { 0 }])
}

pub(crate) fn get_long_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let _unsafe_ref = args[0];
    let obj_ref = args[1];
    let offset = i32toi64(args[3], args[2]);

    let result = get_long(obj_ref, offset)?;

    let high = ((result >> 32) & 0xFFFFFFFF) as i32;
    let low = (result & 0xFFFFFFFF) as i32;

    Ok(vec![high, low])
}
fn get_long(obj_ref: i32, offset: i64) -> Result<i64> {
    if obj_ref != 0 {
        let class_name = HEAP.get_instance_name(obj_ref)?;
        if class_name.starts_with("[") {
            let bytes = HEAP.get_array_value_by_raw_offset(obj_ref, offset as usize, 8)?;
            Ok(vec_to_i64(&bytes))
        } else {
            let class_name = HEAP.get_instance_name(obj_ref)?;

            let jc = with_method_area(|area| area.get(class_name.as_str()))?;

            let (class_name, field_name) = jc.get_field_name_by_offset(offset)?;

            let result = HEAP.get_object_field_value(obj_ref, &class_name, &field_name)?;
            Ok(vec_to_i64(&result))
        }
    } else {
        Ok(read_raw(offset))
    }
}

pub(crate) fn get_long_volatile_wrp(args: &[i32]) -> Result<Vec<i32>> {
    get_long_wrp(args) // todo! make me volatile
}

pub(crate) fn compare_and_set_long_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let _unsafe_ref = args[0];
    let obj_ref = args[1];
    let offset = i32toi64(args[3], args[2]);
    let expected = i32toi64(args[5], args[4]);
    let x = i32toi64(args[7], args[6]);

    let result = compare_and_set_long(obj_ref, offset, expected, x)?;
    Ok(vec![result as i32])
}
fn compare_and_set_long(obj_ref: i32, offset: i64, expected: i64, x: i64) -> Result<bool> {
    let (updated, _old_value) = compare_and_x_long(obj_ref, offset, expected, x)?;
    Ok(updated)
}

pub(crate) fn compare_and_exchange_long_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let _unsafe_ref = args[0];
    let obj_ref = args[1];
    let offset = i32toi64(args[3], args[2]);
    let expected = i32toi64(args[5], args[4]);
    let x = i32toi64(args[7], args[6]);

    let result = compare_and_exchange_long(obj_ref, offset, expected, x)?;
    Ok(i64_to_vec(result))
}
fn compare_and_exchange_long(obj_ref: i32, offset: i64, expected: i64, x: i64) -> Result<i64> {
    let (_updated, old_value) = compare_and_x_long(obj_ref, offset, expected, x)?;
    Ok(old_value)
}

fn compare_and_x_long(obj_ref: i32, offset: i64, expected: i64, x: i64) -> Result<(bool, i64)> {
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

    let jc = with_method_area(|area| area.get(class_name.as_str()))?;

    let (class_name, field_name) = jc.get_field_name_by_offset(offset)?;

    let bytes = HEAP.get_object_field_value(obj_ref, &class_name, &field_name)?;
    let old_value = i32toi64(bytes[0], bytes[1]);

    if old_value == expected {
        HEAP.set_object_field_value(obj_ref, &class_name, &field_name, i64_to_vec(x))?;
        Ok((true, old_value))
    } else {
        Ok((false, old_value))
    }
}

pub(crate) fn put_reference_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let _unsafe_ref = args[0];
    let obj_ref = args[1];
    let offset = i32toi64(args[3], args[2]);
    let ref_value = args[4];

    put_reference(obj_ref, offset, ref_value)?;
    Ok(vec![])
}
fn put_reference(obj_ref: i32, offset: i64, ref_value: i32) -> Result<()> {
    let class_name = HEAP.get_instance_name(obj_ref)?;
    if class_name.starts_with("[") {
        HEAP.set_array_value_by_raw_offset(
            obj_ref,
            offset as usize,
            vec![ref_value],
            ValueType::Int.into(),
        )
    } else {
        if class_name == "java/lang/Class" && offset >= STATIC_FIELDS_START {
            // Special case for java/lang/Class<T>, in fact it is modification of static field of T
            let t_name = with_method_area(|area| area.get_from_reflection_table(obj_ref))?;
            let t_jc = with_method_area(|area| area.get(t_name.as_str()))?;
            let static_field = t_jc.get_static_field_by_offset(offset)?;
            static_field.set_raw_value(vec![ref_value])
        } else {
            let jc = with_method_area(|area| area.get(class_name.as_str()))?;
            let (class_name, field_name) = jc.get_field_name_by_offset(offset)?;

            HEAP.set_object_field_value(obj_ref, &class_name, &field_name, vec![ref_value])
        }
    }
}

pub(crate) fn put_reference_volatile_wrp(args: &[i32]) -> Result<Vec<i32>> {
    put_reference_wrp(args) // todo! make me volatile
}

pub(crate) fn put_char_wrp(args: &[i32]) -> Result<Vec<i32>> {
    put_value_wrapper(args, ValueType::Char)
}

pub(crate) fn put_byte_wrp(args: &[i32]) -> Result<Vec<i32>> {
    put_value_wrapper(args, ValueType::Byte)
}
pub(crate) fn put_short_wrp(args: &[i32]) -> Result<Vec<i32>> {
    put_value_wrapper(args, ValueType::Short)
}
pub(crate) fn put_int_wrp(args: &[i32]) -> Result<Vec<i32>> {
    put_value_wrapper(args, ValueType::Int)
}

pub(crate) fn put_int_volatile_wrp(args: &[i32]) -> Result<Vec<i32>> {
    put_int_wrp(args) // todo! make me volatile
}

pub(crate) fn put_long_wrp(args: &[i32]) -> Result<Vec<i32>> {
    put_value_wrapper(args, ValueType::Long)
}

fn put_value_wrapper(args: &[i32], value_type: ValueType) -> Result<Vec<i32>> {
    let _unsafe_ref = args[0];
    let obj_ref = args[1];
    let offset = i32toi64(args[3], args[2]);
    let value = match value_type {
        ValueType::Byte | ValueType::Char | ValueType::Int | ValueType::Short => args[4] as i64,
        ValueType::Long => i32toi64(args[5], args[4]),
    };

    put_value(obj_ref, offset, value, value_type)?;
    Ok(vec![])
}

fn put_value(obj_ref: i32, offset: i64, value: i64, value_type: ValueType) -> Result<()> {
    if obj_ref == 0 {
        match value_type {
            ValueType::Char => write_raw(offset, value as u16),
            ValueType::Byte => write_raw(offset, value as u8),
            ValueType::Int => write_raw(offset, value as i32),
            ValueType::Long => write_raw(offset, value),
            ValueType::Short => write_raw(offset, value as i16),
        }
        Ok(())
    } else {
        let raw_value = match value_type {
            ValueType::Char | ValueType::Byte | ValueType::Int | ValueType::Short => {
                vec![value as i32]
            }
            ValueType::Long => i64_to_vec(value),
        };
        put_value_via_object(obj_ref, offset, raw_value, value_type)
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
    value_type: ValueType,
) -> Result<()> {
    let class_name = HEAP.get_instance_name(obj_ref)?;
    if class_name.starts_with('[') {
        HEAP.set_array_value_by_raw_offset(obj_ref, offset as usize, raw_value, value_type.into())
    } else {
        let jc = with_method_area(|area| area.get(&class_name))?;
        let (class_name, field_name) = jc.get_field_name_by_offset(offset)?;
        HEAP.set_object_field_value(obj_ref, &class_name, &field_name, raw_value)
    }
}

pub(crate) fn array_index_scale0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let _unsafe_ref = args[0];
    let class_ref = args[1];

    let index_scale = array_index_scale0(class_ref)?;

    Ok(vec![index_scale])
}
fn array_index_scale0(class_ref: i32) -> Result<i32> {
    with_method_area(|method_area| {
        let type_name = method_area.get_from_reflection_table(class_ref)?;
        Ok(match type_name.as_str() {
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
    })
}

pub(crate) fn ensure_class_initialized0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let _unsafe_ref = args[0];
    let class_ref = args[1];

    ensure_class_initialized0(class_ref)?;
    Ok(vec![])
}
fn ensure_class_initialized0(class_ref: i32) -> Result<()> {
    with_method_area(|method_area| {
        let type_name = method_area.get_from_reflection_table(class_ref)?;
        StaticInit::initialize(&type_name)
    })
}

pub(crate) fn should_be_initialized0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let _unsafe_ref = args[0];
    let class_ref = args[1];

    let result = should_be_initialized0(class_ref)?;
    Ok(vec![if result { 1 } else { 0 }])
}
fn should_be_initialized0(class_ref: i32) -> Result<bool> {
    with_method_area(|method_area| {
        let name = method_area.get_from_reflection_table(class_ref)?;
        let rc = method_area.get(&name)?;

        let guard = rc.static_fields_init_state().lock();
        Ok(guard.get_inner_state() != Initialized)
    })
}

pub(crate) fn allocate_memory0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let _unsafe_ref = args[0];
    let bytes = i32toi64(args[2], args[1]);

    let addr = allocate_memory0(bytes)?;
    Ok(i64_to_vec(addr))
}
fn allocate_memory0(bytes: i64) -> Result<i64> {
    let layout = Layout::array::<u8>(bytes as usize)
        .map_err(|_| Error::new_native("Invalid memory allocation"))?;
    let ptr = unsafe { alloc(layout) };
    if ptr.is_null() {
        return Err(Error::new_native("Failed to allocate memory"));
    }

    let address = ptr as usize as i64;

    Ok(address)
}

pub(crate) fn copy_memory0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let _unsafe_ref = args[0];
    let src_base_ref = args[1];
    let src_offset = i32toi64(args[3], args[2]);
    let dest_base_ref = args[4];
    let dest_offset = i32toi64(args[6], args[5]);
    let bytes = i32toi64(args[8], args[7]);

    copy_memory0(src_base_ref, src_offset, dest_base_ref, dest_offset, bytes)?;
    Ok(vec![])
}
// Todo: for all *_ref/offset pairs:
//  *_ref is 0 means that corresponding offset represents absolute address, otherwise it is relative within the object
fn copy_memory0(
    src_base_ref: i32,
    src_offset: i64,
    dest_base_ref: i32,
    dest_offset: i64,
    bytes: i64,
) -> Result<()> {
    let ptr = dest_base_ref as usize as *mut u8;

    if src_base_ref != 0 {
        let arr = HEAP.get_entire_array(src_base_ref)?; // todo: only arrays are supported so far (add check isArray)
        let raw = arr.raw_data();

        let to_copy = raw
            .iter()
            .skip(src_offset as usize)
            .take(bytes as usize)
            .map(|v| *v)
            .collect::<Vec<_>>();
        unsafe {
            let src = to_copy.as_ptr();
            let dst = ptr.add(dest_offset as usize);
            let len = to_copy.len();
            ptr::copy(src, dst, len);
        }
    } else {
        let ptr_copy_from = src_offset as usize as *const u8;
        let mut arr_copy_to = HEAP.get_entire_raw_data_mut(dest_base_ref)?; // todo: only arrays are supported so far (add check isArray)
        unsafe {
            let output = &mut arr_copy_to[dest_offset as usize..(dest_offset + bytes) as usize];

            ptr::copy(ptr_copy_from, output.as_mut_ptr(), bytes as usize);
        }
    }

    Ok(())
}

pub(crate) fn set_memory0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let _unsafe_ref = args[0];
    let obj_ref = args[1];
    let offset = i32toi64(args[3], args[2]);
    let bytes = i32toi64(args[5], args[4]);
    let value = args[6] as u8;

    set_memory0(obj_ref, offset, bytes, value)?;
    Ok(vec![])
}
fn set_memory0(obj_ref: i32, offset: i64, bytes: i64, value: u8) -> Result<()> {
    if obj_ref != 0 {
        unimplemented!("implement this for objects")
    }

    let dst_ptr = offset as *mut u8;
    unsafe {
        ptr::write_bytes(dst_ptr, value, bytes as usize);
    }

    Ok(())
}
