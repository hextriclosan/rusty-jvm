mod utils;
use utils::assert_success;

#[test]
fn should_do_trivial_switch() {
    assert_success("samples.javacore.switches.trivial.SwitchExample", "1300\n");
}
