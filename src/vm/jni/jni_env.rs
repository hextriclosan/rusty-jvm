use crate::vm::jni::array_operations_impl::{
    get_array_length, get_boolean_array_elements, get_boolean_array_region,
    get_byte_array_elements, get_byte_array_region, get_char_array_elements,
    get_char_array_region, get_double_array_elements, get_double_array_region,
    get_float_array_elements, get_float_array_region, get_int_array_elements,
    get_int_array_region, get_long_array_elements, get_long_array_region,
    get_object_array_element, get_short_array_elements, get_short_array_region, new_boolean_array,
    new_byte_array, new_char_array, new_double_array, new_float_array, new_int_array,
    new_long_array, new_object_array, new_short_array, release_boolean_array_elements,
    release_byte_array_elements, release_char_array_elements, release_double_array_elements,
    release_float_array_elements, release_int_array_elements, release_long_array_elements,
    release_short_array_elements, set_boolean_array_region, set_byte_array_region,
    set_char_array_region, set_double_array_region, set_float_array_region, set_int_array_region,
    set_long_array_region, set_object_array_element, set_short_array_region,
};
use crate::vm::jni::exception_impl::{exception_check, exception_occurred};
use crate::vm::jni::global_and_local_references_impl::{pop_local_frame, push_local_frame};
use crate::vm::jni::instance_methods_impl::{
    call_boolean_method_a, call_boolean_method_v, call_byte_method_a, call_byte_method_v,
    call_char_method_a, call_char_method_v, call_double_method_a, call_double_method_v,
    call_float_method_a, call_float_method_v, call_int_method_a, call_int_method_v,
    call_long_method_a, call_long_method_v, call_non_virtual_boolean_method_a,
    call_non_virtual_boolean_method_v, call_non_virtual_byte_method_a,
    call_non_virtual_byte_method_v, call_non_virtual_char_method_a, call_non_virtual_char_method_v,
    call_non_virtual_double_method_a, call_non_virtual_double_method_v,
    call_non_virtual_float_method_a, call_non_virtual_float_method_v,
    call_non_virtual_int_method_a, call_non_virtual_int_method_v, call_non_virtual_long_method_a,
    call_non_virtual_long_method_v, call_non_virtual_object_method_a,
    call_non_virtual_object_method_v, call_non_virtual_short_method_a,
    call_non_virtual_short_method_v, call_non_virtual_void_method_a, call_non_virtual_void_method_v,
    call_object_method_a, call_object_method_v, call_short_method_a, call_short_method_v,
    call_void_method_a, call_void_method_v, get_method_id,
};
use crate::vm::jni::java_vm_interface_impl::get_java_vm;
use crate::vm::jni::object_fields_impl::{
    get_boolean_field, get_byte_field, get_char_field, get_double_field, get_field_id,
    get_float_field, get_int_field, get_long_field, get_object_field, get_short_field,
    set_boolean_field, set_byte_field, set_char_field, set_double_field, set_float_field,
    set_int_field, set_long_field, set_object_field, set_short_field,
};
use crate::vm::jni::object_operations_impl::get_object_class;
use crate::vm::jni::static_fields_impl::{
    get_static_boolean_field, get_static_byte_field, get_static_char_field,
    get_static_double_field, get_static_field_id, get_static_float_field, get_static_int_field,
    get_static_long_field, get_static_object_field, get_static_short_field,
    set_static_boolean_field, set_static_byte_field, set_static_char_field,
    set_static_double_field, set_static_float_field, set_static_int_field, set_static_long_field,
    set_static_object_field, set_static_short_field,
};
use crate::vm::jni::static_methods_impl::{
    call_static_boolean_method_a, call_static_boolean_method_v, call_static_byte_method_a,
    call_static_byte_method_v, call_static_char_method_a, call_static_char_method_v,
    call_static_double_method_a, call_static_double_method_v, call_static_float_method_a,
    call_static_float_method_v, call_static_int_method_a, call_static_int_method_v,
    call_static_long_method_a, call_static_long_method_v, call_static_object_method_a,
    call_static_object_method_v, call_static_short_method_a, call_static_short_method_v,
    call_static_void_method_a, call_static_void_method_v, get_static_method_id,
};
use crate::vm::jni::string_operations_impl::{
    get_string_chars, get_string_length, get_string_utf_chars, get_string_utf_length,
    get_string_utf_length_as_long, new_string, new_string_utf8, release_string_chars,
    release_string_utf_chars,
};
use crate::vm::jni::version_information_impl::get_version;
use jni_sys::{
    jarray, jboolean, jbyte, jchar, jclass, jdouble, jfieldID, jfloat, jint, jlong, jmethodID,
    jobject, jobjectRefType, jshort, jsize, jstring, jthrowable, jvalue, jweak, va_list, JNIEnv,
    JNIInvokeInterface_, JNINativeInterface_, JNINativeMethod, JavaVM,
};
use std::ffi::{c_char, c_void};

