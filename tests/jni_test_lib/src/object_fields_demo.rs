use jni::sys::{
    jboolean, jbyte, jchar, jdouble, jfieldID, jfloat, jint, jlong, jobject, jshort, jstring,
    JNIEnv,
};
use std::any::TypeId;
use std::ptr::null_mut;

macro_rules! process_field_impl {
    ($name:ident, $jni_ty:ty, $get:ident, $set:ident) => {
        #[no_mangle]
        pub extern "system" fn $name(
            env: *mut JNIEnv,
            obj: jobject,
            field_name: jstring,
            signature: jstring,
            new_value: $jni_ty,
        ) {
            unsafe {
                process_field::<$jni_ty>(
                    env,
                    obj,
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
process_field_impl!(
    Java_samples_javacore_loadlibrary_example_ObjectFieldDemo_processObjectStringField,
    jobject,
    GetObjectField,
    SetObjectField
);
process_field_impl!(
    Java_samples_javacore_loadlibrary_example_ObjectFieldDemo_processObjectBooleanField,
    jboolean,
    GetBooleanField,
    SetBooleanField
);
process_field_impl!(
    Java_samples_javacore_loadlibrary_example_ObjectFieldDemo_processObjectByteField,
    jbyte,
    GetByteField,
    SetByteField
);
process_field_impl!(
    Java_samples_javacore_loadlibrary_example_ObjectFieldDemo_processObjectCharField,
    jchar,
    GetCharField,
    SetCharField
);
process_field_impl!(
    Java_samples_javacore_loadlibrary_example_ObjectFieldDemo_processObjectShortField,
    jshort,
    GetShortField,
    SetShortField
);
process_field_impl!(
    Java_samples_javacore_loadlibrary_example_ObjectFieldDemo_processObjectIntField,
    jint,
    GetIntField,
    SetIntField
);
process_field_impl!(
    Java_samples_javacore_loadlibrary_example_ObjectFieldDemo_processObjectLongField,
    jlong,
    GetLongField,
    SetLongField
);
process_field_impl!(
    Java_samples_javacore_loadlibrary_example_ObjectFieldDemo_processObjectFloatField,
    jfloat,
    GetFloatField,
    SetFloatField
);
process_field_impl!(
    Java_samples_javacore_loadlibrary_example_ObjectFieldDemo_processObjectDoubleField,
    jdouble,
    GetDoubleField,
    SetDoubleField
);
unsafe fn process_field<T: Copy + std::fmt::Debug + 'static>(
    env: *mut JNIEnv,
    obj: jobject,
    field_name: jstring,
    signature: jstring,
    new_value: T,
    get_fn: unsafe extern "system" fn(*mut JNIEnv, jobject, jfieldID) -> T,
    set_fn: unsafe extern "system" fn(*mut JNIEnv, jobject, jfieldID, T),
) {
    let field = ((*(*env)).v24.GetStringUTFChars)(env, field_name, null_mut());
    let sig = ((*(*env)).v24.GetStringUTFChars)(env, signature, null_mut());
    let class = ((*(*env)).v24.GetObjectClass)(env, obj);
    let field_id = ((*(*env)).v24.GetFieldID)(env, class, field, sig);

    let current_value = get_fn(env, obj, field_id);
    if TypeId::of::<T>() == TypeId::of::<jobject>() {
        // todo invoke toString() here
        println!("current value=unknown");
    } else {
        println!("current value={current_value:?}");
    }
    set_fn(env, obj, field_id, new_value);

    ((*(*env)).v24.ReleaseStringUTFChars)(env, signature, sig);
    ((*(*env)).v24.ReleaseStringUTFChars)(env, field_name, field);
}
