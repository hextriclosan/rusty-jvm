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
