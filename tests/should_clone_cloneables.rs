mod utils;
use utils::assert_success;

#[test]
fn should_clone_cloneables() {
    assert_success("samples.javacore.cloneable.trivial.CloneExample", "511\n");
}
