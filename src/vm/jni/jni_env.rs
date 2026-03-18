use crate::vm::jni::jni_impl::{exception_check, get_version};
use jni_sys::{
    jarray, jboolean, jbooleanArray, jbyte, jbyteArray, jchar, jcharArray, jclass, jdouble,
    jdoubleArray, jfieldID, jfloat, jfloatArray, jint, jintArray, jlong, jlongArray, jmethodID,
    jobject, jobjectArray, jobjectRefType, jshort, jshortArray, jsize, jstring, jthrowable,
    jvalue, jweak, va_list, JNIEnv, JNINativeInterface_, JNINativeMethod, JavaVM,
};
use std::ffi::{c_char, c_void};

pub(crate) fn get_jni_env() -> *mut JNIEnv {
    static mut ENV: *const JNINativeInterface_ = &VTABLE.0;
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
jni_stub!(GetMethodID(jclass, *const i8, *const i8) -> jmethodID);
jni_variadic_stub!(CallObjectMethod, CallObjectMethod_ptr, (jobject, jmethodID) -> jobject);
jni_stub!(CallObjectMethodV(jobject, jmethodID, va_list) -> jobject);
jni_stub!(CallObjectMethodA(jobject, jmethodID, *const jvalue) -> jobject);
jni_variadic_stub!(CallBooleanMethod, CallBooleanMethod_ptr,(jobject, jmethodID) -> jboolean);
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
jni_variadic_stub!(CallIntMethod, CallIntMethod_ptr,(jobject, jmethodID) -> jint);
jni_stub!(CallIntMethodV(jobject, jmethodID, va_list) -> jint);
jni_stub!(CallIntMethodA(jobject, jmethodID, *const jvalue) -> jint);
jni_variadic_stub!(CallLongMethod, CallLongMethod_ptr,(jobject, jmethodID) -> jlong);
jni_stub!(CallLongMethodV(jobject, jmethodID, va_list) -> jlong);
jni_stub!(CallLongMethodA(jobject, jmethodID, *const jvalue) -> jlong);
jni_variadic_stub!(CallFloatMethod, CallFloatMethod_ptr,(jobject, jmethodID) -> jfloat);
jni_stub!(CallFloatMethodV(jobject, jmethodID, va_list) -> jfloat);
jni_stub!(CallFloatMethodA(jobject, jmethodID, *const jvalue) -> jfloat);
jni_variadic_stub!(CallDoubleMethod, CallDoubleMethod_ptr,(jobject, jmethodID) -> jdouble);
jni_stub!(CallDoubleMethodV(jobject, jmethodID, va_list) -> jdouble);
jni_stub!(CallDoubleMethodA(jobject, jmethodID, *const jvalue) -> jdouble);
jni_variadic_stub!(CallVoidMethod, CallVoidMethod_ptr,(jobject, jmethodID) -> ());
jni_stub!(CallVoidMethodV(jobject, jmethodID, va_list) -> ());
jni_stub!(CallVoidMethodA(jobject, jmethodID, *const jvalue) -> ());
jni_variadic_stub!(CallNonvirtualObjectMethod, CallNonvirtualObjectMethod_ptr,(jobject, jclass, jmethodID) -> jobject);
jni_stub!(CallNonvirtualObjectMethodV(jobject, jclass, jmethodID, va_list) -> jobject);
jni_stub!(CallNonvirtualObjectMethodA(jobject, jclass, jmethodID, *const jvalue) -> jobject);
jni_variadic_stub!(CallNonvirtualBooleanMethod, CallNonvirtualBooleanMethod_ptr,(jobject, jclass, jmethodID) -> jboolean);
jni_stub!(CallNonvirtualBooleanMethodV(jobject, jclass, jmethodID, va_list) -> jboolean);
jni_stub!(CallNonvirtualBooleanMethodA(jobject, jclass, jmethodID, *const jvalue) -> jboolean);
jni_variadic_stub!(CallNonvirtualByteMethod, CallNonvirtualByteMethod_ptr,(jobject, jclass, jmethodID) -> jbyte);
jni_stub!(CallNonvirtualByteMethodV(jobject, jclass, jmethodID, va_list) -> jbyte);
jni_stub!(CallNonvirtualByteMethodA(jobject, jclass, jmethodID, *const jvalue) -> jbyte);
jni_variadic_stub!(CallNonvirtualCharMethod, CallNonvirtualCharMethod_ptr,(jobject, jclass, jmethodID) -> jchar);
jni_stub!(CallNonvirtualCharMethodV(jobject, jclass, jmethodID, va_list) -> jchar);
jni_stub!(CallNonvirtualCharMethodA(jobject, jclass, jmethodID, *const jvalue) -> jchar);
jni_variadic_stub!(CallNonvirtualShortMethod, CallNonvirtualShortMethod_ptr,(jobject, jclass, jmethodID) -> jshort);
jni_stub!(CallNonvirtualShortMethodV(jobject, jclass, jmethodID, va_list) -> jshort);
jni_stub!(CallNonvirtualShortMethodA(jobject, jclass, jmethodID, *const jvalue) -> jshort);
jni_variadic_stub!(CallNonvirtualIntMethod, CallNonvirtualIntMethod_ptr,(jobject, jclass, jmethodID) -> jint);
jni_stub!(CallNonvirtualIntMethodV(jobject, jclass, jmethodID, va_list) -> jint);
jni_stub!(CallNonvirtualIntMethodA(jobject, jclass, jmethodID, *const jvalue) -> jint);
jni_variadic_stub!(CallNonvirtualLongMethod, CallNonvirtualLongMethod_ptr,(jobject, jclass, jmethodID) -> jlong);
jni_stub!(CallNonvirtualLongMethodV(jobject, jclass, jmethodID, va_list) -> jlong);
jni_stub!(CallNonvirtualLongMethodA(jobject, jclass, jmethodID, *const jvalue) -> jlong);
jni_variadic_stub!(CallNonvirtualFloatMethod, CallNonvirtualFloatMethod_ptr,(jobject, jclass, jmethodID) -> jfloat);
jni_stub!(CallNonvirtualFloatMethodV(jobject, jclass, jmethodID, va_list) -> jfloat);
jni_stub!(CallNonvirtualFloatMethodA(jobject, jclass, jmethodID, *const jvalue) -> jfloat);
jni_variadic_stub!(CallNonvirtualDoubleMethod, CallNonvirtualDoubleMethod_ptr,(jobject, jclass, jmethodID) -> jdouble);
jni_stub!(CallNonvirtualDoubleMethodV(jobject, jclass, jmethodID, va_list) -> jdouble);
jni_stub!(CallNonvirtualDoubleMethodA(jobject, jclass, jmethodID, *const jvalue) -> jdouble);
jni_variadic_stub!(CallNonvirtualVoidMethod, CallNonvirtualVoidMethod_ptr,(jobject, jclass, jmethodID) -> ());
jni_stub!(CallNonvirtualVoidMethodV(jobject, jclass, jmethodID, va_list) -> ());
jni_stub!(CallNonvirtualVoidMethodA(jobject, jclass, jmethodID, *const jvalue) -> ());
jni_stub!(GetFieldID(jclass, *const i8, *const i8) -> jfieldID);
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
jni_stub!(GetStaticMethodID(jclass, *const i8, *const i8) -> jmethodID);
jni_variadic_stub!(CallStaticObjectMethod, CallStaticObjectMethod_ptr,(jclass, jmethodID) -> jobject);
jni_stub!(CallStaticObjectMethodV(jclass, jmethodID, va_list) -> jobject);
jni_stub!(CallStaticObjectMethodA(jclass, jmethodID, *const jvalue) -> jobject);
jni_variadic_stub!(CallStaticBooleanMethod, CallStaticBooleanMethod_ptr,(jclass, jmethodID) -> jboolean);
jni_stub!(CallStaticBooleanMethodV(jclass, jmethodID, va_list) -> jboolean);
jni_stub!(CallStaticBooleanMethodA(jclass, jmethodID, *const jvalue) -> jboolean);
jni_variadic_stub!(CallStaticByteMethod, CallStaticByteMethod_ptr,(jclass, jmethodID) -> jbyte);
jni_stub!(CallStaticByteMethodV(jclass, jmethodID, va_list) -> jbyte);
jni_stub!(CallStaticByteMethodA(jclass, jmethodID, *const jvalue) -> jbyte);
jni_variadic_stub!(CallStaticCharMethod, CallStaticCharMethod_ptr,(jclass, jmethodID) -> jchar);
jni_stub!(CallStaticCharMethodV(jclass, jmethodID, va_list) -> jchar);
jni_stub!(CallStaticCharMethodA(jclass, jmethodID, *const jvalue) -> jchar);
jni_variadic_stub!(CallStaticShortMethod, CallStaticShortMethod_ptr,(jclass, jmethodID) -> jshort);
jni_stub!(CallStaticShortMethodV(jclass, jmethodID, va_list) -> jshort);
jni_stub!(CallStaticShortMethodA(jclass, jmethodID, *const jvalue) -> jshort);
jni_variadic_stub!(CallStaticIntMethod, CallStaticIntMethod_ptr,(jclass, jmethodID) -> jint);
jni_stub!(CallStaticIntMethodV(jclass, jmethodID, va_list) -> jint);
jni_stub!(CallStaticIntMethodA(jclass, jmethodID, *const jvalue) -> jint);
jni_variadic_stub!(CallStaticLongMethod, CallStaticLongMethod_ptr,(jclass, jmethodID) -> jlong);
jni_stub!(CallStaticLongMethodV(jclass, jmethodID, va_list) -> jlong);
jni_stub!(CallStaticLongMethodA(jclass, jmethodID, *const jvalue) -> jlong);
jni_variadic_stub!(CallStaticFloatMethod, CallStaticFloatMethod_ptr,(jclass, jmethodID) -> jfloat);
jni_stub!(CallStaticFloatMethodV(jclass, jmethodID, va_list) -> jfloat);
jni_stub!(CallStaticFloatMethodA(jclass, jmethodID, *const jvalue) -> jfloat);
jni_variadic_stub!(CallStaticDoubleMethod, CallStaticDoubleMethod_ptr,(jclass, jmethodID) -> jdouble);
jni_stub!(CallStaticDoubleMethodV(jclass, jmethodID, va_list) -> jdouble);
jni_stub!(CallStaticDoubleMethodA(jclass, jmethodID, *const jvalue) -> jdouble);
jni_variadic_stub!(CallStaticVoidMethod, CallStaticVoidMethod_ptr,(jclass, jmethodID) -> ());
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
jni_stub!(NewString(*const u16, jsize) -> jstring);
jni_stub!(GetStringLength(jstring) -> jsize);
jni_stub!(GetStringChars(jstring, *mut jboolean) -> *const jchar);
jni_stub!(ReleaseStringChars(jstring, *const jchar) -> ());
jni_stub!(NewStringUTF(*const c_char) -> jstring);
jni_stub!(GetStringUTFLength(jstring) -> jsize);
jni_stub!(GetStringUTFChars(jstring, *mut jboolean) -> *const c_char);
jni_stub!(ReleaseStringUTFChars(jstring, *const c_char) -> ());
jni_stub!(GetArrayLength(jarray) -> jsize);
jni_stub!(NewObjectArray(jsize, jclass, jobject) -> jobjectArray);
jni_stub!(GetObjectArrayElement(jobjectArray, jsize) -> jobject);
jni_stub!(SetObjectArrayElement(jobjectArray, jsize, jobject) -> ());
jni_stub!(NewBooleanArray(jsize) -> jbooleanArray);
jni_stub!(NewByteArray(jsize) -> jbyteArray);
jni_stub!(NewCharArray(jsize) -> jcharArray);
jni_stub!(NewShortArray(jsize) -> jshortArray);
jni_stub!(NewIntArray(jsize) -> jintArray);
jni_stub!(NewLongArray(jsize) -> jlongArray);
jni_stub!(NewFloatArray(jsize) -> jfloatArray);
jni_stub!(NewDoubleArray(jsize) -> jdoubleArray);
jni_stub!(GetBooleanArrayElements(jbooleanArray, *mut jboolean) -> *mut jboolean);
jni_stub!(GetByteArrayElements(jbyteArray, *mut jboolean) -> *mut jbyte);
jni_stub!(GetCharArrayElements(jcharArray, *mut jboolean) -> *mut jchar);
jni_stub!(GetShortArrayElements(jshortArray, *mut jboolean) -> *mut jshort);
jni_stub!(GetIntArrayElements(jintArray, *mut jboolean) -> *mut jint);
jni_stub!(GetLongArrayElements(jlongArray, *mut jboolean) -> *mut jlong);
jni_stub!(GetFloatArrayElements(jfloatArray, *mut jboolean) -> *mut jfloat);
jni_stub!(GetDoubleArrayElements(jdoubleArray, *mut jboolean) -> *mut jdouble);
jni_stub!(ReleaseBooleanArrayElements(jbooleanArray, *mut jboolean, jint) -> ());
jni_stub!(ReleaseByteArrayElements(jbyteArray, *mut jbyte, jint) -> ());
jni_stub!(ReleaseCharArrayElements(jcharArray, *mut jchar, jint) -> ());
jni_stub!(ReleaseShortArrayElements(jshortArray, *mut jshort, jint) -> ());
jni_stub!(ReleaseIntArrayElements(jintArray, *mut jint, jint) -> ());
jni_stub!(ReleaseLongArrayElements(jlongArray, *mut jlong, jint) -> ());
jni_stub!(ReleaseFloatArrayElements(jfloatArray, *mut jfloat, jint) -> ());
jni_stub!(ReleaseDoubleArrayElements(jdoubleArray, *mut jdouble, jint) -> ());
jni_stub!(GetBooleanArrayRegion(jbooleanArray, jsize, jsize, *mut jboolean) -> ());
jni_stub!(GetByteArrayRegion(jbyteArray, jsize, jsize, *mut jbyte) -> ());
jni_stub!(GetCharArrayRegion(jcharArray, jsize, jsize, *mut jchar) -> ());
jni_stub!(GetShortArrayRegion(jshortArray, jsize, jsize, *mut jshort) -> ());
jni_stub!(GetIntArrayRegion(jintArray, jsize, jsize, *mut jint) -> ());
jni_stub!(GetLongArrayRegion(jlongArray, jsize, jsize, *mut jlong) -> ());
jni_stub!(GetFloatArrayRegion(jfloatArray, jsize, jsize, *mut jfloat) -> ());
jni_stub!(GetDoubleArrayRegion(jdoubleArray, jsize, jsize, *mut jdouble) -> ());
jni_stub!(SetBooleanArrayRegion(jbooleanArray, jsize, jsize, *const jboolean) -> ());
jni_stub!(SetByteArrayRegion(jbyteArray, jsize, jsize, *const jbyte) -> ());
jni_stub!(SetCharArrayRegion(jcharArray, jsize, jsize, *const jchar) -> ());
jni_stub!(SetShortArrayRegion(jshortArray, jsize, jsize, *const jshort) -> ());
jni_stub!(SetIntArrayRegion(jintArray, jsize, jsize, *const jint) -> ());
jni_stub!(SetLongArrayRegion(jlongArray, jsize, jsize, *const jlong) -> ());
jni_stub!(SetFloatArrayRegion(jfloatArray, jsize, jsize, *const jfloat) -> ());
jni_stub!(SetDoubleArrayRegion(jdoubleArray, jsize, jsize, *const jdouble) -> ());
jni_stub!(RegisterNatives(jclass, *const JNINativeMethod, jint) -> jint);
jni_stub!(UnregisterNatives(jclass) -> jint);
jni_stub!(MonitorEnter(jobject) -> jint);
jni_stub!(MonitorExit(jobject) -> jint);
jni_stub!(GetJavaVM(*mut *mut JavaVM) -> jint);
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

struct Wrapper(JNINativeInterface_);
unsafe impl Sync for Wrapper {}
static VTABLE: Wrapper = {
    let mut x: JNINativeInterface_ = unsafe { std::mem::zeroed() };

    x.v24.GetVersion = get_version;
    x.v24.ExceptionCheck = exception_check;

    ///////////////// STUBS /////////////////////////
    x.v24.DefineClass = DefineClass;
    x.v24.FindClass = FindClass;
    x.v24.FromReflectedMethod = FromReflectedMethod;
    x.v24.FromReflectedField = FromReflectedField;
    x.v24.ToReflectedMethod = ToReflectedMethod;
    x.v24.GetSuperclass = GetSuperclass;
    x.v24.IsAssignableFrom = IsAssignableFrom;
    x.v24.ToReflectedField = ToReflectedField;
    x.v24.Throw = Throw;
    x.v24.ThrowNew = ThrowNew;
    x.v24.ExceptionOccurred = ExceptionOccurred;
    x.v24.ExceptionDescribe = ExceptionDescribe;
    x.v24.ExceptionClear = ExceptionClear;
    x.v24.FatalError = FatalError;
    x.v24.PushLocalFrame = PushLocalFrame;
    x.v24.PopLocalFrame = PopLocalFrame;
    x.v24.NewGlobalRef = NewGlobalRef;
    x.v24.DeleteGlobalRef = DeleteGlobalRef;
    x.v24.DeleteLocalRef = DeleteLocalRef;
    x.v24.IsSameObject = IsSameObject;
    x.v24.NewLocalRef = NewLocalRef;
    x.v24.EnsureLocalCapacity = EnsureLocalCapacity;
    x.v24.AllocObject = AllocObject;
    x.v24.NewObject = NewObject_ptr;
    x.v24.NewObjectV = NewObjectV;
    x.v24.NewObjectA = NewObjectA;
    x.v24.GetObjectClass = GetObjectClass;
    x.v24.IsInstanceOf = IsInstanceOf;
    x.v24.GetMethodID = GetMethodID;
    x.v24.CallObjectMethod = CallObjectMethod_ptr;
    x.v24.CallObjectMethodV = CallObjectMethodV;
    x.v24.CallObjectMethodA = CallObjectMethodA;
    x.v24.CallBooleanMethod = CallBooleanMethod_ptr;
    x.v24.CallBooleanMethodV = CallBooleanMethodV;
    x.v24.CallBooleanMethodA = CallBooleanMethodA;
    x.v24.CallByteMethod = CallByteMethod_ptr;
    x.v24.CallByteMethodV = CallByteMethodV;
    x.v24.CallByteMethodA = CallByteMethodA;
    x.v24.CallCharMethod = CallCharMethod_ptr;
    x.v24.CallCharMethodV = CallCharMethodV;
    x.v24.CallCharMethodA = CallCharMethodA;
    x.v24.CallShortMethod = CallShortMethod_ptr;
    x.v24.CallShortMethodV = CallShortMethodV;
    x.v24.CallShortMethodA = CallShortMethodA;
    x.v24.CallIntMethod = CallIntMethod_ptr;
    x.v24.CallIntMethodV = CallIntMethodV;
    x.v24.CallIntMethodA = CallIntMethodA;
    x.v24.CallLongMethod = CallLongMethod_ptr;
    x.v24.CallLongMethodV = CallLongMethodV;
    x.v24.CallLongMethodA = CallLongMethodA;
    x.v24.CallFloatMethod = CallFloatMethod_ptr;
    x.v24.CallFloatMethodV = CallFloatMethodV;
    x.v24.CallFloatMethodA = CallFloatMethodA;
    x.v24.CallDoubleMethod = CallDoubleMethod_ptr;
    x.v24.CallDoubleMethodV = CallDoubleMethodV;
    x.v24.CallDoubleMethodA = CallDoubleMethodA;
    x.v24.CallVoidMethod = CallVoidMethod_ptr;
    x.v24.CallVoidMethodV = CallVoidMethodV;
    x.v24.CallVoidMethodA = CallVoidMethodA;
    x.v24.CallNonvirtualObjectMethod = CallNonvirtualObjectMethod_ptr;
    x.v24.CallNonvirtualObjectMethodV = CallNonvirtualObjectMethodV;
    x.v24.CallNonvirtualObjectMethodA = CallNonvirtualObjectMethodA;
    x.v24.CallNonvirtualBooleanMethod = CallNonvirtualBooleanMethod_ptr;
    x.v24.CallNonvirtualBooleanMethodV = CallNonvirtualBooleanMethodV;
    x.v24.CallNonvirtualBooleanMethodA = CallNonvirtualBooleanMethodA;
    x.v24.CallNonvirtualByteMethod = CallNonvirtualByteMethod_ptr;
    x.v24.CallNonvirtualByteMethodV = CallNonvirtualByteMethodV;
    x.v24.CallNonvirtualByteMethodA = CallNonvirtualByteMethodA;
    x.v24.CallNonvirtualCharMethod = CallNonvirtualCharMethod_ptr;
    x.v24.CallNonvirtualCharMethodV = CallNonvirtualCharMethodV;
    x.v24.CallNonvirtualCharMethodA = CallNonvirtualCharMethodA;
    x.v24.CallNonvirtualShortMethod = CallNonvirtualShortMethod_ptr;
    x.v24.CallNonvirtualShortMethodV = CallNonvirtualShortMethodV;
    x.v24.CallNonvirtualShortMethodA = CallNonvirtualShortMethodA;
    x.v24.CallNonvirtualIntMethod = CallNonvirtualIntMethod_ptr;
    x.v24.CallNonvirtualIntMethodV = CallNonvirtualIntMethodV;
    x.v24.CallNonvirtualIntMethodA = CallNonvirtualIntMethodA;
    x.v24.CallNonvirtualLongMethod = CallNonvirtualLongMethod_ptr;
    x.v24.CallNonvirtualLongMethodV = CallNonvirtualLongMethodV;
    x.v24.CallNonvirtualLongMethodA = CallNonvirtualLongMethodA;
    x.v24.CallNonvirtualFloatMethod = CallNonvirtualFloatMethod_ptr;
    x.v24.CallNonvirtualFloatMethodV = CallNonvirtualFloatMethodV;
    x.v24.CallNonvirtualFloatMethodA = CallNonvirtualFloatMethodA;
    x.v24.CallNonvirtualDoubleMethod = CallNonvirtualDoubleMethod_ptr;
    x.v24.CallNonvirtualDoubleMethodV = CallNonvirtualDoubleMethodV;
    x.v24.CallNonvirtualDoubleMethodA = CallNonvirtualDoubleMethodA;
    x.v24.CallNonvirtualVoidMethod = CallNonvirtualVoidMethod_ptr;
    x.v24.CallNonvirtualVoidMethodV = CallNonvirtualVoidMethodV;
    x.v24.CallNonvirtualVoidMethodA = CallNonvirtualVoidMethodA;
    x.v24.GetFieldID = GetFieldID;
    x.v24.GetObjectField = GetObjectField;
    x.v24.GetBooleanField = GetBooleanField;
    x.v24.GetByteField = GetByteField;
    x.v24.GetCharField = GetCharField;
    x.v24.GetShortField = GetShortField;
    x.v24.GetIntField = GetIntField;
    x.v24.GetLongField = GetLongField;
    x.v24.GetFloatField = GetFloatField;
    x.v24.GetDoubleField = GetDoubleField;
    x.v24.SetObjectField = SetObjectField;
    x.v24.SetBooleanField = SetBooleanField;
    x.v24.SetByteField = SetByteField;
    x.v24.SetCharField = SetCharField;
    x.v24.SetShortField = SetShortField;
    x.v24.SetIntField = SetIntField;
    x.v24.SetLongField = SetLongField;
    x.v24.SetFloatField = SetFloatField;
    x.v24.SetDoubleField = SetDoubleField;
    x.v24.GetStaticMethodID = GetStaticMethodID;
    x.v24.CallStaticObjectMethod = CallStaticObjectMethod_ptr;
    x.v24.CallStaticObjectMethodV = CallStaticObjectMethodV;
    x.v24.CallStaticObjectMethodA = CallStaticObjectMethodA;
    x.v24.CallStaticBooleanMethod = CallStaticBooleanMethod_ptr;
    x.v24.CallStaticBooleanMethodV = CallStaticBooleanMethodV;
    x.v24.CallStaticBooleanMethodA = CallStaticBooleanMethodA;
    x.v24.CallStaticByteMethod = CallStaticByteMethod_ptr;
    x.v24.CallStaticByteMethodV = CallStaticByteMethodV;
    x.v24.CallStaticByteMethodA = CallStaticByteMethodA;
    x.v24.CallStaticCharMethod = CallStaticCharMethod_ptr;
    x.v24.CallStaticCharMethodV = CallStaticCharMethodV;
    x.v24.CallStaticCharMethodA = CallStaticCharMethodA;
    x.v24.CallStaticShortMethod = CallStaticShortMethod_ptr;
    x.v24.CallStaticShortMethodV = CallStaticShortMethodV;
    x.v24.CallStaticShortMethodA = CallStaticShortMethodA;
    x.v24.CallStaticIntMethod = CallStaticIntMethod_ptr;
    x.v24.CallStaticIntMethodV = CallStaticIntMethodV;
    x.v24.CallStaticIntMethodA = CallStaticIntMethodA;
    x.v24.CallStaticLongMethod = CallStaticLongMethod_ptr;
    x.v24.CallStaticLongMethodV = CallStaticLongMethodV;
    x.v24.CallStaticLongMethodA = CallStaticLongMethodA;
    x.v24.CallStaticFloatMethod = CallStaticFloatMethod_ptr;
    x.v24.CallStaticFloatMethodV = CallStaticFloatMethodV;
    x.v24.CallStaticFloatMethodA = CallStaticFloatMethodA;
    x.v24.CallStaticDoubleMethod = CallStaticDoubleMethod_ptr;
    x.v24.CallStaticDoubleMethodV = CallStaticDoubleMethodV;
    x.v24.CallStaticDoubleMethodA = CallStaticDoubleMethodA;
    x.v24.CallStaticVoidMethod = CallStaticVoidMethod_ptr;
    x.v24.CallStaticVoidMethodV = CallStaticVoidMethodV;
    x.v24.CallStaticVoidMethodA = CallStaticVoidMethodA;
    x.v24.GetStaticFieldID = GetStaticFieldID;
    x.v24.GetStaticObjectField = GetStaticObjectField;
    x.v24.GetStaticBooleanField = GetStaticBooleanField;
    x.v24.GetStaticByteField = GetStaticByteField;
    x.v24.GetStaticCharField = GetStaticCharField;
    x.v24.GetStaticShortField = GetStaticShortField;
    x.v24.GetStaticIntField = GetStaticIntField;
    x.v24.GetStaticLongField = GetStaticLongField;
    x.v24.GetStaticFloatField = GetStaticFloatField;
    x.v24.GetStaticDoubleField = GetStaticDoubleField;
    x.v24.SetStaticObjectField = SetStaticObjectField;
    x.v24.SetStaticBooleanField = SetStaticBooleanField;
    x.v24.SetStaticByteField = SetStaticByteField;
    x.v24.SetStaticCharField = SetStaticCharField;
    x.v24.SetStaticShortField = SetStaticShortField;
    x.v24.SetStaticIntField = SetStaticIntField;
    x.v24.SetStaticLongField = SetStaticLongField;
    x.v24.SetStaticFloatField = SetStaticFloatField;
    x.v24.SetStaticDoubleField = SetStaticDoubleField;
    x.v24.NewString = NewString;
    x.v24.GetStringLength = GetStringLength;
    x.v24.GetStringChars = GetStringChars;
    x.v24.ReleaseStringChars = ReleaseStringChars;
    x.v24.NewStringUTF = NewStringUTF;
    x.v24.GetStringUTFLength = GetStringUTFLength;
    x.v24.GetStringUTFChars = GetStringUTFChars;
    x.v24.ReleaseStringUTFChars = ReleaseStringUTFChars;
    x.v24.GetArrayLength = GetArrayLength;
    x.v24.NewObjectArray = NewObjectArray;
    x.v24.GetObjectArrayElement = GetObjectArrayElement;
    x.v24.SetObjectArrayElement = SetObjectArrayElement;
    x.v24.NewBooleanArray = NewBooleanArray;
    x.v24.NewByteArray = NewByteArray;
    x.v24.NewCharArray = NewCharArray;
    x.v24.NewShortArray = NewShortArray;
    x.v24.NewIntArray = NewIntArray;
    x.v24.NewLongArray = NewLongArray;
    x.v24.NewFloatArray = NewFloatArray;
    x.v24.NewDoubleArray = NewDoubleArray;
    x.v24.GetBooleanArrayElements = GetBooleanArrayElements;
    x.v24.GetByteArrayElements = GetByteArrayElements;
    x.v24.GetCharArrayElements = GetCharArrayElements;
    x.v24.GetShortArrayElements = GetShortArrayElements;
    x.v24.GetIntArrayElements = GetIntArrayElements;
    x.v24.GetLongArrayElements = GetLongArrayElements;
    x.v24.GetFloatArrayElements = GetFloatArrayElements;
    x.v24.GetDoubleArrayElements = GetDoubleArrayElements;
    x.v24.ReleaseBooleanArrayElements = ReleaseBooleanArrayElements;
    x.v24.ReleaseByteArrayElements = ReleaseByteArrayElements;
    x.v24.ReleaseCharArrayElements = ReleaseCharArrayElements;
    x.v24.ReleaseShortArrayElements = ReleaseShortArrayElements;
    x.v24.ReleaseIntArrayElements = ReleaseIntArrayElements;
    x.v24.ReleaseLongArrayElements = ReleaseLongArrayElements;
    x.v24.ReleaseFloatArrayElements = ReleaseFloatArrayElements;
    x.v24.ReleaseDoubleArrayElements = ReleaseDoubleArrayElements;
    x.v24.GetBooleanArrayRegion = GetBooleanArrayRegion;
    x.v24.GetByteArrayRegion = GetByteArrayRegion;
    x.v24.GetCharArrayRegion = GetCharArrayRegion;
    x.v24.GetShortArrayRegion = GetShortArrayRegion;
    x.v24.GetIntArrayRegion = GetIntArrayRegion;
    x.v24.GetLongArrayRegion = GetLongArrayRegion;
    x.v24.GetFloatArrayRegion = GetFloatArrayRegion;
    x.v24.GetDoubleArrayRegion = GetDoubleArrayRegion;
    x.v24.SetBooleanArrayRegion = SetBooleanArrayRegion;
    x.v24.SetByteArrayRegion = SetByteArrayRegion;
    x.v24.SetCharArrayRegion = SetCharArrayRegion;
    x.v24.SetShortArrayRegion = SetShortArrayRegion;
    x.v24.SetIntArrayRegion = SetIntArrayRegion;
    x.v24.SetLongArrayRegion = SetLongArrayRegion;
    x.v24.SetFloatArrayRegion = SetFloatArrayRegion;
    x.v24.SetDoubleArrayRegion = SetDoubleArrayRegion;
    x.v24.RegisterNatives = RegisterNatives;
    x.v24.UnregisterNatives = UnregisterNatives;
    x.v24.MonitorEnter = MonitorEnter;
    x.v24.MonitorExit = MonitorExit;
    x.v24.GetJavaVM = GetJavaVM;
    x.v24.GetStringRegion = GetStringRegion;
    x.v24.GetStringUTFRegion = GetStringUTFRegion;
    x.v24.GetPrimitiveArrayCritical = GetPrimitiveArrayCritical;
    x.v24.ReleasePrimitiveArrayCritical = ReleasePrimitiveArrayCritical;
    x.v24.GetStringCritical = GetStringCritical;
    x.v24.ReleaseStringCritical = ReleaseStringCritical;
    x.v24.NewWeakGlobalRef = NewWeakGlobalRef;
    x.v24.DeleteWeakGlobalRef = DeleteWeakGlobalRef;
    x.v24.NewDirectByteBuffer = NewDirectByteBuffer;
    x.v24.GetDirectBufferAddress = GetDirectBufferAddress;
    x.v24.GetDirectBufferCapacity = GetDirectBufferCapacity;
    x.v24.GetObjectRefType = GetObjectRefType;
    x.v24.GetModule = GetModule;
    x.v24.IsVirtualThread = IsVirtualThread;
    x.v24.GetStringUTFLengthAsLong = GetStringUTFLengthAsLong;

    Wrapper(x)
};