pub(crate) fn get_jni_env() -> *mut JNIEnv {
    static mut ENV: *const JNINativeInterface_ = &VTABLE.0;
    std::ptr::addr_of_mut!(ENV).cast()
}

pub(crate) fn get_jni_vm() -> *mut JavaVM {
    static mut ENV: *const JNIInvokeInterface_ = &VTABLE.1;
    std::ptr::addr_of_mut!(ENV).cast()
}

macro_rules! jni_stub {
    ($name:ident ( $($arg:ty),*) -> $ret:ty) => {
        #[allow(non_snake_case)]
        unsafe extern "system" fn $name(
            _env: *mut JNIEnv,
            $(_: $arg),*
        ) -> $ret {
            unimplemented!(concat!(
                "JNI ",
                stringify!($name),
                "(",
                stringify!($($arg),*),
                ") -> ",
                stringify!($ret)
            ));
        }
    };
}

macro_rules! jni_vm_stub {
    ($name:ident ( $($arg:ty),*) -> $ret:ty) => {
        #[allow(non_snake_case)]
        unsafe extern "system" fn $name(
            _vm: *mut JavaVM,
            $(_: $arg),*
        ) -> $ret {
            unimplemented!(concat!(
                "JNI ",
                stringify!($name),
                "(",
                stringify!($($arg),*),
                ") -> ",
                stringify!($ret)
            ));
        }
    };
}

macro_rules! jni_variadic_stub {
    ($name:ident, $ptr:ident, ( $($arg:ty),* ) -> $ret:ty) => {
        #[allow(non_snake_case)]
        pub unsafe extern "system" fn $name(
            _env: *mut jni_sys::JNIEnv,
            $(_: $arg),*
        ) -> $ret {
            unimplemented!(concat!(
                "JNI ",
                stringify!($name),
                "(",
                stringify!($($arg),*),
                ") -> ",
                stringify!($ret)
            ));
        }

        #[allow(non_upper_case_globals, non_snake_case)]
        pub const $ptr: unsafe extern "C" fn(
            *mut jni_sys::JNIEnv,
            $($arg),*,
            ...
        ) -> $ret = unsafe {
            std::mem::transmute::<
                unsafe extern "system" fn(*mut jni_sys::JNIEnv, $($arg),*) -> $ret, unsafe extern "C" fn(*mut jni_sys::JNIEnv, $($arg),*, ...) -> $ret,
            >($name)
        };
    };
}

jni_stub!(DefineClass(*const c_char, jobject, *const jbyte, jsize) -> jclass);
jni_stub!(FindClass(*const c_char) -> jclass);
jni_stub!(FromReflectedMethod(jobject) -> jmethodID);
jni_stub!(FromReflectedField(jobject) -> jfieldID);
jni_stub!(ToReflectedMethod(jclass, jmethodID, jboolean) -> jobject);
jni_stub!(GetSuperclass(jclass) -> jclass);
jni_stub!(IsAssignableFrom(jclass, jclass) -> jboolean);
jni_stub!(ToReflectedField(jclass, jfieldID, jboolean) -> jobject);
jni_stub!(Throw(jthrowable) -> jint);
jni_stub!(ThrowNew(jclass, *const c_char) -> jint);
jni_stub!(ExceptionDescribe() -> ());
jni_stub!(ExceptionClear() -> ());
jni_stub!(FatalError(*const c_char) -> !);
jni_stub!(NewGlobalRef(jobject) -> jobject);
jni_stub!(DeleteGlobalRef(jobject) -> ());
jni_stub!(DeleteLocalRef(jobject) -> ());
jni_stub!(IsSameObject(jobject, jobject) -> jboolean);
jni_stub!(NewLocalRef(jobject) -> jobject);
jni_stub!(EnsureLocalCapacity(jint) -> jint);
jni_stub!(AllocObject(jclass) -> jobject);
jni_variadic_stub!(NewObject, NewObject_ptr, (jclass, jmethodID) -> jobject);
jni_stub!(NewObjectV(jclass, jmethodID, va_list) -> jobject);
jni_stub!(NewObjectA(jclass, jmethodID, *const jvalue) -> jobject);
jni_stub!(IsInstanceOf(jobject, jclass) -> jboolean);
jni_stub!(RegisterNatives(jclass, *const JNINativeMethod, jint) -> jint);
jni_stub!(UnregisterNatives(jclass) -> jint);
jni_stub!(MonitorEnter(jobject) -> jint);
jni_stub!(MonitorExit(jobject) -> jint);
jni_stub!(GetStringRegion(jstring, jsize, jsize, *mut jchar) -> ());
jni_stub!(GetStringUTFRegion(jstring, jsize, jsize, *mut c_char) -> ());
jni_stub!(GetPrimitiveArrayCritical(jarray, *mut jboolean) -> *mut c_void);
jni_stub!(ReleasePrimitiveArrayCritical(jarray, *mut c_void, jint) -> ());
jni_stub!(GetStringCritical(jstring, *mut jboolean) -> *const jchar);
jni_stub!(ReleaseStringCritical(jstring, *const jchar) -> ());
jni_stub!(NewWeakGlobalRef(jobject) -> jweak);
jni_stub!(DeleteWeakGlobalRef(jweak) -> ());
jni_stub!(NewDirectByteBuffer(*mut c_void, jlong) -> jobject);
jni_stub!(GetDirectBufferAddress(jobject) -> *mut c_void);
jni_stub!(GetDirectBufferCapacity(jobject) -> jlong);
jni_stub!(GetObjectRefType(jobject) -> jobjectRefType);
jni_stub!(GetModule(jclass) -> jobject);
jni_stub!(IsVirtualThread(jobject) -> jboolean);

