use crate::error::Error;
use crate::heap::java_instance::Field;
use jclass::class_file::ClassFile;
use jclass::constant_pool::ConstantPool::{Class, Utf8};
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

pub(crate) fn get_fields(class_file: &ClassFile) -> crate::error::Result<HashMap<String, Field>> {
    let fields = class_file.fields();
    let mut field_by_name: HashMap<String, Field> = HashMap::new();

    for field in fields.iter() {
        let field_name = get_cpool_string(class_file, field.name_index() as usize)
            .ok_or(Error::new_constant_pool("Error getting field name"))?;
        let _field_signature = get_cpool_string(class_file, field.descriptor_index() as usize)
            .ok_or(Error::new_constant_pool("Error getting field signature"))?;

        field_by_name.insert(field_name, Field::new());
    }

    Ok(field_by_name)
}
