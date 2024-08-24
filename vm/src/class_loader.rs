use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use jclass::attributes::Attribute;
use jclass::class_file::{parse, ClassFile};
use jclass::constant_pool::ConstantPool::{Methodref, NameAndType, Utf8};
use jclass::attributes::Attribute::Code;
use crate::error::{Result, Error, ErrorKind};
use crate::method_area::java_class::{JavaClass, Methods};
use crate::method_area::java_method::JavaMethod;
use crate::method_area::method_area::MethodArea;
use crate::method_area::signature::Signature;

#[derive(Debug)]
pub struct ClassLoader {
    method_area: MethodArea,
}

impl ClassLoader {
    pub(crate) fn new(class_file_name: &str) -> Result<Self> {
        let mut file = File::open(class_file_name)?;

        let mut buff = Vec::new();
        file.read_to_end(&mut buff)?;

        let class_file = parse(buff.as_slice())
            .map_err(|err| Error::new(ErrorKind::ClassFile(err.to_string())))?;

        let java_class = Self::to_java_class(class_file)?;

        Ok(Self { method_area: MethodArea::new(java_class) })
    }

    fn to_java_class(class_file: ClassFile) -> Result<JavaClass> {
        let methods = Self::get_methods(&class_file)?;

        Ok(JavaClass::new(methods))
    }

    fn get_methods(class_file: &ClassFile) -> Result<Methods> {
        let methods = class_file.methods();
        let mut methodsignature_by_cpoolindex: HashMap<u16, String> = HashMap::new();
        let mut method_by_signature: HashMap<String, JavaMethod> = HashMap::new();

        for method in methods.iter() {
            let method_name = Self::get_cpool_string(class_file, method.name_index() as usize)
                .ok_or(Error::new_constant_pool("Error getting method name"))?;
            let method_signature = Self::get_cpool_string(class_file, method.descriptor_index() as usize)
                .ok_or(Error::new_constant_pool("Error getting method method_signature"))?;
            let code = Self::get_cpool_code_attribute(method.attributes())
                .ok_or(Error::new_constant_pool("Error getting method code"))?;
            let key_signature = method_name + ":" + method_signature.as_str();

            method_by_signature.insert(key_signature.clone(), JavaMethod::new(Signature::from_str(method_signature.as_str())?, code.0, code.1, code.2));

            let cpool_index = Self::get_cpool_method_index(class_file, method.name_index(), method.descriptor_index());
            if let Some(index) = cpool_index {
                methodsignature_by_cpoolindex.insert(index, key_signature);
            }
        }

        Ok(Methods::new(methodsignature_by_cpoolindex, method_by_signature))
    }

    fn get_cpool_string(class_file: &ClassFile, index: usize) -> Option<String> {
        let constant_pool = class_file.constant_pool();

        constant_pool.get(index)
            .and_then(|item| match item {
                Utf8 { value } => Some(value.clone()),
                _ => None
            })
    }

    fn get_cpool_method_index(class_file: &ClassFile, name_index_to_find: u16, signature_index: u16) -> Option<u16> {
        let constant_pool = class_file.constant_pool();

        let found_name_and_type_index = constant_pool.iter()
            .enumerate()
            .find_map(|index| if let NameAndType { name_index, descriptor_index } = *index.1 {
                if name_index == name_index_to_find && descriptor_index == signature_index {
                    Some(index.0)
                } else { None }
            } else { None })? as u16;

        let this_class_index = class_file.this_class();

        constant_pool.iter()
            .enumerate()
            .find_map(|index| if let Methodref { class_index, name_and_type_index } = *index.1 {
                if class_index == this_class_index && name_and_type_index == found_name_and_type_index {
                    Some(index.0 as u16)
                } else { None }
            } else { None })
    }

    fn get_cpool_code_attribute(attributes: &Vec<Attribute>) -> Option<(u16, u16, Vec<u8>)> {
        attributes.iter()
            .find_map(|item| {
                if let Code { max_stack, max_locals, code, .. } = item {
                    Some((*max_stack, *max_locals, code.clone()))
                } else {
                    None
                }
            })
    }

    pub fn method_area(&self) -> &MethodArea {
        &self.method_area
    }
}
