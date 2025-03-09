use crate::error::{Error, ErrorKind};
use crate::execution_engine::ldc_resolution_manager::LdcResolutionManager;
use crate::heap::java_instance::{ClassName, FieldNameType, JavaInstance};
use crate::method_area::attributes_helper::AttributesHelper;
use crate::method_area::cpool_helper::CPoolHelper;
use crate::method_area::field::Field;
use crate::method_area::java_class::{FieldProperties, FieldProperty, Fields, JavaClass, Methods};
use crate::method_area::java_method::{CodeContext, JavaMethod};
use crate::method_area::primitives_helper::PRIMITIVE_TYPE_BY_CODE;
use indexmap::{IndexMap, IndexSet};
use jclass::class_file::{parse, ClassFile};
use jclass::fields::{FieldFlags, FieldInfo};
use jclass::methods::{MethodFlags, MethodInfo};
use jdescriptor::TypeDescriptor;
use once_cell::sync::OnceCell;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use tracing::trace;

static METHOD_AREA: OnceCell<MethodArea> = OnceCell::new();

pub(crate) fn with_method_area<F, R>(f: F) -> R
where
    F: FnOnce(&MethodArea) -> R,
{
    let method_area = METHOD_AREA.get().expect("error getting method area");

    f(&method_area)
}

#[derive(Debug)]
pub(crate) struct MethodArea {
    std_dir: String,
    pub(crate) loaded_classes: RwLock<HashMap<String, Arc<JavaClass>>>,
    javaclass_by_reflectionref: RwLock<HashMap<i32, String>>,
    ldc_resolution_manager: LdcResolutionManager,
    system_thread_id: RwLock<Option<i32>>, // initial thread, spawned by VM
}

impl MethodArea {
    pub(crate) fn init(std_dir: &str) -> crate::error::Result<()> {
        METHOD_AREA
            .set(MethodArea::new(std_dir))
            .map_err(|_| Error::new_execution("MethodArea already initialized"))
    }

    fn new(std_dir: &str) -> Self {
        let synthetic_classes = Self::generate_synthetic_classes();
        Self {
            std_dir: std_dir.to_string(),
            loaded_classes: RwLock::new(synthetic_classes),
            javaclass_by_reflectionref: RwLock::new(HashMap::new()),
            ldc_resolution_manager: LdcResolutionManager::new(),
            system_thread_id: RwLock::new(None),
        }
    }

    pub(crate) fn get(
        &self,
        fully_qualified_class_name: &str,
    ) -> crate::error::Result<Arc<JavaClass>> {
        if let Some(java_class) = self.loaded_classes.read()?.get(fully_qualified_class_name) {
            return Ok(Arc::clone(java_class));
        }

        if fully_qualified_class_name.starts_with('[') {
            let arc = Self::generate_synthetic_array_class(fully_qualified_class_name);
            self.loaded_classes
                .write()?
                .insert(fully_qualified_class_name.to_string(), Arc::clone(&arc));
            return Ok(arc);
        }

        let fully_qualified_class_name = if fully_qualified_class_name.ends_with(";") {
            &fully_qualified_class_name[1..fully_qualified_class_name.len() - 1]
        } else {
            fully_qualified_class_name
        };

        //todo: make me thread-safe if move to multithreaded jvm
        let java_class = self.load_class_file(fully_qualified_class_name)?;
        self.loaded_classes.write()?.insert(
            fully_qualified_class_name.to_string(),
            Arc::clone(&java_class),
        );
        trace!("<CLASS LOADED> -> {}", java_class.this_class_name());

        Ok(java_class)
    }

    pub(crate) fn create_metaclass(
        &self,
        fully_qualified_class_name: &str,
        bytecode: &[u8],
    ) -> crate::error::Result<()> {
        if let Some(_) = self.loaded_classes.read()?.get(fully_qualified_class_name) {
            return Ok(());
        }

        let class_file =
            parse(bytecode).map_err(|err| Error::new(ErrorKind::ClassFile(err.to_string())))?;

        let (_, java_class) = self.to_java_class(class_file)?;
        self.loaded_classes.write()?.insert(
            fully_qualified_class_name.to_string(),
            Arc::clone(&java_class),
        );
        trace!("<META CLASS LOADED> -> {}", java_class.this_class_name());

        Ok(())
    }

