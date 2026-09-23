package samples.jdkinternal.unsafe.rawatomic;

import jdk.internal.misc.Unsafe;

public class UnsafeRawAtomicCas {
    private static final Unsafe U = Unsafe.getUnsafe();

    public static void main(String[] args) {
        long address = U.allocateMemory(Long.BYTES);
        try {
            U.putLong(null, address, 10L);
            System.out.println("int success=" + U.compareAndSetInt(null, address, 10, 20));
            System.out.println("int witness=" + U.compareAndExchangeInt(null, address, 10, 30));
            U.putLong(null, address, 100L);
            System.out.println("long success=" + U.compareAndSetLong(null, address, 100L, 200L));
            System.out.println("long witness=" + U.compareAndExchangeLong(null, address, 100L, 300L));
        } finally {
            U.freeMemory(address);
        }
    }
}
