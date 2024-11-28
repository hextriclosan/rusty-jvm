mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_operate_with_map_interface() {
    let last_frame_value =
        VM::run("samples.javabase.util.mapinterface.usage.AdvancedMapInterfaceUsage").unwrap();
    assert_eq!(1, get_int(last_frame_value))
}
