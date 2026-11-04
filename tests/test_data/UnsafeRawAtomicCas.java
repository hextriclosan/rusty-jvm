package samples.jdkinternal.unsafe.rawatomic;

import jdk.internal.misc.Unsafe;

public class UnsafeRawAtomicCas {
    private static final Unsafe U = Unsafe.getUnsafe();
    private static final int THREADS = 4;
    private static final int INCREMENTS = 2_000;

    public static void main(String[] args) throws InterruptedException {
        // blocks are aligned for all value types, whatever their size
        boolean aligned = true;
        for (long size = 1; size <= 33; size++) {
            long block = U.allocateMemory(size);
            aligned &= block % Long.BYTES == 0;
            U.freeMemory(block);
        }
        System.out.println("allocations aligned: " + aligned);

        long address = U.allocateMemory(16);
        try {
            U.putInt(null, address, 10);
            U.putInt(null, address + 4, 77);
            System.out.println("int set: " + U.compareAndSetInt(null, address + 4, 77, 20));
            System.out.println("int set, stale: " + U.compareAndSetInt(null, address + 4, 77, 30));
            System.out.println("int exchange, stale: " + U.compareAndExchangeInt(null, address + 4, 77, 30));
            System.out.println("int exchange: " + U.compareAndExchangeInt(null, address + 4, 20, 30));
            System.out.println("ints: " + U.getInt(null, address) + " " + U.getInt(null, address + 4));

            U.putLong(null, address + 8, 100L);
            System.out.println("long set: " + U.compareAndSetLong(null, address + 8, 100L, Long.MIN_VALUE));
            System.out.println("long set, stale: " + U.compareAndSetLong(null, address + 8, 100L, 300L));
            System.out.println("long exchange, stale: " + U.compareAndExchangeLong(null, address + 8, 100L, 300L));
            System.out.println("long exchange: " + U.compareAndExchangeLong(null, address + 8, Long.MIN_VALUE, -1L));
            System.out.println("long: " + U.getLong(null, address + 8));

            // getAndAdd and getAndSet are compare-and-set loops in Java
            System.out.println("getAndAddInt: " + U.getAndAddInt(null, address, 5) + " -> " + U.getInt(null, address));
            System.out.println("getAndSetLong: " + U.getAndSetLong(null, address + 8, 42L) + " -> "
                    + U.getLong(null, address + 8));

            // updates from several threads don't get lost
            U.putInt(null, address, 0);
            U.putLong(null, address + 8, 0L);
            Thread[] threads = new Thread[THREADS];
            for (int t = 0; t < THREADS; t++) {
                threads[t] = new Thread(() -> {
                    for (int i = 0; i < INCREMENTS; i++) {
                        U.getAndAddInt(null, address, 1);
                        U.getAndAddLong(null, address + 8, 3L);
                    }
                });
                threads[t].start();
            }
            for (Thread thread : threads) {
                thread.join();
            }
            System.out.println("concurrent int: " + U.getInt(null, address));
            System.out.println("concurrent long: " + U.getLong(null, address + 8));
        } finally {
            U.freeMemory(address);
        }
    }
}
