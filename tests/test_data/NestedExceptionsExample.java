package samples.javacore.nestedexceptions;

/**
 * Exceptions thrown by Java code the VM itself calls, a class initializer, a reflected method,
 * a lambda's target, reach the code that caused the call: catchable there, wrapped as the JVMS
 * and the reflection API say, and a class whose initialization failed stays unusable, with a
 * NoClassDefFoundError that tells what it died of.
 */
public class NestedExceptionsExample {
    static class Broken {
        static final int VALUE = fail();

        static int fail() {
            throw new IllegalStateException("broken <clinit>");
        }

        static void touch() {
        }
    }

    static class InitError extends Error {
        InitError(String message) {
            super(message);
        }
    }

    /** Fails with an Error, which initialization passes on unwrapped. */
    static class BrokenWithError {
        static final int VALUE = fail();

        static int fail() {
            throw new InitError("error in <clinit>");
        }

        static void touch() {
        }
    }

    /** Its superclass is the broken one, so it cannot be initialized either. */
    static class ChildOfBroken extends Broken {
        static void touch() {
        }
    }

    /** Reached by name, through Class.forName, which initializes it from a native. */
    static class BrokenByName {
        static final int VALUE = fail();

        static int fail() {
            throw new IllegalArgumentException("broken <clinit> reached by name");
        }
    }

    static void thrower() {
        throw new IllegalArgumentException("from a reflected method");
    }

    private interface Action {
        void run() throws Throwable;
    }

    public static void main(String[] args) {
        attempt("first use", Broken::touch);
        attempt("second use", Broken::touch);
        attempt("error in initializer", BrokenWithError::touch);
        attempt("subclass of the broken class", ChildOfBroken::touch);
        attempt("Class.forName", () -> Class.forName(BrokenByName.class.getName()));
        attempt("reflected method", () -> NestedExceptionsExample.class.getDeclaredMethod("thrower").invoke(null));
        System.out.println("still running");
    }

    private static void attempt(String label, Action action) {
        try {
            action.run();
            System.out.println(label + ": no exception");
        } catch (Throwable t) {
            StringBuilder line = new StringBuilder(label).append(": ").append(t);
            for (Throwable cause = t.getCause(); cause != null; cause = cause.getCause()) {
                line.append(" caused by ").append(cause);
            }
            System.out.println(line);
        }
    }
}
