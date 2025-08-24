// javac --add-exports java.base/jdk.internal.misc=ALL-UNNAMED -d . UnsafePutReferenceVolatileExample.java
// java --add-exports java.base/jdk.internal.misc=ALL-UNNAMED samples.jdkinternal.unsafe.putreferencevolatile.UnsafePutReferenceVolatileExample
package samples.jdkinternal.unsafe.putreferencevolatile;

import jdk.internal.misc.Unsafe;
import java.util.Arrays;

public class UnsafePutReferenceVolatileExample {
    private static final Unsafe U = Unsafe.getUnsafe();

    public static void main(String[] args) {
        Long[] longs = new Long[3];
        long baseOffset = U.arrayBaseOffset(Long[].class);
        int indexScale = U.arrayIndexScale(Long[].class);

        for (int index = 0; index < longs.length; index++) {
            long offset = baseOffset + indexScale * index;
            U.putReferenceVolatile(longs, offset, Long.valueOf(5_000_000_000L + index));
        }

        System.out.println(Arrays.toString(longs));
    }
}
