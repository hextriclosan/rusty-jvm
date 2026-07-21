package samples.concurrency.threads;

public class WaitNotifyDemo {
    static final Object lock = new Object();
    static boolean ready = false;

    public static void main(String[] args) throws InterruptedException {
        Thread producer = new Thread(() -> {
            synchronized (lock) {
                ready = true;
                lock.notify();
            }
        }, "producer");

        synchronized (lock) {
            producer.start();
            while (!ready) {
                lock.wait();
            }
        }
        System.out.println("consumed: " + ready);
    }
}