    fn load_class_file(
        &self,
        fully_qualified_class_name: &str,
    ) -> crate::error::Result<Arc<JavaClass>> {
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

    fn try_open_and_parse(&self, path: &PathBuf) -> Option<Arc<JavaClass>> {
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
    ) -> crate::error::Result<(String, Arc<JavaClass>)> {
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
            .collect::<crate::error::Result<IndexSet<String>>>()?;

        let methods = Self::get_methods(&class_file.methods(), &cpool_helper, &class_name)?;
        let (non_static_field_descriptors, static_fields) =
            Self::get_field_descriptors(&class_file.fields(), &cpool_helper)?;

        let access_flags = class_file.access_flags().bits();

        let attributes_helper = AttributesHelper::new(class_file.attributes());
        let declaring_class =
            Self::get_declaring_class(&attributes_helper, &cpool_helper, class_name.as_str());

        let annotations_raw = attributes_helper
            .get_annotations(&cpool_helper)
            .and_then(|(_annotations, annotations_raw)| Some(annotations_raw));

        let enclosing_method = Self::get_enclosing_method(&attributes_helper, &cpool_helper);

        Ok((
            class_name.clone(),
            Arc::new(JavaClass::new(
                methods,
                static_fields,
                non_static_field_descriptors,
                cpool_helper,
                &class_name,
                super_class_name,
                interface_names,
                access_flags,
                declaring_class,
                annotations_raw,
                enclosing_method,
            )),
        ))
    }

    fn get_methods(
        class_file_methods: &[MethodInfo],
        helper: &CPoolHelper,
        class_name: &str,
    ) -> crate::error::Result<Methods> {
        let mut method_by_signature = IndexMap::new();

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

            let full_signature = format!("{method_name}:{method_signature}");
            let attributes_helper = AttributesHelper::new(method_info.attributes());

            let access_flags = method_info.access_flags();
            let code_context =
                if !access_flags.intersects(MethodFlags::ACC_ABSTRACT | MethodFlags::ACC_NATIVE) {
                    attributes_helper
                        .get_code()
                        .map(|(max_stack, max_locals, code)| {
                            Some(CodeContext::new(max_stack, max_locals, Arc::new(code)))
                        })
                        .ok_or_else(|| {
                            Error::new_execution(&format!(
                                "Error getting code attribute for method {full_signature}"
                            ))
                        })?
                } else {
                    None
                };

            let exception_indexes = attributes_helper.get_exception_indexes().unwrap_or(vec![]);

            let annotation_default_raw = attributes_helper.get_annotation_default_raw();
            let result = attributes_helper.get_annotations(&helper);

            let (runtime_visible_annotations, annotations_raw) = match result {
                Some((annotations, annotations_raw)) => (annotations, Some(annotations_raw)),
                None => (HashSet::new(), None),
            };

            let method_descriptor = method_signature.parse().map_err(|err| {
                Error::new_execution(&format!(
                    "Error parsing signature {method_signature}: {err}"
                ))
            })?;

            let native = access_flags.contains(MethodFlags::ACC_NATIVE);

            let key = if native
                && runtime_visible_annotations
                    .contains("Ljava/lang/invoke/MethodHandle$PolymorphicSignature;")
            {
                method_name.clone()
            } else {
                full_signature.clone()
            };

            method_by_signature.insert(
                key,
                Arc::new(JavaMethod::new(
                    method_descriptor,
                    class_name,
                    &full_signature,
                    code_context,
                    native,
                    exception_indexes,
                    access_flags.bits() as i32,
                    &method_name,
                    annotation_default_raw,
                    annotations_raw,
                    runtime_visible_annotations,
                )),
            );
        }

        Ok(Methods::new(method_by_signature))
    }

    fn get_field_descriptors(
        field_infos: &[FieldInfo],
        cpool_helper: &CPoolHelper,
    ) -> crate::error::Result<(FieldProperties, Fields)> {
        let mut non_static_field_properties = IndexMap::new();
        let mut static_field_by_name = IndexMap::new();
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

            let flags = field_info.access_flags();
            if flags.contains(FieldFlags::ACC_STATIC) {
                static_field_by_name
                    .insert(field_name, Arc::new(Field::new(descriptor, flags.bits())?));
            } else {
                non_static_field_properties
                    .insert(field_name, FieldProperty::new(descriptor, flags.bits()));
            }
        }

