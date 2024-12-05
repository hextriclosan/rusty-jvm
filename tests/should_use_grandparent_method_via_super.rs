mod utils;
use utils::assert_success;

#[test]
fn should_use_grandparent_method_via_super() {
    assert_success(
        "samples.inheritance.usegrandparentmethodviasuper.UseGrandParentMethodViaSuperExample",
        "1337\n",
    );
}
