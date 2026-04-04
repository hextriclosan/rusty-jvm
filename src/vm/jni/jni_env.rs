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
use crate::vm::jni::jni_impl::{exception_check, get_java_vm, get_version};
use crate::vm::jni::string_operations_impl::{
    get_string_chars, get_string_length, new_string, release_string_chars,
};
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
jni_stub!(ExceptionOccurred() -> jthrowable);
jni_stub!(ExceptionDescribe() -> ());
jni_stub!(ExceptionClear() -> ());
jni_stub!(FatalError(*const c_char) -> !);
jni_stub!(PushLocalFrame(jint) -> jint);
jni_stub!(PopLocalFrame(jobject) -> jobject);
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
jni_stub!(GetObjectClass(jobject) -> jclass);
jni_stub!(IsInstanceOf(jobject, jclass) -> jboolean);
jni_stub!(GetMethodID(jclass, *const c_char, *const c_char) -> jmethodID);
jni_variadic_stub!(CallObjectMethod, CallObjectMethod_ptr, (jobject, jmethodID) -> jobject);
jni_stub!(CallObjectMethodV(jobject, jmethodID, va_list) -> jobject);
jni_stub!(CallObjectMethodA(jobject, jmethodID, *const jvalue) -> jobject);
jni_variadic_stub!(CallBooleanMethod, CallBooleanMethod_ptr, (jobject, jmethodID) -> jboolean);
jni_stub!(CallBooleanMethodV(jobject, jmethodID, va_list) -> jboolean);
jni_stub!(CallBooleanMethodA(jobject, jmethodID, *const jvalue) -> jboolean);
jni_variadic_stub!(CallByteMethod, CallByteMethod_ptr, (jobject, jmethodID) -> jbyte);
jni_stub!(CallByteMethodV(jobject, jmethodID, va_list) -> jbyte);
jni_stub!(CallByteMethodA(jobject, jmethodID, *const jvalue) -> jbyte);
jni_variadic_stub!(CallCharMethod, CallCharMethod_ptr, (jobject, jmethodID) -> jchar);
jni_stub!(CallCharMethodV(jobject, jmethodID, va_list) -> jchar);
jni_stub!(CallCharMethodA(jobject, jmethodID, *const jvalue) -> jchar);
jni_variadic_stub!(CallShortMethod, CallShortMethod_ptr, (jobject, jmethodID) -> jshort);
jni_stub!(CallShortMethodV(jobject, jmethodID, va_list) -> jshort);
jni_stub!(CallShortMethodA(jobject, jmethodID, *const jvalue) -> jshort);
jni_variadic_stub!(CallIntMethod, CallIntMethod_ptr, (jobject, jmethodID) -> jint);
jni_stub!(CallIntMethodV(jobject, jmethodID, va_list) -> jint);
jni_stub!(CallIntMethodA(jobject, jmethodID, *const jvalue) -> jint);
jni_variadic_stub!(CallLongMethod, CallLongMethod_ptr, (jobject, jmethodID) -> jlong);
jni_stub!(CallLongMethodV(jobject, jmethodID, va_list) -> jlong);
jni_stub!(CallLongMethodA(jobject, jmethodID, *const jvalue) -> jlong);
jni_variadic_stub!(CallFloatMethod, CallFloatMethod_ptr, (jobject, jmethodID) -> jfloat);
jni_stub!(CallFloatMethodV(jobject, jmethodID, va_list) -> jfloat);
jni_stub!(CallFloatMethodA(jobject, jmethodID, *const jvalue) -> jfloat);
jni_variadic_stub!(CallDoubleMethod, CallDoubleMethod_ptr, (jobject, jmethodID) -> jdouble);
jni_stub!(CallDoubleMethodV(jobject, jmethodID, va_list) -> jdouble);
jni_stub!(CallDoubleMethodA(jobject, jmethodID, *const jvalue) -> jdouble);
jni_variadic_stub!(CallVoidMethod, CallVoidMethod_ptr, (jobject, jmethodID) -> ());
jni_stub!(CallVoidMethodV(jobject, jmethodID, va_list) -> ());
jni_stub!(CallVoidMethodA(jobject, jmethodID, *const jvalue) -> ());
jni_variadic_stub!(CallNonvirtualObjectMethod, CallNonvirtualObjectMethod_ptr, (jobject, jclass, jmethodID) -> jobject);
jni_stub!(CallNonvirtualObjectMethodV(jobject, jclass, jmethodID, va_list) -> jobject);
jni_stub!(CallNonvirtualObjectMethodA(jobject, jclass, jmethodID, *const jvalue) -> jobject);
jni_variadic_stub!(CallNonvirtualBooleanMethod, CallNonvirtualBooleanMethod_ptr, (jobject, jclass, jmethodID) -> jboolean);
jni_stub!(CallNonvirtualBooleanMethodV(jobject, jclass, jmethodID, va_list) -> jboolean);
jni_stub!(CallNonvirtualBooleanMethodA(jobject, jclass, jmethodID, *const jvalue) -> jboolean);
jni_variadic_stub!(CallNonvirtualByteMethod, CallNonvirtualByteMethod_ptr, (jobject, jclass, jmethodID) -> jbyte);
jni_stub!(CallNonvirtualByteMethodV(jobject, jclass, jmethodID, va_list) -> jbyte);
jni_stub!(CallNonvirtualByteMethodA(jobject, jclass, jmethodID, *const jvalue) -> jbyte);
jni_variadic_stub!(CallNonvirtualCharMethod, CallNonvirtualCharMethod_ptr, (jobject, jclass, jmethodID) -> jchar);
jni_stub!(CallNonvirtualCharMethodV(jobject, jclass, jmethodID, va_list) -> jchar);
jni_stub!(CallNonvirtualCharMethodA(jobject, jclass, jmethodID, *const jvalue) -> jchar);
jni_variadic_stub!(CallNonvirtualShortMethod, CallNonvirtualShortMethod_ptr, (jobject, jclass, jmethodID) -> jshort);
jni_stub!(CallNonvirtualShortMethodV(jobject, jclass, jmethodID, va_list) -> jshort);
jni_stub!(CallNonvirtualShortMethodA(jobject, jclass, jmethodID, *const jvalue) -> jshort);
jni_variadic_stub!(CallNonvirtualIntMethod, CallNonvirtualIntMethod_ptr, (jobject, jclass, jmethodID) -> jint);
jni_stub!(CallNonvirtualIntMethodV(jobject, jclass, jmethodID, va_list) -> jint);
jni_stub!(CallNonvirtualIntMethodA(jobject, jclass, jmethodID, *const jvalue) -> jint);
jni_variadic_stub!(CallNonvirtualLongMethod, CallNonvirtualLongMethod_ptr, (jobject, jclass, jmethodID) -> jlong);
jni_stub!(CallNonvirtualLongMethodV(jobject, jclass, jmethodID, va_list) -> jlong);
jni_stub!(CallNonvirtualLongMethodA(jobject, jclass, jmethodID, *const jvalue) -> jlong);
jni_variadic_stub!(CallNonvirtualFloatMethod, CallNonvirtualFloatMethod_ptr, (jobject, jclass, jmethodID) -> jfloat);
jni_stub!(CallNonvirtualFloatMethodV(jobject, jclass, jmethodID, va_list) -> jfloat);
jni_stub!(CallNonvirtualFloatMethodA(jobject, jclass, jmethodID, *const jvalue) -> jfloat);
jni_variadic_stub!(CallNonvirtualDoubleMethod, CallNonvirtualDoubleMethod_ptr, (jobject, jclass, jmethodID) -> jdouble);
jni_stub!(CallNonvirtualDoubleMethodV(jobject, jclass, jmethodID, va_list) -> jdouble);
jni_stub!(CallNonvirtualDoubleMethodA(jobject, jclass, jmethodID, *const jvalue) -> jdouble);
jni_variadic_stub!(CallNonvirtualVoidMethod, CallNonvirtualVoidMethod_ptr, (jobject, jclass, jmethodID) -> ());
jni_stub!(CallNonvirtualVoidMethodV(jobject, jclass, jmethodID, va_list) -> ());
jni_stub!(CallNonvirtualVoidMethodA(jobject, jclass, jmethodID, *const jvalue) -> ());
jni_stub!(GetFieldID(jclass, *const c_char, *const c_char) -> jfieldID);
jni_stub!(GetObjectField(jobject, jfieldID) -> jobject);
jni_stub!(GetBooleanField(jobject, jfieldID) -> jboolean);
jni_stub!(GetByteField(jobject, jfieldID) -> jbyte);
jni_stub!(GetCharField(jobject, jfieldID) -> jchar);
jni_stub!(GetShortField(jobject, jfieldID) -> jshort);
jni_stub!(GetIntField(jobject, jfieldID) -> jint);
jni_stub!(GetLongField(jobject, jfieldID) -> jlong);
jni_stub!(GetFloatField(jobject, jfieldID) -> jfloat);
jni_stub!(GetDoubleField(jobject, jfieldID) -> jdouble);
jni_stub!(SetObjectField(jobject, jfieldID, jobject) -> ());
jni_stub!(SetBooleanField(jobject, jfieldID, jboolean) -> ());
jni_stub!(SetByteField(jobject, jfieldID, jbyte) -> ());
jni_stub!(SetCharField(jobject, jfieldID, jchar) -> ());
jni_stub!(SetShortField(jobject, jfieldID, jshort) -> ());
jni_stub!(SetIntField(jobject, jfieldID, jint) -> ());
jni_stub!(SetLongField(jobject, jfieldID, jlong) -> ());
jni_stub!(SetFloatField(jobject, jfieldID, jfloat) -> ());
jni_stub!(SetDoubleField(jobject, jfieldID, jdouble) -> ());
jni_stub!(GetStaticMethodID(jclass, *const c_char, *const c_char) -> jmethodID);
jni_variadic_stub!(CallStaticObjectMethod, CallStaticObjectMethod_ptr, (jclass, jmethodID) -> jobject);
jni_stub!(CallStaticObjectMethodV(jclass, jmethodID, va_list) -> jobject);
jni_stub!(CallStaticObjectMethodA(jclass, jmethodID, *const jvalue) -> jobject);
jni_variadic_stub!(CallStaticBooleanMethod, CallStaticBooleanMethod_ptr, (jclass, jmethodID) -> jboolean);
jni_stub!(CallStaticBooleanMethodV(jclass, jmethodID, va_list) -> jboolean);
jni_stub!(CallStaticBooleanMethodA(jclass, jmethodID, *const jvalue) -> jboolean);
jni_variadic_stub!(CallStaticByteMethod, CallStaticByteMethod_ptr, (jclass, jmethodID) -> jbyte);
jni_stub!(CallStaticByteMethodV(jclass, jmethodID, va_list) -> jbyte);
jni_stub!(CallStaticByteMethodA(jclass, jmethodID, *const jvalue) -> jbyte);
jni_variadic_stub!(CallStaticCharMethod, CallStaticCharMethod_ptr, (jclass, jmethodID) -> jchar);
jni_stub!(CallStaticCharMethodV(jclass, jmethodID, va_list) -> jchar);
jni_stub!(CallStaticCharMethodA(jclass, jmethodID, *const jvalue) -> jchar);
jni_variadic_stub!(CallStaticShortMethod, CallStaticShortMethod_ptr, (jclass, jmethodID) -> jshort);
jni_stub!(CallStaticShortMethodV(jclass, jmethodID, va_list) -> jshort);
jni_stub!(CallStaticShortMethodA(jclass, jmethodID, *const jvalue) -> jshort);
jni_variadic_stub!(CallStaticIntMethod, CallStaticIntMethod_ptr, (jclass, jmethodID) -> jint);
jni_stub!(CallStaticIntMethodV(jclass, jmethodID, va_list) -> jint);
jni_stub!(CallStaticIntMethodA(jclass, jmethodID, *const jvalue) -> jint);
jni_variadic_stub!(CallStaticLongMethod, CallStaticLongMethod_ptr, (jclass, jmethodID) -> jlong);
jni_stub!(CallStaticLongMethodV(jclass, jmethodID, va_list) -> jlong);
jni_stub!(CallStaticLongMethodA(jclass, jmethodID, *const jvalue) -> jlong);
jni_variadic_stub!(CallStaticFloatMethod, CallStaticFloatMethod_ptr, (jclass, jmethodID) -> jfloat);
jni_stub!(CallStaticFloatMethodV(jclass, jmethodID, va_list) -> jfloat);
jni_stub!(CallStaticFloatMethodA(jclass, jmethodID, *const jvalue) -> jfloat);
jni_variadic_stub!(CallStaticDoubleMethod, CallStaticDoubleMethod_ptr, (jclass, jmethodID) -> jdouble);
jni_stub!(CallStaticDoubleMethodV(jclass, jmethodID, va_list) -> jdouble);
jni_stub!(CallStaticDoubleMethodA(jclass, jmethodID, *const jvalue) -> jdouble);
jni_variadic_stub!(CallStaticVoidMethod, CallStaticVoidMethod_ptr, (jclass, jmethodID) -> ());
jni_stub!(CallStaticVoidMethodV(jclass, jmethodID, va_list) -> ());
jni_stub!(CallStaticVoidMethodA(jclass, jmethodID, *const jvalue) -> ());
jni_stub!(GetStaticFieldID(jclass, *const c_char, *const c_char) -> jfieldID);
jni_stub!(GetStaticObjectField(jclass, jfieldID) -> jobject);
jni_stub!(GetStaticBooleanField(jclass, jfieldID) -> jboolean);
jni_stub!(GetStaticByteField(jclass, jfieldID) -> jbyte);
jni_stub!(GetStaticCharField(jclass, jfieldID) -> jchar);
jni_stub!(GetStaticShortField(jclass, jfieldID) -> jshort);
jni_stub!(GetStaticIntField(jclass, jfieldID) -> jint);
jni_stub!(GetStaticLongField(jclass, jfieldID) -> jlong);
jni_stub!(GetStaticFloatField(jclass, jfieldID) -> jfloat);
jni_stub!(GetStaticDoubleField(jclass, jfieldID) -> jdouble);
jni_stub!(SetStaticObjectField(jclass, jfieldID, jobject) -> ());
jni_stub!(SetStaticBooleanField(jclass, jfieldID, jboolean) -> ());
jni_stub!(SetStaticByteField(jclass, jfieldID, jbyte) -> ());
jni_stub!(SetStaticCharField(jclass, jfieldID, jchar) -> ());
jni_stub!(SetStaticShortField(jclass, jfieldID, jshort) -> ());
jni_stub!(SetStaticIntField(jclass, jfieldID, jint) -> ());
jni_stub!(SetStaticLongField(jclass, jfieldID, jlong) -> ());
jni_stub!(SetStaticFloatField(jclass, jfieldID, jfloat) -> ());
jni_stub!(SetStaticDoubleField(jclass, jfieldID, jdouble) -> ());
jni_stub!(NewStringUTF(*const c_char) -> jstring);
jni_stub!(GetStringUTFLength(jstring) -> jsize);
jni_stub!(GetStringUTFChars(jstring, *mut jboolean) -> *const c_char);
jni_stub!(ReleaseStringUTFChars(jstring, *const c_char) -> ());
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
jni_stub!(GetStringUTFLengthAsLong(jstring) -> jlong);

