mod array_operations_demo;
mod string_operations_demo;

use jni::errors::ThrowRuntimeExAndDefault;
use jni::objects::{JClass, JObject};
use jni::sys::{jboolean, jbyte, jdouble, jfloat, jint, jlong};
use jni::EnvUnowned;

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
