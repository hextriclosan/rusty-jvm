package samples.arithmetics.divbyzero;

/**
 * Integral division by zero: {@code idiv}, {@code ldiv}, {@code irem} and {@code lrem} throw
 * {@code ArithmeticException} (JVMS §6.5) with the message HotSpot gives, for {@code /} and
 * {@code %} alike, and the exception is an ordinary one that unwinds through callers. The one
 * overflowing division, the most negative value by -1, throws nothing and wraps instead.
 *
 * With the {@code uncaught} argument a single division by zero is left to propagate out of
 * {@code main}.
 */
public class IntegralDivisionByZero {
    public static void main(String[] args) {
        if (args.length > 0 && args[0].equals("uncaught")) {
            System.out.println(divide(1, zero()));
            return;
        }

        attempt("int /", () -> 7 / zero());
        attempt("int %", () -> 7 % zero());
        attempt("long /", () -> 7L / zero());
        attempt("long %", () -> 7L % zero());
        attempt("int / in callee", () -> divide(7, zero()));
        attempt("int / after a caught callee exception", () -> divideAfterCaughtCall(zero()));

        System.out.println("int MIN_VALUE / -1 = " + (Integer.MIN_VALUE / minusOne()));
        System.out.println("int MIN_VALUE % -1 = " + (Integer.MIN_VALUE % minusOne()));
        System.out.println("long MIN_VALUE / -1 = " + (Long.MIN_VALUE / minusOne()));
        System.out.println("long MIN_VALUE % -1 = " + (Long.MIN_VALUE % minusOne()));
    }

    // Methods rather than literals, so that javac cannot fold the divisions away.
    private static int zero() {
        return 0;
    }

    private static int minusOne() {
        return -1;
    }

    private static int divide(int dividend, int divisor) {
        return dividend / divisor;
    }

    private static void throwing() {
        throw new IllegalStateException("callee");
    }

    // A caught callee exception, then a division by zero with no call in between (not even a string
    // concatenation): the frame must locate the second exception at the division, not at the call
    // whose exception it has already handled.
    private static int divideAfterCaughtCall(int divisor) {
        int outcome = 0;
        try {
            throwing();
        } catch (IllegalStateException e) {
            outcome = 1;
        }
        try {
            outcome += 10 / divisor;
        } catch (ArithmeticException e) {
            outcome += 100;
        }
        return outcome;
    }

    private interface Division {
        Object compute();
    }

    private static void attempt(String label, Division division) {
        try {
            System.out.println(label + " = " + division.compute());
        } catch (ArithmeticException e) {
            System.out.println(label + ": " + e);
        }
    }
}
