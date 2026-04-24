package samples.javacore.loadlibrary.example;

public class JniNoSuchIdDemo {
    private int instanceField = 42;
    private static int staticField = 99;

    static {
        System.loadLibrary("jni_test_lib");
    }

    private native void lookupNonexistentFieldId();
    private static native void lookupNonexistentStaticFieldId();
    private native void lookupNonexistentMethodId();
    private static native void lookupNonexistentStaticMethodId();

    public static void main(String[] args) {
        JniNoSuchIdDemo obj = new JniNoSuchIdDemo();

        System.out.println("=== GetFieldID: nonexistent field ===");
        try {
            obj.lookupNonexistentFieldId();
            System.out.println("ERROR: expected NoSuchFieldError");
        } catch (NoSuchFieldError e) {
            System.out.println("Caught NoSuchFieldError: " + e.getMessage());
        }

        System.out.println("=== GetStaticFieldID: nonexistent field ===");
        try {
            lookupNonexistentStaticFieldId();
            System.out.println("ERROR: expected NoSuchFieldError");
        } catch (NoSuchFieldError e) {
            System.out.println("Caught NoSuchFieldError: " + e.getMessage());
        }

        System.out.println("=== GetMethodID: nonexistent method ===");
        try {
            obj.lookupNonexistentMethodId();
            System.out.println("ERROR: expected NoSuchMethodError");
        } catch (NoSuchMethodError e) {
            System.out.println("Caught NoSuchMethodError: " + e.getMessage());
        }

        System.out.println("=== GetStaticMethodID: nonexistent method ===");
        try {
            lookupNonexistentStaticMethodId();
            System.out.println("ERROR: expected NoSuchMethodError");
        } catch (NoSuchMethodError e) {
            System.out.println("Caught NoSuchMethodError: " + e.getMessage());
        }
    }
}
