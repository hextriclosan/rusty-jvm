mod utils;
use utils::assert_success;

#[test]
fn should_do_trivial_reflection_with_primitives() {
    assert_success(
        "samples.reflection.trivial.synthetic.classes.SyntheticPrimitiveClasses",
        "9369\n",
    );
}
