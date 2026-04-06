#[macro_export]
macro_rules! from_mutf8_ptr {
    ($ptr:expr) => {{
        use std::ffi::CStr;
        // Convert the raw pointer to a byte slice
        cesu8::from_java_cesu8(unsafe { CStr::from_ptr($ptr) }.to_bytes())
    }};
}
