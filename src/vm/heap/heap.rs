use crate::bail_thrown;
use crate::vm::commons::auto_dash_map::auto_dash_map::AutoDashMap;
use crate::vm::commons::auto_dash_map::auto_dash_map_i32::AutoDashMapI32;
use crate::vm::error::{Error, Result};
use crate::vm::exception::helpers::throw_null_pointer_exception;
use crate::vm::exception::pending::Throws;
use crate::vm::gc::budget::copy_string;
use crate::vm::gc::layout::References;
use crate::vm::heap::allocation::Admission;
use crate::vm::heap::charge;
use crate::vm::heap::construction::{ArrayLayout, InstancePlan};
use crate::vm::heap::java_instance::HeapValue::{Arr, Object};
use crate::vm::heap::java_instance::JavaInstance::Class;
use crate::vm::heap::java_instance::{Array, HeapValue};
use crate::vm::jni::java_thread::JavaThread;
use crate::vm::stack::rooted_slots::RootedSlots;
use crate::vm::stack::slot::Slot;
use crate::vm::stack::stack_frame::StackFrames;
use dashmap::mapref::one::{MappedRef, MappedRefMut, Ref, RefMut};
use dashmap::DashMap;
use parking_lot::Mutex;
use serde::Serialize;
use std::fs::File;
use std::ops::{Deref, DerefMut};
use std::sync::LazyLock;
use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) static HEAP: LazyLock<Heap> = LazyLock::new(Heap::default);

/// DashMap's mapped target must be Sized. Keep the Vec guard private and expose
/// only a slice, so a caller cannot resize charged storage.
pub(crate) struct ArrayDataMut<'a>(MappedRefMut<'a, i32, HeapValue, Vec<u8>>);

impl Deref for ArrayDataMut<'_> {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl DerefMut for ArrayDataMut<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut_slice()
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct Heap {
    pub(super) data: AutoDashMapI32<HeapValue>,
    pub(super) ref_by_stringvalue: DashMap<String, i32>,
    #[serde(skip)]
    pub(super) admission: Mutex<Admission>,
}

impl Default for Heap {
    fn default() -> Self {
        Self {
            data: AutoDashMapI32::new(1), // start from 1 to avoid 0 as a valid reference
            ref_by_stringvalue: DashMap::new(),
            // Bootstrap and runtime use the same strict admission limit.
            admission: Mutex::new(Admission::new(
                crate::vm::gc::get().map_or(u64::MAX, |config| config.max_heap_bytes),
                true,
            )),
        }
    }
}

impl Heap {
    /// Read outgoing edges under one heap guard, then release it before the caller
    /// follows any edge. Only call while mutators are quiescent.
    #[allow(dead_code)] // Traversal is connected by the task-7 collector.
    pub(crate) fn references(&self, objectref: i32) -> Result<References> {
        let entry = self.data.get(objectref).ok_or_else(|| {
            Error::new_execution(&format!("Missing strong heap reference {objectref}"))
        })?;
        match entry.value() {
            Object(instance) => instance.references(),
            Arr(array) => array.references(),
        }
    }

    pub(crate) fn interned_refs(&self) -> Vec<i32> {
        self.ref_by_stringvalue
            .iter()
            .map(|entry| *entry.value())
            .collect()
    }

    pub fn create_instance(
        &self,
        plan: InstancePlan,
        destination: &mut RootedSlots,
    ) -> Result<i32> {
        destination.prepare_single_ref()?;
        let reservation = self.reserve(plan.charge()?)?;
        let instance = plan.build()?;
        Ok(reservation.commit(Object(instance), destination)?)
    }

