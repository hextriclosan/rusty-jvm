mod utils;
use utils::assert_success;

#[test]
fn should_do_composite_pattern() {
    assert_success(
        "samples.inheritance.interfaces.compositepattern.CompositePattern",
        "700\n",
    );
}
