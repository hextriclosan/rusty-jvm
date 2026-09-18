package samples.javacore.loadlibrary.stringops;

public class JniStringRegionBoundsDemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native void get(String string, int start, int length);
    private static native void getUtf(String string, int start, int length);

    public static void main(String[] args) {
        run("utf16 negative", () -> get("value", -1, 1));
        run("utf8 past end", () -> getUtf("value", 3, 3));
        run("utf16 null", () -> get(null, 0, 1));
        run("utf8 null", () -> getUtf(null, 0, 1));
    }

    private static void run(String label, Runnable operation) {
        try {
            operation.run();
        } catch (RuntimeException exception) {
            System.out.println(label + "=" + exception.getClass().getSimpleName());
        }
    }
}
