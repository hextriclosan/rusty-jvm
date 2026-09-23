package samples.javacore.loadlibrary.arrayops;

public class JniObjectArrayNullClassDemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native Object[] create(Class<?> component);

    public static void main(String[] args) {
        try {
            create(null);
        } catch (NullPointerException exception) {
            System.out.println("null component caught");
        }
    }
}
