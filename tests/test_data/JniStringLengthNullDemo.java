package samples.javacore.loadlibrary.stringops;

public class JniStringLengthNullDemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native int length(String string);
    private static native long utfLength(String string);

    public static void main(String[] args) {
        System.out.println("length=" + length("a☕"));
        System.out.println("utf length=" + utfLength("a☕"));
        run("length null", () -> length(null));
        run("utf null", () -> utfLength(null));
    }

    private static void run(String label, Runnable operation) {
        try {
            operation.run();
        } catch (NullPointerException exception) {
            System.out.println(label + " caught");
        }
    }
}
