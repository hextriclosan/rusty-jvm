use super::*;
use jclassmodel::modifiers::FieldModifier;

fn static_field(name: &str, descriptor: TypeDescriptor) -> FieldModel {
    FieldModel {
        name: name.to_string(),
        descriptor,
        modifiers: FieldModifier::Static,
    }
}

#[test]
fn gc_roots_static_layout_excludes_primitive_bits_and_tolerates_absent_mirror() {
    let built = JavaClass::build_fields(
        vec![
            static_field(
                "reference",
                TypeDescriptor::Object("java/lang/Object".to_string()),
            ),
            static_field(
                "array",
                TypeDescriptor::Array(Box::new(TypeDescriptor::Integer), 1),
            ),
            static_field("bits", TypeDescriptor::Integer),
        ],
        "MissingMirror",
    )
    .unwrap();
    let mut class = JavaClass::synthetic(
        "MissingMirror".to_string(),
        "MissingMirror".to_string(),
        None,
        IndexSet::new(),
    );
    class.fields_info = built.info;
    class.static_fields = built.static_values;
    class.static_references = built.static_references;
    class
        .static_field("reference")
        .unwrap()
        .set_raw_value(vec![17])
        .unwrap();
    class
        .static_field("array")
        .unwrap()
        .set_raw_value(vec![18])
        .unwrap();
    class
        .static_field("bits")
        .unwrap()
        .set_raw_value(vec![19])
        .unwrap();
    assert_eq!(class.static_fields_init_state.begin(), Ok(true));
    class.static_fields_init_state.finish(Some(20));

    let roots = class.root_refs().unwrap();

    assert_eq!(roots, vec![17, 18, 20]);
    assert_eq!(class.mirror_ref_if_present(), None);
    assert!(class.instance_fields_hierarchy.get().is_none());
    assert!(class.fields_offset_mapping.get().is_none());
    assert!(class.vtable.get().is_none());
    assert!(class
        .fields_info
        .values()
        .all(|field| field.reflection_ref_if_present().is_none()));
}

#[test]
fn gc_roots_ambiguous_duplicate_fields_abort_layout_construction() {
    let fields = vec![
        static_field(
            "same",
            TypeDescriptor::Object("java/lang/Object".to_string()),
        ),
        static_field("same", TypeDescriptor::Integer),
    ];

    assert!(JavaClass::build_fields(fields, "Ambiguous").is_err());
}

#[test]
fn gc_roots_failed_initialization_snapshot_never_waits_for_a_stopped_lock_owner() {
    let state = InitState::default();
    let held = state.state.lock();

    assert!(state.failure_root().is_err());

    drop(held);
    assert_eq!(state.begin(), Ok(true));
    state.finish(Some(71));
    assert_eq!(state.failure_root().unwrap(), Some(71));
}
