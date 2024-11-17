use crate::execution_engine::executor::Executor;
use crate::heap::java_instance::FieldNameType;
use crate::method_area::cpool_helper::CPoolHelper;
use crate::method_area::field::Field;
use crate::method_area::java_method::JavaMethod;
use indexmap::IndexMap;
use jdescriptor::TypeDescriptor;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

const INTERFACE: u16 = 0x00000200;

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

    pub fn default_value_instance_fields(&self) -> HashMap<FieldNameType, Field> {
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

    pub fn get_field_offset(&self, field_name: &str) -> crate::error::Result<i64> {
        let key = self
            .non_static_field_descriptors
            .descriptor_by_name
            .iter()
            .find_map(|(key, _)| {
                let first = key.split(':').next().map(|n| n.to_string())?;
                if first == field_name {
                    Some(key)
                } else {
                    None
                }
            })
            .ok_or_else(|| {
                crate::error::Error::new_native(&format!("Field {field_name} not found"))
            })?;

        let offset = self
            .non_static_field_descriptors
            .descriptor_by_name
            .get_index_of(key)
            .ok_or_else(|| {
                crate::error::Error::new_native(&format!(
                    "Failed to get index by key {field_name}"
                ))
            })?;

        Ok(offset as i64)
    }

    pub fn get_field_name_by_offset(&self, offset: i64) -> crate::error::Result<String> {
        let (field_name, _) = self
            .non_static_field_descriptors
            .descriptor_by_name
            .get_index(offset as usize)
            .ok_or_else(|| {
                crate::error::Error::new_native(&format!("Failed to get entry by index {offset}"))
            })?;

        Ok(field_name.clone())
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
            crate::error::Error::new_native(&format!(
                "Method {full_signature} not found in {}",
                self.this_class_name
            ))
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
