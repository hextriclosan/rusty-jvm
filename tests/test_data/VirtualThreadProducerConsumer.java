package samples.concurrency.virtualthreads;

import java.util.concurrent.atomic.AtomicInteger;

public class VirtualThreadProducerConsumer {
    static final AtomicInteger data = new AtomicInteger(-1);
    static final AtomicInteger state = new AtomicInteger(0); // 0=empty, 1=full, 2=done
    static final int[] consumed = new int[5];

    public static void main(String[] args) {
        Thread producer = new Thread(() -> {
            for (int i = 0; i < 5; i++) {
                // Wait for the consumer to empty the data
                while (state.get() != 0) { Thread.yield(); }
                
                data.set(i * 10); // Produce data
                state.set(1);     // Signal full
            }
        });

        Thread consumer = new Thread(() -> {
            for (int i = 0; i < 5; i++) {
                // Wait for the producer to fill the data
                while (state.get() != 1) { Thread.yield(); }
                
                consumed[i] = data.get(); // Consume data
                state.set(0);             // Signal empty
            }
            state.set(2); // Signal fully done
        });

        producer.start();
        consumer.start();

        // Main thread waits for the entire cycle to finish
        while (state.get() != 2) {
            Thread.yield();
        }

        for (int i = 0; i < 5; i++) {
            System.out.println("Consumed: " + consumed[i]);
        }
    }
}
