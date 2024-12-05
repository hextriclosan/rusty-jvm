mod utils;
use utils::assert_success;

#[test]
fn should_do_3d_arrays() {
    assert_success("samples.arrays.array3d.Array3D", "780\n");
}
