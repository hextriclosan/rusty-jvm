package samples.concurrency.threads;

public class JoinDemo {
    public static void main(String[] args) throws InterruptedException {
        Thread worker = new Thread(() -> System.out.println("worker done"), "worker");
        worker.start();
        worker.join();
        System.out.println("main after join");
    }
}
