package samples.jdkinternal.unsafe.rawchar;

import jdk.internal.misc.Unsafe;

public class UnsafeRawChar {
    private static final Unsafe U = Unsafe.getUnsafe();

    public static void main(String[] args) {
        // Room for every value at an aligned and at an odd, unaligned address
        long address = U.allocateMemory(16);
        try {
            for (long at : new long[] {address, address + 1}) {
                U.putChar(null, at, 'Ж');
                System.out.println("char " + (int) U.getChar(null, at));
                U.putChar(null, at, '￾');
                System.out.println("char " + (int) U.getChar(null, at));
                U.putShort(null, at, (short) -2);
                System.out.println("short " + U.getShort(null, at));
                U.putInt(null, at, 0x89ABCDEF);
                System.out.println("int " + U.getInt(null, at));
                U.putByte(null, at, (byte) -128);
                System.out.println("byte " + U.getByte(null, at));
            }
        } finally {
            U.freeMemory(address);
        }
    }
}
