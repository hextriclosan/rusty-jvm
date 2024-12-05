mod utils;
use utils::assert_success;

#[test]
fn should_do_trivial_concurrent_hash_maps() {
    assert_success(
        "samples.javabase.util.concurrent.hashmap.trivial.TrivialConcurrentHashMap",
        "97\n",
    );
}
