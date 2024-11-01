use crate::heap::heap::{with_heap_read_lock, with_heap_write_lock};
use crate::helper::i32toi64;
use crate::method_area::method_area::with_method_area;
use crate::system_native::string::get_utf8_string_by_ref;

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
    let jc = with_method_area(|area| {
        let class_name = area.get_from_reflection_table(class_ref)?;
        area.get(class_name.as_str())
    })?;

    let offset = jc.get_field_offset(&field_name)?;

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

    let jc = with_method_area(|area| area.get(class_name.as_str()))?;

    let field_name = jc.get_field_name_by_offset(offset)?;

    let updated = with_heap_write_lock(|heap| {
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
    });

    Ok(updated)
}
