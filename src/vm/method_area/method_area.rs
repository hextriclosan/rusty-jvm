use crate::vm;
use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::ldc_resolution_manager::LdcResolutionManager;
use crate::vm::heap::java_instance::{ClassName, FieldNameType, JavaInstance};
use crate::vm::method_area::attributes_helper::AttributesHelper;
use crate::vm::method_area::cpool_helper::{CPoolHelper, CPoolHelperTrait};
use crate::vm::method_area::field::FieldValue;
use crate::vm::method_area::java_class::JavaClass;
use crate::vm::method_area::java_method::{CodeContext, JavaMethod};
use crate::vm::method_area::primitives_helper::PRIMITIVE_TYPE_BY_CODE;
use crate::vm::stack;
use crate::vm::system_native::class_loader::SYNTH_CLASS_DELIM;
use indexmap::{IndexMap, IndexSet};
use jclassfile::class_file::{parse, ClassFile};
use jclassfile::fields::{FieldFlags, FieldInfo};
use jclassfile::methods::{MethodFlags, MethodInfo};
use jdescriptor::TypeDescriptor;
use jimage_rs::jimage::JImage;
use once_cell::sync::OnceCell;
use std::collections::{BTreeMap, HashMap, HashSet};
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
    jimage: JImage,
    pub(crate) loaded_classes: RwLock<HashMap<String, Arc<JavaClass>>>,
    javaclass_by_reflectionref: RwLock<HashMap<i32, String>>,
    ldc_resolution_manager: LdcResolutionManager,
    system_thread_id: OnceCell<i32>, // initial thread, spawned by VM
    system_thread_group_id: OnceCell<i32>, // initial thread group, created by VM
}

impl MethodArea {
    pub(crate) fn init(java_home: &Path) -> Result<()> {
        METHOD_AREA
            .set(MethodArea::new(java_home)?)
            .map_err(|_| Error::new_execution("MethodArea already initialized"))
    }

    fn new(java_home: &Path) -> Result<Self> {
        let modules = java_home.join("lib").join("modules");
        let synthetic_classes = Self::generate_synthetic_classes();
        Ok(Self {
            jimage: JImage::open(modules)?,
            loaded_classes: RwLock::new(synthetic_classes),
            javaclass_by_reflectionref: RwLock::default(),
            ldc_resolution_manager: LdcResolutionManager::default(),
            system_thread_id: OnceCell::new(),
            system_thread_group_id: OnceCell::new(),
        })
    }

    pub(crate) fn get(&self, fully_qualified_class_name: &str) -> Result<Arc<JavaClass>> {
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
    ) -> Result<(String, String)> {
        let (internal, external) = derive_internal_and_external_names(fully_qualified_class_name);

        if let Some(jc) = self.loaded_classes.read()?.get(&internal) {
            return Ok((jc.this_class_name().clone(), jc.external_name().clone()));
        }

        let class_file = parse(bytecode)?;
        let (_, java_class) =
            self.to_java_class(class_file, internal.clone(), external.clone())?;
        self.loaded_classes
            .write()?
            .insert(internal.clone(), Arc::clone(&java_class));
        trace!("<META CLASS LOADED> -> {}", java_class.this_class_name());

        Ok((internal, external))
    }

    fn load_class_file(&self, fully_qualified_class_name: &str) -> Result<Arc<JavaClass>> {
        let maybe_std_file = format!("/java.base/{}.class", fully_qualified_class_name);
        if let Some(res) = self
            .jimage
            .find_resource(&maybe_std_file)
            .map_err(|jimage_error| Error::new_execution(&jimage_error.to_string()))?
        {
            match self.try_parse(&res) {
                Ok(Some(java_class)) => return Ok(java_class),
                Ok(None) => {}
                Err(e) => return Err(e),
            };
        }

        self.try_open_and_parse(&Path::new(fully_qualified_class_name).with_extension("class"))?
            .ok_or_else(|| {
                Error::new_execution(&format!(
                    "error opening class file {fully_qualified_class_name}"
                ))
            })
    }

