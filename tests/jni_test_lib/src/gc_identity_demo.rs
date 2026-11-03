use jni::sys::{
    jboolean, jclass, jint, jobject, jobjectRefType, jstring, JNIEnv, JNI_ABORT, JNI_COMMIT,
    JNI_OK,
};
use std::cell::Cell;
use std::ptr::null_mut;

thread_local! {
    static RETURNED_GLOBAL: Cell<jobject> = const { Cell::new(null_mut()) };
    static STALE_LOCAL: Cell<jobject> = const { Cell::new(null_mut()) };
}

#[no_mangle]
pub extern "system" fn Java_samples_gc_GcJniIdentityDemo_aliases(
    env: *mut JNIEnv,
    _class: jclass,
    object: jobject,
) -> jobject {
    // SAFETY: the VM supplies this thread's live JNI table and valid object argument.
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
        let aliases = [
            object, locals[0], locals[1], globals[0], globals[1], weaks[0], weaks[1],
        ];
        let independent = aliases.iter().enumerate().all(|(index, alias)| {
            !alias.is_null() && aliases[..index].iter().all(|other| other != alias)
        });
        (jni.DeleteLocalRef)(env, locals[0]);
        (jni.DeleteGlobalRef)(env, globals[0]);
        (jni.DeleteWeakGlobalRef)(env, weaks[0]);
        let promoted = (jni.NewLocalRef)(env, weaks[1]);
        let valid = independent
            && matches!(
                (jni.GetObjectRefType)(env, locals[0]),
                jobjectRefType::JNIInvalidRefType
            )
            && matches!(
                (jni.GetObjectRefType)(env, globals[0]),
                jobjectRefType::JNIInvalidRefType
            )
            && matches!(
                (jni.GetObjectRefType)(env, weaks[0]),
                jobjectRefType::JNIInvalidRefType
            )
            && matches!(
                (jni.GetObjectRefType)(env, locals[1]),
                jobjectRefType::JNILocalRefType
            )
            && matches!(
                (jni.GetObjectRefType)(env, globals[1]),
                jobjectRefType::JNIGlobalRefType
            )
            && matches!(
                (jni.GetObjectRefType)(env, weaks[1]),
                jobjectRefType::JNIWeakGlobalRefType
            )
            && matches!(
                (jni.GetObjectRefType)(env, promoted),
                jobjectRefType::JNILocalRefType
            )
            && (jni.IsSameObject)(env, object, locals[1])
            && (jni.IsSameObject)(env, locals[1], globals[1])
            && (jni.IsSameObject)(env, globals[1], weaks[1])
            && (jni.IsSameObject)(env, promoted, object);
        (jni.DeleteWeakGlobalRef)(env, weaks[1]);
        (jni.DeleteLocalRef)(env, promoted);
        (jni.DeleteLocalRef)(env, locals[1]);
        if valid {
            RETURNED_GLOBAL.set(globals[1]);
            globals[1] // Deliberately return a global directly through libffi.
        } else {
            (jni.DeleteGlobalRef)(env, globals[1]);
            null_mut()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_gc_GcJniIdentityDemo_releaseGlobal(
    env: *mut JNIEnv,
    _class: jclass,
    expected: jobject,
) -> jboolean {
    unsafe {
        let jni = &(*(*env)).v24;
        let global = RETURNED_GLOBAL.replace(null_mut());
        let valid = (jni.IsSameObject)(env, global, expected);
        (jni.DeleteGlobalRef)(env, global);
        valid
            && matches!(
                (jni.GetObjectRefType)(env, global),
                jobjectRefType::JNIInvalidRefType
            )
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_gc_GcJniIdentityDemo_frames(
    env: *mut JNIEnv,
    _class: jclass,
    object: jobject,
) -> jobject {
    unsafe {
        let jni = &(*(*env)).v24;
        let outer = (jni.NewLocalRef)(env, object);
        let keep = (jni.NewLocalRef)(env, object);
        if (jni.PushLocalFrame)(env, 4) != JNI_OK {
            return null_mut();
        }
        let child = (jni.NewLocalRef)(env, object);
        (jni.DeleteLocalRef)(env, outer); // Exact deletion of a still-live outer-frame record.
        if (jni.PushLocalFrame)(env, 2) != JNI_OK {
            return null_mut();
        }
        let inner = (jni.NewLocalRef)(env, child);
        let escaped = (jni.PopLocalFrame)(env, inner);
        let valid = escaped != inner
            && escaped != child
            && matches!(
                (jni.GetObjectRefType)(env, inner),
                jobjectRefType::JNIInvalidRefType
            )
            && matches!(
                (jni.GetObjectRefType)(env, child),
                jobjectRefType::JNILocalRefType
            )
            && matches!(
                (jni.GetObjectRefType)(env, outer),
                jobjectRefType::JNIInvalidRefType
            )
            && (jni.IsSameObject)(env, escaped, keep);
        let result = (jni.PopLocalFrame)(env, escaped);
        let valid = valid
            && result != escaped
            && matches!(
                (jni.GetObjectRefType)(env, escaped),
                jobjectRefType::JNIInvalidRefType
            )
            && matches!(
                (jni.GetObjectRefType)(env, child),
                jobjectRefType::JNIInvalidRefType
            )
            && matches!(
                (jni.GetObjectRefType)(env, result),
                jobjectRefType::JNILocalRefType
            )
            && (jni.IsSameObject)(env, result, object);
        STALE_LOCAL.set(keep); // The next native invocation must see this as stale.
        if valid {
            result
        } else {
            null_mut()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_gc_GcJniIdentityDemo_staleLocal(
    env: *mut JNIEnv,
    _class: jclass,
) -> jboolean {
    unsafe {
        matches!(
            ((*(*env)).v24.GetObjectRefType)(env, STALE_LOCAL.replace(null_mut())),
            jobjectRefType::JNIInvalidRefType
        )
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_gc_GcJniIdentityDemo_capacity(
    env: *mut JNIEnv,
    _class: jclass,
    object: jobject,
    push: jboolean,
) -> jobject {
    unsafe {
        let jni = &(*(*env)).v24;
        if (jni.EnsureLocalCapacity)(env, 4) != JNI_OK {
            return null_mut();
        }
        if (jni.PushLocalFrame)(env, 4) != JNI_OK {
            return null_mut();
        }
        let local = (jni.NewLocalRef)(env, object);
        let status = if push {
            (jni.PushLocalFrame)(env, jint::MAX)
        } else {
            (jni.EnsureLocalCapacity)(env, jint::MAX)
        };
        let throwable = (jni.ExceptionOccurred)(env);
        let intact = status < 0
            && !throwable.is_null()
            && matches!(
                (jni.GetObjectRefType)(env, local),
                jobjectRefType::JNILocalRefType
            );
        (jni.ExceptionClear)(env);
        let result = (jni.PopLocalFrame)(env, throwable);
        if intact {
            result
        } else {
            null_mut()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_gc_GcJniIdentityDemo_buffers(
    env: *mut JNIEnv,
    _class: jclass,
    array: jobject,
    text: jstring,
) -> jboolean {
    unsafe {
        let jni = &(*(*env)).v24;
        let elements = (jni.GetIntArrayElements)(env, array, null_mut());
        if elements.is_null() {
            return false;
        }
        *elements = 41;
        (jni.ReleaseIntArrayElements)(env, array, elements, JNI_COMMIT);
        *elements = 99;
        (jni.ReleaseIntArrayElements)(env, array, elements, JNI_ABORT);
        let critical = (jni.GetPrimitiveArrayCritical)(env, array, null_mut());
        if critical.is_null() {
            return false;
        }
        *critical.cast::<jint>().add(1) = 42;
        (jni.ReleasePrimitiveArrayCritical)(env, array, critical, 0);
        let chars = (jni.GetStringChars)(env, text, null_mut());
        let utf = (jni.GetStringUTFChars)(env, text, null_mut());
        if chars.is_null() || utf.is_null() {
            return false;
        }
        let valid = *chars == u16::from(b'A') && *utf == 65;
        (jni.ReleaseStringChars)(env, text, chars);
        (jni.ReleaseStringUTFChars)(env, text, utf);
        valid
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_gc_GcJniIdentityDemo_callback(
    env: *mut JNIEnv,
    class: jclass,
) {
    unsafe {
        let jni = &(*(*env)).v24;
        let method = (jni.GetStaticMethodID)(env, class, c"fail".as_ptr(), c"()V".as_ptr());
        if method.is_null() {
            return;
        }
        let array = (jni.NewIntArray)(env, 1);
        // Intentionally leave both a nested frame and buffer outstanding on the exception path.
        (jni.GetIntArrayElements)(env, array, null_mut());
        (jni.PushLocalFrame)(env, 4);
        (jni.CallStaticVoidMethodA)(env, class, method, std::ptr::null());
    }
}
