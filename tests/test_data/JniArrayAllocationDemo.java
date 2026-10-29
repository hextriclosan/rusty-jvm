package samples.javacore.loadlibrary.arrayops;

public class JniArrayAllocationDemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native Object[] create(Class<?> component);

    private static native Object newObjectArray(Class<?> component, int length);

    private static native Object newIntArray(int length);

    public static void main(String[] args) {
        report("NewObjectArray(String, 2)", newObjectArray(String.class, 2));
        report("NewObjectArray(String, -1)", newObjectArray(String.class, -1));
        report("NewIntArray(3)", newIntArray(3));
        report("NewIntArray(-5)", newIntArray(-5));
        report("NewObjectArray(null, 1)", newObjectArray(null, 1));

        try {
            create(null);
        } catch (NullPointerException exception) {
            System.out.println("null component caught: " + exception.getMessage());
        }
    }

    private static void report(String call, Object result) {
        if (result instanceof Throwable throwable) {
            System.out.println(call + " -> null, pending " + throwable.getClass().getName() + ": " + throwable.getMessage());
        } else if (result == null) {
            System.out.println(call + " -> null without pending exception");
        } else {
            System.out.println(call + " -> " + result.getClass().getName());
        }
    }
}
