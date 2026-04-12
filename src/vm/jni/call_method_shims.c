/*
 * call_method_shims.c
 *
 * Thin C shims that adapt the variadic Call<Type>Method JNI entry points to
 * the corresponding Call<Type>MethodV Rust implementations.
 *
 * Each shim:
 *   1. Collects the variadic arguments with va_start / va_end.
 *   2. Forwards the resulting va_list to the matching exported Rust symbol.
 *
 * The Rust symbols are declared with #[no_mangle] pub extern "C" in
 * instance_methods_impl.rs and static_methods_impl.rs.
 *
 * Only primitive C types are used here; no JNI headers are required.
 */

#include <stdarg.h>

/* ── Type aliases (mirrors jni_sys primitive types) ─────────────────────── */
typedef void*          JNIEnv_ptr;
typedef void*          jobject;
typedef void*          jclass;
typedef void*          jmethodID;
typedef unsigned char  jboolean;
typedef signed char    jbyte;
typedef unsigned short jchar;
typedef short          jshort;
typedef int            jint;
typedef long long      jlong;
typedef float          jfloat;
typedef double         jdouble;

/* ── Forward declarations of the exported Rust V-variant functions ────────
 *
 * On x86-64 the va_list parameter is passed as __va_list_tag* (a pointer),
 * which is ABI-compatible with the void* / va_list used in the Rust side.
 */

/* --- instance methods --- */
extern jobject   rusty_jvm_call_object_method_v (JNIEnv_ptr, jobject, jmethodID, va_list);
extern jboolean  rusty_jvm_call_boolean_method_v(JNIEnv_ptr, jobject, jmethodID, va_list);
extern jbyte     rusty_jvm_call_byte_method_v   (JNIEnv_ptr, jobject, jmethodID, va_list);
extern jchar     rusty_jvm_call_char_method_v   (JNIEnv_ptr, jobject, jmethodID, va_list);
extern jshort    rusty_jvm_call_short_method_v  (JNIEnv_ptr, jobject, jmethodID, va_list);
extern jint      rusty_jvm_call_int_method_v    (JNIEnv_ptr, jobject, jmethodID, va_list);
extern jlong     rusty_jvm_call_long_method_v   (JNIEnv_ptr, jobject, jmethodID, va_list);
extern jfloat    rusty_jvm_call_float_method_v  (JNIEnv_ptr, jobject, jmethodID, va_list);
extern jdouble   rusty_jvm_call_double_method_v (JNIEnv_ptr, jobject, jmethodID, va_list);
extern void      rusty_jvm_call_void_method_v   (JNIEnv_ptr, jobject, jmethodID, va_list);

/* --- non-virtual methods --- */
extern jobject   rusty_jvm_call_non_virtual_object_method_v (JNIEnv_ptr, jobject, jclass, jmethodID, va_list);
extern jboolean  rusty_jvm_call_non_virtual_boolean_method_v(JNIEnv_ptr, jobject, jclass, jmethodID, va_list);
extern jbyte     rusty_jvm_call_non_virtual_byte_method_v   (JNIEnv_ptr, jobject, jclass, jmethodID, va_list);
extern jchar     rusty_jvm_call_non_virtual_char_method_v   (JNIEnv_ptr, jobject, jclass, jmethodID, va_list);
extern jshort    rusty_jvm_call_non_virtual_short_method_v  (JNIEnv_ptr, jobject, jclass, jmethodID, va_list);
extern jint      rusty_jvm_call_non_virtual_int_method_v    (JNIEnv_ptr, jobject, jclass, jmethodID, va_list);
extern jlong     rusty_jvm_call_non_virtual_long_method_v   (JNIEnv_ptr, jobject, jclass, jmethodID, va_list);
extern jfloat    rusty_jvm_call_non_virtual_float_method_v  (JNIEnv_ptr, jobject, jclass, jmethodID, va_list);
extern jdouble   rusty_jvm_call_non_virtual_double_method_v (JNIEnv_ptr, jobject, jclass, jmethodID, va_list);
extern void      rusty_jvm_call_non_virtual_void_method_v   (JNIEnv_ptr, jobject, jclass, jmethodID, va_list);

