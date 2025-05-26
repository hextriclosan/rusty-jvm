use crate::method_area::method_area::with_method_area;
use crate::system_native::string::get_utf8_string_by_ref;

pub(crate) fn object_field_offset_by_refs(
    class_ref: i32,
    string_ref: i32,
) -> crate::error::Result<i64> {
    let field_name = get_utf8_string_by_ref(string_ref)?;
    let class_name = with_method_area(|area| area.get_from_reflection_table(class_ref))?;
    object_field_offset_by_names(&class_name, &field_name)
}
pub(crate) fn object_field_offset_by_names(
    class_name: &str,
    field_name: &str,
) -> crate::error::Result<i64> {
    let jc = with_method_area(|area| area.get(class_name))?;
    let offset = jc.get_field_offset(&format!("{class_name}.{field_name}"))?;
    Ok(offset)
}

pub(crate) fn static_field_offset_by_names(
    class_name: &str,
    field_name: &str,
) -> crate::error::Result<i64> {
    let jc = with_method_area(|area| area.get(class_name))?;
    jc.get_static_field_offset(field_name)
}
