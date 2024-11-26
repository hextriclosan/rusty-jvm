use crate::error::Error;
use crate::heap::heap::{with_heap_read_lock, with_heap_write_lock};
use crate::helper::{i32toi64, i64_to_vec};
use crate::method_area::java_class::JavaClass;
use crate::method_area::method_area::with_method_area;
use crate::system_native::string::get_utf8_string_by_ref;
use std::sync::Arc;

pub(crate) fn object_field_offset_1_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let _unsafe_ref = args[0];
    let class_ref = args[1];
    let string_ref = args[2];
    let offset = object_field_offset_1(class_ref, string_ref)?;

    let high = ((offset >> 32) & 0xFFFFFFFF) as i32;
    let low = (offset & 0xFFFFFFFF) as i32;

    Ok(vec![high, low])
}
fn object_field_offset_1(class_ref: i32, string_ref: i32) -> crate::error::Result<i64> {
    let field_name = get_utf8_string_by_ref(string_ref)?;
    let (class_name, jc) = with_method_area(|area| {
        let class_name = area.get_from_reflection_table(class_ref)?;
        let jc = area.get(class_name.as_str())?;
        Ok::<(String, Arc<JavaClass>), Error>((class_name, jc))
    })?;

    let offset = jc.get_field_offset(&format!("{class_name}.{field_name}"))?;

    Ok(offset)
}

pub(crate) fn compare_and_set_int_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let _unsafe_ref = args[0];
    let obj_ref = args[1];
    let offset = i32toi64(args[3], args[2]);
    let expected = args[4];
    let x = args[5];

    let result = compare_and_set_int(obj_ref, offset, expected, x)?;
    Ok(vec![result as i32])
}
fn compare_and_set_int(
    obj_ref: i32,
    offset: i64,
    expected: i32,
    x: i32,
) -> crate::error::Result<bool> {
    let class_name = with_heap_read_lock(|heap| heap.get_instance_name(obj_ref))?;
    let updated = if class_name.starts_with("[") {
        with_heap_write_lock(|heap| {
            let result = heap
                .get_array_value(obj_ref, offset as i32)
                .expect("error getting array value")[0];
            if result == expected {
                heap.set_array_value(obj_ref, offset as i32, vec![x])
                    .expect("error setting field value");
                true
            } else {
                false
            }
        })
    } else {
        let jc = with_method_area(|area| area.get(class_name.as_str()))?;
        let (class_name, field_name) = jc.get_field_name_by_offset(offset)?;
        with_heap_write_lock(|heap| {
            let result = heap
                .get_object_field_value(obj_ref, &class_name, &field_name)
                .expect("error getting field value")[0];

            if result == expected {
                heap.set_object_field_value(obj_ref, &class_name, &field_name, vec![x])
                    .expect("error setting field value");
                true
            } else {
                false
            }
        })
    };

    Ok(updated)
}

pub(crate) fn get_reference_volatile_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let _unsafe_ref = args[0];
    let obj_ref = args[1];
    let offset = i32toi64(args[3], args[2]);

    let result = get_reference_volatile(obj_ref, offset)?;
    Ok(vec![result])
}
fn get_reference_volatile(obj_ref: i32, offset: i64) -> crate::error::Result<i32> {
    let class_name = with_heap_read_lock(|heap| heap.get_instance_name(obj_ref))?;
    let raw_value = if class_name.starts_with("[") {
        with_heap_read_lock(|heap| heap.get_array_value(obj_ref, offset as i32).cloned())?
    } else {
        let jc = with_method_area(|area| area.get(class_name.as_str()))?;
        let (class_name, field_name) = jc.get_field_name_by_offset(offset)?;
        with_heap_read_lock(|heap| heap.get_object_field_value(obj_ref, &class_name, &field_name))?
    };

    Ok(raw_value[0])
}
pub(crate) fn get_long_volatile_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let _unsafe_ref = args[0];
    let obj_ref = args[1];
    let offset = i32toi64(args[3], args[2]);

    let result = get_long_volatile(obj_ref, offset)?;

    let high = ((result >> 32) & 0xFFFFFFFF) as i32;
    let low = (result & 0xFFFFFFFF) as i32;

    Ok(vec![high, low])
}
fn get_long_volatile(obj_ref: i32, offset: i64) -> crate::error::Result<i64> {
    if obj_ref != 0 {
        todo!("implement get_long_volatile for non null object");
    }
    Ok(offset) // not real implementation, just a placeholder for case when object is null
}

pub(crate) fn compare_and_set_long_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let _unsafe_ref = args[0];
    let obj_ref = args[1];
    let offset = i32toi64(args[3], args[2]);
    let expected = i32toi64(args[5], args[4]);
    let x = i32toi64(args[7], args[6]);

    let result = compare_and_set_long(obj_ref, offset, expected, x)?;
    Ok(vec![result as i32])
}
fn compare_and_set_long(
    obj_ref: i32,
    offset: i64,
    expected: i64,
    x: i64,
) -> crate::error::Result<bool> {
    if obj_ref == 0 {
        return Ok(true); // not real implementation, just a placeholder for case when object is null
    }

    let class_name = with_heap_read_lock(|heap| heap.get_instance_name(obj_ref))?;

    let jc = with_method_area(|area| area.get(class_name.as_str()))?;

    let (class_name, field_name) = jc.get_field_name_by_offset(offset)?;

    let updated = with_heap_write_lock(|heap| {
        let bytes = heap
            .get_object_field_value(obj_ref, &class_name, &field_name)
            .expect("error getting field value");
        let result = i32toi64(bytes[0], bytes[1]);

        if result == expected {
            heap.set_object_field_value(obj_ref, &class_name, &field_name, i64_to_vec(x))
                .expect("error setting field value");
            true
        } else {
            false
        }
    });

    Ok(updated)
}

pub(crate) fn put_reference_volatile_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let _unsafe_ref = args[0];
    let obj_ref = args[1];
    let offset = i32toi64(args[3], args[2]);
    let ref_value = args[4];

    put_reference_volatile(obj_ref, offset, ref_value)?;
    Ok(vec![])
}
fn put_reference_volatile(obj_ref: i32, offset: i64, ref_value: i32) -> crate::error::Result<()> {
    with_heap_write_lock(|heap| {
        let class_name = heap.get_instance_name(obj_ref)?;
        let jc = with_method_area(|area| area.get(class_name.as_str()))?;
        let (class_name, field_name) = jc.get_field_name_by_offset(offset)?;

        heap.set_object_field_value(obj_ref, &class_name, &field_name, vec![ref_value])
            .expect("error setting field value");
        Ok(())
    })
}
