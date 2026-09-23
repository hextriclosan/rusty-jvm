package samples.javacore.loadlibrary.stringops;

public class JniStringCharsNullDemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native boolean chars(String string);
    private static native boolean utfChars(String string);
    private static native boolean critical(String string);

    public static void main(String[] args) {
        run("chars", () -> chars(null));
        run("utf", () -> utfChars(null));
        run("critical", () -> critical(null));
    }

    private static void run(String label, Runnable operation) {
        try {
            operation.run();
        } catch (NullPointerException exception) {
            System.out.println(label + " caught");
        }
    }
}
