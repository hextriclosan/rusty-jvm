package samples.jni.registernatives;

import java.util.function.Supplier;

class NativeParent {
    static native int parentValue();
}

class NativeChild extends NativeParent {
}

public class RegisterNativesDemo {
    // the functions of the test library that register binds
    private static final int NONE = 0;
    private static final int ADD = 1;
    private static final int SUBTRACT = 2;
    private static final int HUNDRED = 3;

    static {
        System.loadLibrary("jni_test_lib");
    }

    // the test library exports no JNI symbol for this one
    private static native int registered(int left, int right);
    // the test library exports a JNI symbol that multiplies
    private static native int exported(int left, int right);

    private static int notNative(int left, int right) {
        return 0;
    }

    private static native int registerAll(Class<?> target, String[] names, String[] signatures, int[] functions);
    private static native int registerCount(Class<?> target, int count);
    private static native int unregister(Class<?> target);

    private static int register(Class<?> target, String name, String signature, int function) {
        return registerAll(target, new String[] {name}, new String[] {signature}, new int[] {function});
    }

    public static void main(String[] args) {
        Class<?> demo = RegisterNativesDemo.class;
        print("register", () -> register(demo, "registered", "(II)I", ADD));
        print("call", () -> registered(19, 23));
        print("register another function", () -> register(demo, "registered", "(II)I", SUBTRACT));
        print("call", () -> registered(19, 23));

        print("call the JNI symbol", () -> exported(6, 7));
        print("register over the JNI symbol", () -> register(demo, "exported", "(II)I", ADD));
        print("call", () -> exported(6, 7));
        print("register no function", () -> register(demo, "exported", "(II)I", NONE));
        print("call", () -> exported(6, 7));
        print("register over the JNI symbol", () -> register(demo, "exported", "(II)I", ADD));
        print("unregister", () -> unregister(demo));
        print("call", () -> exported(6, 7));

        print("register a missing method", () -> register(demo, "toString", "()I", ADD));
        print("register a wrong signature", () -> register(demo, "registered", "(I)I", ADD));
        print("register a malformed signature", () -> register(demo, "registered", "(II", ADD));
        print("register a method that isn't native", () -> register(demo, "notNative", "(II)I", ADD));
        print("register a batch with a missing method", () -> registerAll(demo,
                new String[] {"registered", "toString"}, new String[] {"(II)I", "()I"}, new int[] {ADD, ADD}));
        print("call the method before it", () -> registered(19, 23));
        print("register a negative count", () -> registerCount(demo, -1));
        print("register a count of 0", () -> registerCount(demo, 0));

        print("call the JNI symbol of the superclass", NativeParent::parentValue);
        print("register through the subclass", () -> register(NativeChild.class, "parentValue", "()I", HUNDRED));
        print("call", NativeParent::parentValue);
        print("unregister the subclass", () -> unregister(NativeChild.class));
        print("call", NativeParent::parentValue);
        print("unregister the superclass", () -> unregister(NativeParent.class));
        print("call", NativeParent::parentValue);
    }

    private static void print(String label, Supplier<Object> action) {
        try {
            System.out.println(label + ": " + action.get());
        } catch (LinkageError e) {
            System.out.println(label + ": " + e);
        }
    }
}
