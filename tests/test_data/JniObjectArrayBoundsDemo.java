package samples.javacore.loadlibrary.arrayops;

public class JniObjectArrayBoundsDemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native Object get(Object[] array, int index);
    private static native void set(Object[] array, int index, Object value);

    public static void main(String[] args) {
        Object[] values = new Object[2];
        run("get negative", () -> get(values, -1));
        run("set at length", () -> set(values, 2, "value"));
        run("get null", () -> get(null, 0));
        run("set null", () -> set(null, 0, "value"));
    }

    private static void run(String label, Runnable operation) {
        try {
            operation.run();
        } catch (RuntimeException exception) {
            System.out.println(label + "=" + exception.getClass().getSimpleName());
        }
    }
}
