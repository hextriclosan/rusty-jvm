use super::*;
use crate::vm::monitor;
use crate::vm::safepoint::{self, Event};
use std::sync::mpsc::channel;
use std::time::Duration;

#[test]
fn gc_roots_published_class_without_mirror_never_takes_publication_claim() {
    let classes = Arc::new(LoadedClasses::default());
    let class = Arc::new(JavaClass::synthetic(
        "I".to_string(),
        "int".to_string(),
        None,
        IndexSet::new(),
    ));
    let claim = classes.loading.claim("I");
    classes.insert_full_impl("I", Arc::clone(&class));
    assert_eq!(class.static_fields_init_state().begin(), Ok(true));
    class.static_fields_init_state().finish(Some(73));
    let (scanned, scan) = channel();
    let scanning_classes = Arc::clone(&classes);
    let reader = std::thread::spawn(move || {
        scanned
            .send(
                scanning_classes
                    .root_refs()
                    .map_err(|error| error.to_string()),
            )
            .unwrap();
    });

    // Publication is held until after the scan; a mirror cannot appear in this window.
    let result = scan.recv_timeout(Duration::from_secs(2));
    drop(claim);
    reader.join().unwrap();
    assert_eq!(result.unwrap().unwrap(), vec![73]);
    assert_eq!(class.mirror_ref_if_present(), None);

    class.inject_mirror_clazz_ref(91).unwrap();
    assert_eq!(classes.root_refs().unwrap(), vec![91, 73]);
}

#[test]
fn class_loading_releases_claim_before_java_and_adopts_publication_winner() {
    let _serial = safepoint::tests::SERIAL.lock().unwrap();
    // Given two prepared candidates and a Java monitor held by another thread.
    let classes = Arc::new(LoadedClasses::default());
    let name = "PublicationRace";
    let candidate = || {
        Arc::new(JavaClass::synthetic(
            name.to_string(),
            name.to_string(),
            None,
            IndexSet::new(),
        ))
    };
    let winner = candidate();
    let loser = candidate();
    winner.inject_mirror_clazz_ref(42).unwrap();
    assert_eq!(winner.static_fields_init_state().begin(), Ok(true));
    winner.static_fields_init_state().finish(Some(73));

    let java_monitor = HEAP
        .create_array(
            "[I",
            0,
            &mut crate::vm::stack::rooted_slots::RootedSlots::new(),
        )
        .unwrap();
    monitor::enter(java_monitor);
    let (blocked, loading) = channel();
    let contender_classes = Arc::clone(&classes);
    let contender = std::thread::spawn(move || {
        safepoint::attach();
        // Subscribe before entering the same monitor implementation used by Java.
        safepoint::observe(move |event| {
            if event == Event::SnapshotPublished {
                blocked.send(()).unwrap();
            }
        });
        let result = contender_classes.get_or_load(name, || {
            monitor::enter(java_monitor);
            monitor::exit(java_monitor)?;
            if contender_classes.is_loaded(name) {
                Ok(loser)
            } else {
                // On a retained-claim mutation, let the load unwind after detection
                // so the publisher can finish and both workers can be joined.
                Err(Error::new_execution("publication blocked by loading claim"))
            }
        });
        let result = result.map_err(|error| error.to_string());
        safepoint::retire();
        result
    });

    // When the first load is blocked in Java, another load must still publish.
    let reached_java = loading.recv_timeout(Duration::from_secs(5));
    let (published, publication) = channel();
    let publisher_classes = Arc::clone(&classes);
    let published_winner = Arc::clone(&winner);
    let publisher = std::thread::spawn(move || {
        let result = publisher_classes.get_or_load(name, || {
            let _claim = publisher_classes.loading.claim(name);
            // Stand in for defineClass's completed mirror preparation, using the
            // real registry insertion without bootstrapping a second VM in a test.
            publisher_classes.insert_full_impl(name, Arc::clone(&published_winner));
            Ok(published_winner)
        });
        published.send(()).unwrap();
        result.map_err(|error| error.to_string())
    });
    let published_while_in_java = publication.recv_timeout(Duration::from_secs(2));

    // Then cleanup precedes assertions, including when the old claim scope fails.
    monitor::exit(java_monitor).unwrap();
    let contender_result = contender.join().unwrap();
    let publisher_result = publisher.join().unwrap();
    assert!(
        reached_java.is_ok(),
        "loader never blocked at the Java monitor"
    );
    assert!(
        published_while_in_java.is_ok(),
        "publication claim retained across Java execution"
    );
    let (id, key, adopted) = contender_result.unwrap();
    let (winner_id, _, published_class) = publisher_result.unwrap();
    assert_eq!(id, winner_id);
    assert_eq!(key, name);
    assert!(Arc::ptr_eq(&adopted, &winner));
    assert!(Arc::ptr_eq(&published_class, &winner));
    assert!(Arc::ptr_eq(&classes.get_by_id(id).unwrap(), &winner));
    assert!(Arc::ptr_eq(&classes.get(name).unwrap(), &winner));
    assert_eq!(adopted.mirror_clazz_ref().unwrap(), 42);
    assert_eq!(adopted.static_fields_init_state().begin(), Err(73));
    assert_eq!(classes.loaded_classes.len(), 1);
    assert_eq!(classes.id_index.len(), 1);
}

