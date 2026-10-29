package samples.javacore.loadlibrary.monitor;

import java.util.concurrent.atomic.AtomicBoolean;

public class JniMonitorDemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native int roundTrip(Object object);
    private static native int exit(Object object);
    private static native int enter(Object object);

    public static void main(String[] args) throws Exception {
        Object monitor = new Object();
        System.out.println("round trip=" + roundTrip(monitor));
        System.out.println("held after return=" + Thread.holdsLock(monitor));
        try {
            exit(monitor);
            System.out.println("missing exception");
        } catch (IllegalMonitorStateException expected) {
            System.out.println("illegal exit caught");
        }

        excludesSynchronizedBlocks();

        try {
            enter(null);
            System.out.println("missing exception");
        } catch (NullPointerException expected) {
            System.out.println("null enter caught");
        }
    }

    /// A monitor entered through JNI is the same monitor `synchronized` uses.
    private static void excludesSynchronizedBlocks() throws Exception {
        Object monitor = new Object();
        System.out.println("enter=" + enter(monitor));
        System.out.println("held after enter=" + Thread.holdsLock(monitor));

        AtomicBoolean acquired = new AtomicBoolean();
        Thread other = new Thread(() -> {
            synchronized (monitor) {
                acquired.set(true);
            }
        });
        other.start();
        long deadline = System.currentTimeMillis() + 10_000;
        while (other.getState() != Thread.State.BLOCKED && System.currentTimeMillis() < deadline) {
            Thread.sleep(1);
        }
        System.out.println("other thread " + other.getState() + ", acquired=" + acquired.get());

        System.out.println("exit=" + exit(monitor));
        other.join();
        System.out.println("other thread acquired after exit=" + acquired.get());
    }
}
