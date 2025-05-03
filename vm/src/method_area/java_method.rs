use crate::error::Error;
use crate::execution_engine::executor::Executor;
use crate::execution_engine::string_pool_helper::StringPoolHelper;
use crate::heap::heap::with_heap_write_lock;
use crate::helper::clazz_ref;
use crate::method_area::cpool_helper::CPoolHelperTrait;
use crate::method_area::method_area::with_method_area;
use crate::stack::stack_frame::{ExceptionTable, StackFrame};
use derive_new::new;
use getset::{CopyGetters, Getters};
use jdescriptor::MethodDescriptor;
use once_cell::sync::OnceCell;
use std::collections::{BTreeMap, HashSet};
use std::sync::Arc;

#[derive(Debug)]
pub(crate) struct JavaMethod {
    method_descriptor: MethodDescriptor,
    class_name: String,
    name_signature: String,
    code_context: Option<CodeContext>,
    native: bool,

    reflection_ref: OnceCell<i32>,
    exception_indexes: Vec<u16>,
    access_flags: i32,
    name: String,
    _annotation_default_raw: Option<Vec<u8>>,
    annotations_raw: Option<Vec<u8>>,
    runtime_visible_annotations: HashSet<String>,
}

#[derive(Debug, new, Getters, CopyGetters)]
pub(crate) struct CodeContext {
    #[get_copy = "pub"]
    max_stack: u16,
    #[get_copy = "pub"]
    max_locals: u16,
    #[get = "pub"]
    bytecode: Arc<Vec<u8>>,
    #[get = "pub"]
    line_numbers: Arc<BTreeMap<u16, u16>>,
    #[get = "pub"]
    exception_table: Arc<ExceptionTable>,
}

impl JavaMethod {
    pub fn new(
        method_descriptor: MethodDescriptor,
        class_name: &str,
        name_signature: &str,
        code_context: Option<CodeContext>,
        native: bool,
        exception_indexes: Vec<u16>,
        access_flags: i32,
        name: &str,
        _annotation_default_raw: Option<Vec<u8>>,
        annotations_raw: Option<Vec<u8>>,
        runtime_visible_annotations: HashSet<String>,
    ) -> Self {
        Self {
            method_descriptor,
            class_name: class_name.to_string(),
            name_signature: name_signature.to_string(),
            code_context,
            native,
            reflection_ref: OnceCell::new(),
            exception_indexes,
            access_flags,
            name: name.to_string(),
            _annotation_default_raw,
            annotations_raw,
            runtime_visible_annotations,
        }
    }

    pub fn new_stack_frame(&self) -> crate::error::Result<StackFrame> {
        match &self.code_context {
            Some(context) => Ok(StackFrame::new(
                context.max_locals() as usize,
                context.max_stack() as usize,
                Arc::clone(context.bytecode()),
                self.class_name.clone(),
                Arc::clone(context.line_numbers()),
                Arc::clone(context.exception_table()),
            )),
            None => Err(Error::new_execution(&format!(
                "Code context is missing for {}.{}",
                self.class_name, self.name_signature
            ))),
        }
    }

    pub fn get_method_descriptor(&self) -> &MethodDescriptor {
        &self.method_descriptor
    }

    pub fn is_native(&self) -> bool {
        self.native
    }

    pub fn class_name(&self) -> &str {
        &self.class_name
    }

    pub fn reflection_ref(&self) -> crate::error::Result<i32> {
        self.reflection_ref
            .get_or_try_init(|| self.init_reflection_ref())
            .copied()
    }

    fn init_reflection_ref(&self) -> crate::error::Result<i32> {
        if self.name == "<init>" {
            self.construct_constructor()
        } else {
            self.construct_method()
        }
    }

