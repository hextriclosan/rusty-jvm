package samples.javacore.loadlibrary.monitor;

public class JniMonitorDemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native int roundTrip(Object object);
    private static native int exitWithoutOwnership(Object object);

    public static void main(String[] args) {
        Object monitor = new Object();
        System.out.println("round trip=" + roundTrip(monitor));
        System.out.println("held after return=" + Thread.holdsLock(monitor));
        try {
            exitWithoutOwnership(monitor);
            System.out.println("missing exception");
        } catch (IllegalMonitorStateException expected) {
            System.out.println("illegal exit caught");
        }
    }
}
