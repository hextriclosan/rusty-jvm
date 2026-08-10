use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::invoke_dynamic_runner::InvokeDynamicRunner;
use crate::vm::heap::java_instance::{ClassName, FieldNameType};
use crate::vm::method_area::field::{FieldInfo, FieldValue};
use crate::vm::method_area::java_method::{CodeContext, JavaMethod};
use crate::vm::method_area::lookup;
use crate::vm::stack::stack_frame::ExceptionTable;
use getset::{CopyGetters, Getters};
use indexmap::{IndexMap, IndexSet};
use jclassmodel::attributes::Attributes;
use jclassmodel::constant_pool::ConstantPool;
use jclassmodel::modifiers::{ClassModifier, NestedClassModifier};
use jclassmodel::{ClassModel, EnclosingMethodInfo, FieldModel, MethodModel};
use jdescriptor::TypeDescriptor;
use once_cell::sync::OnceCell;
use parking_lot::ReentrantMutex;
use std::cell::RefCell;
use std::sync::{Arc, OnceLock};

pub const STATIC_FIELDS_START: i64 = i32::MAX as i64;

/// Instance-field offsets are the field's slot index scaled to a 4-byte-aligned byte offset
/// (`index * 4`). Each field therefore owns its own aligned word, so the JDK's sub-word Unsafe ops
/// (`compareAndSetBoolean`/`Byte`/`Short`/`Char`, which word-align the offset via `offset & ~3` and
/// mask in a byte) operate on the intended field instead of aliasing a neighbour. Must divide the
/// offset back out in [`JavaClass::get_field_name_by_offset`].
const FIELD_OFFSET_SCALE: i64 = 4;

type FullyQualifiedFieldName = String; // format: com/example/models/Person.name

/// A class's fields, split the three ways [`JavaClass`] stores them: metadata for every field,
/// values for the static ones, and the template instances are created from.
struct BuiltFields {
    info: IndexMap<String, Arc<FieldInfo>>,
    static_values: IndexMap<String, Arc<FieldValue>>,
    instance_template: IndexMap<String, FieldValue>,
}

#[derive(Debug, Getters, CopyGetters)]
pub(crate) struct JavaClass {
    methods: IndexMap<String, Arc<JavaMethod>>,
    fields_info: IndexMap<String, Arc<FieldInfo>>,
    static_fields: IndexMap<String, Arc<FieldValue>>,
    instance_fields_template: IndexMap<String, FieldValue>,
    #[get = "pub"]
    constant_pool: ConstantPool,
    #[get = "pub"]
    attributes: Attributes,
    #[get = "pub"]
    this_class_name: String,
    #[get = "pub"]
    external_name: String,
    #[get = "pub"]
    parent: Option<String>,
    #[get = "pub"]
    interfaces: IndexSet<String>,
    #[get_copy = "pub"]
    class_modifiers: ClassModifier,
    /// Modifiers from this class's own `InnerClasses` record, present only when it is nested.
    /// The only place `private`, `protected` and `static` are recorded for a nested class; the
    /// `ClassFile`'s own `access_flags` structurally cannot express them.
    #[get_copy = "pub"]
    nested_class_modifiers: Option<NestedClassModifier>,

    #[get = "pub"]
    static_fields_init_state: Arc<ReentrantMutex<InitState>>,

    instance_fields_hierarchy: OnceCell<IndexMap<ClassName, IndexMap<FieldNameType, FieldValue>>>,
    fields_offset_mapping: OnceCell<IndexSet<FullyQualifiedFieldName>>,
    vtable: OnceCell<IndexMap<String, Arc<JavaMethod>>>,
    #[get = "pub"]
    declaring_class: Option<String>,
    #[get = "pub"]
    annotations_raw: Option<Vec<u8>>,
    #[get = "pub"]
    enclosing_method: Option<EnclosingMethodInfo>,
    #[get = "pub"]
    source_file: Option<String>,
    #[get = "pub"]
    invoke_dynamic_runner: InvokeDynamicRunner,
    mirror_clazz_ref: OnceLock<i32>,
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

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum InnerState {
    #[default]
    NotInitialized,
    Initializing,
    Initialized,
}

impl JavaClass {
    /// Builds a class from its resolved class-file view, adding the runtime state (field values,
    /// initialization state, vtable and offset caches) that the class file itself cannot describe.
    pub fn from_model(model: ClassModel) -> Result<Self> {
        let ClassModel {
            // The class file version is not used by the runtime yet; bound explicitly rather than
            // with `..` so a new model field still breaks here.
            version: _,
            nested_class_modifiers,
            name,
            external_name,
            super_class_name,
            interfaces,
            modifiers,
            methods,
            fields,
            declaring_class,
            annotations_raw,
            enclosing_method,
            source_file,
            constant_pool,
            attributes,
        } = model;

        let built_fields = Self::build_fields(fields, &name)?;

        Ok(Self::assemble(
            Self::build_methods(methods, &name),
            built_fields.info,
            built_fields.static_values,
            built_fields.instance_template,
            constant_pool,
            attributes,
            name,
            external_name,
            super_class_name,
            interfaces.into_iter().collect(),
            modifiers,
            nested_class_modifiers,
            declaring_class,
            annotations_raw,
            enclosing_method,
            source_file,
        ))
    }

