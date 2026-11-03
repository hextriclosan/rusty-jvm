package samples.gc;

import java.lang.ref.Cleaner;
import java.lang.ref.PhantomReference;
import java.lang.ref.Reference;
import java.lang.ref.ReferenceQueue;
import java.lang.ref.SoftReference;
import java.lang.ref.WeakReference;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.TimeUnit;

public class GcReferencesDemo {
    private static int legacyCleaned;
    private static int publicCleaned;
    private static boolean finalized;

    private static final class Finalizable {
        @Override
        protected void finalize() {
            finalized = true;
        }
    }

    private static final class Action implements Runnable {
        private final CountDownLatch latch;
        private final boolean legacy;
        private final CountDownLatch release;

        Action(CountDownLatch latch, boolean legacy, CountDownLatch release) {
            this.latch = latch;
            this.legacy = legacy;
            this.release = release;
        }

        @Override
        public void run() {
            if (legacy) {
                legacyCleaned++;
            } else {
                publicCleaned++;
            }
            latch.countDown();
            if (release != null) {
                try {
                    if (!release.await(10, TimeUnit.SECONDS)) {
                        throw new AssertionError("legacy Cleaner release timed out");
                    }
                } catch (InterruptedException e) {
                    throw new AssertionError(e);
                }
            }
        }
    }

    private static WeakReference<Object> weak(ReferenceQueue<Object> queue) {
        return new WeakReference<>(new Object(), queue);
    }

    private static PhantomReference<Object> phantom(ReferenceQueue<Object> queue) {
        return new PhantomReference<>(new Object(), queue);
    }

    private static WeakReference<Finalizable> finalizable(ReferenceQueue<Finalizable> queue) {
        return new WeakReference<>(new Finalizable(), queue);
    }

    private static jdk.internal.ref.Cleaner legacyCleaner(
            CountDownLatch cleaned, CountDownLatch release) {
        return jdk.internal.ref.Cleaner.create(
                new Object(), new Action(cleaned, true, release));
    }

    private static Cleaner.Cleanable publicCleaner(
            Cleaner cleaner, CountDownLatch cleaned) {
        return cleaner.register(new Object(), new Action(cleaned, false, null));
    }

    private static void requireDelivery(
            ReferenceQueue<?> queue, Reference<?> expected) throws InterruptedException {
        if (queue.remove(10000) != expected) {
            throw new AssertionError("reference was not delivered exactly once");
        }
    }

    private static void pressureSoftReference() throws InterruptedException {
        ReferenceQueue<byte[]> queue = new ReferenceQueue<>();
        SoftReference<byte[]> soft = new SoftReference<>(new byte[1024 * 1024], queue);
        System.gc();
        if (soft.get() == null) {
            throw new AssertionError("soft reference cleared without memory pressure");
        }

        byte[][] held = new byte[4096][];
        boolean exhausted = false;
        try {
            for (int i = 0; i < held.length; i++) {
                held[i] = new byte[64 * 1024];
            }
        } catch (OutOfMemoryError expected) {
            exhausted = true;
        }
        held = null;
        System.gc();
        if (!exhausted || !soft.refersTo(null)) {
            throw new AssertionError("soft reference did not clear under memory pressure");
        }
        requireDelivery(queue, soft);
        if (queue.poll() != null) {
            throw new AssertionError("soft reference was enqueued more than once");
        }
    }

