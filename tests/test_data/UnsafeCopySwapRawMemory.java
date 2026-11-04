package samples.jdkinternal.unsafe.copyswapraw;

import jdk.internal.misc.Unsafe;
import java.util.Arrays;

public class UnsafeCopySwapRawMemory {
    private static final Unsafe U = Unsafe.getUnsafe();
    private static final long BYTES = Unsafe.ARRAY_BYTE_BASE_OFFSET;

    public static void main(String[] args) {
        long source = U.allocateMemory(16);
        long destination = U.allocateMemory(16);
        try {
            for (long elemSize : new long[] {2, 4, 8}) {
                fill(source);
                fill(destination);
                U.copySwapMemory(null, source, null, destination, 16, elemSize);
                System.out.println("native to native, " + elemSize + ": " + read(destination));
            }

            // overlapping ranges are swapped as if copied through a temporary buffer
            fill(source);
            U.copySwapMemory(null, source, null, source + 2, 8, 4);
            System.out.println("overlap forward: " + read(source));
            fill(source);
            U.copySwapMemory(null, source + 2, null, source, 8, 4);
            System.out.println("overlap backward: " + read(source));

            byte[] array = new byte[16];
            for (int i = 0; i < array.length; i++) {
                array[i] = (byte) (20 + i);
            }
            fill(destination);
            U.copySwapMemory(array, BYTES + 1, null, destination + 3, 8, 2);
            System.out.println("array to native: " + read(destination));

            fill(source);
            byte[] target = new byte[16];
            U.copySwapMemory(null, source + 1, target, BYTES + 2, 8, 8);
            System.out.println("native to array: " + Arrays.toString(target));

            // a copy between element types; the values depend on the platform's byte order
            int[] ints = new int[2];
            U.copySwapMemory(array, BYTES, ints, Unsafe.ARRAY_INT_BASE_OFFSET, 8, 4);
            System.out.println("array to array: " + Arrays.toString(ints));

            long[] longs = {0x0102030405060708L, 0x1112131415161718L};
            U.copySwapMemory(longs, Unsafe.ARRAY_LONG_BASE_OFFSET, longs, Unsafe.ARRAY_LONG_BASE_OFFSET, 16, 8);
            System.out.printf("in place: %x %x%n", longs[0], longs[1]);

            // a zero-byte copy returns before touching memory
            U.copySwapMemory(null, 0, null, 0, 0, 2);
            System.out.println("zero bytes: done");
        } finally {
            U.freeMemory(source);
            U.freeMemory(destination);
        }

        // Unsafe.copySwapMemory checks its arguments before calling copySwapMemory0
        byte[] bytes = new byte[8];
        tryInvalid("negative bytes", bytes, BYTES, bytes, BYTES, -2, 2);
        tryInvalid("element size 1", bytes, BYTES, bytes, BYTES, 2, 1);
        tryInvalid("element size 3", bytes, BYTES, bytes, BYTES, 6, 3);
        tryInvalid("element size 16", bytes, BYTES, bytes, BYTES, 16, 16);
        tryInvalid("bytes not a multiple of element size", bytes, BYTES, bytes, BYTES, 6, 4);
        tryInvalid("negative source offset", bytes, -1, bytes, BYTES, 2, 2);
        tryInvalid("negative destination offset", bytes, BYTES, bytes, -1, 2, 2);
        tryInvalid("Object[] source", new Object[2], Unsafe.ARRAY_OBJECT_BASE_OFFSET, bytes, BYTES, 4, 2);
        tryInvalid("non-array destination", bytes, BYTES, "text", 12, 2, 2);
    }

    private static void fill(long address) {
        for (int i = 0; i < 16; i++) {
            U.putByte(null, address + i, (byte) i);
        }
    }

    private static String read(long address) {
        byte[] result = new byte[16];
        for (int i = 0; i < 16; i++) {
            result[i] = U.getByte(null, address + i);
        }
        return Arrays.toString(result);
    }

    private static void tryInvalid(String label, Object src, long srcOffset, Object dest, long destOffset, long bytes,
            long elemSize) {
        try {
            U.copySwapMemory(src, srcOffset, dest, destOffset, bytes, elemSize);
            System.out.println(label + ": no exception");
        } catch (IllegalArgumentException e) {
            System.out.println(label + ": " + e.getClass().getName());
        }
    }
}
