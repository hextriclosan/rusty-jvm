package samples.concurrency.threads;

public class NotifyAllDemo {
    static final Object lock = new Object();
    static boolean go = false;
    static int awoken = 0;

    public static void main(String[] args) throws InterruptedException {
        int n = 3;
        Thread[] workers = new Thread[n];
        for (int i = 0; i < n; i++) {
            workers[i] = new Thread(() -> {
                synchronized (lock) {
                    // Guarded wait: a worker that arrives after notifyAll sees `go` and skips waiting,
                    // so the result does not depend on all workers reaching wait() first.
                    while (!go) {
                        try {
                            lock.wait();
                        } catch (InterruptedException e) {
                            throw new RuntimeException(e);
                        }
                    }
                    awoken++;
                }
            }, "worker-" + i);
            workers[i].start();
        }

        synchronized (lock) {
            go = true;
            lock.notifyAll(); // must wake every waiting worker; if it woke only one, the joins hang
        }

        for (Thread w : workers) {
            w.join();
        }
        System.out.println("awoken: " + awoken);
    }
}
