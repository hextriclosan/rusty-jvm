mod utils;
use utils::get_int;
use utils::setup;

#[test]
fn should_clone_cloneables() {
    let mut vm = setup();
    let last_frame_value = vm
        .run("samples.javacore.cloneable.trivial.CloneExample")
        .unwrap();
    assert_eq!(511, get_int(last_frame_value))
}