jni_vm_stub!(DestroyJavaVM() -> jint);
jni_vm_stub!(AttachCurrentThread(*mut *mut c_void, *mut c_void) -> jint);
jni_vm_stub!(DetachCurrentThread() -> jint);
jni_vm_stub!(GetEnv(*mut *mut c_void, jint) -> jint);
jni_vm_stub!(AttachCurrentThreadAsDaemon(*mut *mut c_void, *mut c_void) -> jint);

// ── C shim declarations ───────────────────────────────────────────────────
//
// The variadic Call<Type>Method entry points are implemented as thin C shims
// compiled from call_method_shims.c.  Each shim captures the variadic args
// with va_start and delegates to the matching exported Rust V-variant symbol.
//
// The C functions are declared as non-variadic here and then transmuted to
// variadic pointers (the same pattern used by jni_variadic_stub! above).

extern "C" {
    // instance methods
    fn call_object_method_shim(env: *mut JNIEnv, obj: jobject, mid: jmethodID) -> jobject;
    fn call_boolean_method_shim(env: *mut JNIEnv, obj: jobject, mid: jmethodID) -> jboolean;
    fn call_byte_method_shim(env: *mut JNIEnv, obj: jobject, mid: jmethodID) -> jbyte;
    fn call_char_method_shim(env: *mut JNIEnv, obj: jobject, mid: jmethodID) -> jchar;
    fn call_short_method_shim(env: *mut JNIEnv, obj: jobject, mid: jmethodID) -> jshort;
    fn call_int_method_shim(env: *mut JNIEnv, obj: jobject, mid: jmethodID) -> jint;
    fn call_long_method_shim(env: *mut JNIEnv, obj: jobject, mid: jmethodID) -> jlong;
    fn call_float_method_shim(env: *mut JNIEnv, obj: jobject, mid: jmethodID) -> jfloat;
    fn call_double_method_shim(env: *mut JNIEnv, obj: jobject, mid: jmethodID) -> jdouble;
    fn call_void_method_shim(env: *mut JNIEnv, obj: jobject, mid: jmethodID);
    // non-virtual methods
    fn call_non_virtual_object_method_shim(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        mid: jmethodID,
    ) -> jobject;
    fn call_non_virtual_boolean_method_shim(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        mid: jmethodID,
    ) -> jboolean;
    fn call_non_virtual_byte_method_shim(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        mid: jmethodID,
    ) -> jbyte;
    fn call_non_virtual_char_method_shim(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        mid: jmethodID,
    ) -> jchar;
    fn call_non_virtual_short_method_shim(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        mid: jmethodID,
    ) -> jshort;
    fn call_non_virtual_int_method_shim(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        mid: jmethodID,
    ) -> jint;
    fn call_non_virtual_long_method_shim(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        mid: jmethodID,
    ) -> jlong;
    fn call_non_virtual_float_method_shim(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        mid: jmethodID,
    ) -> jfloat;
    fn call_non_virtual_double_method_shim(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        mid: jmethodID,
    ) -> jdouble;
    fn call_non_virtual_void_method_shim(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        mid: jmethodID,
    );
    // static methods
    fn call_static_object_method_shim(env: *mut JNIEnv, cls: jclass, mid: jmethodID) -> jobject;
    fn call_static_boolean_method_shim(env: *mut JNIEnv, cls: jclass, mid: jmethodID) -> jboolean;
    fn call_static_byte_method_shim(env: *mut JNIEnv, cls: jclass, mid: jmethodID) -> jbyte;
    fn call_static_char_method_shim(env: *mut JNIEnv, cls: jclass, mid: jmethodID) -> jchar;
    fn call_static_short_method_shim(env: *mut JNIEnv, cls: jclass, mid: jmethodID) -> jshort;
    fn call_static_int_method_shim(env: *mut JNIEnv, cls: jclass, mid: jmethodID) -> jint;
    fn call_static_long_method_shim(env: *mut JNIEnv, cls: jclass, mid: jmethodID) -> jlong;
    fn call_static_float_method_shim(env: *mut JNIEnv, cls: jclass, mid: jmethodID) -> jfloat;
    fn call_static_double_method_shim(env: *mut JNIEnv, cls: jclass, mid: jmethodID) -> jdouble;
    fn call_static_void_method_shim(env: *mut JNIEnv, cls: jclass, mid: jmethodID);
}

