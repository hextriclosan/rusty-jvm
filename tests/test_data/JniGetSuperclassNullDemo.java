package samples.javacore.loadlibrary.classops;

public class JniGetSuperclassNullDemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native Class<?> getSuperclass(Class<?> target);

    public static void main(String[] args) {
        System.out.println("string=" + getSuperclass(String.class).getName());
        System.out.println("object=" + getSuperclass(Object.class));
        try {
            getSuperclass(null);
        } catch (NullPointerException exception) {
            System.out.println("null caught");
        }
    }
}