/* --- static methods --- */
extern jobject   rusty_jvm_call_static_object_method_v (JNIEnv_ptr, jclass, jmethodID, va_list);
extern jboolean  rusty_jvm_call_static_boolean_method_v(JNIEnv_ptr, jclass, jmethodID, va_list);
extern jbyte     rusty_jvm_call_static_byte_method_v   (JNIEnv_ptr, jclass, jmethodID, va_list);
extern jchar     rusty_jvm_call_static_char_method_v   (JNIEnv_ptr, jclass, jmethodID, va_list);
extern jshort    rusty_jvm_call_static_short_method_v  (JNIEnv_ptr, jclass, jmethodID, va_list);
extern jint      rusty_jvm_call_static_int_method_v    (JNIEnv_ptr, jclass, jmethodID, va_list);
extern jlong     rusty_jvm_call_static_long_method_v   (JNIEnv_ptr, jclass, jmethodID, va_list);
extern jfloat    rusty_jvm_call_static_float_method_v  (JNIEnv_ptr, jclass, jmethodID, va_list);
extern jdouble   rusty_jvm_call_static_double_method_v (JNIEnv_ptr, jclass, jmethodID, va_list);
extern void      rusty_jvm_call_static_void_method_v   (JNIEnv_ptr, jclass, jmethodID, va_list);

/* ── Helper macro ────────────────────────────────────────────────────────── */

/* Shim for an instance method (env, obj, method_id, ...) -> ret */
#define INSTANCE_SHIM(ret, name, rust_fn)                                     \
ret call_##name##_method_shim(                                                \
        JNIEnv_ptr env, jobject obj, jmethodID mid, ...) {                    \
    va_list args;                                                              \
    va_start(args, mid);                                                      \
    ret result = rust_fn(env, obj, mid, args);                                \
    va_end(args);                                                              \
    return result;                                                             \
}

/* Shim for a void instance method */
#define INSTANCE_VOID_SHIM(name, rust_fn)                                     \
void call_##name##_method_shim(                                               \
        JNIEnv_ptr env, jobject obj, jmethodID mid, ...) {                    \
    va_list args;                                                              \
    va_start(args, mid);                                                      \
    rust_fn(env, obj, mid, args);                                             \
    va_end(args);                                                              \
}

/* Shim for a non-virtual method (env, obj, clazz, method_id, ...) -> ret */
#define NON_VIRTUAL_SHIM(ret, name, rust_fn)                                  \
ret call_non_virtual_##name##_method_shim(                                    \
        JNIEnv_ptr env, jobject obj, jclass clazz, jmethodID mid, ...) {      \
    va_list args;                                                              \
    va_start(args, mid);                                                      \
    ret result = rust_fn(env, obj, clazz, mid, args);                         \
    va_end(args);                                                              \
    return result;                                                             \
}

/* Shim for a void non-virtual method */
#define NON_VIRTUAL_VOID_SHIM(name, rust_fn)                                  \
void call_non_virtual_##name##_method_shim(                                   \
        JNIEnv_ptr env, jobject obj, jclass clazz, jmethodID mid, ...) {      \
    va_list args;                                                              \
    va_start(args, mid);                                                       \
    rust_fn(env, obj, clazz, mid, args);                                      \
    va_end(args);                                                              \
}

/* Shim for a static method (env, clazz, method_id, ...) -> ret */
#define STATIC_SHIM(ret, name, rust_fn)                                       \
ret call_static_##name##_method_shim(                                         \
        JNIEnv_ptr env, jclass clazz, jmethodID mid, ...) {                   \
    va_list args;                                                              \
    va_start(args, mid);                                                       \
    ret result = rust_fn(env, clazz, mid, args);                              \
    va_end(args);                                                              \
    return result;                                                             \
}

