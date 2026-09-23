package samples.jdkinternal.unsafe.setmemory;

import jdk.internal.misc.Unsafe;
import java.util.Arrays;

public class UnsafeSetMemoryArray {
    private static final Unsafe U = Unsafe.getUnsafe();

    public static void main(String[] args) {
        byte[] bytes = new byte[6];
        U.setMemory(bytes, 1, 3, (byte) 7);
        System.out.println(Arrays.toString(bytes));
    }
}