    pub fn set_object_field_value(
        &self,
        objectref: i32,
        class_name: &str,
        field_name_type: &str,
        value: Vec<i32>,
    ) -> Result<()> {
        if objectref == 0 {
            return Err(Error::new_execution(&format!(
                "error setting field value: {class_name}.{field_name_type} to null-object"
            ))); // throw an appropriate exception here
        }

        let mut entry = self
            .data
            .get_mut(objectref)
            .ok_or_else(|| Error::new_execution("Missing object for field write"))?;
        match entry.value_mut() {
            Object(instance) => instance.set_field_value(class_name, field_name_type, value),
            Arr(_) => Err(Error::new_execution(
                "Cannot write an instance field on an array",
            )),
        }
    }

    /// Atomically compares an object field's raw value against `expected` and, if they match, stores
    /// `new`. Returns `(old_value, swapped)`. The read-compare-write executes under a single entry
    /// lock, so this is a genuine compare-and-swap with respect to other threads — unlike a separate
    /// get followed by set, which races. Both `compareAndSet*` (uses `swapped`) and
    /// `compareAndExchange*` (uses `old_value`) are built on this.
    pub fn compare_and_exchange_object_field(
        &self,
        objectref: i32,
        class_name: &str,
        field_name_type: &str,
        expected: &[i32],
        new: &[i32],
    ) -> Result<(Vec<i32>, bool)> {
        if objectref == 0 {
            return Err(Error::new_execution(&format!(
                "error CAS field value: {class_name}.{field_name_type} on null-object"
            )));
        }

        self.data
            .get_mut(objectref)
            .and_then(|mut entry| match entry.value_mut() {
                Object(instance) => {
                    let old = instance.get_field_value(class_name, field_name_type).ok()?;
                    let swapped = old.as_slice() == expected;
                    if swapped {
                        instance
                            .set_field_value(class_name, field_name_type, new.to_vec())
                            .ok()?;
                    }
                    Some((old, swapped))
                }
                _ => None,
            })
            .ok_or_else(|| {
                Error::new_execution(&format!(
                    "error CAS field value: objectref={objectref} class_name={class_name}, field_name_type={field_name_type}"
                ))
            })
    }

    /// Atomic compare-and-exchange on an array element addressed by raw byte `offset` (width `size`
    /// bytes). Returns `(old_value, swapped)`; see [`Self::compare_and_exchange_object_field`].
    pub(crate) fn compare_and_exchange_array_by_raw_offset(
        &self,
        arrayref: i32,
        offset: usize,
        size: usize,
        expected: &[i32],
        new: &[i32],
    ) -> Result<(Vec<i32>, bool)> {
        self.data
            .get_mut(arrayref)
            .and_then(|mut entry| match entry.value_mut() {
                Arr(arr) => {
                    let old = arr.get_value_by_raw_offset(offset, size).ok()?;
                    let swapped = old.as_slice() == expected;
                    if swapped {
                        arr.set_array_value_by_raw_offset(offset, new.to_vec(), size)
                            .ok()?;
                    }
                    Some((old, swapped))
                }
                _ => None,
            })
            .ok_or_else(|| Error::new_execution("error CAS array value by raw offset"))
    }

    pub fn get_object_field_value(
        &self,
        objectref: i32,
        class_name: &str,
        field_name_type: &str,
    ) -> Result<Vec<i32>> {
        if objectref == 0 {
            return Err(Error::new_execution(&format!(
                "error getting field value: {class_name} from null-object"
            ))); // throw an appropriate exception here
        }

        self.data.get(objectref)
            .and_then(|entry| {
                match entry.value() {
                    Object(java_instance) => {
                        let ret = java_instance.get_field_value(class_name, field_name_type).ok()?;
                        Some(ret)
                    },
                    _ => None,
                }
            })
            .ok_or_else(|| Error::new_execution(&format!("error getting field value: objectref={objectref} class_name={class_name}, field_name_type={field_name_type}")))
    }

    pub fn get_object_field_value_throwing(
        &self,
        objectref: i32,
        class_name: &str,
        field_name_type: &str,
        stack_frames: &mut StackFrames,
    ) -> Throws<Vec<i32>> {
        if objectref == 0 {
            bail_thrown!(throw_null_pointer_exception(stack_frames));
        }

        let result = self.get_object_field_value(objectref, class_name, field_name_type)?;
        Ok(Some(result))
    }

