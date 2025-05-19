use crate::error::Error;
use crate::heap::java_instance::{ClassName, FieldNameType};
use crate::method_area::cpool_helper::CPoolHelper;
use crate::method_area::field::Field;
use crate::method_area::java_method::JavaMethod;
use crate::method_area::method_area::with_method_area;
use crate::method_area::primitives_helper::PRIMITIVE_TYPE_BY_CODE;
use derive_new::new;
use getset::Getters;
use indexmap::{IndexMap, IndexSet};
use jdescriptor::TypeDescriptor;
use once_cell::sync::OnceCell;
use parking_lot::ReentrantMutex;
use std::cell::RefCell;
use std::sync::Arc;

const INTERFACE: u16 = 0x00000200;
type FullyQualifiedFieldName = String; // format: com/example/models/Person.name

#[derive(Debug, Getters)]
pub(crate) struct JavaClass {
    methods: Methods,
    static_fields: Fields,
    non_static_field_properties: FieldProperties,
    cpool_helper: CPoolHelper,
    this_class_name: String,
    #[get = "pub"]
    external_name: String,
    parent: Option<String>,
    interfaces: IndexSet<String>,
    access_flags: u16,

    #[get = "pub"]
    static_fields_init_state: Arc<ReentrantMutex<InitState>>,

    instance_fields_hierarchy: OnceCell<IndexMap<ClassName, IndexMap<FieldNameType, Field>>>,
    fields_offset_mapping: OnceCell<IndexSet<FullyQualifiedFieldName>>,
    #[get = "pub"]
    declaring_class: Option<String>,
    #[get = "pub"]
    annotations_raw: Option<Vec<u8>>,
    #[get = "pub"]
    enclosing_method: Option<(String, String, String)>,
    #[get = "pub"]
    source_file: Option<String>,
}

#[derive(Debug, Default)]
pub struct InitState {
    inner_state: RefCell<InnerState>,
}

impl InitState {
    pub fn set_inner_state(&self, new_state: InnerState) {
        *self.inner_state.borrow_mut() = new_state;
    }

