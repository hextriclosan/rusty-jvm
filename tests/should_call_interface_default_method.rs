mod utils;
use utils::assert_success;

#[test]
fn should_call_interface_default_method() {
    assert_success(
        "samples.inheritance.interfacedefaultmethoddirectcall.InterfaceDefaultMethodDirectCall",
        "31\n",
    );
}
