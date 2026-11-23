package samples.javacore.stackoverflow;

/**
 * Runaway recursion ends in a StackOverflowError rather than in the VM eating memory until
 * something else gives: catchable, with the stack usable again afterwards, and fatal for a thread
 * that lets it escape.
 */
public class StackOverflowExample {
    private static int depth;

    static void recurse() {
        depth++;
        recurse();
    }

    static int fibonacci(int n) {
        return n < 2 ? n : fibonacci(n - 1) + fibonacci(n - 2);
    }

    public static void main(String[] args) throws InterruptedException {
        try {
            recurse();
            System.out.println("no overflow");
        } catch (StackOverflowError e) {
            System.out.println("caught " + e.getClass().getName() + ", deeper than 1000: " + (depth > 1000)
                    + ", message: " + e.getMessage());
        }
        System.out.println("fibonacci after the overflow: " + fibonacci(20));

        depth = 0;
        Thread worker = new Thread(StackOverflowExample::recurse);
        worker.setUncaughtExceptionHandler((thread, error) -> System.out.println(
                "worker died of " + error.getClass().getName() + ", deeper than 1000: " + (depth > 1000)));
        worker.start();
        worker.join();
        System.out.println("still running");
    }
}
