mod utils;
use utils::assert_success;

#[test]
fn should_do_adding() {
    assert_success("samples.arithmetics.adder.ints.AdderInt", "55\n");
}
