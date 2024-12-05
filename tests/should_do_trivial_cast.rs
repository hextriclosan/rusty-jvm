mod utils;
use utils::assert_success;

#[test]
fn should_do_trivial_cast() {
    assert_success("samples.javacore.cast.trivial.TrivialCast", "1337\n");
}
