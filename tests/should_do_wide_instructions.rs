mod utils;
use utils::assert_success;

#[test]
fn should_do_wide_instructions() {
    assert_success(
        "samples.javacore.wide.instructions.trivial.WideInstructionsExample",
        "31\n",
    );
}
