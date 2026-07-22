package samples.concurrency.threads;

public class ThreadStateDemo {
    static final Object lock = new Object();
    static boolean done = false;

    public static void main(String[] args) throws InterruptedException {
        Thread t = new Thread(() -> {
            synchronized (lock) {
                while (!done) {
                    try {
                        lock.wait();
                    } catch (InterruptedException e) {
                        throw new RuntimeException(e);
                    }
                }
            }
        }, "worker");

        System.out.println(t.getState());                 // NEW (never started)
        t.start();
        // Spin until it parks in wait(), but bounded: without a deadline a getState() regression
        // would hang the test forever (the Rust harness applies no per-test timeout here).
        long deadlineNanos = System.nanoTime() + 5_000_000_000L; // 5s
        while (t.getState() != Thread.State.WAITING) {
            if (System.nanoTime() >= deadlineNanos) {
                throw new IllegalStateException(
                        "thread never reached WAITING; last state=" + t.getState());
            }
            Thread.sleep(1);
        }
        System.out.println(t.getState());                 // WAITING
        synchronized (lock) {
            done = true;
            lock.notify();
        }
        t.join();
        System.out.println(t.getState());                 // TERMINATED
    }
}
