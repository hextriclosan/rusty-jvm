package samples.jdkinternal.unsafe.copyraw;

import jdk.internal.misc.Unsafe;

public class UnsafeCopyRawMemory {
    private static final Unsafe U = Unsafe.getUnsafe();

    public static void main(String[] args) {
        long source = U.allocateMemory(Long.BYTES);
        long destination = U.allocateMemory(Long.BYTES);
        try {
            U.putLong(null, source, 0x123456789ABCDEFL);
            U.copyMemory(null, source, null, destination, Long.BYTES);
            System.out.printf("%x%n", U.getLong(null, destination));
        } finally {
            U.freeMemory(source);
            U.freeMemory(destination);
        }
    }
}
