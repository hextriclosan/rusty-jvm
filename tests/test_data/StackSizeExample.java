package samples.javacore.stackoverflow;

/** Prints how deep a runaway recursion gets before its StackOverflowError: what -Xss decides. */
public class StackSizeExample {
    private static int depth;

    static void recurse() {
        depth++;
        recurse();
    }

    public static void main(String[] args) {
        try {
            recurse();
        } catch (StackOverflowError e) {
            System.out.println(depth);
        }
    }
}
