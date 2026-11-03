package samples.gc;

import java.lang.reflect.Array;

public class GcBudgetAllocationDemo {
    private static void negative(Runnable action, String message) {
        try {
            action.run();
            throw new AssertionError("negative dimension accepted");
        } catch (NegativeArraySizeException expected) {
            if (!message.equals(expected.getMessage())) {
                throw new AssertionError(expected.getMessage());
            }
        }
    }

    private static void oversized(Runnable action) {
        try {
            action.run();
            throw new AssertionError("oversized array accepted");
        } catch (OutOfMemoryError expected) {
            // The same preallocated throwable can be caught repeatedly.
        }
    }

    public static void main(String[] args) {
        negative(() -> { int[] ignored = new int[-1]; }, "-1");
        negative(() -> { Object[] ignored = new Object[-2]; }, "-2");
        negative(() -> { int[][] ignored = new int[0][-3]; }, "-3");
        negative(() -> { int[][] ignored = new int[Integer.MAX_VALUE][-4]; }, "-4");
        negative(() -> Array.newInstance(int.class, -5), "-5");
        System.out.println("negative-dimensions-ok");

        oversized(() -> { long[] ignored = new long[Integer.MAX_VALUE]; });
        oversized(() -> { Object[] ignored = new Object[Integer.MAX_VALUE]; });
        oversized(() -> Array.newInstance(double.class, Integer.MAX_VALUE));
        try {
            long[] ignored = new long[Integer.MAX_VALUE];
            throw new AssertionError("oversized array accepted");
        } catch (VirtualMachineError expected) {
            // Matching the prepared ancestor metadata must also work.
        }
        System.out.println("oversized-arrays-ok");

        long[] source = { 0x0102030405060708L, -2L };
        long[] clone = source.clone();
        if (clone == source || clone[0] != source[0] || clone[1] != -2L) {
            throw new AssertionError("clone changed wide values");
        }
        int[] ordinary = new int[4];
        ordinary[3] = 17;
        if (ordinary[3] != 17) throw new AssertionError("execution did not resume");
        System.out.println("clone-ok");
    }
}
