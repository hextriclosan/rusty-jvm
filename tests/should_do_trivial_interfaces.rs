mod utils;
use utils::assert_success;

#[test]
fn should_do_trivial_interfaces() {
    assert_success(
        "samples.inheritance.interfaces.trivial.TrivialInterface",
        "-200\n",
    );
}
