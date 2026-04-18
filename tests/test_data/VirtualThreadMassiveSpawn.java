package samples.concurrency.virtualthreads;

import java.util.concurrent.atomic.AtomicInteger;

public class VirtualThreadMassiveSpawn {
    public static void main(String[] args) {
        // Spawn 2,000 threads! A standard OS would struggle with this,
        // but Tokio will multiplex them effortlessly over a few CPU cores.
        int THREAD_COUNT = 2000;
        AtomicInteger counter = new AtomicInteger(0);

        for (int i = 0; i < THREAD_COUNT; i++) {
            new Thread(() -> {
                counter.incrementAndGet();
            }).start();
        }

        // Spin-wait until all 2,000 threads have finished their execution
        while (counter.get() < THREAD_COUNT) {
            Thread.yield(); // Yield so we don't hog the carrier thread
        }

        System.out.println("Successfully spawned and executed " + counter.get() + " threads");
    }
}
