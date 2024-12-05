mod utils;
use utils::assert_success;

#[test]
fn should_initialize_system_on_load() {
    assert_success("samples.system.load.SystemLoad", "4321\n");
}
