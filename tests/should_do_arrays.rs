mod utils;
use utils::assert_success;

#[test]
fn should_do_arrays() {
    assert_success("samples.arrays.array.ints.ArrayInt", "740\n");
}
