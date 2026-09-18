package samples.javacore.loadlibrary.stringops;

public class JniNewStringNullDemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native String newString();
    private static native String newStringUtf();

    public static void main(String[] args) {
        run("utf16", JniNewStringNullDemo::newString);
        run("utf8", JniNewStringNullDemo::newStringUtf);
    }

    private static void run(String label, Runnable operation) {
        try {
            operation.run();
        } catch (NullPointerException exception) {
            System.out.println(label + " caught");
        }
    }
}
