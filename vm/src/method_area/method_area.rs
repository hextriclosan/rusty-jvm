use crate::error::{Error, ErrorKind};
use crate::method_area::attributes_helper::AttributesHelper;
use crate::method_area::cpool_helper::CPoolHelper;
use crate::method_area::field::Field;
use crate::method_area::java_class::{FieldDescriptors, Fields, JavaClass, Methods};
use crate::method_area::java_method::JavaMethod;
use jclass::class_file::{parse, ClassFile};
use jclass::fields::{FieldFlags, FieldInfo};
use jclass::methods::MethodInfo;
use jdescriptor::TypeDescriptor;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::rc::Rc;

#[derive(Debug)]
pub(crate) struct MethodArea {
    std_dir: String,
    pub(crate) loaded_classes: RefCell<HashMap<String, Rc<JavaClass>>>,
}

impl MethodArea {
    pub fn new(std_dir: &str) -> Self {
        Self {
            std_dir: std_dir.to_string(),
            loaded_classes: RefCell::new(HashMap::new()),
        }
    }

    pub fn set_static_field_value(
        &self,
        class_name: &str,
        fieldname: &str,
        value: Vec<i32>,
    ) -> crate::error::Result<()> {
        self.loaded_classes
            .borrow_mut()
            .get(class_name)
            .and_then(|java_class| java_class.static_fields.field_by_name.get(fieldname))
            .and_then(|field| {
                field.borrow_mut().set_raw_value(value);

                Some(())
            })
            .ok_or(Error::new_execution("Error modifying static field"))
    }

    pub(crate) fn get(
        &self,
        fully_qualified_class_name: &str,
    ) -> crate::error::Result<Rc<JavaClass>> {
        if let Some(java_class) = self.loaded_classes.borrow().get(fully_qualified_class_name) {
            return Ok(Rc::clone(java_class));
        }

        //todo: make me thread-safe if move to multithreaded jvm
        let java_class = self.load_class_file(fully_qualified_class_name)?;
        self.loaded_classes.borrow_mut().insert(
            fully_qualified_class_name.to_string(),
            Rc::clone(&java_class),
        );

        Ok(java_class)
    }

    fn load_class_file(
        &self,
        fully_qualified_class_name: &str,
    ) -> crate::error::Result<Rc<JavaClass>> {
        let paths = vec![
            Path::new(&self.std_dir)
                .join(fully_qualified_class_name)
                .with_extension("class"),
            Path::new(fully_qualified_class_name).with_extension("class"),
        ];

        paths
            .iter()
            .find_map(|file_name| Self::try_open_and_parse(file_name))
            .ok_or_else(|| {
                Error::new_execution(&format!("error opening file {fully_qualified_class_name}"))
            })
    }

    fn try_open_and_parse(path: &PathBuf) -> Option<Rc<JavaClass>> {
        let mut file = File::open(path).ok()?;
        let mut buff = Vec::new();
        file.read_to_end(&mut buff).ok()?;

        let class_file = parse(buff.as_slice())
            .map_err(|err| Error::new(ErrorKind::ClassFile(err.to_string())))
            .ok()?;

        Self::to_java_class(class_file)
            .map(|(_, java_class)| java_class)
            .ok()
    }

    fn to_java_class(class_file: ClassFile) -> crate::error::Result<(String, Rc<JavaClass>)> {
        let cpool_helper = CPoolHelper::new(class_file.constant_pool());

        let this_class_index = class_file.this_class();
        let class_name = cpool_helper
            .get_class_name(this_class_index)
            .ok_or_else(|| {
                Error::new_constant_pool(&format!(
                    "Error getting class_name by index={this_class_index}"
                ))
            })?;

        let super_class_index = class_file.super_class();
        let _super_class_name = if super_class_index > 0 {
            cpool_helper
                .get_class_name(super_class_index)
                .map(Some)
                .ok_or_else(|| {
                    Error::new_constant_pool(&format!(
                        "Error getting super_class_name by index={super_class_index}"
                    ))
                })
        } else {
            Ok(None)
        }?;

        let interface_indexes = class_file.interfaces();
        let _interface_names = interface_indexes
            .iter()
            .map(|index| {
                cpool_helper.get_class_name(*index).ok_or_else(|| {
                    Error::new_constant_pool(&format!("Error getting interface by index={index}"))
                })
            })
            .collect::<crate::error::Result<Vec<String>>>()?;

        let methods = Self::get_methods(&class_file.methods(), &cpool_helper, &class_name)?;
        let (field_descriptors, static_fields) =
            Self::get_field_descriptors(&class_file.fields(), &cpool_helper)?;

        Ok((
            class_name.clone(),
            Rc::new(JavaClass::new(
                methods,
                static_fields,
                field_descriptors,
                cpool_helper,
            )),
        ))
    }

    fn get_methods(
        class_file_methods: &[MethodInfo],
        helper: &CPoolHelper,
        class_name: &str,
    ) -> crate::error::Result<Methods> {
        let mut method_by_signature = HashMap::new();

        for method_info in class_file_methods.iter() {
            let name_index = method_info.name_index();
            let method_name = helper.get_utf8(name_index).ok_or_else(|| {
                Error::new_execution(&format!("error getting method name by index {name_index}"))
            })?;

            let descriptor_index = method_info.descriptor_index();
            let method_signature = helper.get_utf8(descriptor_index).ok_or_else(|| {
                Error::new_execution(&format!(
                    "error getting method signature by index {descriptor_index}"
                ))
            })?;

            let key = format!("{method_name}:{method_signature}");

            let attributes_helper = AttributesHelper::new(method_info.attributes());
            let (max_stack, max_locals, code) = attributes_helper.get_code().ok_or_else(|| {
                Error::new_execution(&format!("Error getting code attribute for method {key}"))
            })?;

            method_by_signature.insert(
                key,
                Rc::new(JavaMethod::new(
                    method_signature.parse().map_err(|err| {
                        Error::new_execution(&format!(
                            "Error parsing signature {method_signature}: {err}"
                        ))
                    })?,
                    max_stack,
                    max_locals,
                    code,
                    class_name,
                )),
            );
        }

        Ok(Methods::new(method_by_signature))
    }

    fn get_field_descriptors(
        field_infos: &[FieldInfo],
        cpool_helper: &CPoolHelper,
    ) -> crate::error::Result<(FieldDescriptors, Fields)> {
        let mut descriptor_by_name = HashMap::new();
        let mut static_field_by_name = HashMap::new();
        for field_info in field_infos.iter() {
            let name_index = field_info.name_index();
            let field_name = cpool_helper.get_utf8(name_index).ok_or_else(|| {
                Error::new_execution(&format!("Error getting field name by index {name_index}"))
            })?;

            let descriptor_index = field_info.descriptor_index();
            let field_descriptor = cpool_helper.get_utf8(descriptor_index).ok_or_else(|| {
                Error::new_execution(&format!(
                    "Error getting field descriptor by index {descriptor_index}"
                ))
            })?;
            let descriptor: TypeDescriptor = field_descriptor.parse().map_err(|err| {
                Error::new_execution(&format!(
                    "Error parsing field descriptor {field_descriptor}: {err}"
                ))
            })?;

            descriptor_by_name.insert(field_name.clone(), descriptor.clone());

            if field_info.access_flags().contains(FieldFlags::ACC_STATIC) {
                static_field_by_name.insert(field_name, RefCell::new(Field::new(descriptor)));
            }
        }

        Ok((
            FieldDescriptors::new(descriptor_by_name),
            Fields::new(static_field_by_name),
        ))
    }
}
