#include <jni.h>
#include <stdarg.h>

enum InvocationKind { VIRTUAL, NONVIRTUAL, STATIC, CONSTRUCTOR };

extern jvalue rusty_jvm_invoke_variadic(JNIEnv *, jobject, jclass, jmethodID, void *, jint);

void rusty_jvm_next_argument(void *arguments, char tag, jvalue *output) {
    va_list *list = (va_list *)arguments;
    switch (tag) {
        case 'Z': output->z = va_arg(*list, int) != 0; break;
        case 'B': output->b = (jbyte)va_arg(*list, int); break;
        case 'C': output->c = (jchar)va_arg(*list, int); break;
        case 'S': output->s = (jshort)va_arg(*list, int); break;
        case 'I': output->i = va_arg(*list, jint); break;
        case 'J': output->j = va_arg(*list, jlong); break;
        case 'F': output->f = (jfloat)va_arg(*list, jdouble); break;
        case 'D': output->d = va_arg(*list, jdouble); break;
        default: output->l = va_arg(*list, jobject); break;
    }
}

#define DEFINE_CALL(Name, Type, Member, Parameters, Receiver, Target, Kind) \
Type JNICALL rusty_jvm_##Name##V Parameters { \
    va_list copy; \
    va_copy(copy, arguments); \
    jvalue result = rusty_jvm_invoke_variadic(env, Receiver, Target, method, &copy, Kind); \
    va_end(copy); \
    return result.Member; \
}

#define VIRTUAL(Name, Type, Member) \
DEFINE_CALL(Call##Name##Method, Type, Member, \
    (JNIEnv *env, jobject receiver, jmethodID method, va_list arguments), receiver, NULL, VIRTUAL); \
Type rusty_jvm_Call##Name##Method(JNIEnv *env, jobject receiver, jmethodID method, ...) { \
    va_list arguments; \
    va_start(arguments, method); \
    Type result = rusty_jvm_Call##Name##MethodV(env, receiver, method, arguments); \
    va_end(arguments); \
    return result; \
}

#define NONVIRTUAL(Name, Type, Member) \
DEFINE_CALL(CallNonvirtual##Name##Method, Type, Member, \
    (JNIEnv *env, jobject receiver, jclass target, jmethodID method, va_list arguments), receiver, target, NONVIRTUAL); \
Type rusty_jvm_CallNonvirtual##Name##Method(JNIEnv *env, jobject receiver, jclass target, jmethodID method, ...) { \
    va_list arguments; \
    va_start(arguments, method); \
    Type result = rusty_jvm_CallNonvirtual##Name##MethodV(env, receiver, target, method, arguments); \
    va_end(arguments); \
    return result; \
}

#define STATIC(Name, Type, Member) \
DEFINE_CALL(CallStatic##Name##Method, Type, Member, \
    (JNIEnv *env, jclass target, jmethodID method, va_list arguments), NULL, target, STATIC); \
Type rusty_jvm_CallStatic##Name##Method(JNIEnv *env, jclass target, jmethodID method, ...) { \
    va_list arguments; \
    va_start(arguments, method); \
    Type result = rusty_jvm_CallStatic##Name##MethodV(env, target, method, arguments); \
    va_end(arguments); \
    return result; \
}

VIRTUAL(Object, jobject, l)
NONVIRTUAL(Object, jobject, l)
STATIC(Object, jobject, l)

VIRTUAL(Boolean, jboolean, z)
NONVIRTUAL(Boolean, jboolean, z)
STATIC(Boolean, jboolean, z)

VIRTUAL(Byte, jbyte, b)
NONVIRTUAL(Byte, jbyte, b)
STATIC(Byte, jbyte, b)

VIRTUAL(Char, jchar, c)
NONVIRTUAL(Char, jchar, c)
STATIC(Char, jchar, c)

VIRTUAL(Short, jshort, s)
NONVIRTUAL(Short, jshort, s)
STATIC(Short, jshort, s)

VIRTUAL(Int, jint, i)
NONVIRTUAL(Int, jint, i)
STATIC(Int, jint, i)

VIRTUAL(Long, jlong, j)
NONVIRTUAL(Long, jlong, j)
STATIC(Long, jlong, j)

VIRTUAL(Float, jfloat, f)
NONVIRTUAL(Float, jfloat, f)
STATIC(Float, jfloat, f)

VIRTUAL(Double, jdouble, d)
NONVIRTUAL(Double, jdouble, d)
STATIC(Double, jdouble, d)

void JNICALL rusty_jvm_CallVoidMethodV(JNIEnv *env, jobject receiver, jmethodID method, va_list arguments) {
    va_list copy;
    va_copy(copy, arguments);
    rusty_jvm_invoke_variadic(env, receiver, NULL, method, &copy, VIRTUAL);
    va_end(copy);
}
void rusty_jvm_CallVoidMethod(JNIEnv *env, jobject receiver, jmethodID method, ...) {
    va_list arguments;
    va_start(arguments, method);
    rusty_jvm_CallVoidMethodV(env, receiver, method, arguments);
    va_end(arguments);
}

void JNICALL rusty_jvm_CallNonvirtualVoidMethodV(JNIEnv *env, jobject receiver, jclass target, jmethodID method, va_list arguments) {
    va_list copy;
    va_copy(copy, arguments);
    rusty_jvm_invoke_variadic(env, receiver, target, method, &copy, NONVIRTUAL);
    va_end(copy);
}
void rusty_jvm_CallNonvirtualVoidMethod(JNIEnv *env, jobject receiver, jclass target, jmethodID method, ...) {
    va_list arguments;
    va_start(arguments, method);
    rusty_jvm_CallNonvirtualVoidMethodV(env, receiver, target, method, arguments);
    va_end(arguments);
}

void JNICALL rusty_jvm_CallStaticVoidMethodV(JNIEnv *env, jclass target, jmethodID method, va_list arguments) {
    va_list copy;
    va_copy(copy, arguments);
    rusty_jvm_invoke_variadic(env, NULL, target, method, &copy, STATIC);
    va_end(copy);
}
void rusty_jvm_CallStaticVoidMethod(JNIEnv *env, jclass target, jmethodID method, ...) {
    va_list arguments;
    va_start(arguments, method);
    rusty_jvm_CallStaticVoidMethodV(env, target, method, arguments);
    va_end(arguments);
}

jobject JNICALL rusty_jvm_NewObjectV(JNIEnv *env, jclass target, jmethodID method, va_list arguments) {
    va_list copy;
    va_copy(copy, arguments);
    jvalue result = rusty_jvm_invoke_variadic(env, NULL, target, method, &copy, CONSTRUCTOR);
    va_end(copy);
    return result.l;
}
jobject rusty_jvm_NewObject(JNIEnv *env, jclass target, jmethodID method, ...) {
    va_list arguments;
    va_start(arguments, method);
    jobject result = rusty_jvm_NewObjectV(env, target, method, arguments);
    va_end(arguments);
    return result;
}

