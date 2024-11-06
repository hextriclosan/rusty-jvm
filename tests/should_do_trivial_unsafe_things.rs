mod utils;
use utils::get_int;
use utils::setup;

#[test]
fn should_do_trivial_unsafe_things() {
    let vm = setup();
    let last_frame_value = vm
        .run("samples.jdkinternal.unsafe.trivial.UnsafeUsage")
        .unwrap();
    assert_eq!(1048575, get_int(last_frame_value))
}
