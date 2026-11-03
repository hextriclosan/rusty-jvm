package samples.javacore.loadlibrary.nullargs;

public class JniNullArgumentsDemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native int getArrayLength(Object array);
    private static native int getStringLength(String string);
    private static native int getStringUtfLength(String string);
    private static native long getStringUtfLengthAsLong(String string);
    private static native String getStringChars(String string);
    private static native String getStringUtfChars(String string);
    private static native int getStringCritical(String string);
    private static native String newStringFromNull(int length);
    private static native String newStringUtfFromNull();
    private static native Class<?> getObjectClass(Object object);
    private static native Class<?> getSuperclass(Class<?> clazz);
    private static native boolean isAssignableFrom(Class<?> sub, Class<?> sup);

    public static void main(String[] args) {
        run("GetArrayLength", () -> getArrayLength(new int[3]));
        run("GetStringLength", () -> getStringLength("value"));
        run("GetStringUTFLength", () -> getStringUtfLength("value"));
        run("GetStringUTFLengthAsLong", () -> getStringUtfLengthAsLong("value"));
        run("GetStringChars", () -> getStringChars("value"));
        run("GetStringUTFChars", () -> getStringUtfChars("value"));
        run("GetStringCritical", () -> getStringCritical("value"));
        run("NewString empty", () -> newStringFromNull(0));
        run("GetObjectClass", () -> getObjectClass("value"));
        run("GetSuperclass", () -> getSuperclass(String.class));
        run("GetSuperclass root", () -> getSuperclass(Object.class));
        run("IsAssignableFrom", () -> isAssignableFrom(String.class, Object.class));

        run("NewStringUTF null", () -> newStringUtfFromNull());
        run("NewString null", () -> newStringFromNull(1));
        run("GetArrayLength null", () -> getArrayLength(null));
        run("GetStringLength null", () -> getStringLength(null));
        run("GetStringUTFLength null", () -> getStringUtfLength(null));
        run("GetStringUTFLengthAsLong null", () -> getStringUtfLengthAsLong(null));
        run("GetStringChars null", () -> getStringChars(null));
        run("GetStringUTFChars null", () -> getStringUtfChars(null));
        run("GetStringCritical null", () -> getStringCritical(null));
        run("GetObjectClass null", () -> getObjectClass(null));
        run("GetSuperclass null", () -> getSuperclass(null));
        run("IsAssignableFrom null sub", () -> isAssignableFrom(null, Object.class));
        run("IsAssignableFrom null sup", () -> isAssignableFrom(String.class, null));
    }

    private interface Operation {
        Object run();
    }

    private static void run(String label, Operation operation) {
        try {
            Object result = operation.run();
            System.out.println(label + " = " + (result instanceof String ? "\"" + result + "\"" : result));
        } catch (RuntimeException exception) {
            System.out.println(label + " -> " + exception);
        }
    }
}
