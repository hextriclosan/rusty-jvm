package samples.gc;

import java.lang.reflect.Array;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicInteger;

public final class GcBudgetDemo {
    private static volatile byte[][] concurrentKeep;
    private static native int[] oversizedJniArray(int length);

    private static void require(boolean condition, String message) {
        if (!condition) throw new AssertionError(message);
    }

    private static void requireOom(Runnable allocation, String path) {
        try {
            allocation.run();
            throw new AssertionError(path + " allocation succeeded");
        } catch (OutOfMemoryError expected) {
            System.out.println(path + "-oom-recovered");
        }
    }

    private static void oversizedAllocations() {
        requireOom(() -> Array.newInstance(int.class, Integer.MAX_VALUE), "reflective-array");
        requireOom(() -> oversizedJniArray(Integer.MAX_VALUE), "jni-array");
    }

    private static void concurrentAllocations() throws InterruptedException {
        CountDownLatch ready = new CountDownLatch(2);
        CountDownLatch start = new CountDownLatch(1);
        CountDownLatch finished = new CountDownLatch(2);
        AtomicInteger failures = new AtomicInteger();
        concurrentKeep = new byte[2][];
        for (int index = 0; index < 2; index++) {
            final int slot = index;
            new Thread(() -> {
                ready.countDown();
                try {
                    if (!start.await(30, TimeUnit.SECONDS)) return;
                    concurrentKeep[slot] = new byte[40 * 1024 * 1024];
                } catch (OutOfMemoryError expected) {
                    failures.incrementAndGet();
                } catch (InterruptedException interrupted) {
                    Thread.currentThread().interrupt();
                    throw new AssertionError(interrupted);
                } finally {
                    finished.countDown();
                }
            }, "gc-budget-allocator-" + index).start();
        }
        require(ready.await(30, TimeUnit.SECONDS), "allocators did not rendezvous");
        start.countDown();
        require(finished.await(30, TimeUnit.SECONDS), "allocators did not finish");
        require(failures.get() > 0, "concurrent reservations both exceeded the heap budget");
        concurrentKeep = null;
        System.gc();
        byte[] recovered = new byte[1024];
        require(recovered.length == 1024, "allocation did not recover after concurrent OOM");
        System.out.println("concurrent-oom-recovered");
    }

    private static void cloneAndRecovery() {
        byte[] source = new byte[16 * 1024 * 1024];
        byte[][] pressure = new byte[32][];
        int index = 0;
        while (index < pressure.length) {
            try {
                pressure[index++] = new byte[2 * 1024 * 1024];
            } catch (OutOfMemoryError expected) {
                break;
            }
        }
        try {
            source.clone();
            throw new AssertionError("clone allocation succeeded");
        } catch (OutOfMemoryError expected) {
            System.out.println("clone-oom-recovered");
        }
        source = null;
        pressure = null;
        System.gc();
        byte[] recovered = new byte[1024];
        require(recovered.length == 1024, "allocation did not recover after releasing roots");
        System.out.println("recovery-oom-recovered");
    }

    public static void main(String[] args) throws Exception {
        System.loadLibrary("jni_test_lib");
        System.gc();
        Runtime runtime = Runtime.getRuntime();
        System.out.println("bootstrap-used="
                + (runtime.totalMemory() - runtime.freeMemory()));
        oversizedAllocations();
        concurrentAllocations();
        cloneAndRecovery();
    }
}
