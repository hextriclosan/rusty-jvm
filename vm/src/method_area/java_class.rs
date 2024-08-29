use crate::method_area::java_method::JavaMethod;
use jclass::class_file::ClassFile;
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct JavaClass {
    pub(crate) methods: Methods,
    pub(crate) class_file: ClassFile,
}

#[derive(Debug)]
pub(crate) struct Methods {
    pub(crate) method_by_signature: HashMap<String, JavaMethod>,
}

impl JavaClass {
    pub fn new(methods: Methods, class_file: ClassFile) -> Self {
        Self {
            methods,
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
