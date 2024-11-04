mod utils;
use utils::get_int;
use utils::setup;
#[test]
fn should_initialize_system_on_load() {
    let mut vm = setup();
    let last_frame_value = vm.run("samples.system.load.SystemLoad").unwrap();
    assert_eq!(4321, get_int(last_frame_value))
}