// Transmute each non-variadic C shim declaration to a variadic function pointer
// suitable for storage in the JNI vtable.  The C shims were compiled as true
// variadic functions, so this cast is safe from an ABI standpoint.
macro_rules! variadic_ptr {
    ($name:ident, $const_name:ident, ($($arg:ty),*) -> $ret:ty) => {
        #[allow(non_upper_case_globals)]
        const $const_name: unsafe extern "C" fn(*mut JNIEnv, $($arg),*, ...) -> $ret = unsafe {
            std::mem::transmute::<
                unsafe extern "C" fn(*mut JNIEnv, $($arg),*) -> $ret,
                unsafe extern "C" fn(*mut JNIEnv, $($arg),*, ...) -> $ret,
            >($name)
        };
    };
}

variadic_ptr!(call_object_method_shim,  CallObjectMethod_ptr,  (jobject, jmethodID) -> jobject);
variadic_ptr!(call_boolean_method_shim, CallBooleanMethod_ptr, (jobject, jmethodID) -> jboolean);
variadic_ptr!(call_byte_method_shim,    CallByteMethod_ptr,    (jobject, jmethodID) -> jbyte);
variadic_ptr!(call_char_method_shim,    CallCharMethod_ptr,    (jobject, jmethodID) -> jchar);
variadic_ptr!(call_short_method_shim,   CallShortMethod_ptr,   (jobject, jmethodID) -> jshort);
variadic_ptr!(call_int_method_shim,     CallIntMethod_ptr,     (jobject, jmethodID) -> jint);
variadic_ptr!(call_long_method_shim,    CallLongMethod_ptr,    (jobject, jmethodID) -> jlong);
variadic_ptr!(call_float_method_shim,   CallFloatMethod_ptr,   (jobject, jmethodID) -> jfloat);
variadic_ptr!(call_double_method_shim,  CallDoubleMethod_ptr,  (jobject, jmethodID) -> jdouble);
variadic_ptr!(call_void_method_shim,    CallVoidMethod_ptr,    (jobject, jmethodID) -> ());

variadic_ptr!(call_non_virtual_object_method_shim,  CallNonvirtualObjectMethod_ptr,  (jobject, jclass, jmethodID) -> jobject);
variadic_ptr!(call_non_virtual_boolean_method_shim, CallNonvirtualBooleanMethod_ptr, (jobject, jclass, jmethodID) -> jboolean);
variadic_ptr!(call_non_virtual_byte_method_shim,    CallNonvirtualByteMethod_ptr,    (jobject, jclass, jmethodID) -> jbyte);
variadic_ptr!(call_non_virtual_char_method_shim,    CallNonvirtualCharMethod_ptr,    (jobject, jclass, jmethodID) -> jchar);
variadic_ptr!(call_non_virtual_short_method_shim,   CallNonvirtualShortMethod_ptr,   (jobject, jclass, jmethodID) -> jshort);
variadic_ptr!(call_non_virtual_int_method_shim,     CallNonvirtualIntMethod_ptr,     (jobject, jclass, jmethodID) -> jint);
variadic_ptr!(call_non_virtual_long_method_shim,    CallNonvirtualLongMethod_ptr,    (jobject, jclass, jmethodID) -> jlong);
variadic_ptr!(call_non_virtual_float_method_shim,   CallNonvirtualFloatMethod_ptr,   (jobject, jclass, jmethodID) -> jfloat);
variadic_ptr!(call_non_virtual_double_method_shim,  CallNonvirtualDoubleMethod_ptr,  (jobject, jclass, jmethodID) -> jdouble);
variadic_ptr!(call_non_virtual_void_method_shim,    CallNonvirtualVoidMethod_ptr,    (jobject, jclass, jmethodID) -> ());

