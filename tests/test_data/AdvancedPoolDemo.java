package samples.concurrency.advancedpooldemo;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.BlockingQueue;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.RejectedExecutionHandler;
import java.util.concurrent.ThreadFactory;
import java.util.concurrent.ThreadPoolExecutor;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicInteger;

public class AdvancedPoolDemo {

    // Single serialized logger: no two lines can interleave.
    private static final Object LOG_LOCK = new Object();
    static void log(String msg) {
        synchronized (LOG_LOCK) { System.out.println(msg); }
    }

    // Custom ThreadFactory: deterministic, incrementing names.
    static final class NamedThreadFactory implements ThreadFactory {
        private final String prefix;
        private final AtomicInteger counter = new AtomicInteger(1);
        NamedThreadFactory(String prefix) { this.prefix = prefix; }
        @Override public Thread newThread(Runnable r) {
            Thread t = new Thread(r, prefix + "-" + counter.getAndIncrement());
            t.setUncaughtExceptionHandler((th, ex) ->
                log("[UNCAUGHT] " + th.getName() + ": " + ex));
            return t;
        }
    }

    // A task that blocks on a shared gate so it cannot complete during submission.
    static final class GatedTask implements Runnable {
        final int id;
        final CountDownLatch gate;
        final List<Integer> executed;
        GatedTask(int id, CountDownLatch gate, List<Integer> executed) {
            this.id = id; this.gate = gate; this.executed = executed;
        }
        @Override public void run() {
            try { gate.await(); } catch (InterruptedException e) {
                Thread.currentThread().interrupt(); return;
            }
            synchronized (executed) { executed.add(id); }
        }
    }

    public static void main(String[] args) throws InterruptedException {
        final CountDownLatch gate = new CountDownLatch(1);
        final List<Integer> executed = Collections.synchronizedList(new ArrayList<>());
        final List<Integer> rejected = Collections.synchronizedList(new ArrayList<>());

        BlockingQueue<Runnable> queue = new ArrayBlockingQueue<>(5);

        // Rejection handler that records instead of running/throwing.
        RejectedExecutionHandler handler = (r, exec) -> {
            if (r instanceof GatedTask t) rejected.add(t.id);
        };

        ThreadPoolExecutor pool = new ThreadPoolExecutor(
            2,                      // corePoolSize
            4,                      // maximumPoolSize
            30, TimeUnit.SECONDS,   // keep-alive
            queue,
            new NamedThreadFactory("worker"),
            handler
        );

        log("=== Submission trace (deterministic: nothing completes yet) ===");
        log(String.format("core=%d  max=%d  queueCapacity=%d  -> capacity before reject = %d%n",
                pool.getCorePoolSize(), pool.getMaximumPoolSize(), 5,
                pool.getMaximumPoolSize() + 5));

        for (int i = 1; i <= 12; i++) {
            GatedTask task = new GatedTask(i, gate, executed);
            int poolBefore = pool.getPoolSize();
            int queueBefore = pool.getQueue().size();

            pool.execute(task); // rejection handler (if any) runs synchronously here

            int poolAfter = pool.getPoolSize();
            int queueAfter = pool.getQueue().size();

            if (poolAfter > poolBefore) {
                log(String.format("Task %2d  ->  SPAWN THREAD   (poolSize %d -> %d, queued=%d)",
                        i, poolBefore, poolAfter, queueAfter));
            } else if (queueAfter > queueBefore) {
                log(String.format("Task %2d  ->  ENQUEUE        (poolSize=%d, queued %d -> %d)",
                        i, poolAfter, queueBefore, queueAfter));
            } else {
                log(String.format("Task %2d  ->  REJECTED       (poolSize=%d at max, queue full)",
                        i, poolAfter));
            }
        }

        // Release everything; workers now drain and complete.
        gate.countDown();
        pool.shutdown();
        pool.awaitTermination(20, TimeUnit.SECONDS);

        // Sort the collected results so the summary is byte-for-byte stable.
        List<Integer> exec = new ArrayList<>(executed); Collections.sort(exec);
        List<Integer> rej  = new ArrayList<>(rejected); Collections.sort(rej);

        log("");
        log("=== Final summary (stable) ===");
        log("Executed tasks : " + exec);
        log("Rejected tasks : " + rej);
        log("Completed count: " + pool.getCompletedTaskCount());
        log("Largest pool   : " + pool.getLargestPoolSize());
    }
}
