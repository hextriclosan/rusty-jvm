use crate::error::Error;
use crate::method_area::field::Field;
use jclass::class_file::ClassFile;
use jclass::constant_pool::ConstantPool::{Class, Integer, Utf8};
use jclass::fields::FieldFlags;
use std::collections::HashMap;

pub(crate) fn get_class_name_by_cpool_class_index(
    class_index: usize,
    class_file: &ClassFile,
) -> Option<String> {
    class_file
        .constant_pool()
        .get(class_index)
        .and_then(|cpool| match cpool {
            Class { name_index } => Some(*name_index as usize),
            _ => None,
        })
        .and_then(|index| get_cpool_string(class_file, index))
}

pub(crate) fn get_cpool_string(class_file: &ClassFile, index: usize) -> Option<String> {
    let constant_pool = class_file.constant_pool();

    constant_pool.get(index).and_then(|item| match item {
        Utf8 { value } => Some(value.clone()),
        _ => None,
    })
}

pub(crate) fn get_cpool_integer(class_file: &ClassFile, index: usize) -> Option<i32> {
    let constant_pool = class_file.constant_pool();

    constant_pool.get(index).and_then(|item| match item {
        Integer { value } => Some(value.clone()),
        _ => None,
    })
}

pub(crate) fn get_fields(class_file: &ClassFile) -> crate::error::Result<HashMap<String, Field>> {
    let result = class_file
        .fields()
        .iter()
        .filter_map(|field| {
            if field.access_flags().contains(FieldFlags::ACC_STATIC) {
                None
            } else {
                let field_name = get_cpool_string(class_file, field.name_index() as usize)
                    .ok_or_else(|| Error::new_constant_pool("Error getting field name"))
                    .ok()?;
                let _field_signature =
                    get_cpool_string(class_file, field.descriptor_index() as usize)
                        .ok_or_else(|| Error::new_constant_pool("Error getting field signature"))
                        .ok()?;

                Some((field_name, Field::new()))
            }
        })
        .collect();

    Ok(result)
}
