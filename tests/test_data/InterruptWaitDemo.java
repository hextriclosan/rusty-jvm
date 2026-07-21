package samples.concurrency.threads;

public class InterruptWaitDemo {
    static final Object lock = new Object();

    public static void main(String[] args) throws InterruptedException {
        Thread worker = new Thread(() -> {
            synchronized (lock) {
                try {
                    lock.wait();
                    System.out.println("NOT interrupted");
                } catch (InterruptedException e) {
                    System.out.println("wait interrupted");
                }
            }
        }, "worker");
        worker.start();
        Thread.sleep(100);
        worker.interrupt();
        worker.join();
    }
}
