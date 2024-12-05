mod utils;
use utils::assert_success;

#[test]
fn should_print_to_stdout() {
    assert_success("samples.system.outexample.SystemOutExample", "42\n");
}