    pub fn get_inner_state(&self) -> InnerState {
        *self.inner_state.borrow()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InnerState {
    NotInitialized,
    Initializing,
    Initialized,
}

impl Default for InnerState {
    fn default() -> Self {
        InnerState::NotInitialized
    }
}

#[derive(Debug, new)]
pub(crate) struct Methods {
    method_by_signature: IndexMap<String, Arc<JavaMethod>>,
}

#[derive(Debug, new)]
pub(crate) struct Fields {
    pub(crate) field_by_name: IndexMap<String, Arc<Field>>,
}

#[derive(Debug, new, Getters)]
#[get = "pub"]
pub(crate) struct FieldProperty {
    type_descriptor: TypeDescriptor,
    flags: u16,
}

#[derive(Debug, new)]
pub(crate) struct FieldProperties {
    pub(crate) properties_by_name: IndexMap<String, FieldProperty>,
}

impl JavaClass {
    pub fn new(
        methods: Methods,
        static_fields: Fields,
        non_static_field_properties: FieldProperties,
        cpool_helper: CPoolHelper,
        this_class_name: &str,
        parent: Option<String>,
        interfaces: IndexSet<String>,
        access_flags: u16,
        declaring_class: Option<String>,
        annotations_raw: Option<Vec<u8>>,
        enclosing_method: Option<(String, String, String)>,
        source_file: Option<String>,
    ) -> Self {
        let external_name = PRIMITIVE_TYPE_BY_CODE
            .get(this_class_name)
            .map(|name| name.to_string())
            .unwrap_or_else(|| this_class_name.replace("/", "."));

        Self {
            methods,
            static_fields,
            non_static_field_properties,
            cpool_helper,
            this_class_name: this_class_name.to_string(),
            external_name,
            parent,
            interfaces,
            access_flags,
            static_fields_init_state: Arc::default(),
            instance_fields_hierarchy: OnceCell::new(),
            fields_offset_mapping: OnceCell::new(),
            declaring_class,
            annotations_raw,
            enclosing_method,
            source_file,
        }
    }

    pub fn cpool_helper(&self) -> &CPoolHelper {
        &self.cpool_helper
    }

    pub fn parent(&self) -> &Option<String> {
        &self.parent
    }

    pub fn interfaces(&self) -> &IndexSet<String> {
        &self.interfaces
    }

    pub fn is_interface(&self) -> bool {
        self.access_flags & INTERFACE != 0
    }

    pub fn static_field(&self, field_name: &str) -> crate::error::Result<Option<Arc<Field>>> {
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
        let field_property = self
            .non_static_field_properties
            .properties_by_name
            .get(instance_field_name_type)?;

        Some(field_property.type_descriptor())
    }

    pub fn put_instance_field_descriptor(
        &mut self,
        name: String,
        type_descriptor: TypeDescriptor,
        flags: u16,
    ) -> Option<FieldProperty> {
        self.non_static_field_properties
            .properties_by_name
            .insert(name, FieldProperty::new(type_descriptor, flags))
    }

    pub fn default_value_instance_fields(
        &self,
    ) -> crate::error::Result<IndexMap<FieldNameType, Field>> {
        self.non_static_field_properties
            .properties_by_name
            .iter()
            .map(|(name, property)| Ok((name.clone(), Field::try_from(property)?)))
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
            .fields_offset_mapping()?
            .get_index_of(fully_qualified_field_name)
            .ok_or_else(|| {
                Error::new_execution(&format!(
                    "Failed to get offset by name {fully_qualified_field_name}"
                ))
            })?;
        Ok(offset as i64)
    }

    pub fn get_static_field_offset(&self, field_name: &str) -> crate::error::Result<i64> {
        let offset = self
            .static_fields
            .field_by_name
            .get_index_of(field_name)
            .ok_or_else(|| {
                Error::new_execution(&format!(
                    "Failed to get static field offset by name {field_name}"
                ))
            })?;
        Ok(offset as i64)
    }

    pub fn get_static_field_by_offset(&self, offset: i64) -> crate::error::Result<Arc<Field>> {
        let (_field_name, field) = self
            .static_fields
            .field_by_name
            .get_index(offset as usize)
            .ok_or_else(|| {
                Error::new_execution(&format!("Failed to get static field by offset {offset}"))
            })?;
        Ok(Arc::clone(&field))
    }

    pub fn get_field_name_by_offset(&self, offset: i64) -> crate::error::Result<(String, String)> {
        let result = self
            .fields_offset_mapping()?
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

    pub fn get_method_full(&self, full_signature: &str) -> Option<(usize, Arc<JavaMethod>)> {
        self.methods
            .method_by_signature
            .get_full(full_signature)
            .map(|(index, _key, method)| (index, Arc::clone(method)))
            .or_else(|| {
                // we have not found the method by full signature, let's treat it as polymorphic signature method and try to find it by method name only
                self.methods
                    .method_by_signature
                    .get_full(full_signature.split(':').next()?)
                    .map(|(index, _key, method)| (index, Arc::clone(method)))
            })
    }

    pub fn try_get_method(&self, full_signature: &str) -> Option<Arc<JavaMethod>> {
        self.get_method_full(full_signature)
            .and_then(|(_index, method)| Some(Arc::clone(&method)))
    }

    pub fn get_method(&self, full_signature: &str) -> crate::error::Result<Arc<JavaMethod>> {
        self.get_method_full(full_signature)
            .and_then(|(_index, method)| Some(Arc::clone(&method)))
            .ok_or_else(|| {
                Error::new_execution(&format!(
                    "Method {full_signature} not found in {}",
                    self.this_class_name
                ))
            })
    }

    pub fn get_method_by_index(&self, method_index: i64) -> crate::error::Result<Arc<JavaMethod>> {
        self.methods
            .method_by_signature
            .get_index(method_index as usize)
            .and_then(|(_key, method)| Some(Arc::clone(&method)))
            .ok_or_else(|| {
                Error::new_execution(&format!("Failed to get method by index {method_index}"))
            })
    }

    pub fn get_methods(&self) -> Vec<Arc<JavaMethod>> {
        self.methods
            .method_by_signature
            .iter()
            .map(|(_, v)| Arc::clone(v))
            .collect::<Vec<_>>()
    }

    pub fn instance_fields_hierarchy(
        &self,
    ) -> crate::error::Result<&IndexMap<ClassName, IndexMap<FieldNameType, Field>>> {
        self.instance_fields_hierarchy.get_or_try_init(|| {
            let mut instance_fields_hierarchy = IndexMap::new();
            with_method_area(|area| {
                area.lookup_and_fill_instance_fields_hierarchy(
                    &self.this_class_name,
                    &mut instance_fields_hierarchy,
                )
            })?;

            Ok(instance_fields_hierarchy)
        })
    }

    fn fields_offset_mapping(&self) -> crate::error::Result<&IndexSet<FullyQualifiedFieldName>> {
        self.fields_offset_mapping.get_or_try_init(|| {
            let mut fields_offset_mapping = IndexSet::new();
            let hierarchy = self.instance_fields_hierarchy()?;

            hierarchy.iter().for_each(|(class_name, fields)| {
                fields.iter().for_each(|(field_name, _)| {
                    fields_offset_mapping.insert(format!("{class_name}.{field_name}"));
                });
            });

            Ok(fields_offset_mapping)
        })
    }
}
