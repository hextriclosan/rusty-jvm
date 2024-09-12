use crate::error::{Error, ErrorKind, Result};
use crate::method_area::field::Field;
use crate::method_area::java_class::{Fields, JavaClass, Methods};
use crate::method_area::java_method::JavaMethod;
use crate::method_area::method_area::MethodArea;
use crate::util::{get_class_name_by_cpool_class_index, get_cpool_string};
use jclass::attributes::Attribute;
use jclass::attributes::Attribute::Code;
use jclass::class_file::{parse, ClassFile};
use jclass::fields::FieldFlags;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::ErrorKind::Other;
use std::io::Read;
use std::path::PathBuf;
use std::{fs, io};

#[derive(Debug)]
pub struct ClassLoader {
    method_area: MethodArea,
}

impl ClassLoader {
    pub(crate) fn new(class_file_names: Vec<&str>, std_dir: &str) -> Result<Self> {
        let mut loaded_classes = HashMap::new();

        let std_class_names = Self::get_class_files_in_dir(std_dir)?;

        for path in std_class_names {
            let (class_name, java_class) = Self::load_class(path.to_str().ok_or(
                Error::new_io(io::Error::new(Other, "error getting path".to_string())),
            )?)?;

            loaded_classes.insert(class_name, java_class);
        }

        for class_file_name in class_file_names {
            let (class_name, java_class) = Self::load_class(class_file_name)?;
            loaded_classes.insert(class_name.clone(), java_class);
        }

        Ok(Self {
            method_area: MethodArea::new(loaded_classes),
        })
    }

    pub fn method_area(&self) -> &MethodArea {
        &self.method_area
    }

    fn load_class(class_file_name: &str) -> Result<(String, JavaClass)> {
        let mut file = File::open(class_file_name)?;

        let mut buff = Vec::new();
        file.read_to_end(&mut buff)?;

        let class_file = parse(buff.as_slice())
            .map_err(|err| Error::new(ErrorKind::ClassFile(err.to_string())))?;

        Self::to_java_class(class_file)
    }

    fn to_java_class(class_file: ClassFile) -> Result<(String, JavaClass)> {
        let class_name = Self::get_class_name(&class_file)?;
        let methods = Self::get_methods(&class_file, class_name.as_str())?;
        let static_fields = Self::get_static_fields(&class_file)?;

        Ok((
            class_name.clone(),
            JavaClass::new(methods, static_fields, class_file),
        ))
    }

    fn get_methods(class_file: &ClassFile, class_name: &str) -> Result<Methods> {
        let methods = class_file.methods();
        let mut method_by_signature: HashMap<String, JavaMethod> = HashMap::new();

        for method in methods.iter() {
            let method_name = get_cpool_string(class_file, method.name_index() as usize)
                .ok_or(Error::new_constant_pool("Error getting method name"))?;
            let method_signature = get_cpool_string(class_file, method.descriptor_index() as usize)
                .ok_or(Error::new_constant_pool(
                    "Error getting method method_signature",
                ))?;
            let (max_stack, max_locals, code) = Self::get_cpool_code_attribute(method.attributes())
                .ok_or(Error::new_constant_pool("Error getting method code"))?;
            let key_signature = method_name + ":" + method_signature.as_str();

            method_by_signature.insert(
                key_signature.clone(),
                JavaMethod::new(
                    method_signature.as_str().parse().map_err(|err| Error::new(ErrorKind::ClassFile(err)))?,
                    max_stack,
                    max_locals,
                    code,
                    class_name,
                ),
            );
        }

        Ok(Methods::new(method_by_signature))
    }

    fn get_static_fields(class_file: &ClassFile) -> Result<Fields> {
        let field_by_name = class_file
            .fields()
            .iter()
            .filter_map(|field| {
                if field.access_flags().contains(FieldFlags::ACC_STATIC) {
                    let field_name = get_cpool_string(class_file, field.name_index() as usize)
                        .ok_or_else(|| Error::new_constant_pool("Error getting field name"))
                        .ok()?;

                    let _field_signature =
                        get_cpool_string(class_file, field.descriptor_index() as usize)
                            .ok_or_else(|| {
                                Error::new_constant_pool("Error getting field signature")
                            })
                            .ok()?;

                    Some((field_name, RefCell::new(Field::new())))
                } else {
                    None
                }
            })
            .collect();

        Ok(Fields::new(field_by_name))
    }

    fn get_cpool_code_attribute(attributes: &Vec<Attribute>) -> Option<(u16, u16, Vec<u8>)> {
        attributes.iter().find_map(|item| {
            if let Code {
                max_stack,
                max_locals,
                code,
                ..
            } = item
            {
                Some((*max_stack, *max_locals, code.clone()))
            } else {
                None
            }
        })
    }

    fn get_class_name(class_file: &ClassFile) -> Result<String> {
        let this_class_index = class_file.this_class() as usize;

        get_class_name_by_cpool_class_index(this_class_index, class_file)
            .ok_or(Error::new_constant_pool("error getting class name"))
    }

    fn get_class_files_in_dir(std_dir: &str) -> Result<Vec<PathBuf>> {
        let mut class_files = Vec::new();
        let entries = fs::read_dir(std_dir)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "class" {
                        class_files.push(path);
                    }
                }
            }
        }

        Ok(class_files)
    }
}
