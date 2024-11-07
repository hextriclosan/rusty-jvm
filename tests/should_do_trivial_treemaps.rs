mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_do_trivial_treemaps() {
    let last_frame_value =
        VM::run("samples.javabase.util.treemap.trivial.TrivialTreeMap").unwrap();
    assert_eq!(1, get_int(last_frame_value))
}