    /// Invokes
    /// ```java
    /// Method(Class<?> declaringClass,
    ///     String name,
    ///     Class<?>[] parameterTypes,
    ///     Class<?> returnType,
    ///     Class<?>[] exceptionTypes,
    ///     int modifiers,
    ///     int slot,
    ///     String signature,
    ///     byte[] annotations,
    ///     byte[] parameterAnnotations,
    ///     byte[] annotationDefault);
    /// ```
    fn construct_method(&self) -> crate::error::Result<i32> {
        let declaring_class_ref = clazz_ref(self.class_name())?;

        let mut name_signature_split = self.name_signature.split(':'); //fixme: crete separate field with name of get it from here
        let name_ref =
            StringPoolHelper::get_string(name_signature_split.next().unwrap().to_string())?;

        let parameter_type_clazz_refs = self
            .method_descriptor
            .parameter_types()
            .iter()
            .map(|t| t.to_string())
            .map(|t| clazz_ref(&t))
            .collect::<crate::error::Result<Vec<_>>>()?;
        let parameter_type_refs = with_heap_write_lock(|heap| {
            heap.create_array_with_values("[Ljava/lang/Class;", &parameter_type_clazz_refs)
        });

        let return_type = self.method_descriptor.return_type().to_string();
        let return_type_ref = clazz_ref(&return_type)?;

        let jc = with_method_area(|area| area.get(&self.class_name))?;
        let cpool_helper = jc.cpool_helper();
        let exception_type_clazz_refs = self
            .exception_indexes
            .iter()
            .map(|i| {
                let exception_type = cpool_helper.get_class_name(*i).ok_or_else(|| {
                    Error::new_execution(&format!("Invalid exception index: {}", i))
                })?;
                clazz_ref(&exception_type)
            })
            .collect::<crate::error::Result<Vec<_>>>()?;
        let exception_type_refs = with_heap_write_lock(|heap| {
            heap.create_array_with_values("[Ljava/lang/Class;", &exception_type_clazz_refs)
        });

        let modifiers = self.access_flags;

        let slot = 0; // not used

        let signature_ref =
            StringPoolHelper::get_string(name_signature_split.next().unwrap().to_string())?;

        let annotations = self
            .annotations_raw
            .clone()
            .map(|annotations_raw| {
                let vec = annotations_raw
                    .iter()
                    .map(|x| *x as i32)
                    .collect::<Vec<_>>();
                let annotations =
                    with_heap_write_lock(|heap| heap.create_array_with_values("[B", &vec));
                annotations
            })
            .unwrap_or(0);

        let parameter_annotations = 0; // todo: implement me
        let annotation_default = 0; // todo: implement me

        let args = &[
            declaring_class_ref.into(),
            name_ref.into(),
            parameter_type_refs.into(),
            return_type_ref.into(),
            exception_type_refs.into(),
            modifiers.into(),
            slot.into(),
            signature_ref.into(),
            annotations.into(),
            parameter_annotations.into(),
            annotation_default.into(),
        ];

        let method_ref = Executor::invoke_args_constructor(
            "java/lang/reflect/Method",
            "<init>:(Ljava/lang/Class;Ljava/lang/String;[Ljava/lang/Class;Ljava/lang/Class;[Ljava/lang/Class;IILjava/lang/String;[B[B[B)V",
            args,
            None
        )?;

        Ok(method_ref)
    }

    /// Invokes
    /// ```java
    /// Constructor(Class<T> declaringClass,
    ///     Class<?>[] parameterTypes,
    ///     Class<?>[] checkedExceptions,
    ///     int modifiers,
    ///     int slot,
    ///     String signature,
    ///     byte[] annotations,
    ///     byte[] parameterAnnotations);
    /// ```
    fn construct_constructor(&self) -> crate::error::Result<i32> {
        let declaring_class_ref = clazz_ref(self.class_name())?;

        let mut name_signature_split = self.name_signature.split(':'); //fixme: crete separate field with name of get it from here

        let parameter_type_clazz_refs = self
            .method_descriptor
            .parameter_types()
            .iter()
            .map(|t| t.to_string())
            .map(|t| clazz_ref(&t))
            .collect::<crate::error::Result<Vec<_>>>()?;
        let parameter_type_refs = with_heap_write_lock(|heap| {
            heap.create_array_with_values("[Ljava/lang/Class;", &parameter_type_clazz_refs)
        });

        let jc = with_method_area(|area| area.get(&self.class_name))?;
        let cpool_helper = jc.cpool_helper();
        let exception_type_clazz_refs = self
            .exception_indexes
            .iter()
            .map(|i| {
                let exception_type = cpool_helper.get_class_name(*i).ok_or_else(|| {
                    Error::new_execution(&format!("Invalid exception index: {}", i))
                })?;
                clazz_ref(&exception_type)
            })
            .collect::<crate::error::Result<Vec<_>>>()?;
        let checked_exception_type_refs = with_heap_write_lock(|heap| {
            heap.create_array_with_values("[Ljava/lang/Class;", &exception_type_clazz_refs)
        });

        let modifiers = self.access_flags;

        let slot = 0; // not used

        let signature_ref =
            StringPoolHelper::get_string(name_signature_split.next().unwrap().to_string())?;

        let annotations = self
            .annotations_raw
            .clone()
            .map(|annotations_raw| {
                let vec = annotations_raw
                    .iter()
                    .map(|x| *x as i32)
                    .collect::<Vec<_>>();
                let annotations =
                    with_heap_write_lock(|heap| heap.create_array_with_values("[B", &vec));
                annotations
            })
            .unwrap_or(0);

        let parameter_annotations = 0; // todo: implement me

        let args = &[
            declaring_class_ref.into(),
            parameter_type_refs.into(),
            checked_exception_type_refs.into(),
            modifiers.into(),
            slot.into(),
            signature_ref.into(),
            annotations.into(),
            parameter_annotations.into(),
        ];

        let method_ref = Executor::invoke_args_constructor(
            "java/lang/reflect/Constructor",
            "<init>:(Ljava/lang/Class;[Ljava/lang/Class;[Ljava/lang/Class;IILjava/lang/String;[B[B)V",
            args,
            None
        )?;

        Ok(method_ref)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn access_flags(&self) -> i32 {
        self.access_flags
    }

    pub fn name_signature(&self) -> &str {
        &self.name_signature
    }

    pub fn runtime_visible_annotations(&self) -> &HashSet<String> {
        &self.runtime_visible_annotations
    }
}
