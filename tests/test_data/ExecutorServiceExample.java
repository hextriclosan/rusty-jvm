package samples.concurrency.virtualthreads;

import java.util.concurrent.Callable;
import java.util.concurrent.ExecutionException;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.Future;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicInteger;

public class ExecutorServiceExample {
    static final int TASK_COUNT = 5;

    public static void main(String[] args) throws InterruptedException, ExecutionException {
        // Each ExecutorService worker is itself a Tokio-backed Virtual Thread.
        ExecutorService pool = Executors.newFixedThreadPool(3);
        AtomicInteger completed = new AtomicInteger(0);

        @SuppressWarnings("unchecked")
        Future<Integer>[] futures = (Future<Integer>[]) new Future[TASK_COUNT];

        for (int i = 0; i < TASK_COUNT; i++) {
            final int id = i;
            Callable<Integer> task = () -> {
                // Yield a few times to force cooperative context switching
                // across the pool's carrier tasks.
                for (int j = 0; j < 3; j++) {
                    Thread.yield();
                }
                completed.incrementAndGet();
                return id * id;
            };
            futures[i] = pool.submit(task);
        }

        // Collect results in submission order so the output is deterministic.
        int sum = 0;
        for (int i = 0; i < TASK_COUNT; i++) {
            int value = futures[i].get();
            System.out.println("Task " + i + " result: " + value);
            sum += value;
        }

        pool.shutdown();
        boolean terminated = pool.awaitTermination(5, TimeUnit.SECONDS);

        System.out.println("Completed tasks: " + completed.get());
        System.out.println("Sum of squares: " + sum);
        System.out.println("Pool terminated: " + terminated);
    }
}
