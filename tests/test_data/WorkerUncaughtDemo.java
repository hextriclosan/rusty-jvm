package samples.concurrency.threads;

public class WorkerUncaughtDemo {
    public static void main(String[] args) {
        // A Thread subclass overriding run() (exercises virtual run() dispatch). The worker throws;
        // its uncaught exception must be reported against the worker's own name ("worker"), not
        // "main", proving per-thread uncaught-exception dispatch. main exits normally (code 0) — the
        // worker's failure does not fail the VM.
        Thread worker = new Thread("worker") {
            @Override
            public void run() {
                throw new RuntimeException("boom from worker");
            }
        };
        worker.start();
    }
}
