mod array_operations_demo;
mod object_operations_demo;
mod static_fields_demo;
mod string_operations_demo;

use jni::elements::ReleaseMode::NoCopyBack;
use jni::errors::ThrowRuntimeExAndDefault;
use jni::objects::{JClass, JIntArray, JObject, JString};
use jni::sys::{jboolean, jbyte, jdouble, jfloat, jint, jintArray, jlong, jstring, JNIEnv};
use jni::EnvUnowned;
use std::ptr::null_mut;

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_LoadLibraryExample_sum__BB(
    mut _env: EnvUnowned,
    _class: JClass,
    a: jbyte,
    b: jbyte,
) -> jbyte {
    a.wrapping_add(b)
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_LoadLibraryExample_sum__II(
    mut _env: EnvUnowned,
    _class: JClass,
    a: jint,
    b: jint,
) -> jint {
    a.wrapping_add(b)
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_LoadLibraryExample_sum__JJ(
    mut _env: EnvUnowned,
    _class: JClass,
    a: jlong,
    b: jlong,
) -> jlong {
    a.wrapping_add(b)
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_LoadLibraryExample_multiply(
    mut _env: EnvUnowned,
    _class: JClass,
    a: jdouble,
    b: jdouble,
) -> jdouble {
    a * b
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_LoadLibraryExample_sumInstance(
    mut _env: EnvUnowned,
    _obj: JObject,
    a: jint,
    b: jint,
) -> jint {
    a.wrapping_add(b)
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_LoadLibraryExample_isPositive(
    mut _env: EnvUnowned,
    _class: JClass,
    value: jint,
) -> jboolean {
    (value > 0) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_LoadLibraryExample_printFloat(
    mut _env: EnvUnowned,
    _class: JClass,
    value: jfloat,
) {
    println!("Float value is: {value}");
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_LoadLibraryExample_getJniVersion(
    mut unowned_env: EnvUnowned,
    _class: JClass,
) -> jint {
    let version = unowned_env
        .with_env(|env| {
            let jni_version = env.version().expect("Failed to get JNI version");
            Ok::<jint, jni::errors::Error>(jni_version.into())
        })
        .resolve::<ThrowRuntimeExAndDefault>();

    version
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_LoadLibraryExample_arraySum(
    mut unowned_env: EnvUnowned,
    _class: JClass,
    arr: jintArray,
) -> jint {
    unowned_env
        .with_env(|env| {
            let arr = unsafe { JIntArray::from_raw(env, arr) };
            let length = arr.len(env).expect("Failed to get array length");

            let elements = unsafe {
                arr.get_elements(env, NoCopyBack)
                    .expect("Failed to get array elements")
            };

            // SAFETY: JNI guarantees pointer is valid for `length` elements
            let slice = unsafe { std::slice::from_raw_parts(elements.as_ptr(), length) };

            let sum: jint = slice.iter().copied().sum();

            Ok::<jint, jni::errors::Error>(sum)
        })
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_LoadLibraryExample_hello(
    mut unowned_env: EnvUnowned,
    _class: JClass,
    input: jstring,
) -> jstring {
    unowned_env
        .with_env(|env| {
            let input = unsafe { JString::from_raw(env, input) };
            let input_str = input.to_string();
            let output_str = format!("Hello, {input_str}!");
            let output_jstring = env.new_string(output_str)?;

            Ok::<jstring, jni::errors::Error>(output_jstring.into_raw())
        })
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_LoadLibraryExample_printMessage(
    env: *mut JNIEnv,
    _class: JClass,
    to_print: jstring,
) {
    let len = unsafe { ((*(*env)).v24.GetStringLength)(env, to_print) } as usize;
    let chars = unsafe { ((*(*env)).v24.GetStringChars)(env, to_print, null_mut()) };
    let slice = unsafe { std::slice::from_raw_parts(chars, len) };
    let constructed = String::from_utf16(slice).expect("Failed to convert to Rust String");
    println!("Message from Java: {constructed}");
    unsafe {
        ((*(*env)).v24.ReleaseStringChars)(env, to_print, chars);
    };
}
