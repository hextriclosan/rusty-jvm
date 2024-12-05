mod utils;
use utils::assert_success;

#[test]
fn should_do_subtraction_with_longs() {
    assert_success("samples.arithmetics.sub.longs.SubLongs", "-1000000000\n");
}
