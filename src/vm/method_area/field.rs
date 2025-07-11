use crate::vm::error::Result;
use crate::vm::execution_engine::executor::Executor;
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::helper::{clazz_ref, default_value};
use getset::{CopyGetters, Getters};
use jdescriptor::TypeDescriptor;
use once_cell::sync::OnceCell;
use serde::Serialize;
use std::sync::RwLock;

#[derive(Debug, Getters, CopyGetters)]
pub(crate) struct FieldInfo {
    #[get = "pub"]
    type_descriptor: TypeDescriptor,
    #[get_copy = "pub"]
    flags: u16,

    reflection_ref: OnceCell<i32>,
    class_name: String,
    name: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct FieldValue {
    value: RwLock<Vec<i32>>,
}

impl FieldValue {
    pub fn new(type_descriptor: TypeDescriptor) -> Result<Self> {
        let value = default_value(&type_descriptor)?;
        Ok(Self {
            value: RwLock::new(value),
        })
    }

    pub fn set_raw_value(&self, value: Vec<i32>) -> Result<()> {
        let mut guard = self.value.write()?;
        *guard = value;
        Ok(())
    }

    pub fn raw_value(&self) -> Result<Vec<i32>> {
        let guard = self.value.read()?;
        Ok(guard.clone())
    }
}

impl Clone for FieldValue {
    fn clone(&self) -> Self {
        Self {
            value: RwLock::new(self.value.read().unwrap().clone()),
        }
    }
}

impl FieldInfo {
    pub fn new(
        type_descriptor: TypeDescriptor,
        flags: u16,
        class_name: String,
        name: String,
    ) -> Self {
        Self {
            type_descriptor,
            flags,
            reflection_ref: OnceCell::new(),
            class_name,
            name,
        }
    }

    pub fn reflection_ref(&self) -> Result<i32> {
        self.reflection_ref
            .get_or_try_init(|| self.init_reflection_ref())
            .copied()
    }

    fn init_reflection_ref(&self) -> Result<i32> {
        self.construct_field()
    }

    /// Invokes
    /// ```java
    /// Field(Class<?> declaringClass,
    ///     String name,
    ///     Class<?> type,
    ///     int modifiers,
    ///     boolean trustedFinal,
    ///     int slot,
    ///     String signature,
    ///     byte[] annotations);
    /// ```
    fn construct_field(&self) -> Result<i32> {
        let declaring_class_ref = clazz_ref(&self.class_name)?;

        let name_ref = StringPoolHelper::get_string(self.name.clone())?;
        let descr = self.type_descriptor.to_string();
        let type_ref = clazz_ref(&descr)?;

        let modifiers = self.flags as i32;
        let trusted_final = 0; // we don't support final fields yet
        let slot = 0; // we don't support slots yet
        let signature_ref = 0; // todo https://github.com/hextriclosan/rusty-jvm/issues/386
        let annotations = 0; // todo https://github.com/hextriclosan/rusty-jvm/issues/386

        let args = &[
            declaring_class_ref.into(),
            name_ref.into(),
            type_ref.into(),
            modifiers.into(),
            trusted_final.into(),
            slot.into(),
            signature_ref.into(),
            annotations.into(),
        ];

        let method_ref = Executor::invoke_args_constructor(
            "java/lang/reflect/Field",
            "<init>:(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/Class;IZILjava/lang/String;[B)V",
            args,
            None
        )?;

        Ok(method_ref)
    }
}