/// Hold an actual publication claim until the opcode has published its roots and
/// a real collection has finished. Channels establish the order, not timing.
fn collect_during_opcode_lookup(
    opcode: u8,
    caller: &'static str,
    claimed: &'static str,
    index: u16,
    operand: i32,
    expected: crate::vm::stack::slot::Slot,
    reachable: &[i32],
) {
    use crate::vm::execution_engine::opcode::{ARETURN, INSTANCEOF, IRETURN};
    use crate::vm::execution_engine::Engine;
    use crate::vm::gc::collector::{collect, Cause};
    use crate::vm::stack::slot::Slot;
    use crate::vm::stack::stack_frame::{ExceptionTable, StackFrame};
    use std::cell::Cell;

    let dead = HEAP.create_array("[I", 0, &mut RootedSlots::new()).unwrap();
    safepoint::attach();
    let claim = CLASSES.loading.claim(claimed);
    let (published, publication) = channel();
    let worker = std::thread::spawn(move || {
        safepoint::attach();
        let [high, low] = index.to_be_bytes();
        let return_opcode = if opcode == INSTANCEOF {
            IRETURN
        } else {
            ARETURN
        };
        let mut frame = StackFrame::new(
            Arc::new("rootTransfer:()V".into()),
            0,
            1,
            Arc::new(vec![opcode, high, low, return_opcode]),
            Arc::new(caller.into()),
            Arc::default(),
            Arc::new(ExceptionTable::new(Vec::new())),
        );
        frame.push(Slot::Ref(operand)).unwrap();
        let seen = Cell::new(false);
        safepoint::observe(move |event| {
            if event == Event::SnapshotPublished && !seen.replace(true) {
                published.send(()).unwrap();
            }
        });
        let result = Engine::execute(frame, "root transfer regression")
            .map(|slots| slots[0])
            .map_err(|error| error.to_string());
        safepoint::retire();
        result
    });

    let blocked = publication.recv_timeout(Duration::from_secs(5));
    let collected = blocked.is_ok().then(|| collect(Cause::Explicit)).flatten();
    let survived = reachable.iter().all(|reference| HEAP.contains(*reference));
    let reclaimed = !HEAP.contains(dead);
    // Release the claim and join even when the missing-root mutation fails.
    drop(claim);
    let result = safepoint::blocking(|| worker.join()).unwrap();
    safepoint::retire();

    assert!(
        blocked.is_ok(),
        "opcode never blocked on the held class claim"
    );
    assert!(collected.is_some(), "the real collector did not finish");
    assert!(
        reclaimed,
        "the collection must actually sweep unrooted objects"
    );
    assert!(
        survived,
        "live opcode operand was swept during class lookup"
    );
    assert_eq!(result.unwrap(), expected);
}

fn type_check_keeps_operand(opcode: u8) {
    use crate::vm::execution_engine::opcode::CHECKCAST;
    use crate::vm::stack::slot::Slot;
    use jclassmodel::constant_pool::ConstantPoolLookup;

    let _serial = safepoint::tests::SERIAL.lock().unwrap();
    crate::vm::gc::test_allocations::bootstrap_oom();
    let (id, _, class) = CLASSES.get_full(OBJECT).unwrap();
    let index = (1..=u16::MAX)
        .find(|index| class.constant_pool().get_class_name(*index).as_deref() == Some(OBJECT))
        .unwrap();
    let operand = HEAP
        .create_instance(class.new_instance(id).unwrap(), &mut RootedSlots::new())
        .unwrap();
    let expected = if opcode == CHECKCAST {
        Slot::Ref(operand)
    } else {
        Slot::Value(1)
    };
    collect_during_opcode_lookup(opcode, OBJECT, OBJECT, index, operand, expected, &[operand]);
}

#[test]
fn gc_ref_transfer_checkcast_keeps_operand_during_class_claim_wait() {
    type_check_keeps_operand(crate::vm::execution_engine::opcode::CHECKCAST);
}

#[test]
fn gc_ref_transfer_instanceof_keeps_operand_during_class_claim_wait() {
    type_check_keeps_operand(crate::vm::execution_engine::opcode::INSTANCEOF);
}

#[test]
fn gc_ref_transfer_getfield_keeps_receiver_and_result_during_class_claim_wait() {
    use crate::vm::execution_engine::opcode::GETFIELD;
    use crate::vm::stack::slot::Slot;
    use jclassmodel::constant_pool::ConstantPoolLookup;

    let _serial = safepoint::tests::SERIAL.lock().unwrap();
    crate::vm::gc::test_allocations::bootstrap_oom();
    let caller = "java/util/ArrayList$Itr";
    let owner = "java/util/ArrayList";
    let class = CLASSES.get(caller).unwrap();
    let index = (1..=u16::MAX)
        .find(|index| {
            class
                .constant_pool()
                .get_full_field_info(*index)
                .is_some_and(|(name, field, _)| name == owner && field == "elementData")
        })
        .unwrap();
    let (id, _, class) = CLASSES.get_full(owner).unwrap();
    let operand = HEAP
        .create_instance(class.new_instance(id).unwrap(), &mut RootedSlots::new())
        .unwrap();
    let value = HEAP
        .create_array("[Ljava/lang/Object;", 0, &mut RootedSlots::new())
        .unwrap();
    HEAP.set_object_field_value(operand, owner, "elementData", vec![value])
        .unwrap();
    collect_during_opcode_lookup(
        GETFIELD,
        caller,
        owner,
        index,
        operand,
        Slot::Ref(value),
        &[operand, value],
    );
}
