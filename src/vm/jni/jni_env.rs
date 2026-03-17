use crate::vm::jni::jni_impl::get_version;
use std::ffi::c_void;
use std::sync::LazyLock;

pub(super) type JNIEnv = *const JNINativeInterface;

struct EnvHolder {
    env_inner: *const JNINativeInterface,
}

impl EnvHolder {
    pub fn new() -> Self {
        let env_inner: JNIEnv = &VTABLE;
        Self { env_inner }
    }

    pub fn get(&self) -> *mut c_void {
        &self.env_inner as *const _ as *mut c_void
    }
}

unsafe impl Sync for EnvHolder {}
unsafe impl Send for EnvHolder {}

static JNI_ENV: LazyLock<EnvHolder> = LazyLock::new(EnvHolder::new);

pub(crate) fn get_jni_env() -> *mut c_void {
    JNI_ENV.get()
}

#[repr(C)]
#[allow(non_snake_case)]
pub(super) struct JNINativeInterface {
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
