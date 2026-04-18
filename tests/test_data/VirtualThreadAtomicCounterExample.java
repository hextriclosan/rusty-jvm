package samples.concurrency.virtualthreads;

import java.util.concurrent.atomic.AtomicInteger;

public class VirtualThreadAtomicCounterExample {
    static final AtomicInteger counter = new AtomicInteger(0);

    public static void main(String[] args) {
        Runnable task = () -> {
            for (int i = 0; i < 100; i++) {
                // This invokes Unsafe.compareAndSetInt under the hood, 
                // which mutates the shared JVM DashMap concurrently!
                counter.incrementAndGet();
                Thread.yield(); // Force heavy context switching
            }
        };

        // Spawn 10 Tokio-backed Virtual Threads
        Thread[] threads = new Thread[10];
        for (int i = 0; i < 10; i++) {
            threads[i] = new Thread(task);
            threads[i].start();
        }

        try { Thread.sleep(2000); } catch (InterruptedException e) {}

        System.out.println("Final counter value: " + counter.get());
    }
}
