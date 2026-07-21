package samples.concurrency.threads;

public class InterruptSleepDemo {
    public static void main(String[] args) throws InterruptedException {
        Thread worker = new Thread(() -> {
            try {
                Thread.sleep(60_000);
                System.out.println("NOT interrupted");
            } catch (InterruptedException e) {
                System.out.println("sleep interrupted");
            }
        }, "worker");
        worker.start();
        Thread.sleep(100);
        worker.interrupt();
        worker.join();
    }
}
