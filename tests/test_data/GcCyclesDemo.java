package samples.gc;

import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.lang.reflect.Array;

public final class GcCyclesDemo {
    private static final int LIFETIME_KEYS_PER_CYCLE = 128;
    private static final Object[] LIVE = new Object[32];
    private static final ThreadLocal<Object> LOCAL = new ThreadLocal<>();

    private static int add(int left, int right) {
        return left + right;
    }

    private static void require(boolean condition, String message) {
        if (!condition) throw new AssertionError(message);
    }

    private static void expectNullPointer(Runnable action) {
        try {
            action.run();
            throw new AssertionError("expected NullPointerException");
        } catch (NullPointerException expected) {
            // Expected.
        }
    }

    private static void expectIllegalArgument(Runnable action) {
        try {
            action.run();
            throw new AssertionError("expected IllegalArgumentException");
        } catch (IllegalArgumentException expected) {
            // Expected.
        }
    }

    private static void expectArrayIndexOutOfBounds(Runnable action) {
        try {
            action.run();
            throw new AssertionError("expected ArrayIndexOutOfBoundsException");
        } catch (ArrayIndexOutOfBoundsException expected) {
            // Expected.
        }
    }

    private static void exerciseReflectiveArraySemantics() {
        Object strings = Array.newInstance(String.class, 1);
        require(Array.getLength(strings) == 1, "reflective array length was wrong");
        Array.set(strings, 0, "reflect");
        require("reflect".equals(Array.get(strings, 0)), "reference Array.get/set failed");
        expectIllegalArgument(() -> Array.set(strings, 0, new Object()));

        int[] ints = (int[]) Array.newInstance(int.class, 1);
        Array.set(ints, 0, Byte.valueOf((byte) 7));
        require(((Integer) Array.get(ints, 0)).intValue() == 7,
                "primitive unboxing, widening, or boxing failed");
        Array.setInt(ints, 0, 257);
        require(Array.getInt(ints, 0) == 257, "typed int get/set failed");
        expectIllegalArgument(() -> Array.set(ints, 0, Long.valueOf(9)));
        expectArrayIndexOutOfBounds(() -> Array.get(ints, 1));
        expectIllegalArgument(() -> Array.getLength(new Object()));
        expectNullPointer(() -> Array.getLength(null));
        expectNullPointer(() -> Array.set(null, 0, null));
        expectIllegalArgument(() -> Array.getInt(new long[1], 0));

        long[] longs = new long[1];
        Array.setInt(longs, 0, 42);
        require(longs[0] == 42L, "typed int setter did not widen to long");
    }

    private static void exerciseRuntimeFeatures() throws Throwable {
        exerciseReflectiveArraySemantics();

        String joined = "gc-" + "stress-" + LIVE.length;
        require("gc-stress-32".equals(joined), "string concatenation failed");

        LOCAL.set(new Object());
        require(LOCAL.get() != null, "ThreadLocal lost its value");

        MethodHandle add = MethodHandles.lookup().findStatic(
                GcCyclesDemo.class, "add", MethodType.methodType(int.class, int.class, int.class));
        require((int) add.invokeExact(20, 22) == 42, "method handle failed");

        try {
            throw new IllegalStateException("expected");
        } catch (IllegalStateException expected) {
            require("expected".equals(expected.getMessage()), "exception was not preserved");
        }
    }

    public static void main(String[] args) throws Throwable {
        if (args.length == 1 && "reflection".equals(args[0])) {
            exerciseReflectiveArraySemantics();
            System.out.println("reflect-array-ok");
            return;
        }
        int cycles = Integer.parseInt(args[0]);
        for (int index = 0; index < LIVE.length; index++) LIVE[index] = new Object();
        exerciseRuntimeFeatures();
        long[] usedByCycle = new long[cycles];
        System.gc();
        Runtime runtime = Runtime.getRuntime();
        long baselineUsed = runtime.totalMemory() - runtime.freeMemory();
        for (int cycle = 0; cycle < cycles; cycle++) {
            for (int index = 0; index < LIFETIME_KEYS_PER_CYCLE; index++) {
                Object[] refs = (Object[]) Array.newInstance(Object.class, 2);
                refs[0] = new Object();
                refs[1] = LIVE[(cycle + index) % LIVE.length];
            }
            System.gc();
            long used = runtime.totalMemory() - runtime.freeMemory();
            require(LIVE[cycle % LIVE.length] != null, "live reference array edge was lost");
            require(used <= baselineUsed + 1024 * 1024,
                    "used heap grew across a fixed-live-set collection cycle: baseline="
                            + baselineUsed + " current=" + used);
            usedByCycle[cycle] = used;
        }
        System.out.println("baseline-used=" + baselineUsed);
        for (int cycle = 0; cycle < cycles; cycle++) {
            System.out.println("cycle=" + cycle + " used=" + usedByCycle[cycle]);
        }
        System.out.println("cycles-ok");
    }
}