    /// Builds a class the VM invents rather than loads: primitives (`int`, `void`, …) and array
    /// classes. They have no class file, so they carry no methods, fields, constant pool or
    /// attributes.
    pub fn synthetic(
        this_class_name: String,
        external_name: String,
        parent: Option<String>,
        interfaces: IndexSet<String>,
    ) -> Self {
        Self::assemble(
            IndexMap::new(),
            IndexMap::new(),
            IndexMap::new(),
            IndexMap::new(),
            ConstantPool::empty(),
            Attributes::empty(),
            this_class_name,
            external_name,
            parent,
            interfaces,
            ClassModifier::Public | ClassModifier::Final | ClassModifier::Abstract,
            None,
            None,
            None,
            None,
            None,
        )
    }

    fn build_methods(
        methods: Vec<MethodModel>,
        class_name: &str,
    ) -> IndexMap<String, Arc<JavaMethod>> {
        let mut method_by_signature = IndexMap::new();

        for method in methods {
            // Polymorphic-signature natives have no fixed descriptor, so they are keyed by bare name.
            let key = if method.is_polymorphic_signature() {
                method.name.clone()
            } else {
                method.name_signature.clone()
            };

            // Read before `code` is moved out of `method` below.
            let is_native = method.is_native();

            let code_context = method.code.map(|code| {
                CodeContext::new(
                    code.max_stack,
                    code.max_locals,
                    Arc::new(code.bytecode),
                    Arc::new(code.line_numbers),
                    Arc::new(ExceptionTable::new(code.exception_table)),
                    Arc::new(code.local_variable_table),
                )
            });

            method_by_signature.insert(
                key,
                Arc::new(JavaMethod::new(
                    method.descriptor,
                    class_name,
                    &method.name_signature,
                    code_context,
                    is_native,
                    method.exceptions,
                    method.modifiers.bits() as i32,
                    &method.name,
                    method.annotation_default_raw,
                    method.annotations_raw,
                    method.runtime_visible_annotations,
                    method_by_signature.len() as i32, // God, forgive me, this is a hack to get the method index
                )),
            );
        }

        method_by_signature
    }

