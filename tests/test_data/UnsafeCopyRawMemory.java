package samples.jdkinternal.unsafe.copyraw;

import jdk.internal.misc.Unsafe;
import java.util.Arrays;

public class UnsafeCopyRawMemory {
    private static final Unsafe U = Unsafe.getUnsafe();
    private static final long BYTES = Unsafe.ARRAY_BYTE_BASE_OFFSET;

    public static void main(String[] args) {
        long source = U.allocateMemory(Long.BYTES);
        long destination = U.allocateMemory(Long.BYTES);
        long memory = U.allocateMemory(8);
        try {
            U.putLong(null, source, 0x123456789ABCDEFL);
            U.copyMemory(null, source, null, destination, Long.BYTES);
            System.out.printf("native to native: %x%n", U.getLong(null, destination));

            // overlapping ranges are copied as if through a temporary buffer
            fill(memory);
            U.copyMemory(null, memory, null, memory + 2, 5);
            System.out.println("overlap forward: " + read(memory));
            fill(memory);
            U.copyMemory(null, memory + 2, null, memory, 5);
            System.out.println("overlap backward: " + read(memory));

            byte[] array = {10, 11, 12, 13, 14, 15, 16, 17};
            fill(memory);
            U.copyMemory(array, BYTES + 1, null, memory + 3, 4);
            System.out.println("array to native: " + read(memory));

            byte[] target = new byte[8];
            U.copyMemory(null, memory + 2, target, BYTES + 1, 5);
            System.out.println("native to array: " + Arrays.toString(target));

            // multi-byte elements depend on the platform's byte order
            int[] ints = new int[2];
            U.copyMemory(array, BYTES, ints, Unsafe.ARRAY_INT_BASE_OFFSET, 8);
            System.out.println("array to array: " + Arrays.toString(ints));

            byte[] same = {1, 2, 3, 4, 5, 6, 7, 8};
            U.copyMemory(same, BYTES, same, BYTES + 3, 5);
            System.out.println("same array: " + Arrays.toString(same));

            // a zero-byte copy returns before touching memory
            U.copyMemory(null, 0, null, 0, 0);
            System.out.println("zero bytes: done");
        } finally {
            U.freeMemory(source);
            U.freeMemory(destination);
            U.freeMemory(memory);
        }

        // Unsafe.copyMemory checks its arguments before calling copyMemory0
        byte[] bytes = new byte[4];
        tryInvalid("negative bytes", bytes, BYTES, bytes, BYTES, -1);
        tryInvalid("negative source offset", bytes, -1, bytes, BYTES, 1);
        tryInvalid("negative destination offset", bytes, BYTES, bytes, -1, 1);
        tryInvalid("Object[] source", new Object[2], Unsafe.ARRAY_OBJECT_BASE_OFFSET, bytes, BYTES, 1);
        tryInvalid("non-array destination", bytes, BYTES, "text", 12, 1);
    }

    private static void fill(long address) {
        for (int i = 0; i < 8; i++) {
            U.putByte(null, address + i, (byte) i);
        }
    }

    private static String read(long address) {
        byte[] result = new byte[8];
        for (int i = 0; i < 8; i++) {
            result[i] = U.getByte(null, address + i);
        }
        return Arrays.toString(result);
    }

    private static void tryInvalid(String label, Object src, long srcOffset, Object dest, long destOffset, long bytes) {
        try {
            U.copyMemory(src, srcOffset, dest, destOffset, bytes);
            System.out.println(label + ": no exception");
        } catch (IllegalArgumentException e) {
            System.out.println(label + ": " + e.getClass().getName());
        }
    }
}
