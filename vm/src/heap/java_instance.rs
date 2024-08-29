use crate::method_area::java_class::JavaClass;
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct JavaInstance<'a> {
    class_ref: &'a JavaClass,
    fields: HashMap<String, Field>,
}

impl<'a> JavaInstance<'a> {
    pub fn new(class_ref: &'a JavaClass) -> Self {
        let mut fields = HashMap::new();

        Self { class_ref, fields }
    }
}

#[derive(Debug)]
pub(crate) struct Field {
    value: i32, // todo: support other types
}
