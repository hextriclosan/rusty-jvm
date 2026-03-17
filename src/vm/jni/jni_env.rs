use crate::vm::jni::jni_impl::get_version;
use std::ffi::c_void;

pub(crate) type JNIEnv = *mut *const JNINativeInterface;

pub(crate) fn get_jni_env() -> JNIEnv {
    static mut ENV: *const JNINativeInterface = &VTABLE;
    &raw mut ENV
}

#[repr(C)]
#[allow(non_snake_case)]
pub(crate) struct JNINativeInterface {
    pub reserved0: *mut c_void,
    pub reserved1: *mut c_void,
    pub reserved2: *mut c_void,
    pub reserved3: *mut c_void,
    pub GetVersion: unsafe extern "system" fn(env: *mut JNIEnv) -> i32,
}

unsafe impl Sync for JNINativeInterface {}

static VTABLE: JNINativeInterface = JNINativeInterface {
    reserved0: std::ptr::null_mut(),
    reserved1: std::ptr::null_mut(),
    reserved2: std::ptr::null_mut(),
    reserved3: std::ptr::null_mut(),
    GetVersion: get_version,
};
