use jni::objects::{JClass, JObject};
use jni::sys::{jboolean, jbyte, jdouble, jfloat, jint, jlong};
use jni::EnvUnowned;

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_LoadLibraryExample_sum__BB(
    _env: EnvUnowned,
    _class: JClass,
    a: jbyte,
    b: jbyte,
) -> jbyte {
    a.wrapping_add(b)
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_LoadLibraryExample_sum__II(
    _env: EnvUnowned,
    _class: JClass,
    a: jint,
    b: jint,
) -> jint {
    a.wrapping_add(b)
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_LoadLibraryExample_sum__JJ(
    _env: EnvUnowned,
    _class: JClass,
    a: jlong,
    b: jlong,
) -> jlong {
    a.wrapping_add(b)
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_LoadLibraryExample_multiply(
    _env: EnvUnowned,
    _class: JClass,
    a: jdouble,
    b: jdouble,
) -> jdouble {
    a * b
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_LoadLibraryExample_sumInstance(
    _env: EnvUnowned,
    _obj: JObject,
    a: jint,
    b: jint,
) -> jint {
    a.wrapping_add(b)
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_LoadLibraryExample_isPositive(
    _env: EnvUnowned,
    _class: JClass,
    value: jint,
) -> jboolean {
    (value > 0) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_LoadLibraryExample_printFloat(
    _env: EnvUnowned,
    _class: JClass,
    value: jfloat,
) {
    println!("Float value is: {value}");
}