variadic_ptr!(call_static_object_method_shim,  CallStaticObjectMethod_ptr,  (jclass, jmethodID) -> jobject);
variadic_ptr!(call_static_boolean_method_shim, CallStaticBooleanMethod_ptr, (jclass, jmethodID) -> jboolean);
variadic_ptr!(call_static_byte_method_shim,    CallStaticByteMethod_ptr,    (jclass, jmethodID) -> jbyte);
variadic_ptr!(call_static_char_method_shim,    CallStaticCharMethod_ptr,    (jclass, jmethodID) -> jchar);
variadic_ptr!(call_static_short_method_shim,   CallStaticShortMethod_ptr,   (jclass, jmethodID) -> jshort);
variadic_ptr!(call_static_int_method_shim,     CallStaticIntMethod_ptr,     (jclass, jmethodID) -> jint);
variadic_ptr!(call_static_long_method_shim,    CallStaticLongMethod_ptr,    (jclass, jmethodID) -> jlong);
variadic_ptr!(call_static_float_method_shim,   CallStaticFloatMethod_ptr,   (jclass, jmethodID) -> jfloat);
variadic_ptr!(call_static_double_method_shim,  CallStaticDoubleMethod_ptr,  (jclass, jmethodID) -> jdouble);
variadic_ptr!(call_static_void_method_shim,    CallStaticVoidMethod_ptr,    (jclass, jmethodID) -> ());

