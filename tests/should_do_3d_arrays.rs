mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_do_3d_arrays() {
    let last_frame_value = VM::run("samples.arrays.array3d.Array3D").unwrap();
    assert_eq!(780, get_int(last_frame_value))
}