/* Shim for a void static method */
#define STATIC_VOID_SHIM(name, rust_fn)                                       \
void call_static_##name##_method_shim(                                        \
        JNIEnv_ptr env, jclass clazz, jmethodID mid, ...) {                   \
    va_list args;                                                              \
    va_start(args, mid);                                                       \
    rust_fn(env, clazz, mid, args);                                           \
    va_end(args);                                                              \
}

/* ── Instance method shims ───────────────────────────────────────────────── */
INSTANCE_SHIM     (jobject,  object,  rusty_jvm_call_object_method_v)
INSTANCE_SHIM     (jboolean, boolean, rusty_jvm_call_boolean_method_v)
INSTANCE_SHIM     (jbyte,    byte,    rusty_jvm_call_byte_method_v)
INSTANCE_SHIM     (jchar,    char,    rusty_jvm_call_char_method_v)
INSTANCE_SHIM     (jshort,   short,   rusty_jvm_call_short_method_v)
INSTANCE_SHIM     (jint,     int,     rusty_jvm_call_int_method_v)
INSTANCE_SHIM     (jlong,    long,    rusty_jvm_call_long_method_v)
INSTANCE_SHIM     (jfloat,   float,   rusty_jvm_call_float_method_v)
INSTANCE_SHIM     (jdouble,  double,  rusty_jvm_call_double_method_v)
INSTANCE_VOID_SHIM(          void,    rusty_jvm_call_void_method_v)

/* ── Non-virtual method shims ────────────────────────────────────────────── */
NON_VIRTUAL_SHIM     (jobject,  object,  rusty_jvm_call_non_virtual_object_method_v)
NON_VIRTUAL_SHIM     (jboolean, boolean, rusty_jvm_call_non_virtual_boolean_method_v)
NON_VIRTUAL_SHIM     (jbyte,    byte,    rusty_jvm_call_non_virtual_byte_method_v)
NON_VIRTUAL_SHIM     (jchar,    char,    rusty_jvm_call_non_virtual_char_method_v)
NON_VIRTUAL_SHIM     (jshort,   short,   rusty_jvm_call_non_virtual_short_method_v)
NON_VIRTUAL_SHIM     (jint,     int,     rusty_jvm_call_non_virtual_int_method_v)
NON_VIRTUAL_SHIM     (jlong,    long,    rusty_jvm_call_non_virtual_long_method_v)
NON_VIRTUAL_SHIM     (jfloat,   float,   rusty_jvm_call_non_virtual_float_method_v)
NON_VIRTUAL_SHIM     (jdouble,  double,  rusty_jvm_call_non_virtual_double_method_v)
NON_VIRTUAL_VOID_SHIM(          void,    rusty_jvm_call_non_virtual_void_method_v)

/* ── Static method shims ─────────────────────────────────────────────────── */
STATIC_SHIM     (jobject,  object,  rusty_jvm_call_static_object_method_v)
STATIC_SHIM     (jboolean, boolean, rusty_jvm_call_static_boolean_method_v)
STATIC_SHIM     (jbyte,    byte,    rusty_jvm_call_static_byte_method_v)
STATIC_SHIM     (jchar,    char,    rusty_jvm_call_static_char_method_v)
STATIC_SHIM     (jshort,   short,   rusty_jvm_call_static_short_method_v)
STATIC_SHIM     (jint,     int,     rusty_jvm_call_static_int_method_v)
STATIC_SHIM     (jlong,    long,    rusty_jvm_call_static_long_method_v)
STATIC_SHIM     (jfloat,   float,   rusty_jvm_call_static_float_method_v)
STATIC_SHIM     (jdouble,  double,  rusty_jvm_call_static_double_method_v)
STATIC_VOID_SHIM(           void,   rusty_jvm_call_static_void_method_v)
