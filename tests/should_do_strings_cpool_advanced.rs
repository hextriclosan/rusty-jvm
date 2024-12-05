mod utils;
use utils::assert_success;

#[test]
fn should_do_strings_cpool_advanced() {
    assert_success(
        "samples.javacore.strings.cpool.advanced.StringPoolAdvanced",
        "127\n",
    );
}
