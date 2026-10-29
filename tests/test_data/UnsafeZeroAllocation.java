package samples.jdkinternal.unsafe.zeroallocation;

import jdk.internal.misc.Unsafe;

public class UnsafeZeroAllocation {
    public static void main(String[] args) {
        System.out.println(Unsafe.getUnsafe().allocateMemory(0) == 0);
    }
}
