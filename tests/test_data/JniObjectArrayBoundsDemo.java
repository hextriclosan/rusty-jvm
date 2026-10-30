package samples.javacore.loadlibrary.arrayops;

public class JniObjectArrayBoundsDemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native Object get(Object[] array, int index);
    private static native void set(Object[] array, int index, Object value);

    public static void main(String[] args) {
        Object[] values = new Object[2];
        run("set in bounds", () -> set(values, 1, "value"));
        run("get in bounds", () -> System.out.println("got " + get(values, 1)));
        run("get negative", () -> get(values, -1));
        run("get at length", () -> get(values, 2));
        run("set negative", () -> set(values, -1, "value"));
        run("set at length", () -> set(values, 2, "value"));
        run("get null", () -> get(null, 0));
        run("set null", () -> set(null, 0, "value"));
    }

    private static void run(String label, Runnable operation) {
        try {
            operation.run();
            System.out.println(label + " ok");
        } catch (RuntimeException exception) {
            System.out.println(label + " -> " + exception);
        }
    }
}
