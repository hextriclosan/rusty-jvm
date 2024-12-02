mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_get_caller_class() {
    let last_frame_value =
        VM::run("samples.jdkinternal.reflection.getcallerclass.ReflectionGetCallerClassExample")
            .unwrap();
    assert_eq!(1, get_int(last_frame_value))
}
