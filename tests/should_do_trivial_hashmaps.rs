mod utils;
use utils::get_int;
use utils::setup;

#[test]
#[ignore]
fn should_do_trivial_hashmaps() {
    let mut vm = setup();
    let last_frame_value = vm
        .run("samples.javabase.util.hashmap.trivial.TrivialHashMap")
        .unwrap();
    assert_eq!(84, get_int(last_frame_value))
}
