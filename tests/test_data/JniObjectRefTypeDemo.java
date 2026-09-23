package samples.javacore.loadlibrary.objectops;

public class JniObjectRefTypeDemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native int refType(Object object);

    public static void main(String[] args) {
        System.out.println("object=" + refType(new Object()));
        System.out.println("array=" + refType(new int[1]));
        System.out.println("class=" + refType(String.class));
        System.out.println("null=" + refType(null));
    }
}
