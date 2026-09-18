package samples.javacore.loadlibrary.arrayops;

public class JniObjectArrayStoreDemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native void set(Object[] array, Object value);

    public static void main(String[] args) {
        String[] strings = new String[1];
        try {
            set(strings, 42);
        } catch (ArrayStoreException exception) {
            System.out.println("incompatible caught");
        }
        Number[] numbers = new Number[1];
        set(numbers, 42);
        set(strings, null);
        System.out.println("number=" + numbers[0]);
        System.out.println("null=" + strings[0]);
    }
}