    fn build_fields(fields: Vec<FieldModel>, class_name: &str) -> Result<BuiltFields> {
        let mut info = IndexMap::new();
        let mut instance_template = IndexMap::new();
        let mut static_values = IndexMap::new();
        for field in fields {
            let is_static = field.is_static();
            info.insert(
                field.name.clone(),
                Arc::new(FieldInfo::new(
                    field.descriptor.clone(),
                    field.modifiers.bits(),
                    class_name.to_string(),
                    field.name.clone(),
                )),
            );
            if is_static {
                static_values.insert(field.name, Arc::new(FieldValue::new(field.descriptor)?));
            } else {
                instance_template.insert(field.name, FieldValue::new(field.descriptor)?);
            }
        }

        Ok(BuiltFields {
            info,
            static_values,
            instance_template,
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn assemble(
        methods: IndexMap<String, Arc<JavaMethod>>,
        fields_info: IndexMap<String, Arc<FieldInfo>>,
        static_fields: IndexMap<String, Arc<FieldValue>>,
        instance_fields_template: IndexMap<String, FieldValue>,
        constant_pool: ConstantPool,
        attributes: Attributes,
        this_class_name: String,
        external_name: String,
        parent: Option<String>,
        interfaces: IndexSet<String>,
        class_modifiers: ClassModifier,
        nested_class_modifiers: Option<NestedClassModifier>,
        declaring_class: Option<String>,
        annotations_raw: Option<Vec<u8>>,
        enclosing_method: Option<EnclosingMethodInfo>,
        source_file: Option<String>,
    ) -> Self {
        Self {
            methods,
            fields_info,
            static_fields,
            instance_fields_template,
            constant_pool,
            attributes,
            this_class_name,
            external_name,
            parent,
            interfaces,
            class_modifiers,
            nested_class_modifiers,
            static_fields_init_state: Arc::default(),
            instance_fields_hierarchy: OnceCell::new(),
            fields_offset_mapping: OnceCell::new(),
            vtable: OnceCell::new(),
            declaring_class,
            annotations_raw,
            enclosing_method,
            source_file,
            invoke_dynamic_runner: InvokeDynamicRunner::default(),
            mirror_clazz_ref: OnceLock::default(),
        }
    }

    pub fn is_interface(&self) -> bool {
        self.class_modifiers.contains(ClassModifier::Interface)
    }

    /// Modifiers as `Class.getModifiers()` reports them, which are `java.lang.reflect.Modifier`
    /// bits rather than the class file's `access_flags`.
    ///
    /// A nested class records its real `private`, `protected` and `static` only in the
    /// `InnerClasses` entry describing it, so those flags *replace* the class file's own when
    /// present, matching HotSpot's `InstanceKlass::compute_modifier_flags`. ACC_SUPER and
    /// ACC_MODULE are then stripped, since neither is a `Modifier` bit.
    pub fn reflection_modifiers(&self) -> u16 {
        /// `JVM_ACC_WRITTEN_FLAGS`: everything below ACC_MODULE (0x8000).
        const WRITTEN_FLAGS: u16 = 0x7FFF;

        let bits = self
            .nested_class_modifiers
            .map(|modifiers| modifiers.bits())
            .unwrap_or_else(|| self.class_modifiers.bits());

        bits & !ClassModifier::Super.bits() & WRITTEN_FLAGS
    }

    pub fn static_field(&self, field_name: &str) -> Option<Arc<FieldValue>> {
        self.static_fields.get(field_name).map(Arc::clone)
    }

    pub fn field_info(&self, field_name: &str) -> Option<&Arc<FieldInfo>> {
        self.fields_info.get(field_name)
    }

    pub fn instance_field_descriptor(
        &self,
        instance_field_name_type: &str,
    ) -> Option<&TypeDescriptor> {
        let field_info = self.fields_info.get(instance_field_name_type)?;

        Some(field_info.type_descriptor())
    }

    pub fn put_instance_field_descriptor(
        &mut self,
        name: String,
        type_descriptor: TypeDescriptor,
        flags: u16,
        class_name: &str,
    ) -> Result<Option<Arc<FieldInfo>>> {
        self.instance_fields_template
            .insert(name.clone(), FieldValue::new(type_descriptor.clone())?);

        Ok(self.fields_info.insert(
            name.clone(),
            Arc::new(FieldInfo::new(
                type_descriptor,
                flags,
                class_name.to_string(),
                name,
            )),
        ))
    }

    pub fn default_value_instance_fields(&self) -> IndexMap<FieldNameType, FieldValue> {
        self.instance_fields_template.clone()
    }

    pub fn get_field_offset(&self, fully_qualified_field_name: &str) -> Result<i64> {
        let offset = self
            .fields_offset_mapping()?
            .get_index_of(fully_qualified_field_name)
            .ok_or_else(|| {
                Error::new_execution(&format!(
                    "Failed to get offset by name {fully_qualified_field_name}"
                ))
            })?;
        Ok(offset as i64 * FIELD_OFFSET_SCALE)
    }

    pub fn get_static_field_offset(&self, field_name: &str) -> Result<i64> {
        let offset = self.static_fields.get_index_of(field_name).ok_or_else(|| {
            Error::new_execution(&format!(
                "Failed to get static field offset by name {field_name}"
            ))
        })?;
        let real_offset = STATIC_FIELDS_START + offset as i64;
        Ok(real_offset)
    }

    pub fn get_static_field_by_offset(&self, offset: i64) -> Result<Arc<FieldValue>> {
        let real_offset = offset - STATIC_FIELDS_START; // adjust offset to match the index in static_fields
        let (_field_name, field_value) = self
            .static_fields
            .get_index(real_offset as usize)
            .ok_or_else(|| {
                Error::new_execution(&format!("Failed to get static field by offset {offset}"))
            })?;
        Ok(Arc::clone(field_value))
    }

    /// The declared type of the static field at `offset`.
    ///
    /// [`Self::get_static_field_by_offset`] yields only the value, which is raw `i32` chunks with
    /// nothing to say whether they are a reference. Callers that put such a value on an operand
    /// stack need this to tag it.
    pub fn get_static_field_descriptor_by_offset(&self, offset: i64) -> Result<&TypeDescriptor> {
        let real_offset = offset - STATIC_FIELDS_START;
        let (field_name, _) = self
            .static_fields
            .get_index(real_offset as usize)
            .ok_or_else(|| {
                Error::new_execution(&format!("Failed to get static field by offset {offset}"))
            })?;

        self.fields_info
            .get(field_name)
            .map(|field_info| field_info.type_descriptor())
            .ok_or_else(|| {
                Error::new_execution(&format!(
                    "Failed to get field info for static field {}.{field_name}",
                    self.this_class_name
                ))
            })
    }

    pub fn get_field_name_by_offset(&self, offset: i64) -> Result<(String, String)> {
        // `offset` is a 4-byte-scaled slot index (see [`FIELD_OFFSET_SCALE`]); a sub-word Unsafe op
        // may pass a word-aligned offset (`offset & ~3`), which still divides back to the right slot.
        let result = self
            .fields_offset_mapping()?
            .get_index((offset / FIELD_OFFSET_SCALE) as usize)
            .ok_or_else(|| {
                Error::new_execution(&format!(
                    "Failed to get field name by offset {offset} from fields_offset_mapping"
                ))
            })?;

        let mut parts = result.split('.');
        let class_name = parts.next().ok_or_else(|| {
            Error::new_execution(&format!(
                "Failed to get class name by offset {offset} after splitting"
            ))
        })?;
        let field_name = parts.next().ok_or_else(|| {
            Error::new_execution(&format!(
                "Failed to get field name by offset {offset} after splitting"
            ))
        })?;

        Ok((class_name.to_string(), field_name.to_string()))
    }

    pub fn get_method_full(&self, full_signature: &str) -> Option<(usize, Arc<JavaMethod>)> {
        self.methods
            .get_full(full_signature)
            .map(|(index, _key, method)| (index, Arc::clone(method)))
            .or_else(|| {
                // we have not found the method by full signature, let's treat it as polymorphic signature method and try to find it by method name only
                self.methods
                    .get_full(full_signature.split(':').next()?)
                    .map(|(index, _key, method)| (index, Arc::clone(method)))
            })
    }

    pub fn try_get_method(&self, full_signature: &str) -> Option<Arc<JavaMethod>> {
        self.get_method_full(full_signature)
            .map(|(_index, method)| Arc::clone(&method))
    }

    pub fn get_method(&self, full_signature: &str) -> Result<Arc<JavaMethod>> {
        self.get_method_full(full_signature)
            .map(|(_index, method)| Arc::clone(&method))
            .ok_or_else(|| {
                Error::new_execution(&format!(
                    "Method {full_signature} not found in {}",
                    self.this_class_name
                ))
            })
    }

    pub fn get_method_by_index(&self, method_index: i64) -> Result<Arc<JavaMethod>> {
        self.methods
            .get_index(method_index as usize)
            .map(|(_key, method)| Arc::clone(method))
            .ok_or_else(|| {
                Error::new_execution(&format!("Failed to get method by index {method_index}"))
            })
    }

    pub fn get_fields_info(&self) -> Vec<Arc<FieldInfo>> {
        self.fields_info
            .values()
            .map(Arc::clone)
            .collect::<Vec<_>>()
    }

    pub fn get_methods(&self) -> impl Iterator<Item = (&String, &Arc<JavaMethod>)> {
        self.methods.iter()
    }

    pub fn instance_fields_hierarchy(
        &self,
    ) -> Result<&IndexMap<ClassName, IndexMap<FieldNameType, FieldValue>>> {
        self.instance_fields_hierarchy.get_or_try_init(|| {
            let mut instance_fields_hierarchy = IndexMap::new();
            lookup::lookup_and_fill_instance_fields_hierarchy(
                &self.this_class_name,
                &mut instance_fields_hierarchy,
            )?;
            Ok(instance_fields_hierarchy)
        })
    }

    fn fields_offset_mapping(&self) -> Result<&IndexSet<FullyQualifiedFieldName>> {
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

    pub fn vtable(&self) -> Result<&IndexMap<String, Arc<JavaMethod>>> {
        self.vtable
            .get_or_try_init(|| lookup::build_vtable(self.this_class_name()))
    }

    pub fn inject_mirror_clazz_ref(&self, mirror_class_ref: i32) -> Result<()> {
        self.mirror_clazz_ref.set(mirror_class_ref).map_err(|e| {
            Error::new_execution(&format!(
                "{}: attempt to re-initialize mirror_clazz_ref {} -> {}",
                self.this_class_name, e, mirror_class_ref
            ))
        })
    }

    pub fn mirror_clazz_ref(&self) -> Result<i32> {
        self.mirror_clazz_ref.get().copied().ok_or_else(|| {
            Error::new_execution(&format!(
                "{}: mirror_clazz_ref is not initialized",
                self.this_class_name
            ))
        })
    }
}
