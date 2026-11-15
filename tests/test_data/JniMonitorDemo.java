package samples.javacore.loadlibrary.monitor;

import java.util.concurrent.atomic.AtomicBoolean;

public class JniMonitorDemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native int roundTrip(Object object);
    private static native int exit(Object object);
    private static native int enter(Object object);

    private synchronized native boolean heldInNative();
    private static synchronized native boolean heldInStaticNative();
    private synchronized native void throwInNative();

    /// What the synchronized natives ask through JNI; `Thread.holdsLock` itself is native.
    private static boolean holdsLockOf(Object object) {
        return Thread.holdsLock(object);
    }

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

        synchronizedNatives();
        unbalancedExit();
        unbalancedBlockExit();
    }

    /// JNI MonitorExit of a synchronized block's monitor from inside an inner try: the block's own
    /// monitorexit then finds nothing to release and, as in HotSpot, raises nothing, so the inner
    /// handler is never a candidate and the block is left normally.
    private static void unbalancedBlockExit() {
        Object monitor = new Object();
        try {
            synchronized (monitor) {
                try {
                    System.out.println("jni exit inside synchronized block=" + exit(monitor));
                } catch (IllegalStateException wrongHandler) {
                    System.out.println("wrong handler");
                }
            }
            System.out.println("block left normally");
        } catch (IllegalMonitorStateException unexpected) {
            System.out.println("block monitorexit raised " + unexpected);
        }
        System.out.println("held after leaving the block=" + Thread.holdsLock(monitor));
    }

    /// A synchronized method that releases its own monitor through JNI before returning. The
    /// implicit exit then has nothing to release and, as in HotSpot, nothing to complain about:
    /// no exception, and none left pending to surface somewhere unrelated later.
    private static void unbalancedExit() {
        JniMonitorDemo demo = new JniMonitorDemo();
        System.out.println("exit of own monitor inside synchronized method=" + demo.exitOwnMonitor());
        System.out.println("held after exiting own monitor=" + Thread.holdsLock(demo));
    }

    private synchronized int exitOwnMonitor() {
        return exit(this);
    }

    /// A `synchronized native` method holds its monitor while the native runs, and gives it up
    /// afterwards, whether the native returned or left through a pending exception.
    private static void synchronizedNatives() {
        JniMonitorDemo demo = new JniMonitorDemo();
        System.out.println("held in synchronized native=" + demo.heldInNative());
        System.out.println("held after synchronized native=" + Thread.holdsLock(demo));
        System.out.println("held in static synchronized native=" + heldInStaticNative());
        System.out.println("held after static synchronized native="
                + Thread.holdsLock(JniMonitorDemo.class));
        try {
            demo.throwInNative();
            System.out.println("missing exception");
        } catch (IllegalStateException expected) {
            System.out.println("released after native threw=" + !Thread.holdsLock(demo));
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
