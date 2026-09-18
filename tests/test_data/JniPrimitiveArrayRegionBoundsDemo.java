package samples.javacore.loadlibrary.arrayops;

import java.util.Arrays;

public class JniPrimitiveArrayRegionBoundsDemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native int[] get(int[] array, int start, int length);
    private static native void set(int[] array, int start, int length);
    private static native void getNullBuffer(int[] array, int start, int length);
    private static native void setNullBuffer(int[] array, int start, int length);

    public static void main(String[] args) {
        int[] values = new int[2];
        run("set in bounds", () -> set(values, 0, 2));
        run("get in bounds", () -> System.out.println("got " + Arrays.toString(get(values, 0, 2))));
        run("get empty at length", () -> get(values, 2, 0));
        run("get negative start", () -> get(values, -1, 1));
        run("get past end", () -> get(values, 1, 2));
        run("get overflow", () -> get(values, 1, Integer.MAX_VALUE));
        run("set negative length", () -> set(values, 0, -1));
        run("set past end", () -> set(values, 3, 0));
        run("get null", () -> get(null, 0, 1));
        run("set null", () -> set(null, 0, 1));
        run("get null buffer", () -> getNullBuffer(values, 0, 1));
        run("set null buffer", () -> setNullBuffer(values, 0, 1));
        run("set null buffer empty", () -> setNullBuffer(values, 0, 0));
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
