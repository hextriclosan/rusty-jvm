mod utils;
use utils::get_int;
use utils::setup;

#[test]
fn should_do_3d_arrays() {
    let vm = setup();
    let last_frame_value = vm.run("samples.arrays.array3d.Array3D").unwrap();
    assert_eq!(780, get_int(last_frame_value))
}
