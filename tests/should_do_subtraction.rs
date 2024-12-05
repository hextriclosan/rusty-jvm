mod utils;
use utils::assert_success;

#[test]
fn should_do_subtraction() {
    assert_success("samples.arithmetics.sub.ints.SubInts", "-999\n");
}
