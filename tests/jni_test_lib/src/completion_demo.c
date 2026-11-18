#include <jni.h>
#include <stdarg.h>

#define ARGUMENTS JNI_TRUE, -101, 0xabcd, -23456, 123456, (jlong)0x1122334455667788LL, (jdouble)1.25, (jdouble)-2.5, receiver, 1, 2, 3, 4, 5, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0
#define SIGNATURE "(ZBCSIJFDLjava/lang/Object;IIIIIDDDDDDDD)"

#define DEFINE_LIST_CALL(Name, Type) \
static Type call_##Name##_list(JNIEnv *env, jobject receiver, jmethodID method, ...) { \
    va_list arguments; \
    va_start(arguments, method); \
    Type result = (*env)->Call##Name##MethodV(env, receiver, method, arguments); \
    va_end(arguments); \
    return result; \
} \
static Type nonvirtual_##Name##_list(JNIEnv *env, jobject receiver, jclass target, jmethodID method, ...) { \
    va_list arguments; \
    va_start(arguments, method); \
    Type result = (*env)->CallNonvirtual##Name##MethodV(env, receiver, target, method, arguments); \
    va_end(arguments); \
    return result; \
} \
static Type static_##Name##_list(JNIEnv *env, jclass target, jmethodID method, ...) { \
    va_list arguments; \
    va_start(arguments, method); \
    Type result = (*env)->CallStatic##Name##MethodV(env, target, method, arguments); \
    va_end(arguments); \
    return result; \
}

DEFINE_LIST_CALL(Object, jobject)
DEFINE_LIST_CALL(Boolean, jboolean)
DEFINE_LIST_CALL(Byte, jbyte)
DEFINE_LIST_CALL(Char, jchar)
DEFINE_LIST_CALL(Short, jshort)
DEFINE_LIST_CALL(Int, jint)
DEFINE_LIST_CALL(Long, jlong)
DEFINE_LIST_CALL(Float, jfloat)
DEFINE_LIST_CALL(Double, jdouble)

