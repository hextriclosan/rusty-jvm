mod utils;
use utils::get_int;
use utils::setup;

#[test]
fn should_do_trivial_treemaps() {
    let mut vm = setup();
    let last_frame_value = vm
        .run("samples.javabase.util.treemap.trivial.TrivialTreeMap")
        .unwrap();
    assert_eq!(1, get_int(last_frame_value))
}