struct Wrapper(JNINativeInterface_, JNIInvokeInterface_);
unsafe impl Sync for Wrapper {}
static VTABLE: Wrapper = {
    let mut ni: JNINativeInterface_ = unsafe { std::mem::zeroed() };
    ni.v24.GetVersion = get_version;
    ni.v24.DefineClass = DefineClass;
    ni.v24.FindClass = FindClass;
    ni.v24.FromReflectedMethod = FromReflectedMethod;
    ni.v24.FromReflectedField = FromReflectedField;
    ni.v24.ToReflectedMethod = ToReflectedMethod;
    ni.v24.GetSuperclass = GetSuperclass;
    ni.v24.IsAssignableFrom = IsAssignableFrom;
    ni.v24.ToReflectedField = ToReflectedField;
    ni.v24.Throw = Throw;
    ni.v24.ThrowNew = ThrowNew;
    ni.v24.ExceptionOccurred = exception_occurred;
    ni.v24.ExceptionDescribe = ExceptionDescribe;
    ni.v24.ExceptionClear = ExceptionClear;
    ni.v24.FatalError = FatalError;
    ni.v24.PushLocalFrame = push_local_frame;
    ni.v24.PopLocalFrame = pop_local_frame;
    ni.v24.NewGlobalRef = NewGlobalRef;
    ni.v24.DeleteGlobalRef = DeleteGlobalRef;
    ni.v24.DeleteLocalRef = DeleteLocalRef;
    ni.v24.IsSameObject = IsSameObject;
    ni.v24.NewLocalRef = NewLocalRef;
    ni.v24.EnsureLocalCapacity = EnsureLocalCapacity;
    ni.v24.AllocObject = AllocObject;
    ni.v24.NewObject = NewObject_ptr;
    ni.v24.NewObjectV = NewObjectV;
    ni.v24.NewObjectA = NewObjectA;
    ni.v24.GetObjectClass = get_object_class;
    ni.v24.IsInstanceOf = IsInstanceOf;
    ni.v24.GetMethodID = get_method_id;
    ni.v24.CallObjectMethod = CallObjectMethod_ptr;
    ni.v24.CallObjectMethodV = call_object_method_v;
    ni.v24.CallObjectMethodA = call_object_method_a;
    ni.v24.CallBooleanMethod = CallBooleanMethod_ptr;
    ni.v24.CallBooleanMethodV = call_boolean_method_v;
    ni.v24.CallBooleanMethodA = call_boolean_method_a;
    ni.v24.CallByteMethod = CallByteMethod_ptr;
    ni.v24.CallByteMethodV = call_byte_method_v;
    ni.v24.CallByteMethodA = call_byte_method_a;
    ni.v24.CallCharMethod = CallCharMethod_ptr;
    ni.v24.CallCharMethodV = call_char_method_v;
    ni.v24.CallCharMethodA = call_char_method_a;
    ni.v24.CallShortMethod = CallShortMethod_ptr;
    ni.v24.CallShortMethodV = call_short_method_v;
    ni.v24.CallShortMethodA = call_short_method_a;
    ni.v24.CallIntMethod = CallIntMethod_ptr;
    ni.v24.CallIntMethodV = call_int_method_v;
    ni.v24.CallIntMethodA = call_int_method_a;
    ni.v24.CallLongMethod = CallLongMethod_ptr;
    ni.v24.CallLongMethodV = call_long_method_v;
    ni.v24.CallLongMethodA = call_long_method_a;
    ni.v24.CallFloatMethod = CallFloatMethod_ptr;
    ni.v24.CallFloatMethodV = call_float_method_v;
    ni.v24.CallFloatMethodA = call_float_method_a;
    ni.v24.CallDoubleMethod = CallDoubleMethod_ptr;
    ni.v24.CallDoubleMethodV = call_double_method_v;
    ni.v24.CallDoubleMethodA = call_double_method_a;
    ni.v24.CallVoidMethod = CallVoidMethod_ptr;
    ni.v24.CallVoidMethodV = call_void_method_v;
    ni.v24.CallVoidMethodA = call_void_method_a;
    ni.v24.CallNonvirtualObjectMethod = CallNonvirtualObjectMethod_ptr;
    ni.v24.CallNonvirtualObjectMethodV = call_non_virtual_object_method_v;
    ni.v24.CallNonvirtualObjectMethodA = call_non_virtual_object_method_a;
    ni.v24.CallNonvirtualBooleanMethod = CallNonvirtualBooleanMethod_ptr;
    ni.v24.CallNonvirtualBooleanMethodV = call_non_virtual_boolean_method_v;
    ni.v24.CallNonvirtualBooleanMethodA = call_non_virtual_boolean_method_a;
    ni.v24.CallNonvirtualByteMethod = CallNonvirtualByteMethod_ptr;
    ni.v24.CallNonvirtualByteMethodV = call_non_virtual_byte_method_v;
    ni.v24.CallNonvirtualByteMethodA = call_non_virtual_byte_method_a;
    ni.v24.CallNonvirtualCharMethod = CallNonvirtualCharMethod_ptr;
    ni.v24.CallNonvirtualCharMethodV = call_non_virtual_char_method_v;
    ni.v24.CallNonvirtualCharMethodA = call_non_virtual_char_method_a;
    ni.v24.CallNonvirtualShortMethod = CallNonvirtualShortMethod_ptr;
    ni.v24.CallNonvirtualShortMethodV = call_non_virtual_short_method_v;
    ni.v24.CallNonvirtualShortMethodA = call_non_virtual_short_method_a;
    ni.v24.CallNonvirtualIntMethod = CallNonvirtualIntMethod_ptr;
    ni.v24.CallNonvirtualIntMethodV = call_non_virtual_int_method_v;
    ni.v24.CallNonvirtualIntMethodA = call_non_virtual_int_method_a;
    ni.v24.CallNonvirtualLongMethod = CallNonvirtualLongMethod_ptr;
    ni.v24.CallNonvirtualLongMethodV = call_non_virtual_long_method_v;
    ni.v24.CallNonvirtualLongMethodA = call_non_virtual_long_method_a;
    ni.v24.CallNonvirtualFloatMethod = CallNonvirtualFloatMethod_ptr;
    ni.v24.CallNonvirtualFloatMethodV = call_non_virtual_float_method_v;
    ni.v24.CallNonvirtualFloatMethodA = call_non_virtual_float_method_a;
    ni.v24.CallNonvirtualDoubleMethod = CallNonvirtualDoubleMethod_ptr;
    ni.v24.CallNonvirtualDoubleMethodV = call_non_virtual_double_method_v;
    ni.v24.CallNonvirtualDoubleMethodA = call_non_virtual_double_method_a;
    ni.v24.CallNonvirtualVoidMethod = CallNonvirtualVoidMethod_ptr;
    ni.v24.CallNonvirtualVoidMethodV = call_non_virtual_void_method_v;
    ni.v24.CallNonvirtualVoidMethodA = call_non_virtual_void_method_a;
    ni.v24.GetFieldID = get_field_id;
    ni.v24.GetObjectField = get_object_field;
    ni.v24.GetBooleanField = get_boolean_field;
    ni.v24.GetByteField = get_byte_field;
    ni.v24.GetCharField = get_char_field;
    ni.v24.GetShortField = get_short_field;
    ni.v24.GetIntField = get_int_field;
    ni.v24.GetLongField = get_long_field;
    ni.v24.GetFloatField = get_float_field;
    ni.v24.GetDoubleField = get_double_field;
    ni.v24.SetObjectField = set_object_field;
    ni.v24.SetBooleanField = set_boolean_field;
    ni.v24.SetByteField = set_byte_field;
    ni.v24.SetCharField = set_char_field;
    ni.v24.SetShortField = set_short_field;
    ni.v24.SetIntField = set_int_field;
    ni.v24.SetLongField = set_long_field;
    ni.v24.SetFloatField = set_float_field;
    ni.v24.SetDoubleField = set_double_field;
    ni.v24.GetStaticMethodID = get_static_method_id;
    ni.v24.CallStaticObjectMethod = CallStaticObjectMethod_ptr;
    ni.v24.CallStaticObjectMethodV = call_static_object_method_v;
    ni.v24.CallStaticObjectMethodA = call_static_object_method_a;
    ni.v24.CallStaticBooleanMethod = CallStaticBooleanMethod_ptr;
    ni.v24.CallStaticBooleanMethodV = call_static_boolean_method_v;
    ni.v24.CallStaticBooleanMethodA = call_static_boolean_method_a;
    ni.v24.CallStaticByteMethod = CallStaticByteMethod_ptr;
    ni.v24.CallStaticByteMethodV = call_static_byte_method_v;
    ni.v24.CallStaticByteMethodA = call_static_byte_method_a;
    ni.v24.CallStaticCharMethod = CallStaticCharMethod_ptr;
    ni.v24.CallStaticCharMethodV = call_static_char_method_v;
    ni.v24.CallStaticCharMethodA = call_static_char_method_a;
    ni.v24.CallStaticShortMethod = CallStaticShortMethod_ptr;
    ni.v24.CallStaticShortMethodV = call_static_short_method_v;
    ni.v24.CallStaticShortMethodA = call_static_short_method_a;
    ni.v24.CallStaticIntMethod = CallStaticIntMethod_ptr;
    ni.v24.CallStaticIntMethodV = call_static_int_method_v;
    ni.v24.CallStaticIntMethodA = call_static_int_method_a;
    ni.v24.CallStaticLongMethod = CallStaticLongMethod_ptr;
    ni.v24.CallStaticLongMethodV = call_static_long_method_v;
    ni.v24.CallStaticLongMethodA = call_static_long_method_a;
    ni.v24.CallStaticFloatMethod = CallStaticFloatMethod_ptr;
    ni.v24.CallStaticFloatMethodV = call_static_float_method_v;
    ni.v24.CallStaticFloatMethodA = call_static_float_method_a;
    ni.v24.CallStaticDoubleMethod = CallStaticDoubleMethod_ptr;
    ni.v24.CallStaticDoubleMethodV = call_static_double_method_v;
    ni.v24.CallStaticDoubleMethodA = call_static_double_method_a;
    ni.v24.CallStaticVoidMethod = CallStaticVoidMethod_ptr;
    ni.v24.CallStaticVoidMethodV = call_static_void_method_v;
    ni.v24.CallStaticVoidMethodA = call_static_void_method_a;
    ni.v24.GetStaticFieldID = get_static_field_id;
    ni.v24.GetStaticObjectField = get_static_object_field;
    ni.v24.GetStaticBooleanField = get_static_boolean_field;
    ni.v24.GetStaticByteField = get_static_byte_field;
    ni.v24.GetStaticCharField = get_static_char_field;
    ni.v24.GetStaticShortField = get_static_short_field;
    ni.v24.GetStaticIntField = get_static_int_field;
    ni.v24.GetStaticLongField = get_static_long_field;
    ni.v24.GetStaticFloatField = get_static_float_field;
    ni.v24.GetStaticDoubleField = get_static_double_field;
    ni.v24.SetStaticObjectField = set_static_object_field;
    ni.v24.SetStaticBooleanField = set_static_boolean_field;
    ni.v24.SetStaticByteField = set_static_byte_field;
    ni.v24.SetStaticCharField = set_static_char_field;
    ni.v24.SetStaticShortField = set_static_short_field;
    ni.v24.SetStaticIntField = set_static_int_field;
    ni.v24.SetStaticLongField = set_static_long_field;
    ni.v24.SetStaticFloatField = set_static_float_field;
    ni.v24.SetStaticDoubleField = set_static_double_field;
    ni.v24.NewString = new_string;
    ni.v24.GetStringLength = get_string_length;
    ni.v24.GetStringChars = get_string_chars;
    ni.v24.ReleaseStringChars = release_string_chars;
    ni.v24.NewStringUTF = new_string_utf8;
    ni.v24.GetStringUTFLength = get_string_utf_length;
    ni.v24.GetStringUTFChars = get_string_utf_chars;
    ni.v24.ReleaseStringUTFChars = release_string_utf_chars;
    ni.v24.GetArrayLength = get_array_length;
    ni.v24.NewObjectArray = new_object_array;
    ni.v24.GetObjectArrayElement = get_object_array_element;
    ni.v24.SetObjectArrayElement = set_object_array_element;
    ni.v24.NewBooleanArray = new_boolean_array;
    ni.v24.NewByteArray = new_byte_array;
    ni.v24.NewCharArray = new_char_array;
    ni.v24.NewShortArray = new_short_array;
    ni.v24.NewIntArray = new_int_array;
    ni.v24.NewLongArray = new_long_array;
    ni.v24.NewFloatArray = new_float_array;
    ni.v24.NewDoubleArray = new_double_array;
    ni.v24.GetBooleanArrayElements = get_boolean_array_elements;
    ni.v24.GetByteArrayElements = get_byte_array_elements;
    ni.v24.GetCharArrayElements = get_char_array_elements;
    ni.v24.GetShortArrayElements = get_short_array_elements;
    ni.v24.GetIntArrayElements = get_int_array_elements;
    ni.v24.GetLongArrayElements = get_long_array_elements;
    ni.v24.GetFloatArrayElements = get_float_array_elements;
    ni.v24.GetDoubleArrayElements = get_double_array_elements;
    ni.v24.ReleaseBooleanArrayElements = release_boolean_array_elements;
    ni.v24.ReleaseByteArrayElements = release_byte_array_elements;
    ni.v24.ReleaseCharArrayElements = release_char_array_elements;
    ni.v24.ReleaseShortArrayElements = release_short_array_elements;
    ni.v24.ReleaseIntArrayElements = release_int_array_elements;
    ni.v24.ReleaseLongArrayElements = release_long_array_elements;
    ni.v24.ReleaseFloatArrayElements = release_float_array_elements;
    ni.v24.ReleaseDoubleArrayElements = release_double_array_elements;
    ni.v24.GetBooleanArrayRegion = get_boolean_array_region;
    ni.v24.GetByteArrayRegion = get_byte_array_region;
    ni.v24.GetCharArrayRegion = get_char_array_region;
    ni.v24.GetShortArrayRegion = get_short_array_region;
    ni.v24.GetIntArrayRegion = get_int_array_region;
    ni.v24.GetLongArrayRegion = get_long_array_region;
    ni.v24.GetFloatArrayRegion = get_float_array_region;
    ni.v24.GetDoubleArrayRegion = get_double_array_region;
    ni.v24.SetBooleanArrayRegion = set_boolean_array_region;
    ni.v24.SetByteArrayRegion = set_byte_array_region;
    ni.v24.SetCharArrayRegion = set_char_array_region;
    ni.v24.SetShortArrayRegion = set_short_array_region;
    ni.v24.SetIntArrayRegion = set_int_array_region;
    ni.v24.SetLongArrayRegion = set_long_array_region;
    ni.v24.SetFloatArrayRegion = set_float_array_region;
    ni.v24.SetDoubleArrayRegion = set_double_array_region;
    ni.v24.RegisterNatives = RegisterNatives;
    ni.v24.UnregisterNatives = UnregisterNatives;
    ni.v24.MonitorEnter = MonitorEnter;
    ni.v24.MonitorExit = MonitorExit;
    ni.v24.GetJavaVM = get_java_vm;
    ni.v24.GetStringRegion = GetStringRegion;
    ni.v24.GetStringUTFRegion = GetStringUTFRegion;
    ni.v24.GetPrimitiveArrayCritical = GetPrimitiveArrayCritical;
    ni.v24.ReleasePrimitiveArrayCritical = ReleasePrimitiveArrayCritical;
    ni.v24.GetStringCritical = GetStringCritical;
    ni.v24.ReleaseStringCritical = ReleaseStringCritical;
    ni.v24.NewWeakGlobalRef = NewWeakGlobalRef;
    ni.v24.DeleteWeakGlobalRef = DeleteWeakGlobalRef;
    ni.v24.ExceptionCheck = exception_check;
    ni.v24.NewDirectByteBuffer = NewDirectByteBuffer;
    ni.v24.GetDirectBufferAddress = GetDirectBufferAddress;
    ni.v24.GetDirectBufferCapacity = GetDirectBufferCapacity;
    ni.v24.GetObjectRefType = GetObjectRefType;
    ni.v24.GetModule = GetModule;
    ni.v24.IsVirtualThread = IsVirtualThread;
    ni.v24.GetStringUTFLengthAsLong = get_string_utf_length_as_long;

    let mut ii: JNIInvokeInterface_ = unsafe { std::mem::zeroed() };
    ii.v1_4.DestroyJavaVM = DestroyJavaVM;
    ii.v1_4.AttachCurrentThread = AttachCurrentThread;
    ii.v1_4.DetachCurrentThread = DetachCurrentThread;
    ii.v1_4.GetEnv = GetEnv;
    ii.v1_4.AttachCurrentThreadAsDaemon = AttachCurrentThreadAsDaemon;

    Wrapper(ni, ii)
};
