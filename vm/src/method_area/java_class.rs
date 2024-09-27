use crate::method_area::cpool_helper::CPoolHelper;
use crate::method_area::field::Field;
use crate::method_area::java_method::JavaMethod;
use jdescriptor::TypeDescriptor;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub(crate) struct JavaClass {
    pub(crate) methods: Methods,
    pub(crate) static_fields: Fields,
    pub(crate) field_descriptors: FieldDescriptors,
    cpool_helper: CPoolHelper,
}

#[derive(Debug)]
pub(crate) struct Methods {
    pub(crate) method_by_signature: HashMap<String, Rc<JavaMethod>>,
}

#[derive(Debug)]
pub(crate) struct Fields {
    pub(crate) field_by_name: HashMap<String, RefCell<Field>>,
}

#[derive(Debug)]
pub(crate) struct FieldDescriptors {
    pub(crate) descriptor_by_name: HashMap<String, TypeDescriptor>,
}

impl FieldDescriptors {
    pub fn new(descriptor_by_name: HashMap<String, TypeDescriptor>) -> Self {
        Self { descriptor_by_name }
    }
}

impl Fields {
    pub fn new(field_by_name: HashMap<String, RefCell<Field>>) -> Self {
        Self { field_by_name }
    }
}

impl JavaClass {
    pub fn new(
        methods: Methods,
        static_fields: Fields,
        field_descriptors: FieldDescriptors,
        cpool_helper: CPoolHelper,
    ) -> Self {
        Self {
            methods,
            static_fields,
            field_descriptors,
            cpool_helper,
        }
    }

    pub fn cpool_helper(&self) -> &CPoolHelper {
        &self.cpool_helper
    }
}

impl Methods {
    pub fn new(method_by_signature: HashMap<String, Rc<JavaMethod>>) -> Self {
        Self {
            method_by_signature,
        }
    }
}
