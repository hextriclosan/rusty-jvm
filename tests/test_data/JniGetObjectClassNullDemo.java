package samples.javacore.loadlibrary.objectops;

public class JniGetObjectClassNullDemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native Class<?> getClass(Object object);

    public static void main(String[] args) {
        System.out.println("valid=" + getClass("value").getName());
        try {
            getClass(null);
        } catch (NullPointerException exception) {
            System.out.println("null caught");
        }
    }
}
