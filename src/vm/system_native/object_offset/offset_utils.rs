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

/// Offset of the instance field `field_name` declared by the class behind `class_ref`, or `-1` when
/// the class declares no such field and `-2` when it is static (the `knownObjectFieldOffset0`
/// contract).
pub(crate) fn known_object_field_offset_by_refs(class_ref: i32, string_ref: i32) -> Result<i64> {
    const NOT_FOUND: i64 = -1;
    const STATIC_FIELD: i64 = -2;

    let field_name = get_utf8_string_by_ref(string_ref)?;
    let klass = klass(class_ref)?;
    let class_name = klass.this_class_name();
    if let Some(offset) = klass.find_field_offset(&format!("{class_name}.{field_name}"))? {
        return Ok(offset);
    }
    if klass.static_field(&field_name).is_some() {
        return Ok(STATIC_FIELD);
    }
    Ok(NOT_FOUND)
}

pub(crate) fn static_field_offset_by_names(class_name: &str, field_name: &str) -> Result<i64> {
    let klass = CLASSES.get(class_name)?;
    klass.get_static_field_offset(field_name)
}
