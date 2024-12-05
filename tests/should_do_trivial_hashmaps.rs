mod utils;
use utils::assert_success;

#[test]
fn should_do_trivial_hashmaps() {
    assert_success(
        "samples.javabase.util.hashmap.trivial.TrivialHashMap",
        "1\n",
    );
}
