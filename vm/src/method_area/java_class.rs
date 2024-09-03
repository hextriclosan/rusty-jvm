use crate::method_area::field::Field;
use crate::method_area::java_method::JavaMethod;
use jclass::class_file::ClassFile;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct JavaClass {
    pub(crate) methods: Methods,
    pub(crate) fields: Fields,
    pub(crate) class_file: ClassFile,
}

#[derive(Debug)]
pub(crate) struct Methods {
    pub(crate) method_by_signature: HashMap<String, JavaMethod>,
}

#[derive(Debug)]
pub(crate) struct Fields {
    pub(crate) field_by_name: HashMap<String, RefCell<Field>>,
}

impl Fields {
    pub fn new(field_by_name: HashMap<String, RefCell<Field>>) -> Self {
        Self { field_by_name }
    }
}

impl JavaClass {
    pub fn new(methods: Methods, fields: Fields, class_file: ClassFile) -> Self {
        Self {
            methods,
            fields,
            class_file,
        }
    }
}

impl Methods {
    pub fn new(method_by_signature: HashMap<String, JavaMethod>) -> Self {
        Self {
            method_by_signature,
        }
    }
}
