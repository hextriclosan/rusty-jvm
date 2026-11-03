package samples.gc;

import java.lang.ref.Cleaner;
import java.lang.ref.PhantomReference;
import java.lang.ref.Reference;
import java.lang.ref.ReferenceQueue;
import java.lang.ref.SoftReference;
import java.lang.ref.WeakReference;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.TimeUnit;

public class GcCollectDemo {
    private static Object garbage;
    private static Object[] live;
    private static volatile int cleanedCount;
    private static final CountDownLatch cleaned = new CountDownLatch(1);

    private static void allocateGarbage() {
        byte[][] arrays = new byte[128][];
        for (int i = 0; i < arrays.length; i++) {
            arrays[i] = new byte[4096];
        }
        garbage = arrays;
    }

    private static void basic() {
        Runtime runtime = Runtime.getRuntime();
        live = new Object[] {null, new int[] {17, 29}};
        live[0] = live;
        int identity = System.identityHashCode(live);
        allocateGarbage();
        long before = runtime.freeMemory();
        garbage = null;
        System.gc();
        long after = runtime.freeMemory();
        if (runtime.maxMemory() != 64L * 1024 * 1024
                || runtime.totalMemory() != runtime.maxMemory()
                || after <= before || after > runtime.totalMemory()
                || live[0] != live || ((int[]) live[1])[1] != 29
                || System.identityHashCode(live) != identity) {
            throw new AssertionError("reclamation, memory counters, live cycle or identity");
        }
        System.out.println("collect-ok");
    }

    private static WeakReference<Object> weak(ReferenceQueue<Object> queue) {
        return new WeakReference<>(new Object(), queue);
    }

    private static PhantomReference<Object> phantom(ReferenceQueue<Object> queue) {
        return new PhantomReference<>(new Object(), queue);
    }

    private static class Action implements Runnable {
        public void run() {
            cleanedCount++;
            cleaned.countDown();
        }
    }

    private static Cleaner.Cleanable register(Cleaner cleaner) {
        return cleaner.register(new Object(), new Action());
    }

    private static void references() throws Exception {
        ReferenceQueue<Object> weakQueue = new ReferenceQueue<>();
        ReferenceQueue<Object> phantomQueue = new ReferenceQueue<>();
        ReferenceQueue<Object> clearedQueue = new ReferenceQueue<>();
        WeakReference<Object> weak = weak(weakQueue);
        PhantomReference<Object> phantom = phantom(phantomQueue);
        WeakReference<Object> cleared = weak(clearedQueue);
        cleared.clear();
        PhantomReference<Object> clearedPhantom = phantom(clearedQueue);
        clearedPhantom.clear();
        SoftReference<SoftReference<byte[]>> soft =
                new SoftReference<>(new SoftReference<>(new byte[] {37}));
        Cleaner cleaner = Cleaner.create();
        Cleaner.Cleanable cleanable = register(cleaner);
        System.gc();
        if (weakQueue.remove(10000) != weak || phantomQueue.remove(10000) != phantom
                || !cleaned.await(10, TimeUnit.SECONDS)) {
            throw new AssertionError("pending reference or Cleaner was not delivered");
        }
        System.gc();
        System.gc();
        cleanable.clean();
        if (!weak.refersTo(null) || !phantom.refersTo(null)
                || !clearedPhantom.refersTo(null)
                || soft.get() == null || soft.get().get()[0] != 37
                || weakQueue.poll() != null || phantomQueue.poll() != null
                || clearedQueue.poll() != null || cleanedCount != 1) {
            throw new AssertionError("reference policy or exactly-once delivery");
        }
        Reference.reachabilityFence(cleared);
        Reference.reachabilityFence(clearedPhantom);
        Reference.reachabilityFence(cleaner);
        System.out.println("references-ok");
    }

    private static void pressure() throws Exception {
        Runtime runtime = Runtime.getRuntime();
        ReferenceQueue<byte[]> queue = new ReferenceQueue<>();
        SoftReference<byte[]> soft = new SoftReference<>(new byte[1024 * 1024], queue);
        System.gc();
        if (soft.get() == null) {
            throw new AssertionError("explicit GC cleared a soft reference");
        }
        byte[][] held = new byte[4096][];
        boolean caught = false;
        try {
            for (int i = 0; i < held.length; i++) {
                held[i] = new byte[64 * 1024];
            }
        } catch (OutOfMemoryError expected) {
            caught = true;
        }
        if (!caught || !soft.refersTo(null) || runtime.freeMemory() < 0
                || runtime.freeMemory() > runtime.maxMemory()) {
            throw new AssertionError("strict admission or pressure soft clearing");
        }
        held = null;
        System.gc();
        byte[] recovered = new byte[256 * 1024];
        recovered[0] = 71;
        if (queue.remove(10000) != soft || recovered[0] != 71) {
            throw new AssertionError("OOM recovery or soft delivery");
        }
        System.out.println("pressure-ok");
    }

    public static void main(String[] args) throws Exception {
        if (args[0].equals("references")) {
            references();
        } else if (args[0].equals("pressure")) {
            pressure();
        } else {
            basic();
        }
    }
}
