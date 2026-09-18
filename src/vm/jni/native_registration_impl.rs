use crate::vm::exception::pending_helpers::set_pending_null_pointer_exception;
use crate::vm::helper::klass;
use crate::vm::jni::utils::{set_pending_internal_error, set_pending_no_such_method_error};
use dashmap::DashMap;
use jni_sys::{jclass, jint, JNIEnv, JNINativeMethod, JNI_ERR, JNI_OK};
use std::ffi::CStr;
use std::sync::LazyLock;

static REGISTERED_NATIVES: LazyLock<DashMap<(String, String), usize>> =
    LazyLock::new(DashMap::new);

pub(crate) unsafe extern "system" fn register_natives(
    _env: *mut JNIEnv,
    clazz: jclass,
    methods: *const JNINativeMethod,
    method_count: jint,
) -> jint {
    if clazz.is_null() {
        let _ = set_pending_null_pointer_exception();
        return JNI_ERR;
    }
    if method_count < 0 || (method_count > 0 && methods.is_null()) {
        return JNI_ERR;
    }
    let class = match klass(clazz as i32) {
        Ok(class) => class,
        Err(error) => {
            let _ = set_pending_internal_error(&error.to_string());
            return JNI_ERR;
        }
    };
    let class_name = class.this_class_name().to_owned();
    let methods = if method_count == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(methods, method_count as usize) }
    };
    let mut registrations = Vec::with_capacity(methods.len());

    for method in methods {
        let Some(name) = decode_mutf8(method.name) else {
            let _ = set_pending_no_such_method_error("invalid native method name");
            return JNI_ERR;
        };
        let Some(signature) = decode_mutf8(method.signature) else {
            let _ = set_pending_no_such_method_error(&name);
            return JNI_ERR;
        };
        let name_signature = format!("{name}:{signature}");
        let Some(java_method) = class.try_get_method(&name_signature) else {
            let _ = set_pending_no_such_method_error(&name_signature);
            return JNI_ERR;
        };
        if !java_method.is_native() || method.fnPtr.is_null() {
            let _ = set_pending_no_such_method_error(&name_signature);
            return JNI_ERR;
        }
        registrations.push((name_signature, method.fnPtr as usize));
    }

    for (name_signature, address) in registrations {
        REGISTERED_NATIVES.insert((class_name.clone(), name_signature), address);
    }
    JNI_OK
}

pub(crate) unsafe extern "system" fn unregister_natives(_env: *mut JNIEnv, clazz: jclass) -> jint {
    if clazz.is_null() {
        let _ = set_pending_null_pointer_exception();
        return JNI_ERR;
    }
    let class_name = match klass(clazz as i32) {
        Ok(class) => class.this_class_name().to_owned(),
        Err(error) => {
            let _ = set_pending_internal_error(&error.to_string());
            return JNI_ERR;
        }
    };
    REGISTERED_NATIVES.retain(|(registered_class, _), _| registered_class != &class_name);
    JNI_OK
}

pub(crate) fn registered_native_address(class_name: &str, name_signature: &str) -> Option<i64> {
    REGISTERED_NATIVES
        .get(&(class_name.to_owned(), name_signature.to_owned()))
        .map(|entry| *entry as i64)
}

fn decode_mutf8(value: *const std::ffi::c_char) -> Option<String> {
    if value.is_null() {
        return None;
    }
    let bytes = unsafe { CStr::from_ptr(value) }.to_bytes();
    cesu8::from_java_cesu8(bytes)
        .ok()
        .map(|decoded| decoded.into_owned())
}
