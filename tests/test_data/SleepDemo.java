package samples.concurrency.threads;

public class SleepDemo {
    public static void main(String[] args) throws InterruptedException {
        long t0 = System.nanoTime();
        Thread.sleep(200);
        long elapsedMs = (System.nanoTime() - t0) / 1_000_000;
        System.out.println(elapsedMs >= 150 ? "slept" : "too short: " + elapsedMs);
    }
}
