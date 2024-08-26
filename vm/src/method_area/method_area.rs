use crate::error::Error;
use crate::method_area::java_class::JavaClass;
use crate::method_area::java_method::JavaMethod;

#[derive(Debug)]
pub(crate) struct MethodArea {
    pub(crate) loaded_class: JavaClass, //todo: use HashMap with multiple JavaClass here in next version
}

impl MethodArea {
    pub fn new(loaded_class: JavaClass) -> Self {
        Self { loaded_class }
    }

    pub(crate) fn get_method_by_name_signature(
        &self,
        method_name_signature: &str,
    ) -> crate::error::Result<&JavaMethod> {
        //todo add class name
        if let Some(found) = self
            .loaded_class
            .methods
            .method_by_signature
            .get(method_name_signature)
        {
            return Ok(found);
        }

        Err(Error::new_constant_pool(
            "Error getting method by name from methods map",
        ))
    }

    pub(crate) fn get_method_by_cpool_index(
        &self,
        cpool_index: u16,
    ) -> crate::error::Result<&JavaMethod> {
        //todo add class name
        if let Some(found_signature) = self
            .loaded_class
            .methods
            .methodsignature_by_cpoolindex
            .get(&cpool_index)
        {
            return self.get_method_by_name_signature(found_signature);
        }

        Err(Error::new_constant_pool(
            "Error getting method by name from methods map",
        ))
    }
}