static void call_Void_list(JNIEnv *env, jobject receiver, jmethodID method, ...) {
    va_list arguments;
    va_start(arguments, method);
    (*env)->CallVoidMethodV(env, receiver, method, arguments);
    va_end(arguments);
}
static void nonvirtual_Void_list(JNIEnv *env, jobject receiver, jclass target, jmethodID method, ...) {
    va_list arguments;
    va_start(arguments, method);
    (*env)->CallNonvirtualVoidMethodV(env, receiver, target, method, arguments);
    va_end(arguments);
}
static void static_Void_list(JNIEnv *env, jclass target, jmethodID method, ...) {
    va_list arguments;
    va_start(arguments, method);
    (*env)->CallStaticVoidMethodV(env, target, method, arguments);
    va_end(arguments);
}
static jobject new_object_list(JNIEnv *env, jclass target, jmethodID method, ...) {
    va_list arguments;
    va_start(arguments, method);
    jobject result = (*env)->NewObjectV(env, target, method, arguments);
    va_end(arguments);
    return result;
}
static jint twice_list(JNIEnv *env, jobject receiver, jmethodID method, ...) {
    va_list arguments;
    va_start(arguments, method);
    jint first = (*env)->CallIntMethodV(env, receiver, method, arguments);
    jint second = (*env)->CallIntMethodV(env, receiver, method, arguments);
    va_end(arguments);
    return first == 654321 && second == first;
}
jint jni_completion_variadic(JNIEnv *env, jobject receiver, jclass base) {
    jmethodID method;
    jmethodID static_method;
    method = (*env)->GetMethodID(env, base, "valueObject", SIGNATURE "Ljava/lang/Object;");
    static_method = (*env)->GetStaticMethodID(env, base, "staticObject", SIGNATURE "Ljava/lang/Object;");
    if (!method || !static_method) return 1;
    if (!(*env)->IsSameObject(env, (*env)->CallObjectMethod(env, receiver, method, ARGUMENTS), receiver)) return 2;
    if ((*env)->ExceptionCheck(env)) return 3;
    if (!(*env)->IsSameObject(env, call_Object_list(env, receiver, method, ARGUMENTS), receiver)) return 4;
    if ((*env)->ExceptionCheck(env)) return 5;
    if (!(*env)->IsSameObject(env, (*env)->CallNonvirtualObjectMethod(env, receiver, base, method, ARGUMENTS), receiver)) return 6;
    if ((*env)->ExceptionCheck(env)) return 7;
    if (!(*env)->IsSameObject(env, nonvirtual_Object_list(env, receiver, base, method, ARGUMENTS), receiver)) return 8;
    if ((*env)->ExceptionCheck(env)) return 9;
    if (!(*env)->IsSameObject(env, (*env)->CallStaticObjectMethod(env, base, static_method, ARGUMENTS), receiver)) return 10;
    if ((*env)->ExceptionCheck(env)) return 11;
    if (!(*env)->IsSameObject(env, static_Object_list(env, base, static_method, ARGUMENTS), receiver)) return 12;
    if ((*env)->ExceptionCheck(env)) return 13;
    method = (*env)->GetMethodID(env, base, "valueBoolean", SIGNATURE "Z");
    static_method = (*env)->GetStaticMethodID(env, base, "staticBoolean", SIGNATURE "Z");
    if (!method || !static_method) return 14;
    if ((*env)->CallBooleanMethod(env, receiver, method, ARGUMENTS) != JNI_TRUE) return 15;
    if ((*env)->ExceptionCheck(env)) return 16;
    if (call_Boolean_list(env, receiver, method, ARGUMENTS) != JNI_TRUE) return 17;
    if ((*env)->ExceptionCheck(env)) return 18;
    if ((*env)->CallNonvirtualBooleanMethod(env, receiver, base, method, ARGUMENTS) != JNI_TRUE) return 19;
    if ((*env)->ExceptionCheck(env)) return 20;
    if (nonvirtual_Boolean_list(env, receiver, base, method, ARGUMENTS) != JNI_TRUE) return 21;
    if ((*env)->ExceptionCheck(env)) return 22;
    if ((*env)->CallStaticBooleanMethod(env, base, static_method, ARGUMENTS) != JNI_TRUE) return 23;
    if ((*env)->ExceptionCheck(env)) return 24;
    if (static_Boolean_list(env, base, static_method, ARGUMENTS) != JNI_TRUE) return 25;
    if ((*env)->ExceptionCheck(env)) return 26;
    method = (*env)->GetMethodID(env, base, "valueByte", SIGNATURE "B");
    static_method = (*env)->GetStaticMethodID(env, base, "staticByte", SIGNATURE "B");
    if (!method || !static_method) return 27;
    if ((*env)->CallByteMethod(env, receiver, method, ARGUMENTS) != -101) return 28;
    if ((*env)->ExceptionCheck(env)) return 29;
    if (call_Byte_list(env, receiver, method, ARGUMENTS) != -101) return 30;
    if ((*env)->ExceptionCheck(env)) return 31;
    if ((*env)->CallNonvirtualByteMethod(env, receiver, base, method, ARGUMENTS) != -101) return 32;
    if ((*env)->ExceptionCheck(env)) return 33;
    if (nonvirtual_Byte_list(env, receiver, base, method, ARGUMENTS) != -101) return 34;
    if ((*env)->ExceptionCheck(env)) return 35;
    if ((*env)->CallStaticByteMethod(env, base, static_method, ARGUMENTS) != -101) return 36;
    if ((*env)->ExceptionCheck(env)) return 37;
    if (static_Byte_list(env, base, static_method, ARGUMENTS) != -101) return 38;
    if ((*env)->ExceptionCheck(env)) return 39;
    method = (*env)->GetMethodID(env, base, "valueChar", SIGNATURE "C");
    static_method = (*env)->GetStaticMethodID(env, base, "staticChar", SIGNATURE "C");
    if (!method || !static_method) return 40;
    if ((*env)->CallCharMethod(env, receiver, method, ARGUMENTS) != 0xabcd) return 41;
    if ((*env)->ExceptionCheck(env)) return 42;
    if (call_Char_list(env, receiver, method, ARGUMENTS) != 0xabcd) return 43;
    if ((*env)->ExceptionCheck(env)) return 44;
    if ((*env)->CallNonvirtualCharMethod(env, receiver, base, method, ARGUMENTS) != 0xabcd) return 45;
    if ((*env)->ExceptionCheck(env)) return 46;
    if (nonvirtual_Char_list(env, receiver, base, method, ARGUMENTS) != 0xabcd) return 47;
    if ((*env)->ExceptionCheck(env)) return 48;
    if ((*env)->CallStaticCharMethod(env, base, static_method, ARGUMENTS) != 0xabcd) return 49;
    if ((*env)->ExceptionCheck(env)) return 50;
    if (static_Char_list(env, base, static_method, ARGUMENTS) != 0xabcd) return 51;
    if ((*env)->ExceptionCheck(env)) return 52;
    method = (*env)->GetMethodID(env, base, "valueShort", SIGNATURE "S");
    static_method = (*env)->GetStaticMethodID(env, base, "staticShort", SIGNATURE "S");
    if (!method || !static_method) return 53;
    if ((*env)->CallShortMethod(env, receiver, method, ARGUMENTS) != -23456) return 54;
    if ((*env)->ExceptionCheck(env)) return 55;
    if (call_Short_list(env, receiver, method, ARGUMENTS) != -23456) return 56;
    if ((*env)->ExceptionCheck(env)) return 57;
    if ((*env)->CallNonvirtualShortMethod(env, receiver, base, method, ARGUMENTS) != -23456) return 58;
    if ((*env)->ExceptionCheck(env)) return 59;
    if (nonvirtual_Short_list(env, receiver, base, method, ARGUMENTS) != -23456) return 60;
    if ((*env)->ExceptionCheck(env)) return 61;
    if ((*env)->CallStaticShortMethod(env, base, static_method, ARGUMENTS) != -23456) return 62;
    if ((*env)->ExceptionCheck(env)) return 63;
    if (static_Short_list(env, base, static_method, ARGUMENTS) != -23456) return 64;
    if ((*env)->ExceptionCheck(env)) return 65;
    method = (*env)->GetMethodID(env, base, "valueInt", SIGNATURE "I");
    static_method = (*env)->GetStaticMethodID(env, base, "staticInt", SIGNATURE "I");
    if (!method || !static_method) return 66;
    if ((*env)->CallIntMethod(env, receiver, method, ARGUMENTS) != 654321) return 67;
    if ((*env)->ExceptionCheck(env)) return 68;
    if (call_Int_list(env, receiver, method, ARGUMENTS) != 654321) return 69;
    if ((*env)->ExceptionCheck(env)) return 70;
    if ((*env)->CallNonvirtualIntMethod(env, receiver, base, method, ARGUMENTS) != 123456) return 71;
    if ((*env)->ExceptionCheck(env)) return 72;
    if (nonvirtual_Int_list(env, receiver, base, method, ARGUMENTS) != 123456) return 73;
    if ((*env)->ExceptionCheck(env)) return 74;
    if ((*env)->CallStaticIntMethod(env, base, static_method, ARGUMENTS) != 123456) return 75;
    if ((*env)->ExceptionCheck(env)) return 76;
    if (static_Int_list(env, base, static_method, ARGUMENTS) != 123456) return 77;
    if ((*env)->ExceptionCheck(env)) return 78;
    method = (*env)->GetMethodID(env, base, "valueLong", SIGNATURE "J");
    static_method = (*env)->GetStaticMethodID(env, base, "staticLong", SIGNATURE "J");
    if (!method || !static_method) return 79;
    if ((*env)->CallLongMethod(env, receiver, method, ARGUMENTS) != (jlong)0x1122334455667788LL) return 80;
    if ((*env)->ExceptionCheck(env)) return 81;
    if (call_Long_list(env, receiver, method, ARGUMENTS) != (jlong)0x1122334455667788LL) return 82;
    if ((*env)->ExceptionCheck(env)) return 83;
    if ((*env)->CallNonvirtualLongMethod(env, receiver, base, method, ARGUMENTS) != (jlong)0x1122334455667788LL) return 84;
    if ((*env)->ExceptionCheck(env)) return 85;
    if (nonvirtual_Long_list(env, receiver, base, method, ARGUMENTS) != (jlong)0x1122334455667788LL) return 86;
    if ((*env)->ExceptionCheck(env)) return 87;
    if ((*env)->CallStaticLongMethod(env, base, static_method, ARGUMENTS) != (jlong)0x1122334455667788LL) return 88;
    if ((*env)->ExceptionCheck(env)) return 89;
    if (static_Long_list(env, base, static_method, ARGUMENTS) != (jlong)0x1122334455667788LL) return 90;
    if ((*env)->ExceptionCheck(env)) return 91;
    method = (*env)->GetMethodID(env, base, "valueFloat", SIGNATURE "F");
    static_method = (*env)->GetStaticMethodID(env, base, "staticFloat", SIGNATURE "F");
    if (!method || !static_method) return 92;
    if ((*env)->CallFloatMethod(env, receiver, method, ARGUMENTS) != 1.25f) return 93;
    if ((*env)->ExceptionCheck(env)) return 94;
    if (call_Float_list(env, receiver, method, ARGUMENTS) != 1.25f) return 95;
    if ((*env)->ExceptionCheck(env)) return 96;
    if ((*env)->CallNonvirtualFloatMethod(env, receiver, base, method, ARGUMENTS) != 1.25f) return 97;
    if ((*env)->ExceptionCheck(env)) return 98;
    if (nonvirtual_Float_list(env, receiver, base, method, ARGUMENTS) != 1.25f) return 99;
    if ((*env)->ExceptionCheck(env)) return 100;
    if ((*env)->CallStaticFloatMethod(env, base, static_method, ARGUMENTS) != 1.25f) return 101;
    if ((*env)->ExceptionCheck(env)) return 102;
    if (static_Float_list(env, base, static_method, ARGUMENTS) != 1.25f) return 103;
    if ((*env)->ExceptionCheck(env)) return 104;
    method = (*env)->GetMethodID(env, base, "valueDouble", SIGNATURE "D");
    static_method = (*env)->GetStaticMethodID(env, base, "staticDouble", SIGNATURE "D");
    if (!method || !static_method) return 105;
    if ((*env)->CallDoubleMethod(env, receiver, method, ARGUMENTS) != -2.5) return 106;
    if ((*env)->ExceptionCheck(env)) return 107;
    if (call_Double_list(env, receiver, method, ARGUMENTS) != -2.5) return 108;
    if ((*env)->ExceptionCheck(env)) return 109;
    if ((*env)->CallNonvirtualDoubleMethod(env, receiver, base, method, ARGUMENTS) != -2.5) return 110;
    if ((*env)->ExceptionCheck(env)) return 111;
    if (nonvirtual_Double_list(env, receiver, base, method, ARGUMENTS) != -2.5) return 112;
    if ((*env)->ExceptionCheck(env)) return 113;
    if ((*env)->CallStaticDoubleMethod(env, base, static_method, ARGUMENTS) != -2.5) return 114;
    if ((*env)->ExceptionCheck(env)) return 115;
    if (static_Double_list(env, base, static_method, ARGUMENTS) != -2.5) return 116;
    if ((*env)->ExceptionCheck(env)) return 117;
    method = (*env)->GetMethodID(env, base, "valueVoid", SIGNATURE "V");
    static_method = (*env)->GetStaticMethodID(env, base, "staticVoid", SIGNATURE "V");
    if (!method || !static_method) return 118;
    (*env)->CallVoidMethod(env, receiver, method, ARGUMENTS);
    if ((*env)->ExceptionCheck(env)) return 119;
    call_Void_list(env, receiver, method, ARGUMENTS);
    if ((*env)->ExceptionCheck(env)) return 120;
    (*env)->CallNonvirtualVoidMethod(env, receiver, base, method, ARGUMENTS);
    if ((*env)->ExceptionCheck(env)) return 121;
    nonvirtual_Void_list(env, receiver, base, method, ARGUMENTS);
    if ((*env)->ExceptionCheck(env)) return 122;
    (*env)->CallStaticVoidMethod(env, base, static_method, ARGUMENTS);
    if ((*env)->ExceptionCheck(env)) return 123;
    static_Void_list(env, base, static_method, ARGUMENTS);
    if ((*env)->ExceptionCheck(env)) return 124;
    jfieldID count = (*env)->GetStaticFieldID(env, base, "voidCalls", "I");
    if ((*env)->GetStaticIntField(env, base, count) != 6) return 200;
    method = (*env)->GetMethodID(env, base, "valueInt", SIGNATURE "I");
    if (!twice_list(env, receiver, method, ARGUMENTS)) return 201;
    method = (*env)->GetMethodID(env, base, "<init>", SIGNATURE "V");
    jfieldID marker = (*env)->GetFieldID(env, base, "marker", "I");
    jobject first = (*env)->NewObject(env, base, method, ARGUMENTS);
    jobject second = new_object_list(env, base, method, ARGUMENTS);
    if (!first || !second || (*env)->ExceptionCheck(env)) return 202;
    if ((*env)->GetIntField(env, first, marker) != 42 || (*env)->GetIntField(env, second, marker) != 42) return 203;
    method = (*env)->GetMethodID(env, base, "zero", "()I");
    if ((*env)->CallIntMethod(env, receiver, method) != 11) return 204;
    if (call_Int_list(env, receiver, method) != 11) return 205;
    if ((*env)->CallNonvirtualIntMethod(env, receiver, base, method) != 9) return 206;
    if (nonvirtual_Int_list(env, receiver, base, method) != 9) return 207;
    method = (*env)->GetStaticMethodID(env, base, "staticZero", "()I");
    if ((*env)->CallStaticIntMethod(env, base, method) != 10 || static_Int_list(env, base, method) != 10) return 208;
    method = (*env)->GetMethodID(env, base, "throwing", "()I");
    if ((*env)->CallIntMethod(env, receiver, method) != 0 || !(*env)->ExceptionCheck(env)) return 209;
    (*env)->ExceptionClear(env);
    if (call_Int_list(env, receiver, method) != 0 || !(*env)->ExceptionCheck(env)) return 210;
    (*env)->ExceptionClear(env);
    method = (*env)->GetMethodID(env, base, "throwing", "()I");
    if ((*env)->CallNonvirtualIntMethod(env, receiver, base, method) != 0 || !(*env)->ExceptionCheck(env)) return 211;
    (*env)->ExceptionClear(env);
    if (nonvirtual_Int_list(env, receiver, base, method) != 0 || !(*env)->ExceptionCheck(env)) return 212;
    (*env)->ExceptionClear(env);
    method = (*env)->GetStaticMethodID(env, base, "staticThrowing", "()I");
    if ((*env)->CallStaticIntMethod(env, base, method) != 0 || !(*env)->ExceptionCheck(env)) return 213;
    (*env)->ExceptionClear(env);
    if (static_Int_list(env, base, method) != 0 || !(*env)->ExceptionCheck(env)) return 214;
    (*env)->ExceptionClear(env);
    method = (*env)->GetMethodID(env, base, "<init>", "(I)V");
    if ((*env)->NewObject(env, base, method, -1) != NULL || !(*env)->ExceptionCheck(env)) return 215;
    (*env)->ExceptionClear(env);
    if (new_object_list(env, base, method, -1) != NULL || !(*env)->ExceptionCheck(env)) return 216;
    (*env)->ExceptionClear(env);
    return 0;
}
