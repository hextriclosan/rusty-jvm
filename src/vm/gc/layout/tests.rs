use super::*;
use crate::vm::heap::heap::Heap;
use crate::vm::heap::java_instance::{Array, JavaInstance, JavaInstanceBase};
use crate::vm::stack::rooted_slots::RootedSlots;

fn metadata(class: &str, fields: &[(&str, &str)]) -> (String, Vec<Arc<FieldInfo>>) {
    (
        class.to_string(),
        fields
            .iter()
            .map(|(name, descriptor)| {
                Arc::new(FieldInfo::new(
                    descriptor.parse().unwrap(),
                    0,
                    class.to_string(),
                    name.to_string(),
                ))
            })
            .collect(),
    )
}

fn storage(segments: &[(String, Vec<Arc<FieldInfo>>)]) -> InstanceFields {
    segments
        .iter()
        .map(|(class, fields)| {
            (
                class.clone(),
                fields
                    .iter()
                    .map(|field| {
                        (
                            field.name().to_string(),
                            FieldValue::new(field.type_descriptor().clone()).unwrap(),
                        )
                    })
                    .collect(),
            )
        })
        .collect()
}

#[test]
fn gc_roots_injected_layout_distinguishes_inherited_bare_names_and_clone_edges() {
    // Given the same bare names with opposite types in two declaring classes.
    let segments = vec![
        metadata("Parent", &[("value", "Ljava/lang/Object;"), ("other", "I")]),
        metadata("Child", &[("value", "I"), ("other", "[I")]),
    ];
    let fields = storage(&segments);
    let layout = Arc::new(InstanceLayout::new(segments).unwrap());
    let heap = Heap::default();
    let parent_edge = heap.create_array("[I", 0);
    let child_edge = heap.create_array("[I", 0);
    let primitive_bits = heap.create_array("[I", 0);
    let mut object =
        JavaInstance::Base(JavaInstanceBase::new(usize::MAX, fields, layout).unwrap());
    object
        .set_field_value("Parent", "value", vec![parent_edge])
        .unwrap();
    object
        .set_field_value("Parent", "other", vec![primitive_bits])
        .unwrap();
    object
        .set_field_value("Child", "value", vec![primitive_bits])
        .unwrap();
    object
        .set_field_value("Child", "other", vec![child_edge])
        .unwrap();
    let reference = heap.create_instance(object);
    // When scanning an object with no registered klass_id, no lazy class lookup is possible.
    let edges = heap.references(reference).unwrap();
    let mut cloned = RootedSlots::new();
    heap.clone_instance(reference, &mut cloned).unwrap();
    // Then only descriptor-typed edges survive, including after cloning.
    assert_eq!(edges.strong, vec![parent_edge, child_edge]);
    assert_eq!(edges.referent, None);
    assert_eq!(heap.references(cloned[0].value()).unwrap(), edges);
}

#[test]
fn gc_roots_only_the_reference_class_referent_is_special() {
    let segments = vec![
        metadata(
            "java/lang/ref/Reference",
            &[
                ("referent", "Ljava/lang/Object;"),
                ("discovered", "Ljava/lang/ref/Reference;"),
                ("next", "Ljava/lang/ref/Reference;"),
            ],
        ),
        metadata("UserReference", &[("referent", "Ljava/lang/Object;")]),
    ];
    let fields = storage(&segments);
    let layout = Arc::new(InstanceLayout::new(segments).unwrap());
    let mut object = JavaInstance::Base(JavaInstanceBase::new(0, fields, layout).unwrap());
    object
        .set_field_value("java/lang/ref/Reference", "referent", vec![41])
        .unwrap();
    object
        .set_field_value("java/lang/ref/Reference", "discovered", vec![42])
        .unwrap();
    object
        .set_field_value("java/lang/ref/Reference", "next", vec![43])
        .unwrap();
    object
        .set_field_value("UserReference", "referent", vec![44])
        .unwrap();

    let edges = object.references().unwrap();

    assert_eq!(
        edges,
        References {
            strong: vec![42, 43, 44],
            referent: Some(41)
        }
    );
}

#[test]
fn gc_roots_incomplete_layout_rejects_construction_before_heap_publication() {
    let segments = vec![metadata("Test", &[("reference", "Ljava/lang/Object;")])];
    let fields = storage(&segments);
    let missing = Arc::new(InstanceLayout::new(Vec::new()).unwrap());

    let instance = JavaInstanceBase::new(0, fields, missing);

    assert!(instance.is_err());
    assert!(InstanceLayout::new(vec![metadata("Test", &[("bad", "V")])]).is_err());
}

#[test]
fn gc_roots_malformed_reference_width_aborts_instead_of_tracing_a_prefix() {
    let segments = vec![metadata("Test", &[("reference", "Ljava/lang/Object;")])];
    let fields = storage(&segments);
    let layout = Arc::new(InstanceLayout::new(segments).unwrap());
    let mut object = JavaInstance::Base(JavaInstanceBase::new(0, fields, layout).unwrap());
    object
        .set_field_value("Test", "reference", vec![11, 22])
        .unwrap();

    assert!(object.references().is_err());
}

#[test]
fn gc_roots_array_descriptors_not_element_width_decide_edges() {
    // Equal four-byte payloads must be traced differently.
    let values = [0x0102_0304, 0, 73];
    for descriptor in ["[I", "[F"] {
        let array = Array::new_with_values(descriptor, &values);
        assert!(array.references().unwrap().strong.is_empty());
    }
    for descriptor in ["[[I", "[[J", "[Ljava/lang/Object;", "[[Ljava/lang/String;"] {
        let array = Array::new_with_values(descriptor, &values);
        assert_eq!(array.references().unwrap().strong, vec![0x0102_0304, 73]);
    }
    for descriptor in ["[B", "[C", "[S", "[Z", "[J", "[D"] {
        let mut array = Array::new(descriptor, 1);
        let value = if matches!(descriptor, "[J" | "[D") {
            vec![11, 22]
        } else {
            vec![11]
        };
        array.set_value(0, value).unwrap();
        assert!(array.references().unwrap().strong.is_empty());
    }
    assert!(Array::new("not-an-array", 0).references().is_err());
    let mut truncated = Array::new("[[I", 1);
    truncated.raw_data_mut().pop();
    assert!(truncated.references().is_err());
}