    fn try_open_and_parse(&self, path: &PathBuf) -> Result<Option<Arc<JavaClass>>> {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None), // File not found is not considered as an error
            Err(e) => return Err(e.into()),
        };
        let mut buff = Vec::new();
        file.read_to_end(&mut buff)?;

        self.try_parse(&buff)
    }

    fn try_parse(&self, buff: &[u8]) -> Result<Option<Arc<JavaClass>>> {
        let class_file = parse(&buff)?;

        // todo: add and use handy wrapper_getter here
        let cpool_helper = CPoolHelper::new(class_file.constant_pool());
        let this_class_index = class_file.this_class();
        let class_name = cpool_helper
            .get_class_name(this_class_index)
            .ok_or_else(|| {
                Error::new_constant_pool(&format!(
                    "Error getting class_name by index={this_class_index}"
                ))
            })?;

        let (internal, external) = derive_internal_and_external_names(&class_name);
        self.to_java_class(class_file, internal, external)
            .map(|(_, java_class)| Ok(Some(java_class)))?
    }

    fn to_java_class(
        &self,
        class_file: ClassFile,
        class_name: String,
        external_name: String,
    ) -> Result<(String, Arc<JavaClass>)> {
        let cpool_helper = CPoolHelper::new(class_file.constant_pool());

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
            .collect::<Result<IndexSet<String>>>()?;

        let methods = Self::get_methods(&class_file.methods(), &cpool_helper, &class_name)?;
        let (fields_info, static_fields, instance_fields_template) =
            Self::get_fields(&class_file.fields(), &cpool_helper, &class_name)?;

        let access_flags = class_file.access_flags().bits();

        let attributes_helper = AttributesHelper::new(class_file.attributes());
        let declaring_class =
            Self::get_declaring_class(&attributes_helper, &cpool_helper, class_name.as_str());

        let annotations_raw = attributes_helper
            .get_annotations(&cpool_helper)
            .and_then(|(_annotations, annotations_raw)| Some(annotations_raw));

        let enclosing_method = Self::get_enclosing_method(&attributes_helper, &cpool_helper);
        let source_file = Self::get_source_file(&attributes_helper, &cpool_helper);

        Ok((
            class_name.clone(),
            Arc::new(JavaClass::new(
                methods,
                fields_info,
                static_fields,
                instance_fields_template,
                cpool_helper,
                attributes_helper,
                class_name,
                external_name,
                super_class_name,
                interface_names,
                access_flags,
                declaring_class,
                annotations_raw,
                enclosing_method,
                source_file,
            )),
        ))
    }

    fn get_methods(
        class_file_methods: &[MethodInfo],
        helper: &CPoolHelper,
        class_name: &str,
    ) -> Result<IndexMap<String, Arc<JavaMethod>>> {
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
            let code_context = if !access_flags
                .intersects(MethodFlags::ACC_ABSTRACT | MethodFlags::ACC_NATIVE)
            {
                attributes_helper
                    .get_code(helper)
                    .map(
                        |(max_stack, max_locals, code, line_numbers, exception_table)| {
                            let line_numbers = line_numbers
                                .iter()
                                .map(|record| (record.start_pc(), record.line_number()))
                                .collect::<BTreeMap<_, _>>();
                            Some(CodeContext::new(
                                max_stack,
                                max_locals,
                                Arc::new(code),
                                Arc::new(line_numbers),
                                Arc::new(stack::stack_frame::ExceptionTable::new(exception_table)),
                            ))
                        },
                    )
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

        Ok(method_by_signature)
    }

    fn get_fields(
        field_infos: &[FieldInfo],
        cpool_helper: &CPoolHelper,
        class_name: &str,
    ) -> Result<(
        IndexMap<String, Arc<vm::method_area::field::FieldInfo>>,
        IndexMap<String, Arc<FieldValue>>,
        IndexMap<String, FieldValue>,
    )> {
        let mut fields_info = IndexMap::new();
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
            fields_info.insert(
                field_name.clone(),
                Arc::new(vm::method_area::field::FieldInfo::new(
                    descriptor.clone(),
                    flags.bits(),
                    class_name.to_string(),
                    field_name.clone(),
                )),
            );
            if flags.contains(FieldFlags::ACC_STATIC) {
                static_field_by_name
                    .insert(field_name.clone(), Arc::new(FieldValue::new(descriptor)?));
            } else {
                non_static_field_properties.insert(field_name, FieldValue::new(descriptor)?);
            }
        }

        Ok((
            fields_info,
            static_field_by_name,
            non_static_field_properties,
        ))
    }

    pub fn lookup_for_static_field(
        &self,
        class_name: &str,
        field_name: &str,
    ) -> Result<(String, Arc<FieldValue>)> {
        let rc = self.get(class_name)?;

        if rc.is_interface() {
            self.lookup_for_static_field_in_interface(&rc, class_name, field_name)
        } else {
            self.lookup_for_static_field_in_class(&rc, class_name, field_name)
        }
    }

    fn lookup_for_static_field_in_class(
        &self,
        rc: &Arc<JavaClass>,
        class_name: &str,
        field_name: &str,
    ) -> Result<(String, Arc<FieldValue>)> {
        match rc.static_field(field_name) {
            Some(field) => Ok((class_name.to_string(), Arc::clone(&field))),
            None => match rc.parent() {
                Some(parent_class_name) => {
                    self.lookup_for_static_field(&parent_class_name, field_name)
                }
                None => Err(Error::new_execution(&format!(
                    "No field {class_name}.{field_name} found in class hierarchy"
                ))),
            },
        }
    }

    fn lookup_for_static_field_in_interface(
        &self,
        rc: &Arc<JavaClass>,
        class_name: &str,
        field_name: &str,
    ) -> Result<(String, Arc<FieldValue>)> {
        match rc.static_field(field_name) {
            Some(field) => Ok((class_name.to_string(), Arc::clone(&field))),
            None => {
                let interfaces = rc.interfaces();
                for interface_name in interfaces.iter() {
                    match self.lookup_for_static_field(&interface_name, field_name) {
                        Ok((interface_class_name, field)) => {
                            return Ok((interface_class_name, field));
                        }
                        Err(_) => continue,
                    }
                }

                Err(Error::new_execution(&format!(
                    "No field {class_name}.{field_name} found in class hierarchy"
                )))
            }
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

    pub fn create_instance_with_default_fields(&self, class_name: &str) -> Result<JavaInstance> {
        let jc = with_method_area(|area| area.get(class_name))?;
        Ok(JavaInstance::new(
            class_name.to_string(),
            jc.instance_fields_hierarchy()?.clone(),
        ))
    }

    pub(crate) fn lookup_and_fill_instance_fields_hierarchy(
        &self,
        class_name: &str,
        instance_fields_hierarchy: &mut IndexMap<ClassName, IndexMap<FieldNameType, FieldValue>>,
    ) -> Result<()> {
        let rc = self.get(class_name)?;
        if let Some(parent_class_name) = rc.parent().as_ref() {
            self.lookup_and_fill_instance_fields_hierarchy(
                parent_class_name,
                instance_fields_hierarchy,
            )?;
        };

        let instance_fields = rc.default_value_instance_fields();
        instance_fields_hierarchy.insert(class_name.to_string(), instance_fields);

        Ok(())
    }

    pub(crate) fn put_to_reflection_table(
        &self,
        reflection_ref: i32,
        java_class_name: &str,
    ) -> Result<()> {
        self.javaclass_by_reflectionref
            .write()?
            .insert(reflection_ref, java_class_name.to_string());
        Ok(())
    }

    pub(crate) fn get_from_reflection_table(&self, reflection_ref: i32) -> Result<String> {
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

        let (internal, external) = derive_internal_and_external_names(class_name);
        Arc::new(JavaClass::new(
            IndexMap::new(),
            IndexMap::new(),
            IndexMap::new(),
            IndexMap::new(),
            CPoolHelper::new(&Vec::new()),
            AttributesHelper::new(&Vec::new()),
            internal,
            external,
            None,
            IndexSet::new(),
            PUBLIC | FINAL | ABSTRACT,
            None,
            None,
            None,
            None,
        ))
    }

    fn generate_synthetic_array_class(array_class_name: &str) -> Arc<JavaClass> {
        const PUBLIC: u16 = 0x00000001;
        const FINAL: u16 = 0x00000010;
        const ABSTRACT: u16 = 0x00000400;

        let (internal, external) = derive_internal_and_external_names(array_class_name);
        Arc::new(JavaClass::new(
            IndexMap::new(),
            IndexMap::new(),
            IndexMap::new(),
            IndexMap::new(),
            CPoolHelper::new(&Vec::new()),
            AttributesHelper::new(&Vec::new()),
            internal,
            external,
            Some("java/lang/Object".to_string()),
            IndexSet::from([
                "java/lang/Cloneable".to_string(),
                "java/io/Serializable".to_string(),
            ]),
            PUBLIC | FINAL | ABSTRACT,
            None,
            None,
            None,
            None,
        ))
    }

    pub(crate) fn resolve_ldc(&self, current_class_name: &str, cpoolindex: u16) -> Result<i32> {
        self.ldc_resolution_manager
            .resolve_ldc(current_class_name, cpoolindex)
    }

    pub(crate) fn resolve_ldc2_w(&self, current_class_name: &str, cpoolindex: u16) -> Result<i64> {
        self.ldc_resolution_manager
            .resolve_ldc2_w(current_class_name, cpoolindex)
    }

    pub(crate) fn load_reflection_class(&self, name: &str) -> Result<i32> {
        self.ldc_resolution_manager.load_reflection_class(name)
    }

    pub fn system_thread_id(&self) -> Result<i32> {
        self.system_thread_id
            .get()
            .copied()
            .ok_or_else(|| Error::new_execution("system_thread_id wasn't set"))
    }

    pub fn set_system_thread_id(&self, thread_id: i32) -> Result<()> {
        self.system_thread_id.set(thread_id).map_err(|_| {
            Error::new_execution("system_thread_id was already set, cannot be set again")
        })
    }

    pub fn system_thread_group_id(&self) -> Result<i32> {
        self.system_thread_group_id
            .get()
            .copied()
            .ok_or_else(|| Error::new_execution("system_thread_group_id wasn't set"))
    }

    pub fn set_system_thread_group_id(&self, thread_group_id: i32) -> Result<()> {
        self.system_thread_group_id
            .set(thread_group_id)
            .map_err(|_| {
                Error::new_execution("system_thread_group_id was already set, cannot be set again")
            })
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

    fn get_source_file(
        attributes_helper: &AttributesHelper,
        cpool_helper: &CPoolHelper,
    ) -> Option<String> {
        attributes_helper.get_source_file(cpool_helper)
    }
}

/// Takes a raw name like "my/package/MyClass#0xABCDEF"
/// Returns (internal_name, external_name)
///
/// Examples:
///     "my/package/MyClass#0xABCDEF" -> ("my/package/MyClass/0xABCDEF", "my.package.MyClass/0xABCDEF")
///     "I" -> ("I", "int")
///     "my/package/MyClass" -> ("my/package/MyClass", "my.package.MyClass")
fn derive_internal_and_external_names(raw: &str) -> (String, String) {
    if let Some(external_name) = PRIMITIVE_TYPE_BY_CODE.get(raw) {
        // Check if the raw name is a primitive type
        let internal = raw.to_string();
        let external = external_name.to_string();
        (internal, external)
    } else if let Some(pos) = raw.rfind(SYNTH_CLASS_DELIM) {
        // Check for synthetic class delimiter
        let (base, suffix) = raw.split_at(pos);
        let suffix = &suffix[1..];

        let internal = format!("{}/{}", base, suffix);
        let external = format!("{}/{}", base.replace('/', "."), suffix);

        (internal, external)
    } else {
        // Just ordinary class name
        let internal = raw.to_string();
        let external = raw.replace('/', ".");
        (internal, external)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derive_internal_and_external_names_with_primitive_type() {
        let result = derive_internal_and_external_names("I");
        assert_eq!(result, ("I".to_string(), "int".to_string()));
    }

    #[test]
    fn derive_internal_and_external_names_with_synthetic_class_delim() {
        let result = derive_internal_and_external_names(
            "java/lang/invoke/LambdaForm$MH#0x0000000000000002",
        );
        assert_eq!(
            result,
            (
                "java/lang/invoke/LambdaForm$MH/0x0000000000000002".to_string(),
                "java.lang.invoke.LambdaForm$MH/0x0000000000000002".to_string()
            )
        );
    }

    #[test]
    fn derive_internal_and_external_names_without_synthetic_class_delim() {
        let result = derive_internal_and_external_names(
            "java/util/concurrent/ConcurrentHashMap$CollectionView",
        );
        assert_eq!(
            result,
            (
                "java/util/concurrent/ConcurrentHashMap$CollectionView".to_string(),
                "java.util.concurrent.ConcurrentHashMap$CollectionView".to_string()
            )
        );
    }

    #[test]
    fn derive_internal_and_external_names_with_arrays() {
        let result = derive_internal_and_external_names("[Ljava/lang/Class;");
        assert_eq!(
            result,
            (
                "[Ljava/lang/Class;".to_string(),
                "[Ljava.lang.Class;".to_string()
            )
        );
    }
}