    pub fn get_instance_name(&self, objectref: i32) -> Result<String> {
        if let Some(entry) = self.data.get(objectref) {
            match entry.value() {
                Object(java_instance) => java_instance.instance_name(),
                Arr(array) => {
                    let name = array.type_name().to_string();
                    Ok(name)
                }
            }
        } else {
            Err(Error::new_execution(&format!(
                "error getting object from heap by ref {objectref}"
            )))
        }
    }

    pub fn get_mirror_klass_id(&self, clazz_ref: i32) -> Result<usize> {
        self.data
            .get(clazz_ref)
            .and_then(|entry| match entry.value() {
                Object(Class(java_instance_class)) => Some(java_instance_class.mirror_klass_id()),
                _ => None,
            })
            .ok_or_else(|| {
                Error::new_execution(&format!(
                    "error getting mirror klass id from heap by ref {clazz_ref}"
                ))
            })
    }

    pub(crate) fn create_array(
        &self,
        type_name: &str,
        len: i32,
        destination: &mut RootedSlots,
    ) -> Result<i32> {
        let layout = ArrayLayout::new(type_name, len)?;
        destination.prepare_single_ref()?;
        let reservation = self.reserve(layout.charge(type_name)?)?;
        let array = match Array::from_layout(type_name, layout) {
            Ok(array) => array,
            Err(error) => {
                reservation.cancel()?;
                return Err(error.into());
            }
        };
        let reference = reservation.commit(Arr(array), destination)?;
        #[cfg(test)]
        observe_array_commit(reference);
        Ok(reference)
    }

    pub(crate) fn create_array_with_values(
        &self,
        type_name: &str,
        array: &[i32],
        destination: &mut RootedSlots,
    ) -> Result<i32> {
        let layout = ArrayLayout::with_values(type_name, array.len())?;
        // The descriptor, not numeric membership in HEAP, identifies input roots.
        let _inputs = if type_name.starts_with("[L") || type_name.starts_with("[[") {
            Some(RootedSlots::from_slots(
                array.iter().copied().map(Slot::Ref).collect(),
            ))
        } else {
            None
        };
        destination.prepare_single_ref()?;
        let reservation = self.reserve(layout.charge(type_name)?)?;
        Ok(reservation.commit(Arr(Array::new_with_values(type_name, array)?), destination)?)
    }

    pub(crate) fn get_entire_array(&self, array_ref: i32) -> Result<Array> {
        if array_ref == 0 {
            return Err(Error::new_execution(
                "NullPointerException: null array reference",
            )); //todo: throw an appropriate exception here
        }
        let entry = self
            .data
            .get(array_ref)
            .ok_or_else(|| Error::new_execution("Missing array"))?;
        match entry.value() {
            Arr(array) => Ok(array.try_clone()?),
            Object(_) => Err(Error::new_execution("error getting array")),
        }
    }

    pub(crate) fn get_entire_raw_data(
        &self,
        array_ref: i32,
    ) -> Result<MappedRef<'_, i32, HeapValue, Vec<u8>>> {
        if array_ref == 0 {
            return Err(Error::new_execution(
                "NullPointerException: null array reference",
            )); //todo: throw an appropriate exception here
        }

