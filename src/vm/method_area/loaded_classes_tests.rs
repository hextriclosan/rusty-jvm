use super::*;
use crate::vm::monitor;
use crate::vm::safepoint::{self, Event};
use std::sync::mpsc::channel;
use std::time::Duration;

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

    let java_monitor = HEAP.create_array("[I", 0);
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
