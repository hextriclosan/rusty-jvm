package samples.javacore.loadlibrary.objectops;

public class JniIsSameObjectDemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native boolean isSame(Object first, Object second);

    public static void main(String[] args) {
        Object first = new Object();
        Object second = new Object();
        System.out.println("same reference=" + isSame(first, first));
        System.out.println("different references=" + isSame(first, second));
        System.out.println("two nulls=" + isSame(null, null));
        System.out.println("object and null=" + isSame(first, null));
    }
}