        Ok((
            FieldProperties::new(non_static_field_properties),
            Fields::new(static_field_by_name),
        ))
    }

    pub fn lookup_for_static_field(
        &self,
        class_name: &str,
        field_name: &str,
    ) -> crate::error::Result<Arc<Field>> {
        let rc = self.get(class_name)?;

        match rc.static_field(field_name)? {
            Some(field) => Ok(Arc::clone(&field)),
            None => match rc.parent() {
                Some(parent_class_name) => {
                    self.lookup_for_static_field(&parent_class_name, field_name)
                }
                None => Err(Error::new_execution(&format!(
                    "No field {field_name} found in class hierarchy"
                ))),
            },
        }
    }

    pub fn lookup_for_implementation(
        &self,
        class_name: &str,
        full_method_signature: &str,
    ) -> Option<Arc<JavaMethod>> {
        let rc = self.get(class_name).ok()?;

        if let Some(java_method) = rc.try_get_method(full_method_signature) {
            Some(Arc::clone(&java_method))
        } else {
            let parent_class_name = rc.parent().as_ref()?;
            self.lookup_for_implementation(parent_class_name, full_method_signature)
        }
    }

    pub fn lookup_for_implementation_interface(
        &self,
        class_name: &str,
        full_method_signature: &str,
    ) -> Option<Arc<JavaMethod>> {
        let rc = self.get(class_name).ok()?;
        if let Some(java_method) =
            // lookup in interfaces for default methods
            self.lookup_in_interface_hierarchy(rc.interfaces(), full_method_signature)
        {
            return Some(java_method);
        }

        // if not found in interfaces of current class, lookup in parent class
        let parent_class_name = rc.parent().as_ref()?;
        self.lookup_for_implementation_interface(parent_class_name, full_method_signature)
    }

    fn lookup_in_interface_hierarchy(
        &self,
        interfaces: &IndexSet<String>,
        full_method_signature: &str,
    ) -> Option<Arc<JavaMethod>> {
        for interface_name in interfaces.iter() {
            if let Some(interface_class) = self.get(interface_name).ok() {
                if let Some(java_method) = interface_class.try_get_method(full_method_signature) {
                    return Some(java_method);
                }

                if let Some(java_method) = self.lookup_in_interface_hierarchy(
                    interface_class.interfaces(),
                    full_method_signature,
                ) {
                    return Some(java_method);
                }
            }
        }

        None
    }

    pub fn lookup_for_field_descriptor(
        &self,
        class_name: &str,
        field_name: &str,
    ) -> Option<TypeDescriptor> {
        let rc = self.get(class_name).ok()?;

        if let Some(type_descriptor) = rc.instance_field_descriptor(field_name) {
            Some(type_descriptor.clone())
        } else {
            let parent_class_name = rc.parent().clone()?;

            self.lookup_for_field_descriptor(&parent_class_name, field_name)
        }
    }

    pub fn create_instance_with_default_fields(
        &self,
        class_name: &str,
    ) -> crate::error::Result<JavaInstance> {
        let jc = with_method_area(|area| area.get(class_name))?;
        Ok(JavaInstance::new(
            class_name.to_string(),
            jc.instance_fields_hierarchy()?.clone(),
        ))
    }

    pub(crate) fn lookup_and_fill_instance_fields_hierarchy(
        &self,
        class_name: &str,
        instance_fields_hierarchy: &mut IndexMap<ClassName, IndexMap<FieldNameType, Field>>,
    ) -> crate::error::Result<()> {
        let rc = self.get(class_name)?;
        if let Some(parent_class_name) = rc.parent().as_ref() {
            self.lookup_and_fill_instance_fields_hierarchy(
                parent_class_name,
                instance_fields_hierarchy,
            )?;
        };

        let instance_fields = rc.default_value_instance_fields()?;
        instance_fields_hierarchy.insert(class_name.to_string(), instance_fields);

        Ok(())
    }

    pub(crate) fn put_to_reflection_table(
        &self,
        reflection_ref: i32,
        java_class_name: &str,
    ) -> crate::error::Result<()> {
        self.javaclass_by_reflectionref
            .write()?
            .insert(reflection_ref, java_class_name.to_string());
        Ok(())
    }

    pub(crate) fn get_from_reflection_table(
        &self,
        reflection_ref: i32,
    ) -> crate::error::Result<String> {
        self.javaclass_by_reflectionref
            .read()?
            .get(&reflection_ref)
            .and_then(|class_name| Some(class_name.clone()))
            .ok_or_else(|| {
                Error::new_execution(&format!(
                    "error getting class name by reflection ref {reflection_ref}"
                ))
            })
    }

    fn generate_synthetic_classes() -> HashMap<String, Arc<JavaClass>> {
        PRIMITIVE_TYPE_BY_CODE
            .keys()
            .map(|class_name| {
                (
                    class_name.to_string(),
                    Self::generate_synthetic_class(class_name),
                )
            })
            .collect()
    }

    fn generate_synthetic_class(class_name: &str) -> Arc<JavaClass> {
        const PUBLIC: u16 = 0x00000001;
        const FINAL: u16 = 0x00000010;
        const ABSTRACT: u16 = 0x00000400;
        Arc::new(JavaClass::new(
            Methods::new(IndexMap::new()),
            Fields::new(IndexMap::new()),
            FieldProperties::new(IndexMap::new()),
            CPoolHelper::new(&Vec::new()),
            class_name,
            None,
            IndexSet::new(),
            PUBLIC | FINAL | ABSTRACT,
            None,
            None,
            None,
        ))
    }

    fn generate_synthetic_array_class(array_class_name: &str) -> Arc<JavaClass> {
        const PUBLIC: u16 = 0x00000001;
        const FINAL: u16 = 0x00000010;
        const ABSTRACT: u16 = 0x00000400;
        Arc::new(JavaClass::new(
            Methods::new(IndexMap::new()),
            Fields::new(IndexMap::new()),
            FieldProperties::new(IndexMap::new()),
            CPoolHelper::new(&Vec::new()),
            array_class_name,
            Some("java/lang/Object".to_string()),
            IndexSet::from([
                "java/lang/Cloneable".to_string(),
                "java/io/Serializable".to_string(),
            ]),
            PUBLIC | FINAL | ABSTRACT,
            None,
            None,
            None,
        ))
    }

    pub(crate) fn resolve_ldc(
        &self,
        current_class_name: &str,
        cpoolindex: u16,
    ) -> crate::error::Result<i32> {
        self.ldc_resolution_manager
            .resolve_ldc(current_class_name, cpoolindex)
    }

    pub(crate) fn resolve_ldc2_w(
        &self,
        current_class_name: &str,
        cpoolindex: u16,
    ) -> crate::error::Result<i64> {
        self.ldc_resolution_manager
            .resolve_ldc2_w(current_class_name, cpoolindex)
    }

    pub(crate) fn load_reflection_class(&self, name: &str) -> crate::error::Result<i32> {
        self.ldc_resolution_manager.load_reflection_class(name)
    }

    pub fn system_thread_id(&self) -> crate::error::Result<i32> {
        self.system_thread_id
            .read()?
            .ok_or_else(|| Error::new_execution("system_thread_id wasn't set"))
    }

    pub fn set_system_thread_id(&self, thread_id: i32) -> crate::error::Result<()> {
        let mut guard = self.system_thread_id.write()?;
        *guard = Some(thread_id);
        Ok(())
    }

    fn get_declaring_class(
        attributes_helper: &AttributesHelper,
        cpool_helper: &CPoolHelper,
        class_name: &str,
    ) -> Option<String> {
        let inner_class_records = attributes_helper.get_inner_class_records()?;

        inner_class_records.iter().find_map(|inner_class_record| {
            let inner_class_info_index = inner_class_record.inner_class_info_index();
            let inner_class_info = cpool_helper.get_class_name(inner_class_info_index)?;

            if class_name == inner_class_info {
                let outer_class_info_index = inner_class_record.outer_class_info_index();
                let outer_class_info = cpool_helper.get_class_name(outer_class_info_index)?;

                Some(outer_class_info)
            } else {
                None
            }
        })
    }

    fn get_enclosing_method(
        attributes_helper: &AttributesHelper,
        cpool_helper: &CPoolHelper,
    ) -> Option<(String, String, String)> {
        let (class_index, method_index) = attributes_helper.get_enclosing_method()?;

        let class_name = cpool_helper.get_class_name(class_index)?;
        let (name, descriptor) = cpool_helper.get_name_and_type(method_index)?;

        Some((class_name, name, descriptor))
    }
}
