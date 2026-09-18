package samples.jdkinternal.unsafe.rawchar;

import jdk.internal.misc.Unsafe;

public class UnsafeRawChar {
    private static final Unsafe U = Unsafe.getUnsafe();

    public static void main(String[] args) {
        long address = U.allocateMemory(Character.BYTES);
        try {
            U.putChar(null, address, 'Ж');
            System.out.println(U.getChar(null, address));
        } finally {
            U.freeMemory(address);
        }
    }
}
