package samples.concurrency.virtualthreads;

import java.util.concurrent.atomic.AtomicInteger;

public class VirtualThreadLocalExample {
    // ThreadLocal relies internally on Thread.currentThread()
    private static final ThreadLocal<String> context = new ThreadLocal<>();
    private static final String[] results = new String[2];
    private static final AtomicInteger finished = new AtomicInteger(0);

    public static void main(String[] args) {
        context.set("MainContext");

        Thread t1 = new Thread(() -> {
            context.set("Task1");
            Thread.yield(); // Force a context switch to try and corrupt the data
            results[0] = context.get();
            finished.incrementAndGet();
        });

        Thread t2 = new Thread(() -> {
            context.set("Task2");
            Thread.yield();
            results[1] = context.get();
            finished.incrementAndGet();
        });

        t1.start();
        t2.start();

        while (finished.get() < 2) {
            Thread.yield();
        }

        // Output must be completely deterministic
        System.out.println("results[0]: " + results[0]);
        System.out.println("results[1]: " + results[1]);
        System.out.println("Main reads: " + context.get());
    }
}
