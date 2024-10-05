use crate::error::{Error, ErrorKind};
use crate::heap::heap::Heap;
use crate::heap::java_instance::{ClassName, FieldNameType, JavaInstance};
use crate::method_area::attributes_helper::AttributesHelper;
use crate::method_area::cpool_helper::CPoolHelper;
use crate::method_area::field::Field;
use crate::method_area::java_class::{FieldDescriptors, Fields, JavaClass, Methods};
use crate::method_area::java_method::{CodeContext, JavaMethod};
use indexmap::IndexMap;
use jclass::class_file::{parse, ClassFile};
use jclass::fields::{FieldFlags, FieldInfo};
use jclass::methods::{MethodFlags, MethodInfo};
use jdescriptor::TypeDescriptor;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub(crate) struct MethodArea {
    std_dir: String,
    pub(crate) loaded_classes: RefCell<HashMap<String, Rc<JavaClass>>>,
    heap: Rc<RefCell<Heap>>,
    self_ref: RefCell<Weak<RefCell<MethodArea>>>,
}

impl MethodArea {
    pub fn new(std_dir: &str, heap: Rc<RefCell<Heap>>) -> Rc<RefCell<Self>> {
        let method_area = Rc::new(RefCell::new(MethodArea {
            std_dir: std_dir.to_string(),
            loaded_classes: RefCell::new(HashMap::new()),
            heap,
            self_ref: RefCell::new(Weak::new()),
        }));

        method_area
            .borrow_mut()
            .self_ref
            .replace(Rc::downgrade(&method_area));
        method_area
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
        println!("<CLASS LOADED> -> {}", java_class.this_class_name());

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
            .find_map(|file_name| self.try_open_and_parse(file_name))
            .ok_or_else(|| {
                Error::new_execution(&format!("error opening file {fully_qualified_class_name}"))
            })
    }

    fn try_open_and_parse(&self, path: &PathBuf) -> Option<Rc<JavaClass>> {
        let mut file = File::open(path).ok()?;
        let mut buff = Vec::new();
        file.read_to_end(&mut buff).ok()?;

        let class_file = parse(buff.as_slice())
            .map_err(|err| Error::new(ErrorKind::ClassFile(err.to_string())))
            .ok()?;

        self.to_java_class(class_file)
            .map(|(_, java_class)| java_class)
            .ok()
    }

    fn to_java_class(
        &self,
        class_file: ClassFile,
    ) -> crate::error::Result<(String, Rc<JavaClass>)> {
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
        let super_class_name = if super_class_index > 0 {
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
        let interface_names = interface_indexes
            .iter()
            .map(|index| {
                cpool_helper.get_class_name(*index).ok_or_else(|| {
                    Error::new_constant_pool(&format!("Error getting interface by index={index}"))
                })
            })
            .collect::<crate::error::Result<Vec<String>>>()?;

        let methods = Self::get_methods(&class_file.methods(), &cpool_helper, &class_name)?;
        let (non_static_field_descriptors, static_fields) =
            Self::get_field_descriptors(&class_file.fields(), &cpool_helper)?;

        if let Some(method_area) = self.self_ref.borrow().upgrade() {
            Ok((
                class_name.clone(),
                Rc::new(JavaClass::new(
                    methods,
                    static_fields,
                    non_static_field_descriptors,
                    cpool_helper,
                    class_name,
                    super_class_name,
                    interface_names,
                    Rc::clone(&self.heap),
                    Rc::clone(&method_area),
                )),
            ))
        } else {
            Err(Error::new_execution("Error upgrading method_area"))
        }
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

            let code_context = if !method_info
                .access_flags()
                .intersects(MethodFlags::ACC_ABSTRACT | MethodFlags::ACC_NATIVE)
            {
                AttributesHelper::new(method_info.attributes())
                    .get_code()
                    .map(|(max_stack, max_locals, code)| {
                        Some(CodeContext::new(max_stack, max_locals, code))
                    })
                    .ok_or_else(|| {
                        Error::new_execution(&format!(
                            "Error getting code attribute for method {key}"
                        ))
                    })?
            } else {
                None
            };

            let method_descriptor = method_signature.parse().map_err(|err| {
                Error::new_execution(&format!(
                    "Error parsing signature {method_signature}: {err}"
                ))
            })?;

            let native = method_info.access_flags().contains(MethodFlags::ACC_NATIVE);

            method_by_signature.insert(
                key.clone(),
                Rc::new(JavaMethod::new(
                    method_descriptor,
                    class_name,
                    &key,
                    code_context,
                    native,
                )),
            );
        }

        Ok(Methods::new(method_by_signature))
    }

