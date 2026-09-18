package samples.jni.registernatives;

public class RegisterNativesDemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native int register();
    private static native int unregister();
    private static native int registerMissing();
    private static native int registeredAdd(int left, int right);

    public static void main(String[] args) {
        System.out.println(register());
        System.out.println(registeredAdd(19, 23));
        System.out.println(unregister());
        System.out.println(register());
        System.out.println(registeredAdd(-10, 17));
        try {
            registerMissing();
        } catch (NoSuchMethodError error) {
            System.out.println(error.getMessage());
        }
    }
}
