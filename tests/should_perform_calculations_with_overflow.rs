mod utils;
use utils::assert_success;

#[test]
fn should_perform_calculations_with_overflow() {
    assert_success("samples.arithmetics.overflow.ArithmeticOverflow", "1\n");
}
