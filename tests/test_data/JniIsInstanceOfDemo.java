package samples.javacore.loadlibrary.objectops;

public class JniIsInstanceOfDemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native boolean isInstance(Object object, Class<?> target);

    public static void main(String[] args) {
        System.out.println("string/object=" + isInstance("value", Object.class));
        System.out.println("string/char-sequence=" + isInstance("value", CharSequence.class));
        System.out.println("string/number=" + isInstance("value", Number.class));
        System.out.println("int-array/object=" + isInstance(new int[0], Object.class));
        System.out.println("null/number=" + isInstance(null, Number.class));
    }
}
