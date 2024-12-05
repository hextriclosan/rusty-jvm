mod utils;
use utils::assert_success;

#[test]
fn should_do_adding_with_negative_longs() {
    assert_success(
        "samples.arithmetics.addernegative.AdderNegativeLong",
        "-1990000000000000\n",
    );
}
