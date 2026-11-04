use jni::sys::{
    jboolean, jclass, jintArray, jobject, jstring, JNIEnv, JNI_ABORT, JNI_COMMIT, JNI_TRUE,
};
use std::ffi::CStr;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex, OnceLock};
use std::thread;
use std::time::Duration;

static LEASE_RELEASE: OnceLock<Mutex<Option<Sender<()>>>> = OnceLock::new();

#[no_mangle]
pub extern "system" fn Java_samples_gc_GcJniDemo_aliasesAndPromotes(
    env: *mut JNIEnv,
    _class: jclass,
    object: jobject,
) -> jboolean {
    unsafe {
        let jni = &(*(*env)).v24;
        let locals = [
            (jni.NewLocalRef)(env, object),
            (jni.NewLocalRef)(env, object),
        ];
        let globals = [
            (jni.NewGlobalRef)(env, object),
            (jni.NewGlobalRef)(env, object),
        ];
        let weaks = [
            (jni.NewWeakGlobalRef)(env, object),
            (jni.NewWeakGlobalRef)(env, object),
        ];
        let promoted = (jni.NewLocalRef)(env, weaks[1]);
        let aliases = [
            object, locals[0], locals[1], globals[0], globals[1], weaks[0], weaks[1], promoted,
        ];
        let independent = aliases.iter().enumerate().all(|(index, alias)| {
            !alias.is_null() && aliases[..index].iter().all(|other| other != alias)
        });
        let valid = independent
            && (jni.IsSameObject)(env, object, promoted)
            && (jni.IsSameObject)(env, object, globals[1])
            && (jni.IsSameObject)(env, object, weaks[1]);
        for local in locals {
            (jni.DeleteLocalRef)(env, local);
        }
        for global in globals {
            (jni.DeleteGlobalRef)(env, global);
        }
        for weak in weaks {
            (jni.DeleteWeakGlobalRef)(env, weak);
        }
        (jni.DeleteLocalRef)(env, promoted);
        valid as jboolean
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_gc_GcJniDemo_nestedFrameEscape(
    env: *mut JNIEnv,
    _class: jclass,
    object: jobject,
) -> jobject {
    unsafe {
        let jni = &(*(*env)).v24;
        let keep = (jni.NewLocalRef)(env, object);
        if (jni.PushLocalFrame)(env, 4) != 0 {
            return std::ptr::null_mut();
        }
        let outer = (jni.NewLocalRef)(env, object);
        if (jni.PushLocalFrame)(env, 2) != 0 {
            (jni.PopLocalFrame)(env, std::ptr::null_mut());
            return std::ptr::null_mut();
        }
        let inner = (jni.NewLocalRef)(env, object);
        let escaped = (jni.PopLocalFrame)(env, inner);
        let nested_ok = escaped != inner
            && matches!(
                (jni.GetObjectRefType)(env, inner),
                jni::sys::jobjectRefType::JNIInvalidRefType
            )
            && (jni.IsSameObject)(env, escaped, keep);
        let result = (jni.PopLocalFrame)(env, escaped);
        let valid = nested_ok
            && result != escaped
            && matches!(
                (jni.GetObjectRefType)(env, escaped),
                jni::sys::jobjectRefType::JNIInvalidRefType
            )
            && matches!(
                (jni.GetObjectRefType)(env, outer),
                jni::sys::jobjectRefType::JNIInvalidRefType
            )
            && (jni.IsSameObject)(env, result, object);
        (jni.DeleteLocalRef)(env, keep);
        if valid {
            result
        } else {
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_gc_GcJniDemo_returnGlobal(
    env: *mut JNIEnv,
    _class: jclass,
    object: jobject,
) -> jobject {
    unsafe { ((*(*env)).v24.NewGlobalRef)(env, object) }
}

#[no_mangle]
pub extern "system" fn Java_samples_gc_GcJniDemo_deleteGlobal(
    env: *mut JNIEnv,
    _class: jclass,
    global: jobject,
) {
    unsafe { ((*(*env)).v24.DeleteGlobalRef)(env, global) }
}

#[no_mangle]
pub extern "system" fn Java_samples_gc_GcJniDemo_returnWeak(
    env: *mut JNIEnv,
    _class: jclass,
    object: jobject,
) -> jobject {
    unsafe { ((*(*env)).v24.NewWeakGlobalRef)(env, object) }
}

#[no_mangle]
pub extern "system" fn Java_samples_gc_GcJniDemo_deleteWeak(
    env: *mut JNIEnv,
    _class: jclass,
    weak: jobject,
) {
    unsafe { ((*(*env)).v24.DeleteWeakGlobalRef)(env, weak) }
}

#[no_mangle]
pub extern "system" fn Java_samples_gc_GcJniDemo_createString(
    env: *mut JNIEnv,
    _class: jclass,
) -> jstring {
    let value = c"native-created-across-gc";
    unsafe { ((*(*env)).v24.NewStringUTF)(env, value.as_ptr()) }
}

#[no_mangle]
pub extern "system" fn Java_samples_gc_GcJniDemo_findMissingClass(
    env: *mut JNIEnv,
    _class: jclass,
) -> jclass {
    unsafe {
        ((*(*env)).v24.FindClass)(env, c"samples/gc/GcJniDemo$MissingForJniFindClass".as_ptr())
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_gc_GcJniDemo_stringLeaseAcrossGc(
    env: *mut JNIEnv,
    _class: jclass,
    string: jstring,
) -> jboolean {
    unsafe {
        let jni = &(*(*env)).v24;
        let chars = (jni.GetStringUTFChars)(env, string, std::ptr::null_mut());
        if chars.is_null() {
            return false;
        }
        let system = (jni.FindClass)(env, c"java/lang/System".as_ptr());
        if system.is_null() {
            (jni.ReleaseStringUTFChars)(env, string, chars);
            return false;
        }
        let gc = (jni.GetStaticMethodID)(env, system, c"gc".as_ptr(), c"()V".as_ptr());
        if !gc.is_null() {
            (jni.CallStaticVoidMethodA)(env, system, gc, std::ptr::null());
        }
        let valid =
            !gc.is_null() && CStr::from_ptr(chars).to_bytes() == b"leased-string-across-gc";
        (jni.ReleaseStringUTFChars)(env, string, chars);
        (jni.DeleteLocalRef)(env, system);
        valid
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_gc_GcJniDemo_invokeCallback(
    env: *mut JNIEnv,
    _class: jclass,
    callback: jobject,
) {
    unsafe {
        let jni = &(*(*env)).v24;
        let class = (jni.GetObjectClass)(env, callback);
        let method = (jni.GetMethodID)(env, class, c"run".as_ptr(), c"()V".as_ptr());
        if !method.is_null() {
            (jni.CallVoidMethodA)(env, callback, method, std::ptr::null());
        }
        (jni.DeleteLocalRef)(env, class);
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_gc_GcJniDemo_holdCopiedBufferAcrossGc(
    env: *mut JNIEnv,
    _class: jclass,
    array: jintArray,
    ready: jobject,
) -> jboolean {
    unsafe {
        let jni = &(*(*env)).v24;
        let mut is_copy = false;
        let elements = (jni.GetIntArrayElements)(env, array, &mut is_copy);
        if elements.is_null() {
            return false;
        }

        let (release_tx, release_rx) = mpsc::channel();
        let (done_tx, done_rx) = mpsc::channel();
        let pointer = Arc::new(AtomicPtr::new(elements));
        let holder_pointer = Arc::clone(&pointer);
        let holder = thread::spawn(move || {
            if release_rx.recv_timeout(Duration::from_secs(30)).is_ok() {
                holder_pointer.load(Ordering::Relaxed).write(77);
                let _ = done_tx.send(());
            }
        });
        let sender = LEASE_RELEASE.get_or_init(|| Mutex::new(None));
        *sender.lock().unwrap_or_else(|error| error.into_inner()) = Some(release_tx);

        let ready_class = (jni.GetObjectClass)(env, ready);
        let count_down =
            (jni.GetMethodID)(env, ready_class, c"countDown".as_ptr(), c"()V".as_ptr());
        if !count_down.is_null() {
            (jni.CallVoidMethodA)(env, ready, count_down, std::ptr::null());
        }
        (jni.DeleteLocalRef)(env, ready_class);
        let finished = done_rx.recv_timeout(Duration::from_secs(30)).is_ok();
        if !finished {
            if let Some(sender) = sender
                .lock()
                .unwrap_or_else(|error| error.into_inner())
                .take()
            {
                let _ = sender.send(());
            }
        }
        let _ = holder.join();
        (jni.ReleaseIntArrayElements)(env, array, elements, 0);
        finished
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_gc_GcJniDemo_releaseHeldBuffer(
    _env: *mut JNIEnv,
    _class: jclass,
) {
    if let Some(sender) = LEASE_RELEASE.get() {
        if let Some(sender) = sender
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .take()
        {
            let _ = sender.send(());
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_gc_GcJniDemo_exerciseReleaseModes(
    env: *mut JNIEnv,
    _class: jclass,
    array: jintArray,
) -> jboolean {
    unsafe {
        let jni = &(*(*env)).v24;
        let elements = (jni.GetIntArrayElements)(env, array, std::ptr::null_mut());
        if elements.is_null() {
            return false;
        }
        elements.write(10);
        (jni.ReleaseIntArrayElements)(env, array, elements, JNI_COMMIT);
        elements.add(1).write(20);
        (jni.ReleaseIntArrayElements)(env, array, elements, JNI_ABORT);

        let elements = (jni.GetIntArrayElements)(env, array, std::ptr::null_mut());
        if elements.is_null() {
            return false;
        }
        elements.write(30);
        (jni.ReleaseIntArrayElements)(env, array, elements, 0);
        JNI_TRUE
    }
}
