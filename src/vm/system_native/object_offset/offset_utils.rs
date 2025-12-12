use crate::vm::error::Result;
use crate::vm::helper::klass;
use crate::vm::method_area::loaded_classes::CLASSES;
use crate::vm::system_native::string::get_utf8_string_by_ref;

pub(crate) fn object_field_offset_by_refs(class_ref: i32, string_ref: i32) -> Result<i64> {
    let field_name = get_utf8_string_by_ref(string_ref)?;
    let klass = klass(class_ref)?;
    let class_name = klass.this_class_name();
    object_field_offset_by_names(class_name, &field_name)
}
pub(crate) fn object_field_offset_by_names(class_name: &str, field_name: &str) -> Result<i64> {
    let klass = CLASSES.get(class_name)?;
    let offset = klass.get_field_offset(&format!("{class_name}.{field_name}"))?;
    Ok(offset)
}

pub(crate) fn static_field_offset_by_names(class_name: &str, field_name: &str) -> Result<i64> {
    let klass = CLASSES.get(class_name)?;
    klass.get_static_field_offset(field_name)
}
