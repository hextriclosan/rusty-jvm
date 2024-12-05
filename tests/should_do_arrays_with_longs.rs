mod utils;
use utils::assert_success;

#[test]
fn should_do_arrays_with_longs() {
    assert_success("samples.arrays.array.longs.ArrayLong", "233646220932000\n");
}
