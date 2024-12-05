mod utils;
use utils::assert_success;

#[test]
fn should_do_unsafe_object_field_offset() {
    assert_success(
        "samples.jdkinternal.unsafe.objectfieldoffset.UnsafeObjectFieldOffset",
        "127\n",
    );
}
