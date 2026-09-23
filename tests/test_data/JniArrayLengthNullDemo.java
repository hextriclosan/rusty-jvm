package samples.javacore.loadlibrary.arrayops;

public class JniArrayLengthNullDemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native int length(Object array);

    public static void main(String[] args) {
        System.out.println("valid=" + length(new int[3]));
        try {
            length(null);
        } catch (NullPointerException exception) {
            System.out.println("null caught");
        }
    }
}
