mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_call_interface_default_method() {
    let last_frame_value = VM::run(
        "samples.inheritance.interfacedefaultmethoddirectcall.InterfaceDefaultMethodDirectCall",
    )
    .unwrap();
    assert_eq!(15, get_int(last_frame_value))
}
