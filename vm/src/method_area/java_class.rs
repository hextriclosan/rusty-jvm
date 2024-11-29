use crate::error::Error;
use crate::execution_engine::executor::Executor;
use crate::heap::java_instance::{ClassName, FieldNameType};
use crate::method_area::cpool_helper::CPoolHelper;
use crate::method_area::field::Field;
use crate::method_area::java_method::JavaMethod;
use crate::method_area::method_area::with_method_area;
use indexmap::{IndexMap, IndexSet};
use jdescriptor::TypeDescriptor;
use once_cell::sync::OnceCell;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

const INTERFACE: u16 = 0x00000200;
type FullyQualifiedFieldName = String; // format: com/example/models/Person.name

#[derive(Debug)]
pub(crate) struct JavaClass {
    methods: Methods,
    static_fields: Fields,
    non_static_field_descriptors: FieldDescriptors,
    cpool_helper: CPoolHelper,
    this_class_name: String,
    parent: Option<String>,
    interfaces: HashSet<String>,
    access_flags: u16,

    static_fields_initialized: AtomicBool,

    instance_fields_hierarchy: OnceCell<IndexMap<ClassName, IndexMap<FieldNameType, Field>>>,
    fields_offset_mapping: OnceCell<IndexSet<FullyQualifiedFieldName>>,
}

#[derive(Debug)]
pub(crate) struct Methods {
    method_by_signature: HashMap<String, Arc<JavaMethod>>,
}

#[derive(Debug)]
pub(crate) struct Fields {
    pub(crate) field_by_name: HashMap<String, Arc<Field>>,
}

#[derive(Debug)]
pub(crate) struct FieldDescriptors {
    pub(crate) descriptor_by_name: IndexMap<String, TypeDescriptor>,
}

impl FieldDescriptors {
    pub fn new(descriptor_by_name: IndexMap<String, TypeDescriptor>) -> Self {
        Self { descriptor_by_name }
    }
}

impl Fields {
    pub fn new(field_by_name: HashMap<String, Arc<Field>>) -> Self {
        Self { field_by_name }
    }
}

impl JavaClass {
    pub fn new(
        methods: Methods,
        static_fields: Fields,
        non_static_field_descriptors: FieldDescriptors,
        cpool_helper: CPoolHelper,
        this_class_name: String,
        parent: Option<String>,
        interfaces: HashSet<String>,
        access_flags: u16,
    ) -> Self {
        Self {
            methods,
            static_fields,
            non_static_field_descriptors,
            cpool_helper,
            this_class_name,
            parent,
            interfaces,
            access_flags,
            static_fields_initialized: AtomicBool::new(false),
            instance_fields_hierarchy: OnceCell::new(),
            fields_offset_mapping: OnceCell::new(),
        }
    }

    pub fn cpool_helper(&self) -> &CPoolHelper {
        &self.cpool_helper
    }

    pub fn parent(&self) -> &Option<String> {
        &self.parent
    }

    pub fn interfaces(&self) -> &HashSet<String> {
        &self.interfaces
    }

    pub fn is_interface(&self) -> bool {
        self.access_flags & INTERFACE != 0
    }

    pub fn static_field(&self, field_name: &str) -> crate::error::Result<Option<Arc<Field>>> {
        if self
            .static_fields_initialized
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
        {
            Executor::do_java_class_static_fields_initialization(self)?;
        }

        Ok(self
            .static_fields
            .field_by_name
            .get(field_name)
            .map(|field| Arc::clone(field)))
    }

    pub fn instance_field_descriptor(
        &self,
        instance_field_name_type: &str,
    ) -> Option<&TypeDescriptor> {
        self.non_static_field_descriptors
            .descriptor_by_name
            .get(instance_field_name_type)
    }

    pub fn default_value_instance_fields(&self) -> IndexMap<FieldNameType, Field> {
        self.non_static_field_descriptors
            .descriptor_by_name
            .iter()
            .map(|(name, descriptor)| (name.clone(), Field::new(descriptor.to_owned())))
            .collect()
    }

    pub fn this_class_name(&self) -> &str {
        &self.this_class_name
    }

    pub fn access_flags(&self) -> u16 {
        self.access_flags
    }

    pub fn get_field_offset(&self, fully_qualified_field_name: &str) -> crate::error::Result<i64> {
        let offset = self
            .fields_offset_mapping()
            .get_index_of(fully_qualified_field_name)
            .ok_or_else(|| {
                Error::new_execution(&format!(
                    "Failed to get offset by name {fully_qualified_field_name}"
                ))
            })?;
        Ok(offset as i64)
    }

    pub fn get_field_name_by_offset(&self, offset: i64) -> crate::error::Result<(String, String)> {
        let result = self
            .fields_offset_mapping()
            .get_index(offset as usize)
            .ok_or_else(|| {
                Error::new_execution(&format!("Failed to get field name by offset {offset}"))
            })?;

        let mut parts = result.split('.');
        let class_name = parts.next().ok_or_else(|| {
            Error::new_execution(&format!("Failed to get class name by offset {offset}"))
        })?;
        let field_name = parts.next().ok_or_else(|| {
            Error::new_execution(&format!("Failed to get field name by offset {offset}"))
        })?;

        Ok((class_name.to_string(), field_name.to_string()))
    }

    fn get_method_internal(&self, full_signature: &str) -> Option<Arc<JavaMethod>> {
        self.methods
            .method_by_signature
            .get(full_signature)
            .map(|method| Arc::clone(method))
    }

    pub fn try_get_method(&self, full_signature: &str) -> Option<Arc<JavaMethod>> {
        self.get_method_internal(full_signature)
    }

    pub fn get_method(&self, full_signature: &str) -> crate::error::Result<Arc<JavaMethod>> {
        self.get_method_internal(full_signature).ok_or_else(|| {
            Error::new_native(&format!(
                "Method {full_signature} not found in {}",
                self.this_class_name
            ))
        })
    }

    pub fn instance_fields_hierarchy(
        &self,
    ) -> &IndexMap<ClassName, IndexMap<FieldNameType, Field>> {
        &self.instance_fields_hierarchy.get_or_init(|| {
            let mut instance_fields_hierarchy = IndexMap::new();
            with_method_area(|area| {
                area.lookup_and_fill_instance_fields_hierarchy(
                    &self.this_class_name,
                    &mut instance_fields_hierarchy,
                )
            })
            .expect("error getting instance fields hierarchy");

            instance_fields_hierarchy
        })
    }

    fn fields_offset_mapping(&self) -> &IndexSet<FullyQualifiedFieldName> {
        &self.fields_offset_mapping.get_or_init(|| {
            let mut fields_offset_mapping = IndexSet::new();
            let hierarchy = self.instance_fields_hierarchy();

            hierarchy.iter().for_each(|(class_name, fields)| {
                fields.iter().for_each(|(field_name, _)| {
                    fields_offset_mapping.insert(format!("{class_name}.{field_name}"));
                });
            });

            fields_offset_mapping
        })
    }
}

impl Methods {
    pub fn new(method_by_signature: HashMap<String, Arc<JavaMethod>>) -> Self {
        Self {
            method_by_signature,
        }
    }
}
