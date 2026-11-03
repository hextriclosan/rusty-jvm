use super::*;
use crate::vm::gc::budget::AllocError;
use crate::vm::gc::layout::InstanceLayout;
use crate::vm::gc::test_allocations::measure;
use crate::vm::method_area::field::{FieldInfo, FieldValue};
use indexmap::IndexMap;
use std::sync::Arc;

#[test]
fn gc_budget_wide_initializers_preserve_both_words_without_panicking() {
    let values = [0x0102_0304, 0x0506_0708, -1, -2];
    for descriptor in ["[J", "[D"] {
        let array = Array::new_with_values(descriptor, &values).unwrap();
        assert_eq!(array.get_length(), 2);
        assert_eq!(array.get_value(0).unwrap(), values[..2]);
        assert_eq!(array.get_value(1).unwrap(), values[2..]);
        assert!(matches!(
            Array::new_with_values(descriptor, &[1]),
            Err(AllocError::InvalidLayout)
        ));
    }
}

#[test]
fn gc_budget_dimensions_fail_before_any_host_payload_allocation() {
    for descriptor in ["[B", "[I", "[J", "[D", "[Ljava/lang/Object;"] {
        let (negative, allocations) = measure(1024, || Array::new(descriptor, -5));
        assert!(matches!(negative, Err(AllocError::NegativeLength(-5))));
        assert_eq!(allocations, 0);
        let (oversized, allocations) = measure(1024, || Array::new(descriptor, i32::MAX));
        assert!(matches!(oversized, Err(AllocError::Capacity)));
        assert_eq!(allocations, 0);
    }
    assert_eq!(
        ArrayLayout::with_values("[I", usize::MAX).unwrap_err(),
        AllocError::Capacity
    );
}

#[test]
fn gc_budget_array_copyback_preserves_capacity_and_rejects_growth() {
    let heap = Heap::default();
    let reference = heap.create_array("[I", 2, &mut RootedSlots::new()).unwrap();
    let used = heap.admission.lock().budget.used;
    let mut replacement = Array::new_with_values("[I", &[7, 11]).unwrap();
    replacement.raw_data_mut().reserve(4096);
    heap.set_entire_array(reference, replacement).unwrap();
    assert_eq!(heap.get_array_value(reference, 1).unwrap(), vec![11]);
    assert_eq!(heap.admission.lock().budget.used, used);
    assert!(heap
        .set_entire_array(reference, Array::new("[I", 3).unwrap())
        .is_err());
    assert_eq!(heap.get_array_len(reference).unwrap(), 2);
    let mut view = heap.get_entire_raw_data_mut(reference).unwrap();
    let bytes: &mut [u8] = &mut view;
    bytes[..4].copy_from_slice(&13i32.to_ne_bytes());
    drop(view);
    assert_eq!(heap.get_array_value(reference, 0).unwrap(), vec![13]);
}

#[test]
fn gc_budget_instance_payload_is_copied_only_after_reservation() {
    let metadata = Arc::new(FieldInfo::new(
        "I".parse().unwrap(),
        0,
        "Test".into(),
        "value".into(),
    ));
    let layout = Arc::new(InstanceLayout::new(vec![("Test".into(), vec![metadata])]).unwrap());
    let fields = Arc::new(IndexMap::from([(
        "Test".to_string(),
        IndexMap::from([(
            "value".to_string(),
            FieldValue::new("I".parse().unwrap()).unwrap(),
        )]),
    )]));
    let plan = InstancePlan::new(0, Arc::clone(&fields), Arc::clone(&layout)).unwrap();
    let charge = plan.charge().unwrap();
    let heap = Heap::default();
    heap.admission.lock().budget.max = charge - 1;
    heap.admission.lock().budget.enforced = true;
    let mut destination = RootedSlots::new();
    destination.prepare_single_ref().unwrap();
    let (result, allocations) =
        measure(usize::MAX, || heap.create_instance(plan, &mut destination));
    assert!(matches!(
        result.unwrap_err().kind(),
        crate::vm::error::ErrorKind::Allocation(AllocError::Capacity)
    ));
    assert_eq!(allocations, 0);
    heap.admission.lock().budget.max = charge;
    let reference = heap
        .create_instance(
            InstancePlan::new(0, fields, layout).unwrap(),
            &mut destination,
        )
        .unwrap();
    assert_eq!(heap.admission.lock().budget.used, charge);
    heap.set_object_field_value(reference, "Test", "value", vec![23])
        .unwrap();
    assert!(heap
        .set_object_field_value(reference, "Test", "value", vec![1, 2])
        .is_err());
    assert_eq!(
        heap.get_object_field_value(reference, "Test", "value")
            .unwrap(),
        vec![23]
    );
    assert_eq!(heap.admission.lock().budget.used, charge);
}
