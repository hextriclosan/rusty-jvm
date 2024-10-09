use crate::error::Error;
use crate::execution_engine::engine::Engine;
use crate::heap::heap::with_heap_write_lock;
use crate::heap::java_instance::FieldNameType;
use crate::method_area::cpool_helper::CPoolHelper;
use crate::method_area::field::Field;
use crate::method_area::java_method::JavaMethod;
use crate::method_area::method_area::with_method_area;
use jdescriptor::TypeDescriptor;
use once_cell::sync::OnceCell;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

const INTERFACE: u16 = 0x00000200;

#[derive(Debug)]
pub(crate) struct JavaClass {
    pub(crate) methods: Methods,
    static_fields: Fields,
    non_static_field_descriptors: FieldDescriptors,
    cpool_helper: CPoolHelper,
    this_class_name: String,
    parent: Option<String>,
    interfaces: HashSet<String>,
    access_flags: u16,

    static_fields_initialized: AtomicBool,

    reflection_ref: OnceCell<i32>,
}

#[derive(Debug)]
pub(crate) struct Methods {
    pub(crate) method_by_signature: HashMap<String, Arc<JavaMethod>>,
}

#[derive(Debug)]
pub(crate) struct Fields {
    pub(crate) field_by_name: HashMap<String, Arc<Field>>,
}

#[derive(Debug)]
pub(crate) struct FieldDescriptors {
    pub(crate) descriptor_by_name: HashMap<String, TypeDescriptor>,
}

impl FieldDescriptors {
    pub fn new(descriptor_by_name: HashMap<String, TypeDescriptor>) -> Self {
        Self { descriptor_by_name }
    }
}

impl Fields {
    pub fn new(field_by_name: HashMap<String, Arc<Field>>) -> Self {
        Self { field_by_name }
    }
}

impl JavaClass {
    const STATIC_INIT_METHOD: &'static str = "<clinit>:()V";

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
            reflection_ref: OnceCell::new(),
        }
    }

    fn create_reflection_instance(&self) -> i32 {
        let reflection_instance = with_method_area(|method_area| {
            method_area.create_instance_with_default_fields("java/lang/Class")
        });

        let reflection_reference =
            with_heap_write_lock(|heap| heap.create_instance(reflection_instance));

        with_method_area(|method_area| {
            method_area.put_to_reflection_table(reflection_reference, &self.this_class_name)
        });

        reflection_reference
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

    pub fn static_field(&self, field_name: &str) -> crate::error::Result<Arc<Field>> {
        if !self.static_fields_initialized.load(Ordering::SeqCst) {
            self.static_fields_initialized.store(true, Ordering::SeqCst);
            self.do_static_fields_initialization()?;
        }

        match self.static_fields.field_by_name.get(field_name) {
            Some(field) => Ok(Arc::clone(field)),
            None => Err(Error::new_constant_pool(&format!(
                "Error getting field: {}.{field_name}",
                self.this_class_name
            ))),
        }
    }

    fn do_static_fields_initialization(&self) -> crate::error::Result<()> {
        //todo: protect me with recursive mutex

        if let Some(static_init_method) = self
            .methods
            .method_by_signature
            .get(Self::STATIC_INIT_METHOD)
        {
            println!(
                "<INVOKE> -> {}.{}",
                self.this_class_name,
                Self::STATIC_INIT_METHOD
            );
            let mut engine = Engine::new();
            engine.execute(static_init_method)?;
            println!(
                "<RETURN> -> {}.{}",
                self.this_class_name,
                Self::STATIC_INIT_METHOD
            );
        }

        Ok(())
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

    pub fn reflection_ref(&self) -> i32 {
        let class_ref = self
            .reflection_ref
            .get_or_init(|| self.create_reflection_instance());
        *class_ref
    }

    pub fn access_flags(&self) -> u16 {
        self.access_flags
    }
}

impl Methods {
    pub fn new(method_by_signature: HashMap<String, Arc<JavaMethod>>) -> Self {
        Self {
            method_by_signature,
        }
    }
}
