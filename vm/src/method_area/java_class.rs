use std::collections::HashMap;
use crate::method_area::java_method::JavaMethod;


#[derive(Debug)]
pub(crate) struct JavaClass {
    pub(crate) methods: Methods,
}

#[derive(Debug)]
pub(crate) struct Methods {
    pub(crate) methodsignature_by_cpoolindex: HashMap<u16, String>, //for Methodref methods (except main and <clinit>)
    pub(crate) method_by_signature: HashMap<String, JavaMethod>,
}


impl JavaClass {
    pub fn new(methods: Methods) -> Self {
        Self { methods }
    }
}

impl Methods {
    pub fn new(methodsignature_by_cpoolindex: HashMap<u16, String>, method_by_signature: HashMap<String, JavaMethod>) -> Self {
        Self { methodsignature_by_cpoolindex, method_by_signature }
    }
}
