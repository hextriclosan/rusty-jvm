package samples.concurrency.threads;

public class ThreadLocalDemo {
    static final ThreadLocal<Integer> tl = ThreadLocal.withInitial(() -> 0);

    public static void main(String[] args) throws InterruptedException {
        StringBuilder out = new StringBuilder();

        tl.set(100);
        out.append("main-set=").append(tl.get()).append("\n");            // 100

        // A worker's value is isolated from main's (per-thread storage).
        Thread w = new Thread(() -> tl.set(7), "w");
        w.start();
        w.join();
        out.append("main-after-worker=").append(tl.get()).append("\n");    // still 100

        // remove() clears the entry (exercises Reference.clear0) -> back to the initial value.
        tl.remove();
        out.append("main-after-remove=").append(tl.get());                 // 0

        System.out.println(out);
    }
}
