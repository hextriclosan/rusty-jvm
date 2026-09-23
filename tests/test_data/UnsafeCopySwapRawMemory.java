package samples.jdkinternal.unsafe.copyswapraw;

import jdk.internal.misc.Unsafe;
import java.util.Arrays;

public class UnsafeCopySwapRawMemory {
    private static final Unsafe U = Unsafe.getUnsafe();

    public static void main(String[] args) {
        long source = U.allocateMemory(4);
        long destination = U.allocateMemory(4);
        try {
            for (int index = 0; index < 4; index++) {
                U.putByte(null, source + index, (byte) (index + 1));
            }
            U.copySwapMemory(null, source, null, destination, 4, 2);
            byte[] result = new byte[4];
            for (int index = 0; index < 4; index++) {
                result[index] = U.getByte(null, destination + index);
            }
            System.out.println(Arrays.toString(result));
        } finally {
            U.freeMemory(source);
            U.freeMemory(destination);
        }
    }
}
