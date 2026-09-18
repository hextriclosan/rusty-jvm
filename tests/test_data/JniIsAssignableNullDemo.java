package samples.javacore.loadlibrary.classops;

public class JniIsAssignableNullDemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native boolean isAssignable(Class<?> sub, Class<?> sup);

    public static void main(String[] args) {
        System.out.println("valid=" + isAssignable(String.class, Object.class));
        run("null sub", () -> isAssignable(null, Object.class));
        run("null sup", () -> isAssignable(String.class, null));
    }

    private static void run(String label, Runnable operation) {
        try {
            operation.run();
        } catch (NullPointerException exception) {
            System.out.println(label + " caught");
        }
    }
}