    public static void main(String[] args) throws Exception {
        ReferenceQueue<Object> weakQueue = new ReferenceQueue<>();
        ReferenceQueue<Object> phantomQueue = new ReferenceQueue<>();
        ReferenceQueue<Object> laterWeakQueue = new ReferenceQueue<>();
        ReferenceQueue<Object> laterPhantomQueue = new ReferenceQueue<>();
        ReferenceQueue<Object> clearedQueue = new ReferenceQueue<>();
        ReferenceQueue<Finalizable> finalQueue = new ReferenceQueue<>();
        CountDownLatch legacyDone = new CountDownLatch(1);
        CountDownLatch legacyRelease = new CountDownLatch(1);
        CountDownLatch publicDone = new CountDownLatch(1);
        Cleaner cleaner = Cleaner.create();

        WeakReference<Object> weak;
        PhantomReference<Object> phantom;
        WeakReference<Object> explicitlyCleared;
        PhantomReference<Object> explicitlyClearedPhantom;
        WeakReference<Finalizable> finalizable;
        Reference<?> deliveredLaterWeak;
        Reference<?> deliveredLaterPhantom;
        SoftReference<SoftReference<byte[]>> soft;
        jdk.internal.ref.Cleaner legacy;
        Cleaner.Cleanable publicCleanable;

        Object weakTarget = new Object();
        weak = new WeakReference<>(weakTarget, weakQueue);
        if (!weak.refersTo(weakTarget)) {
            throw new AssertionError("weak refersTo lost a live referent");
        }
        weakTarget = null;
        Object phantomTarget = new Object();
        phantom = new PhantomReference<>(phantomTarget, phantomQueue);
        if (!phantom.refersTo(phantomTarget) || phantom.get() != null) {
            throw new AssertionError("phantom refersTo/get contract");
        }
        phantomTarget = null;
        explicitlyCleared = weak(clearedQueue);
        explicitlyCleared.clear();
        explicitlyClearedPhantom = phantom(clearedQueue);
        explicitlyClearedPhantom.clear();
        finalizable = finalizable(finalQueue);
        soft = new SoftReference<>(new SoftReference<>(new byte[] {37}));
        legacy = legacyCleaner(legacyDone, legacyRelease);
        publicCleanable = publicCleaner(cleaner, publicDone);

        // The legacy Cleaner runs on ReferenceHandler itself. Its test action
        // blocks that real daemon at an event signal while later GCs accumulate
        // separate VM pending batches behind it.
        System.gc();
        if (!legacyDone.await(10, TimeUnit.SECONDS)) {
            throw new AssertionError("legacy Cleaner handler did not reach its action");
        }
        WeakReference<Object> laterWeak = weak(laterWeakQueue);
        System.gc();
        laterWeak = null;
        PhantomReference<Object> laterPhantom = phantom(laterPhantomQueue);
        System.gc();
        laterPhantom = null;
        System.gc();
        if (laterWeakQueue.poll() != null || laterPhantomQueue.poll() != null) {
            throw new AssertionError("handler drained while the legacy Cleaner was blocked");
        }
        if (!weak.refersTo(null) || !phantom.refersTo(null)
                || phantom.get() != null
                || !explicitlyCleared.refersTo(null)
                || !explicitlyClearedPhantom.refersTo(null)
                || soft.get() == null || soft.get().get()[0] != 37) {
            throw new AssertionError("reference clear, refersTo, or soft-retention semantics");
        }
        legacyRelease.countDown();

        requireDelivery(weakQueue, weak);
        requireDelivery(phantomQueue, phantom);
        deliveredLaterWeak = laterWeakQueue.remove(10000);
        deliveredLaterPhantom = laterPhantomQueue.remove(10000);
        if (!(deliveredLaterWeak instanceof WeakReference)) {
            throw new AssertionError("pending weak reference was not delivered");
        }
        if (!(deliveredLaterPhantom instanceof PhantomReference)) {
            throw new AssertionError("pending phantom reference was not delivered");
        }
        if (!deliveredLaterWeak.refersTo(null)
                || !deliveredLaterPhantom.refersTo(null)) {
            throw new AssertionError("pending referents were not cleared");
        }
        requireDelivery(finalQueue, finalizable);
        if (!legacyDone.await(10, TimeUnit.SECONDS)
                || !publicDone.await(10, TimeUnit.SECONDS)) {
            throw new AssertionError("Cleaner action was not delivered");
        }
        legacy.clean();
        publicCleanable.clean();
        System.gc();
        System.gc();
        System.gc();
        if (weakQueue.poll() != null || phantomQueue.poll() != null
                || finalQueue.poll() != null || clearedQueue.poll() != null
                || legacyCleaned != 1 || publicCleaned != 1 || finalized
                || soft.get() == null || soft.get().get()[0] != 37) {
            throw new AssertionError("duplicate delivery, explicit-clear enqueue, or finalization");
        }
        Reference.reachabilityFence(cleaner);
        Reference.reachabilityFence(legacy);
        Reference.reachabilityFence(publicCleanable);
        pressureSoftReference();
        System.out.println("references-ok");
    }
}
