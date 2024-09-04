use crate::error::Error;
use crate::method_area::java_class::JavaClass;
use crate::method_area::java_method::JavaMethod;
use crate::util::{get_class_name_by_cpool_class_index, get_cpool_string};
use jclass::constant_pool::ConstantPool::{Fieldref, Methodref, NameAndType};
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct MethodArea {
    pub(crate) loaded_classes: HashMap<String, JavaClass>,
}

impl MethodArea {
    pub fn new(loaded_classes: HashMap<String, JavaClass>) -> Self {
        Self { loaded_classes }
    }

    pub fn set_static_field_value(
        &self,
        class_name: &str,
        fieldname: &str,
        value: i32,
    ) -> crate::error::Result<()> {
        self.loaded_classes
            .get(class_name)
            .and_then(|java_class| java_class.fields.field_by_name.get(fieldname))
            .and_then(|field| {
                field.borrow_mut().set_value(value);

                Some(())
            })
            .ok_or(Error::new_execution("Error modifying static field"))
    }

    pub(crate) fn get_method_by_name_signature(
        &self,
        class_name: &str,
        method_name_signature: &str,
    ) -> crate::error::Result<&JavaMethod> {
        if let Some(found) = self
            .loaded_classes
            .get(class_name)
            .unwrap()
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

    pub(crate) fn get_method_by_methodref_cpool_index(
        &self,
        java_class: &JavaClass,
        methodref_cpool_index: u16,
    ) -> crate::error::Result<&JavaMethod> {
        let cpool = java_class.class_file.constant_pool();

        // Retrieve Methodref from the constant pool
        let methodref = cpool
            .get(methodref_cpool_index as usize)
            .and_then(|entry| match entry {
                Methodref {
                    class_index,
                    name_and_type_index,
                } => Some((*class_index as usize, *name_and_type_index as usize)),
                _ => None,
            })
            .ok_or_else(|| {
                Error::new_constant_pool(
                    format!(
                        "Invalid Methodref at index {} in class {:?}",
                        methodref_cpool_index, java_class
                    )
                    .as_str(),
                )
            })?;

        // Retrieve class name from the constant pool
        let class_name = get_class_name_by_cpool_class_index(methodref.0, &java_class.class_file);

        // Retrieve method name and signature from the constant pool
        let (method_name, method_signature) = if let NameAndType {
            name_index,
            descriptor_index,
        } = cpool.get(methodref.1).ok_or_else(|| {
            Error::new_constant_pool(
                format!(
                    "Invalid NameAndType reference at index {} in class {:?}",
                    methodref_cpool_index, java_class
                )
                .as_str(),
            )
        })? {
            let name = get_cpool_string(&java_class.class_file, *name_index as usize);
            let signature = get_cpool_string(&java_class.class_file, *descriptor_index as usize);
            (name, signature)
        } else {
            return Err(Error::new_constant_pool(
                format!(
                    "Expected NameAndType at index {} in class {:?}",
                    methodref_cpool_index, java_class
                )
                .as_str(),
            ));
        };

        // Construct method signature and retrieve method
        let full_signature = format!("{}:{}", method_name.unwrap(), method_signature.unwrap());
        self.get_method_by_name_signature(class_name.unwrap().as_str(), full_signature.as_str())
    }

    pub(crate) fn get_fieldname_by_fieldref_cpool_index(
        &self,
        java_class: &JavaClass,
        fieldref_cpool_index: u16,
    ) -> crate::error::Result<(String, String)> {
        let cpool = java_class.class_file.constant_pool();

        // Retrieve Fieldref from the constant pool
        let fieldref = cpool
            .get(fieldref_cpool_index as usize)
            .and_then(|entry| match entry {
                Fieldref {
                    class_index,
                    name_and_type_index,
                } => Some((*class_index as usize, *name_and_type_index as usize)),
                _ => None,
            })
            .ok_or_else(|| {
                Error::new_constant_pool(
                    format!(
                        "Invalid Fieldref at index {} in class {:?}",
                        fieldref_cpool_index, java_class
                    )
                    .as_str(),
                )
            })?;

        // Retrieve class name from the constant pool
        let class_name = get_class_name_by_cpool_class_index(fieldref.0, &java_class.class_file);

        // Retrieve field name from the constant pool
        let field_name = if let NameAndType {
            name_index,
            descriptor_index: _,
        } = cpool.get(fieldref.1).ok_or_else(|| {
            Error::new_constant_pool(
                format!(
                    "Invalid NameAndType reference at index {} in class {:?}",
                    fieldref_cpool_index, java_class
                )
                .as_str(),
            )
        })? {
            get_cpool_string(&java_class.class_file, *name_index as usize)
        } else {
            return Err(Error::new_constant_pool(
                format!(
                    "Expected NameAndType at index {} in class {:?}",
                    fieldref_cpool_index, java_class
                )
                .as_str(),
            ));
        };

        Ok((class_name.unwrap(), field_name.unwrap()))
    }
}
