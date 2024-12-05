mod utils;
use utils::assert_success;

#[test]
fn should_do_subtraction_with_doubles() {
    assert_success(
        "samples.arithmetics.sub.doubles.SubDoubles",
        "-8.76543211E200\n",
    );
}
