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
        while (t.getState() != Thread.State.WAITING) {     // spin until it parks in wait()
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
