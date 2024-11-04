mod utils;
use utils::get_int;
use utils::setup;

#[test]
fn should_do_trivial_concurrent_hash_maps() {
    let mut vm = setup();
    let last_frame_value = vm
        .run("samples.javabase.util.concurrent.hashmap.trivial.TrivialConcurrentHashMap")
        .unwrap();
    assert_eq!(97, get_int(last_frame_value))
}
