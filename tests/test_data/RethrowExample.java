package samples.javacore.rethrowloopexample;

public class RethrowExample {
    public static void main(String[] args) {
        try {
            testRethrowLoop();
        } catch (Throwable t) {
            System.out.println("Caught in main: " + t);
        }
    }

    static void testRethrowLoop() {
        try {
            throwAssertion();
        } catch (Error e) {
            throw e;
        }
    }

    static void throwAssertion() {
        throw new AssertionError("boom");
    }
}
