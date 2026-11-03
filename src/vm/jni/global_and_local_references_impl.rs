use crate::vm::jni::handles::{self, HandleError, HandleResult, ReferenceKind};
use crate::vm::jni::java_thread::JavaThread;
use jni_sys::{jint, jobject, jweak, JNIEnv, JNI_ERR, JNI_OK};

fn capacity(value: jint) -> HandleResult<usize> {
    usize::try_from(value).map_err(|_| HandleError::OutOfMemory)
}

fn status(result: HandleResult<()>) -> jint {
    match result {
        Ok(()) => JNI_OK,
        Err(error) => {
            error.set_pending();
            JNI_ERR
        }
    }
}

pub(super) extern "system" fn push_local_frame(_env: *mut JNIEnv, count: jint) -> jint {
    status(capacity(count).and_then(|count| {
        JavaThread::with_jni_frames(|frames| frames.push(count, false).map(|_| ()))
    }))
}

pub(super) extern "system" fn pop_local_frame(_env: *mut JNIEnv, result: jobject) -> jobject {
    handles::finish(JavaThread::with_jni_frames(|frames| frames.pop(result)))
}

pub(super) extern "system" fn ensure_local_capacity(_env: *mut JNIEnv, count: jint) -> jint {
    status(
        capacity(count)
            .and_then(|count| JavaThread::with_jni_frames(|frames| frames.ensure_capacity(count))),
    )
}

pub(super) extern "system" fn new_local_ref(_env: *mut JNIEnv, object: jobject) -> jobject {
    handles::new_reference(object, ReferenceKind::Local)
}

pub(super) extern "system" fn new_global_ref(_env: *mut JNIEnv, object: jobject) -> jobject {
    handles::new_reference(object, ReferenceKind::Global)
}

pub(super) extern "system" fn new_weak_global_ref(_env: *mut JNIEnv, object: jobject) -> jweak {
    handles::new_reference(object, ReferenceKind::WeakGlobal)
}

pub(super) extern "system" fn delete_local_ref(_env: *mut JNIEnv, object: jobject) {
    handles::delete_reference(object, ReferenceKind::Local);
}

pub(super) extern "system" fn delete_global_ref(_env: *mut JNIEnv, object: jobject) {
    handles::delete_reference(object, ReferenceKind::Global);
}

pub(super) extern "system" fn delete_weak_global_ref(_env: *mut JNIEnv, object: jweak) {
    handles::delete_reference(object, ReferenceKind::WeakGlobal);
}
