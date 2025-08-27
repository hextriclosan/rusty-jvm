use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use winapi::um::winnt::WCHAR;

pub struct WideCString {
    inner: Vec<WCHAR>,
}

impl WideCString {
    pub fn new(s: &str) -> Self {
        let inner = OsStr::new(s)
            .encode_wide()
            .chain(once(0)) // null-terminated
            .collect::<Vec<WCHAR>>();
        WideCString { inner }
    }

    pub fn as_ptr(&self) -> *const WCHAR {
        self.inner.as_ptr()
    }
}
