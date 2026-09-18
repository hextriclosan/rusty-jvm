package samples.javacore.loadlibrary.arrayops;

public class JniPrimitiveArrayRegionBoundsDemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native void get(int[] array, int start, int length);
    private static native void set(int[] array, int start, int length);

    public static void main(String[] args) {
        int[] values = new int[2];
        run("get negative", () -> get(values, -1, 1));
        run("get past end", () -> get(values, 1, 2));
        run("set negative length", () -> set(values, 0, -1));
        run("set null", () -> set(null, 0, 1));
    }

    private static void run(String label, Runnable operation) {
        try {
            operation.run();
        } catch (RuntimeException exception) {
            System.out.println(label + "=" + exception.getClass().getSimpleName());
        }
    }
}
