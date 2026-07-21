package samples.concurrency.threads;

public class ThreadStartDemo {
    public static void main(String[] args) {
        // A non-daemon worker with an explicit name. main starts it and returns without joining;
        // the VM must keep running until the worker finishes, so its output is deterministic.
        // The worker prints its own thread name, proving Thread.currentThread() is per-thread.
        Thread worker = new Thread(
                () -> System.out.println("running in: " + Thread.currentThread().getName()),
                "worker");
        worker.start();
    }
}
