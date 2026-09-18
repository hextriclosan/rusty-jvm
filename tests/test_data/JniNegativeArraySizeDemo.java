package samples.javacore.loadlibrary.arrayops;

public class JniNegativeArraySizeDemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native int[] newIntArray();
    private static native Object[] newObjectArray(Class<?> component);

    public static void main(String[] args) {
        try {
            newIntArray();
        } catch (NegativeArraySizeException exception) {
            System.out.println("primitive=" + exception.getMessage());
        }
        try {
            newObjectArray(String.class);
        } catch (NegativeArraySizeException exception) {
            System.out.println("object=" + exception.getMessage());
        }
    }
}