        self.data
            .get(array_ref)
            .and_then(|entry| match entry.value() {
                Arr(_) => Some(Ref::map(entry, |value| match value {
                    Arr(arr) => arr.raw_data(),
                    _ => unreachable!("already checked that this is an array"),
                })),
                Object(_) => None,
            })
            .ok_or_else(|| Error::new_execution(&format!("instance is not an array: {array_ref}")))
    }

    pub(crate) fn get_entire_raw_data_mut(&self, array_ref: i32) -> Result<ArrayDataMut<'_>> {
        if array_ref == 0 {
            return Err(Error::new_execution(
                "NullPointerException: null array reference",
            )); //todo: throw an appropriate exception here
        }

        self.data
            .get_mut(array_ref)
            .and_then(|entry| match entry.value() {
                Arr(_) => Some(ArrayDataMut(RefMut::map(entry, |value| match value {
                    Arr(arr) => arr.raw_data_mut(),
                    _ => unreachable!("already checked that this is an array"),
                }))),
                Object(_) => None,
            })
            .ok_or_else(|| Error::new_execution(&format!("instance is not an array: {array_ref}")))
    }

    pub(crate) fn set_entire_array(&self, array_ref: i32, array_val: Array) -> Result<()> {
        let mut entry = self
            .data
            .get_mut(array_ref)
            .ok_or_else(|| Error::new_execution("Missing array"))?;
        match entry.value_mut() {
            Arr(array) => array.set_entire_value(array_val),
            Object(_) => Err(Error::new_execution("error getting array")),
        }
    }

    pub(crate) fn get_array_value(&self, arrayref: i32, index: i32) -> Result<Vec<i32>> {
        self.data
            .get(arrayref)
            .and_then(|entry| match entry.value() {
                Arr(arr) => {
                    let value = arr.get_value(index).ok()?;
                    Some(value)
                }
                _ => None,
            })
            .ok_or_else(|| Error::new_execution("error getting array value from heap"))
    }

    pub(crate) fn get_array_value_by_raw_offset(
        &self,
        arrayref: i32,
        offset: usize,
        len: usize,
    ) -> Result<Vec<i32>> {
        self.data
            .get(arrayref)
            .and_then(|entry| match entry.value() {
                Arr(arr) => {
                    let value = arr.get_value_by_raw_offset(offset, len).ok()?;
                    Some(value)
                }
                _ => None,
            })
            .ok_or_else(|| {
                Error::new_execution("error getting array value from heap by raw offset")
            })
    }

    pub(crate) fn set_array_value(
        &self,
        arrayref: i32,
        index: i32,
        value: Vec<i32>,
    ) -> Result<()> {
        self.data
            .get_mut(arrayref)
            .and_then(|mut entry| match entry.value_mut() {
                Arr(arr) => {
                    arr.set_value(index, value).ok()?;
                    Some(())
                }
                _ => None,
            })
            .ok_or_else(|| Error::new_execution("error setting array value"))
    }

    pub(crate) fn set_array_value_by_raw_offset(
        &self,
        arrayref: i32,
        offset: usize,
        value: Vec<i32>,
        size: usize,
    ) -> Result<()> {
        self.data
            .get_mut(arrayref)
            .and_then(|mut entry| match entry.value_mut() {
                Arr(arr) => {
                    arr.set_array_value_by_raw_offset(offset, value, size)
                        .ok()?;
                    Some(())
                }
                _ => None,
            })
            .ok_or_else(|| Error::new_execution("error setting array value"))
    }

    pub(crate) fn get_array_len(&self, arrayref: i32) -> Result<i32> {
        self.data
            .get(arrayref)
            .and_then(|entry| match entry.value() {
                Arr(arr) => {
                    let len = arr.get_length();
                    Some(len)
                }
                _ => None,
            })
            .ok_or_else(|| {
                Error::new_execution(&format!("error getting array length by ref={arrayref}"))
            })
    }

    pub fn get_array_len_throwing(
        &self,
        arrayref: i32,
        stack_frames: &mut StackFrames,
    ) -> Throws<i32> {
        if arrayref == 0 {
            bail_thrown!(throw_null_pointer_exception(stack_frames));
        }

        let result = self.get_array_len(arrayref)?;
        Ok(Some(result))
    }

    pub(crate) fn get_const_string_ref(&self, string: &str) -> Option<i32> {
        self.ref_by_stringvalue.get(string).map(|v| *v)
    }

    pub(crate) fn put_const_string_ref(
        &self,
        string: &str,
        reference: i32,
    ) -> Result<Option<i32>> {
        if let Some(mut entry) = self.ref_by_stringvalue.get_mut(string) {
            return Ok(Some(std::mem::replace(entry.value_mut(), reference)));
        }
        let _input = RootedSlots::from_slots(vec![Slot::Ref(reference)]);
        let reservation = self.reserve(charge::intern(string.len())?)?;
        Ok(reservation.commit_intern(copy_string(string)?, reference)?)
    }

    #[allow(dead_code)]
    pub(crate) fn dump(&self) -> Result<()> {
        let json_string = serde_json::to_string(self)?;
        let ts = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
        File::create_new(format!("heap_dump_{ts}.json")).and_then(|mut file| {
            use std::io::Write;
            file.write_all(json_string.as_bytes())
        })?;

        Ok(())
    }

    pub(crate) fn clone_instance(
        &self,
        objectref: i32,
        destination: &mut RootedSlots,
    ) -> Result<()> {
        let _input = JavaThread::hold_temp_roots(vec![Slot::Ref(objectref)]);
        destination.prepare_single_ref()?;
        let charge = self
            .data
            .get(objectref)
            .ok_or_else(|| Error::new_execution("Missing clone source"))?
            .planned_charge()?;
        // Release the source shard before admission (task 7 may collect on failure).
        let reservation = self.reserve(charge)?;
        let cloned = self
            .data
            .get(objectref)
            .ok_or_else(|| Error::new_execution("Missing rooted clone source"))?
            .try_clone()?;
        let reference = reservation.commit(cloned, destination)?;
        #[cfg(test)]
        observe_array_commit(reference);
        #[cfg(not(test))]
        let _ = reference;

        Ok(())
    }
}