    fn get_field_descriptors(
        field_infos: &[FieldInfo],
        cpool_helper: &CPoolHelper,
    ) -> crate::error::Result<(FieldDescriptors, Fields)> {
        let mut non_static_field_descriptors = HashMap::new();
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

            if field_info.access_flags().contains(FieldFlags::ACC_STATIC) {
                static_field_by_name
                    .insert(field_name, Rc::new(RefCell::new(Field::new(descriptor))));
            } else {
                let field_name_key = format!("{field_name}:{field_descriptor}");
                non_static_field_descriptors.insert(field_name_key, descriptor);
            }
        }

        Ok((
            FieldDescriptors::new(non_static_field_descriptors),
            Fields::new(static_field_by_name),
        ))
    }

    pub fn lookup_for_static_field(
        &self,
        class_name: &str,
        field_name: &str,
    ) -> Option<Rc<RefCell<Field>>> {
        let rc = self.get(class_name).ok()?;

        if let Some(field) = rc.static_field(field_name).ok() {
            Some(Rc::clone(&field))
        } else {
            let parent_class_name = rc.parent().clone()?;

            self.lookup_for_static_field(&parent_class_name, field_name)
        }
    }

    pub fn lookup_for_implementation(
        &self,
        class_name: &str,
        full_method_signature: &str,
    ) -> Option<Rc<JavaMethod>> {
        let rc = self.get(class_name).ok()?;

        if let Some(java_method) = rc.methods.method_by_signature.get(full_method_signature) {
            Some(Rc::clone(&java_method))
        } else {
            let parent_class_name = rc.parent().clone()?;

            self.lookup_for_implementation(&parent_class_name, full_method_signature)
        }
    }

    pub fn lookup_for_field_descriptor(
        &self,
        class_name: &str,
        field_name_type: &str,
    ) -> Option<TypeDescriptor> {
        let rc = self.get(class_name).ok()?;

        if let Some(type_descriptor) = rc.instance_field_descriptor(field_name_type) {
            Some(type_descriptor.clone())
        } else {
            let parent_class_name = rc.parent().clone()?;

            self.lookup_for_field_descriptor(&parent_class_name, field_name_type)
        }
    }

    pub fn create_instance_with_default_fields(&self, class_name: &str) -> JavaInstance {
        let mut instance_fields_hierarchy = IndexMap::new();
        self.lookup_and_fill_instance_fields_hierarchy(class_name, &mut instance_fields_hierarchy);
        JavaInstance::new(class_name.to_string(), instance_fields_hierarchy)
    }

    fn lookup_and_fill_instance_fields_hierarchy(
        &self,
        class_name: &str,
        instance_fields_hierarchy: &mut IndexMap<ClassName, HashMap<FieldNameType, Field>>,
    ) -> Option<Rc<JavaMethod>> {
        let rc = self.get(class_name).ok()?;
        let instance_fields = rc.default_value_instance_fields();
        instance_fields_hierarchy.insert(class_name.to_string(), instance_fields);

        let parent_class_name = rc.parent().clone()?;

        self.lookup_and_fill_instance_fields_hierarchy(
            &parent_class_name,
            instance_fields_hierarchy,
        )
    }
}
