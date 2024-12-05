mod utils;
use utils::assert_success;

#[test]
fn should_do_trivial_unsafe_things() {
    assert_success(
        "samples.jdkinternal.unsafe.trivial.UnsafeUsage",
        "2097151\n",
    );
}
