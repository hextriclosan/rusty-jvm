use jni::sys::{
    jboolean, jbyte, jchar, jclass, jdouble, jfieldID, jfloat, jint, jlong, jobject, jshort,
    jstring, JNIEnv,
};
use std::any::TypeId;
use std::ptr::null_mut;

macro_rules! process_static_field_impl {
    ($name:ident, $jni_ty:ty, $get:ident, $set:ident) => {
        #[no_mangle]
        pub extern "system" fn $name(
            env: *mut JNIEnv,
            class: jclass,
            field_name: jstring,
            signature: jstring,
            new_value: $jni_ty,
        ) {
            unsafe {
                process_static_field::<$jni_ty>(
                    env,
                    class,
                    field_name,
                    signature,
                    new_value,
                    (*(*env)).v24.$get,
                    (*(*env)).v24.$set,
                );
            }
        }
    };
}
process_static_field_impl!(
    Java_samples_javacore_loadlibrary_example_StaticFieldDemo_processStaticObjectField,
    jobject,
    GetStaticObjectField,
    SetStaticObjectField
);
process_static_field_impl!(
    Java_samples_javacore_loadlibrary_example_StaticFieldDemo_processStaticBooleanField,
    jboolean,
    GetStaticBooleanField,
    SetStaticBooleanField
);
process_static_field_impl!(
    Java_samples_javacore_loadlibrary_example_StaticFieldDemo_processStaticByteField,
    jbyte,
    GetStaticByteField,
    SetStaticByteField
);
process_static_field_impl!(
    Java_samples_javacore_loadlibrary_example_StaticFieldDemo_processStaticCharField,
    jchar,
    GetStaticCharField,
    SetStaticCharField
);
process_static_field_impl!(
    Java_samples_javacore_loadlibrary_example_StaticFieldDemo_processStaticShortField,
    jshort,
    GetStaticShortField,
    SetStaticShortField
);
process_static_field_impl!(
    Java_samples_javacore_loadlibrary_example_StaticFieldDemo_processStaticIntField,
    jint,
    GetStaticIntField,
    SetStaticIntField
);
process_static_field_impl!(
    Java_samples_javacore_loadlibrary_example_StaticFieldDemo_processStaticLongField,
    jlong,
    GetStaticLongField,
    SetStaticLongField
);
process_static_field_impl!(
    Java_samples_javacore_loadlibrary_example_StaticFieldDemo_processStaticFloatField,
    jfloat,
    GetStaticFloatField,
    SetStaticFloatField
);
process_static_field_impl!(
    Java_samples_javacore_loadlibrary_example_StaticFieldDemo_processStaticDoubleField,
    jdouble,
    GetStaticDoubleField,
    SetStaticDoubleField
);
unsafe fn process_static_field<T: Copy + std::fmt::Debug + 'static>(
    env: *mut JNIEnv,
    class: jclass,
    field_name: jstring,
    signature: jstring,
    new_value: T,
    get_fn: unsafe extern "system" fn(*mut JNIEnv, jclass, jfieldID) -> T,
    set_fn: unsafe extern "system" fn(*mut JNIEnv, jclass, jfieldID, T),
) {
    let field = ((*(*env)).v24.GetStringUTFChars)(env, field_name, null_mut());
    let sig = ((*(*env)).v24.GetStringUTFChars)(env, signature, null_mut());
    let field_id = ((*(*env)).v24.GetStaticFieldID)(env, class, field, sig);

    let current_value = get_fn(env, class, field_id);
    if TypeId::of::<T>() == TypeId::of::<jobject>() {
        // todo invoke toString() here
        println!("current value=unknown");
    } else {
        println!("current value={current_value:?}");
    }
    set_fn(env, class, field_id, new_value);

    ((*(*env)).v24.ReleaseStringUTFChars)(env, signature, sig);
    ((*(*env)).v24.ReleaseStringUTFChars)(env, field_name, field);
}