#[cfg(test)]
#[path = "budget_tests.rs"]
mod budget_tests;

#[cfg(test)]
#[path = "construction_tests.rs"]
mod construction_tests;

#[cfg(test)]
thread_local! {
    static ARRAY_ALLOCATION_OBSERVER: std::cell::RefCell<Option<Box<dyn Fn(i32)>>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(test)]
fn observe_array_commit(reference: i32) {
    ARRAY_ALLOCATION_OBSERVER.with(|observer| {
        if let Some(observer) = observer.borrow().as_ref() {
            observer(reference);
        }
    });
}

#[cfg(test)]
pub(crate) fn observe_array_allocations<R>(
    observer: impl Fn(i32) + 'static,
    operation: impl FnOnce() -> R,
) -> R {
    struct Reset;
    impl Drop for Reset {
        fn drop(&mut self) {
            ARRAY_ALLOCATION_OBSERVER.with(|observer| observer.borrow_mut().take());
        }
    }
    ARRAY_ALLOCATION_OBSERVER.with(|slot| {
        assert!(slot.borrow().is_none());
        *slot.borrow_mut() = Some(Box::new(observer));
    });
    let _reset = Reset;
    operation()
}

#[cfg(test)]
mod ref_transfer_tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn ref_transfer_clone_roots_input_during_allocation_and_publishes_output() {
        let source = HEAP
            .create_array_with_values(
                "[I",
                &[17, 23],
                &mut crate::vm::stack::rooted_slots::RootedSlots::new(),
            )
            .unwrap();
        let mut destination = RootedSlots::new();
        let observed = Rc::new(Cell::new(false));
        let seen = Rc::clone(&observed);

        observe_array_allocations(
            move |_| {
                assert!(JavaThread::temp_root_refs().contains(&source));
                seen.set(true);
            },
            || HEAP.clone_instance(source, &mut destination).unwrap(),
        );

        let clone = destination[0].value();
        assert!(
            observed.get(),
            "clone must cross the observed allocation boundary"
        );
        assert_ne!(clone, source);
        assert_eq!(HEAP.get_array_value(clone, 1).unwrap(), vec![23]);
        assert_eq!(JavaThread::temp_root_refs(), vec![clone]);
    }
}
