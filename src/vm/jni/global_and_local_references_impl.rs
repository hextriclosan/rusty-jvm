use jni_sys::{jint, jobject, JNIEnv};

// Since we don't have a GC (yet) and local references, we can use a stub implementation.
pub(super) extern "system" fn push_local_frame(_env: *mut JNIEnv, _capacity: jint) -> jint {
    0 // todo: implement me
}

// Since we don't have a GC (yet) and local references, we can use a stub implementation.
pub(super) extern "system" fn pop_local_frame(_env: *mut JNIEnv, result: jobject) -> jobject {
    result // todo: implement me
}
