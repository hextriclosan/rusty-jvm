// javac --add-exports java.base/jdk.internal.misc=ALL-UNNAMED -d . UnsafeGetLongUnalignedExample.java
// java --add-exports java.base/jdk.internal.misc=ALL-UNNAMED samples.jdkinternal.unsafe.getlongunaligned.UnsafeGetLongUnalignedExample
package samples.jdkinternal.unsafe.getlongunaligned;

import jdk.internal.misc.Unsafe;

public class UnsafeGetLongUnalignedExample {
    private static final Unsafe U = Unsafe.getUnsafe();

    public static void main(String[] args) {
        long[] array = new long[] {
            2892188478761536005L, // { 5, 10, 15, 20, 25, 30, 35, 40 } in little-endian
            5785795392284602925L,  // { 45, 50, 55, 60, 65, 70, 75, 80 } in little-endian
        };

        final int offset = U.ARRAY_LONG_BASE_OFFSET;
        for (int i = 0; i < 9; ++i) {
            System.out.println(U.getLongUnaligned(array, offset + i));
        }
//         0: 2892188478761536005 [ 5, 10, 15, 20, 25, 30, 35, 40]
//         1: 3253889342951919370 [10, 15, 20, 25, 30, 35, 40, 45]
//         2: 3615590207142302735 [15, 20, 25, 30, 35, 40, 45, 50]
//         3: 3977291071332686100 [20, 25, 30, 35, 40, 45, 50, 55]
//         4: 4338991935523069465 [25, 30, 35, 40, 45, 50, 55, 60]
//         5: 4700692799713452830 [30, 35, 40, 45, 50, 55, 60, 65]
//         6: 5062393663903836195 [35, 40, 45, 50, 55, 60, 65, 70]
//         7: 5424094528094219560 [40, 45, 50, 55, 60, 65, 70, 75]
//         8: 5785795392284602925 [45, 50, 55, 60, 65, 70, 75, 80]

    }
}
