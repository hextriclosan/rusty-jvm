mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_do_trivial_concurrent_hash_maps() {
    let last_frame_value =
        VM::run("samples.javabase.util.concurrent.hashmap.trivial.TrivialConcurrentHashMap")
            .unwrap();
    assert_eq!(97, get_int(last_frame_value))
}
