package samples.jdkinternal.unsafe.setmemory;

import jdk.internal.misc.Unsafe;
import java.util.Arrays;

public class UnsafeSetMemoryArray {
    private static final Unsafe U = Unsafe.getUnsafe();

    public static void main(String[] args) {
        byte[] bytes = new byte[6];
        U.setMemory(bytes, Unsafe.ARRAY_BYTE_BASE_OFFSET + 1, 3, (byte) 7);
        System.out.println("byte[]: " + Arrays.toString(bytes));

        boolean[] booleans = new boolean[4];
        U.setMemory(booleans, Unsafe.ARRAY_BOOLEAN_BASE_OFFSET + 2, 2, (byte) 1);
        System.out.println("boolean[]: " + Arrays.toString(booleans));

        // partial elements; the result depends on byte order, which matches the reference JVM's
        short[] shorts = new short[3];
        U.setMemory(shorts, Unsafe.ARRAY_SHORT_BASE_OFFSET + 1, 2, (byte) 0x12);
        System.out.println("short[]: " + Arrays.toString(shorts));

        char[] chars = new char[3];
        U.setMemory(chars, Unsafe.ARRAY_CHAR_BASE_OFFSET, 4, (byte) 0x41);
        System.out.println("char[]: " + (int) chars[0] + " " + (int) chars[1] + " " + (int) chars[2]);

        int[] ints = new int[3];
        U.setMemory(ints, Unsafe.ARRAY_INT_BASE_OFFSET + 2, 6, (byte) 0x11);
        System.out.println("int[]: " + Arrays.toString(ints));

        long[] longs = new long[2];
        U.setMemory(longs, Unsafe.ARRAY_LONG_BASE_OFFSET, 16, (byte) 0xAB);
        System.out.println("long[]: " + Arrays.toString(longs));

        float[] floats = new float[2];
        U.setMemory(floats, Unsafe.ARRAY_FLOAT_BASE_OFFSET + 4, 4, (byte) 0x3F);
        System.out.println("float[]: " + Arrays.toString(floats));

        double[] doubles = new double[2];
        U.setMemory(doubles, Unsafe.ARRAY_DOUBLE_BASE_OFFSET, 8, (byte) 0x40);
        System.out.println("double[]: " + Arrays.toString(doubles));

        byte[] untouched = {1, 2, 3};
        U.setMemory(untouched, Unsafe.ARRAY_BYTE_BASE_OFFSET, 0, (byte) 9);
        System.out.println("zero bytes: " + Arrays.toString(untouched));

        // Unsafe.setMemory checks its arguments before calling setMemory0
        tryInvalid("negative bytes", bytes, Unsafe.ARRAY_BYTE_BASE_OFFSET, -1);
        tryInvalid("negative offset", bytes, -1, 1);
        tryInvalid("Object[]", new Object[2], Unsafe.ARRAY_OBJECT_BASE_OFFSET, 4);
        tryInvalid("Object[] with zero bytes", new Object[2], Unsafe.ARRAY_OBJECT_BASE_OFFSET, 0);
        tryInvalid("non-array", "text", 12, 1);
    }

    private static void tryInvalid(String label, Object o, long offset, long bytes) {
        try {
            U.setMemory(o, offset, bytes, (byte) 1);
            System.out.println(label + ": no exception");
        } catch (IllegalArgumentException e) {
            System.out.println(label + ": " + e.getClass().getName());
        }
    }
}
