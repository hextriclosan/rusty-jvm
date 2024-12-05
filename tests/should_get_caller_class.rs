mod utils;
use utils::assert_success;

#[test]
fn should_get_caller_class() {
    assert_success(
        "samples.jdkinternal.reflection.getcallerclass.ReflectionGetCallerClassExample",
        "1\n",
    );
}
