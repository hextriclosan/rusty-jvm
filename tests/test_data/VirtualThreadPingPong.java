package samples.concurrency.virtualthreads;

import java.util.concurrent.atomic.AtomicInteger;

public class VirtualThreadPingPong {
    static final AtomicInteger state = new AtomicInteger(0);

    public static void main(String[] args) {
        Thread t1 = new Thread(() -> {
            // Spin-wait, but yield to the Tokio scheduler so we don't block the OS thread
            while (state.get() != 1) { Thread.yield(); }
            System.out.println("Ping");
            state.set(2);
        });

        Thread t2 = new Thread(() -> {
            while (state.get() != 2) { Thread.yield(); }
            System.out.println("Pong");
            state.set(3);
        });

        t1.start();
        t2.start();

        // Main thread kicks off the chain
        state.set(1);

        // Wait for workers to finish
        while (state.get() != 3) {
            try { Thread.sleep(10); } catch (InterruptedException e) {}
        }
        System.out.println("Done");
    }
}
