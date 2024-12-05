mod utils;
use utils::assert_success;

#[test]
fn should_operate_with_map_interface() {
    assert_success(
        "samples.javabase.util.mapinterface.usage.AdvancedMapInterfaceUsage",
        "1\n",
    );
}
