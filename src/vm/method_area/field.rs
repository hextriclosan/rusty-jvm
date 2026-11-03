use crate::vm::error::Result;
use crate::vm::execution_engine::executor::Executor;
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::gc::budget::{copy_slice, AllocError, AllocResult};
use crate::vm::helper::{clazz_ref, default_value};
use crate::vm::jni::java_thread::JavaThread;
use crate::vm::stack::slot::Slot;
use crate::vm::stack::stack_value::StackValueKind;
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
    pub(crate) fn storage_capacity(&self, planned: bool) -> AllocResult<usize> {
        let value = self.value.read().map_err(|_| AllocError::InvalidLayout)?;
        Ok(if planned {
            value.len()
        } else {
            value.capacity()
        })
    }

    pub(crate) fn try_clone(&self) -> AllocResult<Self> {
        let value = self.value.read().map_err(|_| AllocError::InvalidLayout)?;
        Ok(Self {
            value: RwLock::new(copy_slice(&value)?),
        })
    }

    /// Read one typed reference during a quiescent scan. Contention or malformed
    /// storage aborts the scan; it must never wait for a stopped mutator.
    pub(crate) fn reference_snapshot(&self) -> Result<i32> {
        let value = self
            .value
            .try_read()
            .map_err(|_| AllocError::InvalidLayout)?;
        match value.as_slice() {
            [reference] => Ok(*reference),
            _ => Err(AllocError::InvalidLayout.into()),
        }
    }

    /// Fixed-width collector write; the complete trace validates these slots first.
    pub(crate) fn set_reference(&self, reference: i32) -> Result<()> {
        let mut value = self
            .value
            .try_write()
            .map_err(|_| AllocError::InvalidLayout)?;
        match value.as_mut_slice() {
            [slot] => {
                *slot = reference;
                Ok(())
            }
            _ => Err(AllocError::InvalidLayout.into()),
        }
    }

    pub fn new(type_descriptor: TypeDescriptor) -> Result<Self> {
        let value = default_value(&type_descriptor)?;
        Ok(Self {
            value: RwLock::new(value),
        })
    }

    pub fn set_raw_value(&self, value: Vec<i32>) -> Result<()> {
        let mut guard = self.value.write()?;
        if guard.len() != value.len() {
            return Err(AllocError::InvalidLayout.into());
        }
        guard.copy_from_slice(&value);
        Ok(())
    }

    pub fn raw_value(&self) -> Result<Vec<i32>> {
        let guard = self.value.read()?;
        Ok(guard.clone())
    }

    /// Atomically compares the raw value against `expected` and, if equal, stores `new` — all under
    /// a single write lock, so it is a genuine compare-and-swap. Returns `(old_value, swapped)`.
    /// Using this instead of a separate [`Self::raw_value`] + [`Self::set_raw_value`] is what keeps
    /// `Unsafe.compareAndSet*` on a static field race-free (two threads cannot both observe the same
    /// old value and both store).
    pub fn compare_and_exchange(&self, expected: &[i32], new: &[i32]) -> Result<(Vec<i32>, bool)> {
        let mut guard = self.value.write()?;
        let swapped = guard.as_slice() == expected;
        let old = guard.clone();
        if swapped {
            if guard.len() != new.len() {
                return Err(AllocError::InvalidLayout.into());
            }
            guard.copy_from_slice(new);
        }
        Ok((old, swapped))
    }

    #[cfg(test)]
    pub(crate) fn corrupt_width_for_test(&self, value: Vec<i32>) {
        *self.value.write().unwrap() = value;
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
    pub(crate) fn declaring_class(&self) -> &str {
        &self.class_name
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn reflection_ref_if_present(&self) -> Option<i32> {
        self.reflection_ref.get().copied()
    }

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
        if self.reflection_ref.get().is_none() {
            let reference = self.init_reflection_ref()?;
            let _ = self.reflection_ref.set(reference);
        }
        Ok(*self
            .reflection_ref
            .get()
            .expect("published reflection field"))
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
        let mut roots = JavaThread::hold_temp_roots(vec![]);
        let declaring_class_ref = clazz_ref(&self.class_name)?;
        roots.add_root(Slot::Ref(declaring_class_ref));

        let name_ref = StringPoolHelper::get_string(&self.name)?;
        roots.add_root(Slot::Ref(name_ref));
        let descr = self.type_descriptor.to_string();
        let type_ref = clazz_ref(&descr)?;
        roots.add_root(Slot::Ref(type_ref));

        let modifiers = self.flags as i32;
        let trusted_final = 0; // we don't support this yet
        let slot = 0; // we don't support this yet
        let signature_ref = 0; // todo https://github.com/hextriclosan/rusty-jvm/issues/386
        let annotations = 0; // todo https://github.com/hextriclosan/rusty-jvm/issues/386

        let args = &[
            StackValueKind::Ref(declaring_class_ref),
            StackValueKind::Ref(name_ref),
            StackValueKind::Ref(type_ref),
            modifiers.into(),
            trusted_final.into(),
            slot.into(),
            StackValueKind::Ref(signature_ref),
            StackValueKind::Ref(annotations),
        ];

        let method_ref = Executor::invoke_args_constructor(
            "java/lang/reflect/Field",
            "<init>:(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/Class;IZILjava/lang/String;[B)V",
            args,
            None
        )?;

        Ok(method_ref[0].value())
    }
}

#[cfg(test)]
mod gc_tests {
    use super::*;

    #[test]
    fn gc_roots_field_reflection_cache_is_non_initializing() {
        let field = FieldInfo::new(
            TypeDescriptor::Integer,
            0,
            "Unloaded".into(),
            "field".into(),
        );
        assert_eq!(field.reflection_ref_if_present(), None);
        field.reflection_ref.set(71).unwrap();
        assert_eq!(field.reflection_ref_if_present(), Some(71));
    }

    #[test]
    fn gc_roots_reference_field_snapshot_rejects_lock_contention() {
        let value = FieldValue::new(TypeDescriptor::Object("Test".into())).unwrap();
        let guard = value.value.write().unwrap();
        assert!(value.reference_snapshot().is_err());
        drop(guard);
        assert_eq!(value.reference_snapshot().unwrap(), 0);
    }
}
