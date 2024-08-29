use jclass::class_file::ClassFile;
use jclass::constant_pool::ConstantPool::{Class, Utf8};

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
