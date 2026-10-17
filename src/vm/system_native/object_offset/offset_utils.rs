use crate::vm::error::Result;
use crate::vm::helper::klass;
use crate::vm::method_area::loaded_classes::CLASSES;
use crate::vm::system_native::string::get_utf8_string_by_ref;

pub(crate) fn object_field_offset_by_names(class_name: &str, field_name: &str) -> Result<i64> {
    let klass = CLASSES.get(class_name)?;
    let offset = klass.get_field_offset(&format!("{class_name}.{field_name}"))?;
    Ok(offset)
}

/// What the class behind `class_ref` declares under the name `string_ref`, as the
/// `Unsafe.objectFieldOffset(Class, String)` natives look it up.
pub(crate) enum DeclaredField {
    Instance(i64),
    Static(i64),
    NotFound,
}

pub(crate) fn declared_field_offset_by_refs(
    class_ref: i32,
    string_ref: i32,
) -> Result<DeclaredField> {
    let field_name = get_utf8_string_by_ref(string_ref)?;
    let klass = klass(class_ref)?;
    let class_name = klass.this_class_name();
    if let Some(offset) = klass.find_field_offset(&format!("{class_name}.{field_name}"))? {
        return Ok(DeclaredField::Instance(offset));
    }
    if klass.static_field(&field_name).is_some() {
        return Ok(DeclaredField::Static(
            klass.get_static_field_offset(&field_name)?,
        ));
    }
    Ok(DeclaredField::NotFound)
}

pub(crate) fn static_field_offset_by_names(class_name: &str, field_name: &str) -> Result<i64> {
    let klass = CLASSES.get(class_name)?;
    klass.get_static_field_offset(field_name)
}