jni_vm_stub!(DestroyJavaVM() -> jint);
jni_vm_stub!(AttachCurrentThread(*mut *mut c_void, *mut c_void) -> jint);
jni_vm_stub!(DetachCurrentThread() -> jint);
jni_vm_stub!(GetEnv(*mut *mut c_void, jint) -> jint);
jni_vm_stub!(AttachCurrentThreadAsDaemon(*mut *mut c_void, *mut c_void) -> jint);

struct Wrapper(JNINativeInterface_, JNIInvokeInterface_);
unsafe impl Sync for Wrapper {}
static VTABLE: Wrapper = {
    let mut ni: JNINativeInterface_ = unsafe { std::mem::zeroed() };

    ni.v24.GetVersion = get_version;
    ni.v24.ExceptionCheck = exception_check;
    ni.v24.GetJavaVM = get_java_vm;

    ///////////////// STUBS /////////////////////////
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
    ni.v24.ExceptionOccurred = ExceptionOccurred;
    ni.v24.ExceptionDescribe = ExceptionDescribe;
    ni.v24.ExceptionClear = ExceptionClear;
    ni.v24.FatalError = FatalError;
    ni.v24.PushLocalFrame = PushLocalFrame;
    ni.v24.PopLocalFrame = PopLocalFrame;
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
    ni.v24.GetObjectClass = GetObjectClass;
    ni.v24.IsInstanceOf = IsInstanceOf;
    ni.v24.GetMethodID = GetMethodID;
    ni.v24.CallObjectMethod = CallObjectMethod_ptr;
    ni.v24.CallObjectMethodV = CallObjectMethodV;
    ni.v24.CallObjectMethodA = CallObjectMethodA;
    ni.v24.CallBooleanMethod = CallBooleanMethod_ptr;
    ni.v24.CallBooleanMethodV = CallBooleanMethodV;
    ni.v24.CallBooleanMethodA = CallBooleanMethodA;
    ni.v24.CallByteMethod = CallByteMethod_ptr;
    ni.v24.CallByteMethodV = CallByteMethodV;
    ni.v24.CallByteMethodA = CallByteMethodA;
    ni.v24.CallCharMethod = CallCharMethod_ptr;
    ni.v24.CallCharMethodV = CallCharMethodV;
    ni.v24.CallCharMethodA = CallCharMethodA;
    ni.v24.CallShortMethod = CallShortMethod_ptr;
    ni.v24.CallShortMethodV = CallShortMethodV;
    ni.v24.CallShortMethodA = CallShortMethodA;
    ni.v24.CallIntMethod = CallIntMethod_ptr;
    ni.v24.CallIntMethodV = CallIntMethodV;
    ni.v24.CallIntMethodA = CallIntMethodA;
    ni.v24.CallLongMethod = CallLongMethod_ptr;
    ni.v24.CallLongMethodV = CallLongMethodV;
    ni.v24.CallLongMethodA = CallLongMethodA;
    ni.v24.CallFloatMethod = CallFloatMethod_ptr;
    ni.v24.CallFloatMethodV = CallFloatMethodV;
    ni.v24.CallFloatMethodA = CallFloatMethodA;
    ni.v24.CallDoubleMethod = CallDoubleMethod_ptr;
    ni.v24.CallDoubleMethodV = CallDoubleMethodV;
    ni.v24.CallDoubleMethodA = CallDoubleMethodA;
    ni.v24.CallVoidMethod = CallVoidMethod_ptr;
    ni.v24.CallVoidMethodV = CallVoidMethodV;
    ni.v24.CallVoidMethodA = CallVoidMethodA;
    ni.v24.CallNonvirtualObjectMethod = CallNonvirtualObjectMethod_ptr;
    ni.v24.CallNonvirtualObjectMethodV = CallNonvirtualObjectMethodV;
    ni.v24.CallNonvirtualObjectMethodA = CallNonvirtualObjectMethodA;
    ni.v24.CallNonvirtualBooleanMethod = CallNonvirtualBooleanMethod_ptr;
    ni.v24.CallNonvirtualBooleanMethodV = CallNonvirtualBooleanMethodV;
    ni.v24.CallNonvirtualBooleanMethodA = CallNonvirtualBooleanMethodA;
    ni.v24.CallNonvirtualByteMethod = CallNonvirtualByteMethod_ptr;
    ni.v24.CallNonvirtualByteMethodV = CallNonvirtualByteMethodV;
    ni.v24.CallNonvirtualByteMethodA = CallNonvirtualByteMethodA;
    ni.v24.CallNonvirtualCharMethod = CallNonvirtualCharMethod_ptr;
    ni.v24.CallNonvirtualCharMethodV = CallNonvirtualCharMethodV;
    ni.v24.CallNonvirtualCharMethodA = CallNonvirtualCharMethodA;
    ni.v24.CallNonvirtualShortMethod = CallNonvirtualShortMethod_ptr;
    ni.v24.CallNonvirtualShortMethodV = CallNonvirtualShortMethodV;
    ni.v24.CallNonvirtualShortMethodA = CallNonvirtualShortMethodA;
    ni.v24.CallNonvirtualIntMethod = CallNonvirtualIntMethod_ptr;
    ni.v24.CallNonvirtualIntMethodV = CallNonvirtualIntMethodV;
    ni.v24.CallNonvirtualIntMethodA = CallNonvirtualIntMethodA;
    ni.v24.CallNonvirtualLongMethod = CallNonvirtualLongMethod_ptr;
    ni.v24.CallNonvirtualLongMethodV = CallNonvirtualLongMethodV;
    ni.v24.CallNonvirtualLongMethodA = CallNonvirtualLongMethodA;
    ni.v24.CallNonvirtualFloatMethod = CallNonvirtualFloatMethod_ptr;
    ni.v24.CallNonvirtualFloatMethodV = CallNonvirtualFloatMethodV;
    ni.v24.CallNonvirtualFloatMethodA = CallNonvirtualFloatMethodA;
    ni.v24.CallNonvirtualDoubleMethod = CallNonvirtualDoubleMethod_ptr;
    ni.v24.CallNonvirtualDoubleMethodV = CallNonvirtualDoubleMethodV;
    ni.v24.CallNonvirtualDoubleMethodA = CallNonvirtualDoubleMethodA;
    ni.v24.CallNonvirtualVoidMethod = CallNonvirtualVoidMethod_ptr;
    ni.v24.CallNonvirtualVoidMethodV = CallNonvirtualVoidMethodV;
    ni.v24.CallNonvirtualVoidMethodA = CallNonvirtualVoidMethodA;
    ni.v24.GetFieldID = GetFieldID;
    ni.v24.GetObjectField = GetObjectField;
    ni.v24.GetBooleanField = GetBooleanField;
    ni.v24.GetByteField = GetByteField;
    ni.v24.GetCharField = GetCharField;
    ni.v24.GetShortField = GetShortField;
    ni.v24.GetIntField = GetIntField;
    ni.v24.GetLongField = GetLongField;
    ni.v24.GetFloatField = GetFloatField;
    ni.v24.GetDoubleField = GetDoubleField;
    ni.v24.SetObjectField = SetObjectField;
    ni.v24.SetBooleanField = SetBooleanField;
    ni.v24.SetByteField = SetByteField;
    ni.v24.SetCharField = SetCharField;
    ni.v24.SetShortField = SetShortField;
    ni.v24.SetIntField = SetIntField;
    ni.v24.SetLongField = SetLongField;
    ni.v24.SetFloatField = SetFloatField;
    ni.v24.SetDoubleField = SetDoubleField;
    ni.v24.GetStaticMethodID = GetStaticMethodID;
    ni.v24.CallStaticObjectMethod = CallStaticObjectMethod_ptr;
    ni.v24.CallStaticObjectMethodV = CallStaticObjectMethodV;
    ni.v24.CallStaticObjectMethodA = CallStaticObjectMethodA;
    ni.v24.CallStaticBooleanMethod = CallStaticBooleanMethod_ptr;
    ni.v24.CallStaticBooleanMethodV = CallStaticBooleanMethodV;
    ni.v24.CallStaticBooleanMethodA = CallStaticBooleanMethodA;
    ni.v24.CallStaticByteMethod = CallStaticByteMethod_ptr;
    ni.v24.CallStaticByteMethodV = CallStaticByteMethodV;
    ni.v24.CallStaticByteMethodA = CallStaticByteMethodA;
    ni.v24.CallStaticCharMethod = CallStaticCharMethod_ptr;
    ni.v24.CallStaticCharMethodV = CallStaticCharMethodV;
    ni.v24.CallStaticCharMethodA = CallStaticCharMethodA;
    ni.v24.CallStaticShortMethod = CallStaticShortMethod_ptr;
    ni.v24.CallStaticShortMethodV = CallStaticShortMethodV;
    ni.v24.CallStaticShortMethodA = CallStaticShortMethodA;
    ni.v24.CallStaticIntMethod = CallStaticIntMethod_ptr;
    ni.v24.CallStaticIntMethodV = CallStaticIntMethodV;
    ni.v24.CallStaticIntMethodA = CallStaticIntMethodA;
    ni.v24.CallStaticLongMethod = CallStaticLongMethod_ptr;
    ni.v24.CallStaticLongMethodV = CallStaticLongMethodV;
    ni.v24.CallStaticLongMethodA = CallStaticLongMethodA;
    ni.v24.CallStaticFloatMethod = CallStaticFloatMethod_ptr;
    ni.v24.CallStaticFloatMethodV = CallStaticFloatMethodV;
    ni.v24.CallStaticFloatMethodA = CallStaticFloatMethodA;
    ni.v24.CallStaticDoubleMethod = CallStaticDoubleMethod_ptr;
    ni.v24.CallStaticDoubleMethodV = CallStaticDoubleMethodV;
    ni.v24.CallStaticDoubleMethodA = CallStaticDoubleMethodA;
    ni.v24.CallStaticVoidMethod = CallStaticVoidMethod_ptr;
    ni.v24.CallStaticVoidMethodV = CallStaticVoidMethodV;
    ni.v24.CallStaticVoidMethodA = CallStaticVoidMethodA;
    ni.v24.GetStaticFieldID = GetStaticFieldID;
    ni.v24.GetStaticObjectField = GetStaticObjectField;
    ni.v24.GetStaticBooleanField = GetStaticBooleanField;
    ni.v24.GetStaticByteField = GetStaticByteField;
    ni.v24.GetStaticCharField = GetStaticCharField;
    ni.v24.GetStaticShortField = GetStaticShortField;
    ni.v24.GetStaticIntField = GetStaticIntField;
    ni.v24.GetStaticLongField = GetStaticLongField;
    ni.v24.GetStaticFloatField = GetStaticFloatField;
    ni.v24.GetStaticDoubleField = GetStaticDoubleField;
    ni.v24.SetStaticObjectField = SetStaticObjectField;
    ni.v24.SetStaticBooleanField = SetStaticBooleanField;
    ni.v24.SetStaticByteField = SetStaticByteField;
    ni.v24.SetStaticCharField = SetStaticCharField;
    ni.v24.SetStaticShortField = SetStaticShortField;
    ni.v24.SetStaticIntField = SetStaticIntField;
    ni.v24.SetStaticLongField = SetStaticLongField;
    ni.v24.SetStaticFloatField = SetStaticFloatField;
    ni.v24.SetStaticDoubleField = SetStaticDoubleField;
    ni.v24.NewString = new_string;
    ni.v24.GetStringLength = get_string_length;
    ni.v24.GetStringChars = get_string_chars;
    ni.v24.ReleaseStringChars = release_string_chars;
    ni.v24.NewStringUTF = NewStringUTF;
    ni.v24.GetStringUTFLength = GetStringUTFLength;
    ni.v24.GetStringUTFChars = GetStringUTFChars;
    ni.v24.ReleaseStringUTFChars = ReleaseStringUTFChars;
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
    ni.v24.GetStringRegion = GetStringRegion;
    ni.v24.GetStringUTFRegion = GetStringUTFRegion;
    ni.v24.GetPrimitiveArrayCritical = GetPrimitiveArrayCritical;
    ni.v24.ReleasePrimitiveArrayCritical = ReleasePrimitiveArrayCritical;
    ni.v24.GetStringCritical = GetStringCritical;
    ni.v24.ReleaseStringCritical = ReleaseStringCritical;
    ni.v24.NewWeakGlobalRef = NewWeakGlobalRef;
    ni.v24.DeleteWeakGlobalRef = DeleteWeakGlobalRef;
    ni.v24.NewDirectByteBuffer = NewDirectByteBuffer;
    ni.v24.GetDirectBufferAddress = GetDirectBufferAddress;
    ni.v24.GetDirectBufferCapacity = GetDirectBufferCapacity;
    ni.v24.GetObjectRefType = GetObjectRefType;
    ni.v24.GetModule = GetModule;
    ni.v24.IsVirtualThread = IsVirtualThread;
    ni.v24.GetStringUTFLengthAsLong = GetStringUTFLengthAsLong;

    let mut ii: JNIInvokeInterface_ = unsafe { std::mem::zeroed() };
    ii.v1_4.DestroyJavaVM = DestroyJavaVM;
    ii.v1_4.AttachCurrentThread = AttachCurrentThread;
    ii.v1_4.DetachCurrentThread = DetachCurrentThread;
    ii.v1_4.GetEnv = GetEnv;
    ii.v1_4.AttachCurrentThreadAsDaemon = AttachCurrentThreadAsDaemon;

    Wrapper(ni, ii)
};
