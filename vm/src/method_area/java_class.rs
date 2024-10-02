use crate::error::Error;
use crate::execution_engine::engine::Engine;
use crate::heap::heap::Heap;
use crate::heap::java_instance::FieldNameType;
use crate::method_area::cpool_helper::CPoolHelper;
use crate::method_area::field::Field;
use crate::method_area::java_method::JavaMethod;
use crate::method_area::method_area::MethodArea;
use jdescriptor::TypeDescriptor;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug)]
pub(crate) struct JavaClass {
    pub(crate) methods: Methods,
    static_fields: Fields,
    non_static_field_descriptors: FieldDescriptors,
    cpool_helper: CPoolHelper,
    this_class_name: String,
    parent: Option<String>,
    _interfaces: Vec<String>,

    static_fields_initialized: AtomicBool,
    heap: Rc<RefCell<Heap>>,
    method_area: Rc<RefCell<MethodArea>>,
}

#[derive(Debug)]
pub(crate) struct Methods {
    pub(crate) method_by_signature: HashMap<String, Rc<JavaMethod>>,
}

#[derive(Debug)]
pub(crate) struct Fields {
    pub(crate) field_by_name: HashMap<String, Rc<RefCell<Field>>>,
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
    pub fn new(field_by_name: HashMap<String, Rc<RefCell<Field>>>) -> Self {
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
        interfaces: Vec<String>,
        heap: Rc<RefCell<Heap>>,
        method_area: Rc<RefCell<MethodArea>>,
    ) -> Self {
        Self {
            methods,
            static_fields,
            non_static_field_descriptors,
            cpool_helper,
            this_class_name,
            parent,
            _interfaces: interfaces,
            static_fields_initialized: AtomicBool::new(false),
            heap,
            method_area,
        }
    }

    pub fn cpool_helper(&self) -> &CPoolHelper {
        &self.cpool_helper
    }

    pub fn parent(&self) -> &Option<String> {
        &self.parent
    }

    pub fn _interfaces(&self) -> &Vec<String> {
        &self._interfaces
    }

    pub fn static_field(&self, field_name: &str) -> crate::error::Result<Rc<RefCell<Field>>> {
        if !self.static_fields_initialized.load(Ordering::SeqCst) {
            self.static_fields_initialized.store(true, Ordering::SeqCst);
            self.do_static_fields_initialization()?;
        }

        match self.static_fields.field_by_name.get(field_name) {
            Some(field) => Ok(Rc::clone(field)),
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
            let mut engine = Engine::new(Rc::clone(&self.method_area), Rc::clone(&self.heap));
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
}

impl Methods {
    pub fn new(method_by_signature: HashMap<String, Rc<JavaMethod>>) -> Self {
        Self {
            method_by_signature,
        }
    }
}
